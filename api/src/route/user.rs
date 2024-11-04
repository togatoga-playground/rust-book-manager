use axum::{
    routing::{delete, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{change_password, delete_user, get_current_user, register_user};
use axum::routing::get;

pub fn build_user_router() -> Router<AppRegistry> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/med/password", put(change_password))
        .route("/users", post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/role", put(change_role))
}
