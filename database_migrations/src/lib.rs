use anyhow::Context;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

pub async fn get_database_client() -> anyhow::Result<Client> {
    let url = std::env::var("DATABASE_URL").context("Failed to get DATABASE_URL from env")?;

    let tls_connector = TlsConnector::builder().build()?;
    let tls_connector = MakeTlsConnector::new(tls_connector);

    let (client, connection) = tokio_postgres::connect(&url, tls_connector).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}
