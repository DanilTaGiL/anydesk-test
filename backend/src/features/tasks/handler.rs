use axum::{Json, response::IntoResponse, routing::{get, post}, Router};
use uuid::Uuid;
use super::model::*;

#[utoipa::path(
    get,
    path = "/tasks",
    tag = "Tasks",
    responses(
        (status = 200, description = "List tasks", body = [TaskListItem])
    )
)]
pub async fn list_tasks() -> impl IntoResponse {
    Json(vec![
        TaskListItem {
            id: 1,
            title: "Found some bug".into(),
            category: TaskCategory::Bug,
        },
        TaskListItem {
            id: 2,
            title: "Need for Fix: Most Wanted".into(),
            category: TaskCategory::Task,
        },
    ])
}

#[utoipa::path(
    get,
    path = "/task/{id}",
    tag = "Tasks",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    responses(
        (status = 200, body = TaskDetail)
    )
)]
pub async fn get_task(axum::extract::Path(id): axum::extract::Path<i32>) -> impl IntoResponse {
    Json(TaskDetail {
        id,
        title: "Found some bug".into(),
        category: TaskCategory::Bug,
        description: "some long description".into(),
        creator_id: Uuid::new_v4(),
        assigned_to: Uuid::new_v4(),
    })
}

#[utoipa::path(
    post,
    path = "/task",
    tag = "Tasks",
    request_body = TaskCreate,
    responses(
        (status = 201, body = TaskCreate)
    )
)]
pub async fn create_task(Json(payload): Json<TaskDetail>) -> impl IntoResponse {
    (axum::http::StatusCode::CREATED, Json(payload))
}

pub fn router() -> Router {
    Router::new()
        .route("/tasks", get(list_tasks))
        .route("/task/{id}", get(get_task))
        .route("/task", post(create_task))
}
