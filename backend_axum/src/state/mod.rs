use std::sync::Arc;

use reqwest::Client;

use crate::{
    database::{DatabaseConnectionRef, DatabasePool, create_pool},
    error::AppError,
    service::auth::AuthService,
};

pub struct AppStateInner {
    pub auth_service: AuthService,
    pub database_pool: DatabasePool,
}

pub type AppState = Arc<AppStateInner>;

impl AppStateInner {
    pub async fn new() -> Result<Self, AppError> {
        let database_pool = create_pool().await?;
        let http_client = Client::new();
        let auth_service = AuthService::new(&http_client).await?;

        Ok(Self {
            auth_service,
            database_pool,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_database_connection(&self) -> Result<DatabaseConnectionRef<'_>, AppError> {
        let conn = self.database_pool.get().await?;
        Ok(conn)
    }
}
