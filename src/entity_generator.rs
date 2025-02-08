use prettyplease;
use tiberius::{Client, QueryItem};
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use convert_case::{Case, Casing};

type ColumnInfo = (String, String, String);

#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
}

pub async fn information_schema(mut client: Client<tokio_util::compat::Compat<TcpStream>>, table_name: &str) -> Vec<SchemaInfo> {
    let query = format!(
        "
        SELECT 
            COLUMNS.TABLE_NAME,
            COLUMNS.COLUMN_NAME,
            COLUMNS.DATA_TYPE,
            COLUMNS.IS_NULLABLE
        FROM 
            INFORMATION_SCHEMA.COLUMNS
        JOIN
            INFORMATION_SCHEMA.TABLES ON 
            TABLES.TABLE_NAME = COLUMNS.TABLE_NAME 
        WHERE 
            TABLES.TABLE_TYPE = 'BASE TABLE' AND
            TABLES.TABLE_NAME = '{}'", table_name
    );
    let mut stream = client.simple_query(&query).await.unwrap();
    let mut schema: Vec<SchemaInfo> = Vec::new();
    
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            QueryItem::Row(row) => {
                let table_name: &str = row.get(0).unwrap();
                let column_name: &str = row.get(1).unwrap();
                let data_type: &str = row.get(2).unwrap();
                let is_nullable: &str = row.get(3).unwrap();
                schema.push(SchemaInfo {
                    table_name: table_name.to_string(),
                    column_name: column_name.to_string(),
                    data_type: data_type.to_string(),
                    is_nullable: is_nullable.to_string(),
                });
            },
            _ => {},
        }
    }
    schema
}


pub fn generate_schema(schema: Vec<SchemaInfo>) -> String {
    let mut entities = String::new();
    let mut entity_name = String::new();
    let mut columns: Vec<ColumnInfo> = Vec::new();

    for row in schema {
        println!("Entity name: {}, column count: {}", row.table_name, columns.len());
        if row.table_name != entity_name && !columns.is_empty() {
            println!("Processing table: {}, column: {}", row.table_name, row.column_name);
            entities.push_str(&generate_entity(columns.clone(), &entity_name));
            columns.clear();
        }
        entity_name = row.table_name;
        columns.push((row.column_name, row.data_type, row.is_nullable));
    }
    entities.push_str(&generate_entity(columns.clone(), &entity_name));
    entities
}

pub fn generate_entity(columns: Vec<ColumnInfo>, table_name: &str) -> String {
    let mut entity_string = String::new();
    entity_string.push_str("#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]\n");
    entity_string.push_str(&format!("#[sea_orm(table_name = \"{}\")]\n", table_name));
    entity_string.push_str("pub struct Model {\n");

    for (column_name, data_type, is_nullable) in columns {
        let rust_type = map_sql_type_to_rust(&data_type, is_nullable == "YES");
        if column_name == "id" {
            entity_string.push_str("#[sea_orm(primary_key)]\n");
        }
        entity_string.push_str(&format!("pub {}: {},\n", column_name.to_case(Case::Snake), rust_type));
    }

    entity_string.push_str("}\n");

    prettyplease::unparse(&syn::parse_file(&entity_string).unwrap())
}

pub fn map_sql_type_to_rust(sql_type: &str, is_nullable: bool) -> String {
    let mut rust_type = match sql_type {
        "int" => "i32".to_string(),
        "bigint" => "i64".to_string(),
        "varchar" | "nvarchar" | "text" | "ntext" => "String".to_string(),
        "datetime" | "smalldatetime" => "chrono::DateTime<chrono::Utc>".to_string(),
        "bit" => "bool".to_string(),
        "decimal" | "numeric" => "f64".to_string(),
        "uniqueidentifier" => "uuid::Uuid".to_string(),
        _ => {
            eprintln!("Warning: Unhandled SQL type: {}", sql_type);
            "sqlx::types::BigDecimal".to_string()
        }
    };

    if is_nullable {
        rust_type = format!("Option<{}>", rust_type);
    }

    rust_type
}
