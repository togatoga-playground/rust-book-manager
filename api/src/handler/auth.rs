use axum::{extract::State, http::StatusCode, Json};
use kernel::model::auth::event::CreateToken;
use registry::AppRegistryImpl;
use shared::error::AppResult;

use crate::{
    extractor::AuthorizedUser,
    model::auth::{AccessTokenResponse, LoginRequest},
};

pub async fn login(
    State(registry): State<AppRegistryImpl>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let user_id = registry
        .auth_repository()
        .verify_user(&req.email, &req.password)
        .await?;
    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;
    Ok(Json(AccessTokenResponse {
        user_id,
        access_token: access_token.0,
    }))
}

pub async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistryImpl>,
) -> AppResult<StatusCode> {
    registry
        .auth_repository()
        .delete_token(&user.access_token)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
