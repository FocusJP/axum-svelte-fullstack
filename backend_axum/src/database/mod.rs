pub mod ssa;

use std::time::Duration;

use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;

use crate::error::AppError;

pub type DatabasePool = Pool<PostgresConnectionManager<MakeTlsConnector>>;
pub type DatabaseConnectionRef<'a> =
    PooledConnection<'a, PostgresConnectionManager<MakeTlsConnector>>;

#[tracing::instrument]
pub async fn create_pool() -> Result<DatabasePool, AppError> {
    let url = std::env::var("DATABASE_URL")?;

    let tls_connector = TlsConnector::builder().build()?;
    let tls_connector = MakeTlsConnector::new(tls_connector);

    let manager = PostgresConnectionManager::new_from_stringlike(url, tls_connector)?;

    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(30))
        .build(manager)
        .await?;

    Ok(pool)
}
