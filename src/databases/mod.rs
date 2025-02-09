pub mod mssql;

use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

use crate::entity_generator::SchemaInfo;

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
    async fn read_schema_info(&mut self) -> Result<Vec<SchemaInfo>, DatabaseError>;
    fn set_client(&mut self, client: Client<Compat<TcpStream>>);
    fn set_table_name(&mut self, name: String);
    fn get_query(&self) -> String;
}