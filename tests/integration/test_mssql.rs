#[cfg(test)]
mod tests {
    use crate::connect_client;
    use emgen::databases::Database;
    use emgen::databases::mssql::Mssql;


    #[tokio::test]
    async fn test_read_schema_info() {
        let client = connect_client()
            .await
            .expect("Failed to connect client");
        let mut mssql: Box<dyn Database> = Box::new(Mssql::default());
        mssql.set_client(client);
        mssql.set_table_name("entity".to_string());
        let schema_info = mssql.read_schema_info()
            .await
            .expect("Failed to read schema info");

        assert!(!schema_info.is_empty(), "Schema Info should not be empty");
    }
}