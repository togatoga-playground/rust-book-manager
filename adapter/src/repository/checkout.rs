use crate::database::{model::checkout::CheckoutStateRow, ConnectionPool};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        checkout::{
            event::{CreateCheckout, UpdateReturned},
            Checkout,
        },
        id::{BookId, CheckoutId, UserId},
    },
    repository::checkout::CheckoutRepository,
};
use shared::error::{AppError, AppResult};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        let mut tx = self.db.begin().await?;
        self.set_transaction_serializable(&mut tx).await?;
        let res = sqlx::query_as!(
            CheckoutStateRow,
            r#"
                SELECT
                    b.book_id,
                    c.checkout_id AS "checkout_id?: CheckoutId",
                    NULL AS "user_id?: UserId"
                FROM books as b
                LEFT OUTER JOIN checkouts AS c USING(book_id)
                WHERE b.book_id = $1
            "#,
            event.book_id as _
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        match res {
            None => {
                return Err(AppError::EntityNotFound(format!(
                    "The book ({}) was not found.",
                    event.book_id
                )));
            }
            Some(CheckoutStateRow {
                checkout_id: Some(_),
                ..
            }) => {
                return Err(AppError::UnprocessableEntity(format!(
                    "The book ({}) is already checked out.",
                    event.book_id
                )));
            }
            _ => {}
        }

        let checkout_id = CheckoutId::new();
        let res = sqlx::query!(
            r#"
            INSERT INTO checkouts (checkout_id, book_id, user_id, checked_out_at)
            VALUES ($1, $2, $3, $4)
        "#,
            checkout_id as _,
            event.book_id as _,
            event.checked_out_by as _,
            event.checked_out_at
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "No checkout has been created".into(),
            ));
        }
        tx.commit().await.map_err(AppError::TransactionError)?;
        Ok(())
    }

    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        let mut tx = self.db.begin().await?;
        self.set_transaction_serializable(&mut tx).await?;
        let res = sqlx::query_as!(
            CheckoutStateRow,
            r#"
                SELECT
                    b.book_id,
                    c.checkout_id AS "checkout_id?: CheckoutId",
                    c.user_id AS "user_id?: UserId"
                FROM books as b
                LEFT OUTER JOIN checkouts AS c USING(book_id)
                WHERE b.book_id = $1
            "#,
            event.book_id as _
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        match res {
            None => {
                return Err(AppError::EntityNotFound(format!(
                    "The book ({}) was not found.",
                    event.book_id
                )));
            }
            Some(CheckoutStateRow {
                checkout_id: Some(c),
                user_id: Some(u),
                ..
            }) if (c, u) != (event.checkout_id, event.returned_by) => {
                return Err(AppError::UnprocessableEntity(format!(
                    "The specified checkout (ID({}), User({}), Book({})) cannot be returned.",
                    event.checkout_id, event.returned_by, event.book_id
                )));
            }
            _ => {}
        }

        let res = sqlx::query!(
            r#"
                INSERT INTO returned_checkouts
                (checkout_id, book_id, user_id, checked_out_at, returned_at)
                SELECT checkout_id, book_id, user_id, checked_out_at, $1
                FROM checkouts
                WHERE checkout_id = $2;
            "#,
            event.returned_at,
            event.checkout_id as _
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "No returning record has been created".into(),
            ));
        }

        let res = sqlx::query!(
            r#"
                DELETE FROM checkouts
                WHERE checkout_id = $1;
            "#,
            event.checkout_id as _
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "No checkout record has been deleted".into(),
            ));
        }

        tx.commit().await.map_err(AppError::TransactionError)?;

        todo!()
    }

    async fn find_returned_all(&self) -> AppResult<Vec<Checkout>> {
        todo!()
    }
    async fn find_unreturned_all_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        todo!()
    }
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        todo!()
    }
}

impl CheckoutRepositoryImpl {
    async fn set_transaction_serializable(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> AppResult<()> {
        sqlx::query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut **tx)
            .await
            .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }
}
