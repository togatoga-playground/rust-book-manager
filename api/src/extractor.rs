use axum::extract::FromRequestParts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use kernel::model::role::Role;
use kernel::model::{auth::AccessToken, id::UserId, user::User};
use registry::AppRegistryImpl;
use shared::error::AppError;

pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }
    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistryImpl> for AuthorizedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        registry: &AppRegistryImpl,
    ) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError)?;
        let access_token = AccessToken(bearer.token().to_string());

        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        let user = registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        Ok(Self { access_token, user })
    }
}
