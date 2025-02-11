pub mod mssql;

use tiberius::Client;
use tiberius::Config;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Tiberius error: {0}")]
    Tiberius(#[from] tiberius::error::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other database error: {0}")]
    Other(String),
}

#[async_trait::async_trait]
pub trait Database {
    async fn connect(&mut self, config: Config);
    async fn query_schema_info(&mut self);
    fn map_sql_type_to_rust(&self, sql_type: &str, is_nullable: bool) -> String;
    fn set_client(&mut self, client: Client<Compat<TcpStream>>);
    fn set_table_name(&mut self, name: String);
    fn set_schema_info(&mut self, schema_info: Vec<SchemaInfo>);
    fn get_query(&self) -> String;
    fn get_table_name(&self) -> String;
    fn get_schema_info(&self) -> Vec<SchemaInfo>;
}