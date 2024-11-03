use crate::database::model::user::UserRow;
use crate::database::ConnectionPool;
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        id::UserId,
        user::{
            event::{CreateUser, DeleteUser, UpdateUserPassword},
            User,
        },
    },
    repository::user::UserRepository,
};
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, current_user_id: UserId) -> AppResult<Option<User>> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                u.user_id,
                u.name,
                u.email,
                r.name AS role_name,
                u.created_at,
                u.updated_at
            FROM
                users AS u
            INNER JOIN roles AS r USING(role_id)
            WHERE
                u.user_id = $1
            "#,
            current_user_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(row) => Ok(Some(User::try_from(row)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> AppResult<Vec<User>> {
        unimplemented!()
    }

    async fn create(&self, event: CreateUser) -> AppResult<User> {
        unimplemented!()
    }

    async fn update_password(&self, event: UpdateUserPassword) -> AppResult<()> {
        unimplemented!()
    }

    async fn update_role(&self, user_id: UserId, role: String) -> AppResult<()> {
        unimplemented!()
    }

    async fn delete(&self, event: DeleteUser) -> AppResult<()> {
        unimplemented!()
    }
}
