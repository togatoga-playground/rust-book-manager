use axum::{
    routing::{delete, put},
    Router,
};
use registry::AppRegistryImpl;

use crate::handler::user::{
    change_password, change_role, delete_user, get_checkouts, get_current_user, list_users,
    register_user,
};
use axum::routing::get;

pub fn build_user_router() -> Router<AppRegistryImpl> {
    Router::new()
        .route("/users/me", get(get_current_user))
        .route("/users/me/password", put(change_password))
        .route("/users/me/checkouts", get(get_checkouts))
        .route("/users", get(list_users).post(register_user))
        .route("/users/:user_id", delete(delete_user))
        .route("/users/:user_id/role", put(change_role))
}
