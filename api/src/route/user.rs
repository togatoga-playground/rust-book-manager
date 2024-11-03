use derive_new::new;
use kernel::model::role::Role;
use serde::{Deserialize, Serialize};
use strum::VariantNames;

#[derive(Serialize, Deserialize, VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub enum RoleName {
    Admin,
    User,
}

impl From<Role> for RoleName {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => RoleName::Admin,
            Role::User => RoleName::User,
        }
    }
}

impl From<RoleName> for Role {
    fn from(role: RoleName) -> Self {
        match role {
            RoleName::Admin => Role::Admin,
            RoleName::User => Role::User,
        }
    }
}

pub struct UserResponse {}
