use crate::handler::health::{health_check, health_check_db};
use axum::Router;
use registry::AppRegistryImpl;

pub fn build_health_check_routers() -> Router<AppRegistryImpl> {
    let routers = Router::new()
        .route("/", axum::routing::get(health_check))
        .route("/db", axum::routing::get(health_check_db));
    Router::new().nest("/health", routers)
}
