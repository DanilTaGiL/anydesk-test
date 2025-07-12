use crate::api_error::ApiError;
use crate::features::tasks::model::*;
use sqlx::{PgPool, query_as};

pub async fn get_all_tasks(pool: &PgPool) -> sqlx::Result<Vec<TaskDAO>> {
    query_as!(
        TaskDAO,
        r#"
        SELECT id, title, description, creator_id, assigned_to, category AS "category: _"
        FROM tasks
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_task_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<TaskDAO>> {
    query_as!(
        TaskDAO,
        r#"
        SELECT id, title, description, creator_id, assigned_to, category AS "category: _"
        FROM tasks
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_task(pool: &PgPool, dto: TaskCreateDTO) -> sqlx::Result<TaskDAO, ApiError> {
    query_as!(
        TaskDAO,
        r#"
        INSERT INTO tasks (title, category, description, creator_id, assigned_to)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, category AS "category: _", description, creator_id, assigned_to
        "#,
        dto.title,
        dto.category as TaskCategory,
        dto.description,
        dto.creator_id,
        dto.assigned_to,
    )
    .fetch_one(pool)
    .await
    .map_err(ApiError::from)
}

pub async fn update_task(
    pool: &PgPool,
    id: i32,
    dto: TaskUpdateDTO,
) -> sqlx::Result<Option<TaskDAO>, ApiError> {
    // Use COALESCE to keep old value when field is None
    query_as!(
        TaskDAO,
        r#"
        UPDATE tasks SET
            title       = COALESCE($2, title),
            category    = COALESCE($3, category),
            description = COALESCE($4, description),
            assigned_to = COALESCE($5, assigned_to)
        WHERE id = $1
        RETURNING id, title, category AS "category: _",
                  description, creator_id, assigned_to
        "#,
        id,
        dto.title,
        dto.category as Option<TaskCategory>, // wtf - https://github.com/launchbadge/sqlx/issues/1004#issuecomment-854641523
        dto.description,
        dto.assigned_to,
    )
    .fetch_optional(pool)
    .await
    .map_err(ApiError::from)
}

pub async fn delete_task(pool: &PgPool, id: i32) -> sqlx::Result<u64> {
    sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(pool)
        .await
        .map(|res| res.rows_affected())
}
