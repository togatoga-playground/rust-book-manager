use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Error, Result};
use api::route::{auth, v1};
use axum::{http::Method, Router};
use registry::AppRegistryImpl;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use tower_http::cors::{self, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;
    Ok(())
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    let registry = Arc::new(AppRegistryImpl::new(pool, kv, app_config));
    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        .layer(cors())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
