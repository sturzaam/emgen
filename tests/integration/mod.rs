pub mod test_generator;
pub mod test_mssql;

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio_util::compat::Compat;
use tracing::log::info;

pub async fn tcp_stream() -> Result<TcpStream, Box<dyn std::error::Error>> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
    config.trust_cert();
    
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    Ok(tcp)
}

pub async fn connect_client() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "<YourStrong@Passw0rd>"));
    config.trust_cert();

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