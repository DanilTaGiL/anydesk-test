use crate::features::users::model::*;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_all(pool: &PgPool) -> anyhow::Result<Vec<UserDAO>> {
    let rows = sqlx::query_as!(
        UserDAO,
        r#"
        SELECT id, first_name, last_name, headline, role AS "role: _"
        FROM users
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<UserDAO>> {
    let row = sqlx::query_as!(
        UserDAO,
        r#"
        SELECT id, first_name, last_name, headline, role AS "role: _"
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
