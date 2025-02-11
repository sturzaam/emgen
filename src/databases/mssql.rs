use tokio::net::TcpStream;
use tiberius::{Client, Config, QueryItem, Row};
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use futures::stream::StreamExt;


use super::Database;
use super::DatabaseError;
use super::SchemaInfo;

pub struct Mssql {
    client: Option<Client<Compat<TcpStream>>>,
    table_name: Option<String>,
    schema_info: Option<Vec<SchemaInfo>>,
}

impl Default for Mssql {
    fn default() -> Self {
        Self {
            client: None,
            table_name: None,
            schema_info: None
        }
    }
}

#[async_trait::async_trait]
impl Database for Mssql {

    async fn connect(&mut self, config: Config) {
        let tcp = TcpStream::connect(config.get_addr())
            .await
            .expect("Failed to connect to tcp stream");
        let _ = tcp.set_nodelay(true);
        let client = Client::connect(config, tcp.compat_write())
            .await
            .expect("Failed to connect to client");
        self.set_client(client);
    }

    fn set_client(&mut self, client: Client<Compat<TcpStream>>) {
        self.client = Some(client);
    }

    fn set_table_name(&mut self, name: String) {
        self.table_name = Some(name);
    }

    fn get_table_name(&self) -> String {
        self.table_name.clone().expect("Table name not set")
    }

    fn get_query(&self) -> String {
        format!("
            SELECT COLUMNS.TABLE_NAME, COLUMNS.COLUMN_NAME, COLUMNS.DATA_TYPE, COLUMNS.IS_NULLABLE
            FROM INFORMATION_SCHEMA.COLUMNS
            JOIN INFORMATION_SCHEMA.TABLES ON TABLES.TABLE_NAME = COLUMNS.TABLE_NAME
            WHERE TABLES.TABLE_TYPE = 'BASE TABLE' AND TABLES.TABLE_NAME = '{}'",
            self.get_table_name()
        )
    }

    fn set_schema_info(&mut self, schema_info: Vec<SchemaInfo>) {
        self.schema_info = Some(schema_info);
    }

    fn get_schema_info(&self) -> Vec<SchemaInfo> {
        self.schema_info.clone().expect("Schema info not set")
    }

    async fn query_schema_info(&mut self) {
        let query = self.get_query();
        let mut stream = self.client
            .as_mut()
            .expect("Failed to connect to database")
            .simple_query(&query)
            .await
            .unwrap();
        let mut schema: Vec<SchemaInfo> = Vec::new();
        while let Some(item) = stream.next().await {
            match item {
                Ok(QueryItem::Row(row)) => {
                    schema.push(
                        parse_schema_info(&row)
                        .expect("Failed to parse schema info")
                    );
                }
                _ => {},
            }
        }
        drop(stream);
        self.set_schema_info(schema);
    }

    fn map_sql_type_to_rust(&self, sql_type: &str, is_nullable: bool) -> String {
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
}

fn parse_schema_info(row: &Row) -> Result<SchemaInfo, DatabaseError> {
    let table_name: &str = row.get(0).ok_or(DatabaseError::Other("Missing table_name".to_string()))?;
    let column_name: &str = row.get(1).ok_or(DatabaseError::Other("Missing column_name".to_string()))?;
    let data_type: &str = row.get(2).ok_or(DatabaseError::Other("Missing data_type".to_string()))?;
    let is_nullable: &str = row.get(3).ok_or(DatabaseError::Other("Missing is_nullable".to_string()))?;

    Ok(SchemaInfo {
        table_name: table_name.to_string(),
        column_name: column_name.to_string(),
        data_type: data_type.to_string(),
        is_nullable: is_nullable.to_string(),
    })
}