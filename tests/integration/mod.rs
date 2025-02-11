pub mod test_generator;
pub mod test_mssql;

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio_util::compat::Compat;
use tracing::log::info;

const ENTITY: &str = stringify! {
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "entity")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub name: String,
        pub active: bool,
        pub belongs_to_id: i32,
    }
};

pub async fn tiberius_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
    config.trust_cert();
    Ok(config)
}

pub async fn ephemeral_client() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    let config = tiberius_config()
        .await
        .expect("Failed to configure client");

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    client
        .execute("DROP TABLE IF EXISTS entity", &[])
        .await?;
    info!("drop table");
    client
        .execute(r#"
            CREATE TABLE entity (
                id INT PRIMARY KEY IDENTITY(1,1),
                name NVARCHAR(255) NOT NULL,
                active BIT NOT NULL,
                belongs_to_id INT
            )"#,
            &[],
        )
        .await?;
    info!("create table done");
    Ok(client)
}