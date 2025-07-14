use crate::api_error::ApiError;
use crate::app_state::AppState;
use crate::features::tasks::model::*;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TasksRepo: Send + Sync {
    async fn get_all(&self) -> sqlx::Result<Vec<TaskDAO>>;
    async fn get_by_id(&self, id: i32) -> sqlx::Result<Option<TaskDAO>>;
    async fn create(&self, dto: TaskCreateDTO) -> Result<TaskDAO, ApiError>;
    async fn update(&self, id: i32, dto: TaskUpdateDTO) -> Result<Option<TaskDAO>, ApiError>;
    async fn delete(&self, id: i32) -> sqlx::Result<u64>;
}

#[async_trait]
impl TasksRepo for AppState {
    async fn get_all(&self) -> sqlx::Result<Vec<TaskDAO>> {
        sqlx::query_as!(
            TaskDAO,
            r#"
                SELECT id, title, description, creator_id, assigned_to, category AS "category: _"
                FROM tasks
            "#
        )
        .fetch_all(&self.db)
        .await
    }

    async fn get_by_id(&self, id: i32) -> sqlx::Result<Option<TaskDAO>> {
        sqlx::query_as!(
            TaskDAO,
            r#"
                SELECT id, title, description, creator_id, assigned_to, category AS "category: _"
                FROM tasks WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await
    }

    async fn create(&self, dto: TaskCreateDTO) -> Result<TaskDAO, ApiError> {
        sqlx::query_as!(
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
            dto.assigned_to
        )
        .fetch_one(&self.db)
        .await
        .map_err(ApiError::from)
    }

    async fn update(&self, id: i32, dto: TaskUpdateDTO) -> Result<Option<TaskDAO>, ApiError> {
        sqlx::query_as!(
            TaskDAO,
            r#"
                UPDATE tasks SET
                    title       = COALESCE($2, title),
                    category    = COALESCE($3, category),
                    description = COALESCE($4, description),
                    assigned_to = COALESCE($5, assigned_to)
                WHERE id = $1
                RETURNING id, title, category AS "category: _", description, creator_id, assigned_to
            "#,
            id,
            dto.title,
            dto.category as Option<TaskCategory>, // wtf - https://github.com/launchbadge/sqlx/issues/1004#issuecomment-854641523
            dto.description,
            dto.assigned_to
        )
        .fetch_optional(&self.db)
        .await
        .map_err(ApiError::from)
    }

    async fn delete(&self, id: i32) -> sqlx::Result<u64> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(&self.db)
            .await
            .map(|r| r.rows_affected())
    }
}
