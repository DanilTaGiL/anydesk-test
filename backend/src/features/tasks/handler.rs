use super::model::*;
use crate::api_error::ApiError;
use crate::app_state::AppState;
use crate::features::tasks::service;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, put};
use axum::{
    Json, Router,
    routing::{get, post},
};

#[utoipa::path(
    get,
    path = "/task/all",
    tag = "Tasks",
    summary = "Short description of all tasks",
    responses(
        (status = 200, description = "List tasks", body = [TaskListItemDTO])
    )
)]
pub async fn all_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskListItemDTO>>, ApiError> {
    Ok(Json(service::get_all_tasks(&state).await?))
}

#[utoipa::path(
    get,
    path = "/task/{id}",
    tag = "Tasks",
    summary = "Detailed description of specific task",
    responses(
        (status = 200, body = TaskDetailsDTO),
        (status = 404, description = "Task with {id} does not exist")
    )
)]
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<TaskDetailsDTO>, ApiError> {
    Ok(Json(service::get_task(&state, id).await?))
}

#[utoipa::path(
    post,
    path = "/task",
    tag = "Tasks",
    summary = "Create new task",
    request_body = TaskCreateDTO,
    responses(
        (status = 201, body = TaskDetailsDTO),
        (status = 400, description = "Referenced user does not exist")
    )
)]
pub async fn create_task(
    State(state): State<AppState>,
    Json(dto): Json<TaskCreateDTO>,
) -> Result<(StatusCode, Json<TaskDetailsDTO>), ApiError> {
    let created = service::create_task(&state, dto).await?;
    Ok((StatusCode::CREATED, Json(created)))
}

#[utoipa::path(
    put,
    path = "/task/{id}",
    tag = "Tasks",
    summary = "Update existing task",
    request_body = TaskUpdateDTO,
    responses(
        (status = 200, body = TaskDetailsDTO),
        (status = 400, description = "Referenced user does not exist"),
        (status = 404, description = "Task with {id} does not exist")
    )
)]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<TaskUpdateDTO>,
) -> Result<Json<TaskDetailsDTO>, ApiError> {
    Ok(Json(service::update_task(&state, id, dto).await?))
}

#[utoipa::path(
    delete,
    path = "/task/{id}",
    tag = "Tasks",
    summary = "Delete specific task",
    responses(
        (status = 200, body = TaskListItemDTO),
        (status = 404, description = "Task with {id} does not exist")
    )
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    service::delete_task(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/task/all", get(all_tasks))
        .route("/task", post(create_task))
        .route("/task/{id}", get(get_task))
        .route("/task/{id}", put(update_task))
        .route("/task/{id}", delete(delete_task))
}
