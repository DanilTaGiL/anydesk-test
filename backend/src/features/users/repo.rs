use crate::app_state::AppState;
use crate::features::users::model::*;
use async_trait::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UsersRepo: Send + Sync {
    async fn get_all(&self) -> sqlx::Result<Vec<UserDAO>>;
    async fn get_by_id(&self, id: Uuid) -> sqlx::Result<Option<UserDAO>>;
}

#[async_trait]
impl UsersRepo for AppState {
    async fn get_all(&self) -> sqlx::Result<Vec<UserDAO>> {
        sqlx::query_as!(
            UserDAO,
            r#"
              SELECT id, first_name, last_name, headline, role AS "role: _"
              FROM users
            "#
        )
            .fetch_all(&self.db)
            .await
    }

    async fn get_by_id(&self, id: Uuid) -> sqlx::Result<Option<UserDAO>> {
        sqlx::query_as!(
            UserDAO,
            r#"
              SELECT id, first_name, last_name, headline, role AS "role: _"
              FROM users
              WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.db)
            .await
    }
}
