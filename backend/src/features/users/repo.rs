use crate::features::users::model::*;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all_users(pool: &PgPool) -> sqlx::Result<Vec<UserDAO>> {
    sqlx::query_as!(
        UserDAO,
        r#"
        SELECT id, first_name, last_name, headline, role AS "role: _"
        FROM users
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<UserDAO>> {
    sqlx::query_as!(
        UserDAO,
        r#"
        SELECT id, first_name, last_name, headline, role AS "role: _"
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
