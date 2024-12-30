use sqlx::{postgres::PgConnectOptions, PgPool};
use shared::{
    config::DatabaseConfig,
    error::{AppError, AppResult},
}

pub mod model;

fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }

    pub async fn begin(
        &self,
    ) -> AppResult<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.0.begin().await.map_err(AppError::TransactionError)
    }
}

pub fn connection_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}