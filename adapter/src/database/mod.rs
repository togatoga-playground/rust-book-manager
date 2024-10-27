use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(make_pg_connect_options(&cfg))
}