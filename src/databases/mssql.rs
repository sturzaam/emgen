use tokio::net::TcpStream;
use tiberius::{Client, QueryItem, Row};
use tokio_util::compat::Compat;
use futures::stream::StreamExt;


use super::Database;
use super::DatabaseError;
use crate::entity_generator::SchemaInfo;

#[derive(Default)]
pub struct Mssql {
    client: Option<Client<Compat<TcpStream>>>,
    table_name: Option<String>,
}

#[async_trait::async_trait]
impl Database for Mssql {

    async fn read_schema_info(&mut self) -> Result<Vec<SchemaInfo>, DatabaseError> {
        let query = self.get_query();
        let mut stream = self.client
            .as_mut()
            .expect("Failed to connect to database")
            .simple_query(&query)
            .await
            .unwrap();
        let mut schema: Vec<SchemaInfo> = Vec::new();
        while let Some(item) = stream.next().await {
            match item? {
                QueryItem::Row(row) => {
                    schema.push(parse_schema_info(&row)?);
                }
                _ => {},
            }
        }
        Ok(schema.into())
    }

    fn set_client(&mut self, client: Client<Compat<TcpStream>>) {
        self.client = Some(client);
    }

    fn set_table_name(&mut self, name: String) {
        self.table_name = Some(name);
    }

    fn get_query(&self) -> String {
        format!("
            SELECT COLUMNS.TABLE_NAME, COLUMNS.COLUMN_NAME, COLUMNS.DATA_TYPE, COLUMNS.IS_NULLABLE
            FROM INFORMATION_SCHEMA.COLUMNS
            JOIN INFORMATION_SCHEMA.TABLES ON TABLES.TABLE_NAME = COLUMNS.TABLE_NAME
            WHERE TABLES.TABLE_TYPE = 'BASE TABLE' AND TABLES.TABLE_NAME = '{}'",
            self.table_name.clone().expect("Table name not set")
        )
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