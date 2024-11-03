use axum::Router;
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let auth_router = Router::new()
        .route("/login", axum::routing::post(crate::handler::auth::login))
        .route("/logout", axum::routing::post(crate::handler::auth::logout));
    Router::new().nest("/auth", auth_router)
}
