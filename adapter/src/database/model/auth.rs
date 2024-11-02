use std::str::FromStr;

use kernel::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};
use shared::error::{AppError, AppResult};

use crate::redis::model::{RedisKey, RedisValue};

pub struct UserItem {
    pub user_id: UserId,
    pub password_hash: String,
}

pub struct AuthorizationKey(String);
pub struct AuhtorizedUserId(UserId);

pub fn from(event: CreateToken) -> (AuthorizationKey, AuhtorizedUserId) {
    (
        AuthorizationKey(event.access_token),
        AuhtorizedUserId(event.user_id),
    )
}

impl From<AuthorizationKey> for AccessToken {
    fn from(value: AuthorizationKey) -> Self {
        Self(value.0)
    }
}

impl From<AccessToken> for AuthorizationKey {
    fn from(value: AccessToken) -> Self {
        Self(value.0)
    }
}

impl From<&AccessToken> for AuthorizationKey {
    fn from(value: &AccessToken) -> Self {
        Self(value.0.clone())
    }
}

impl RedisKey for AuthorizationKey {
    type Value = AuhtorizedUserId;
    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl RedisValue for AuhtorizedUserId {
    fn inner(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for AuhtorizedUserId {
    type Error = AppError;
    fn try_from(s: String) -> AppResult<Self> {
        Ok(Self(UserId::from_str(&s).map_err(|e| {
            AppError::ConversionEntityError(e.to_string())
        })?))
    }
}

impl AuhtorizedUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}
