use super::model::*;
use crate::api_error::ApiError;
use crate::app_state::AppState;
use crate::features::users::service;
use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/user/all",
    tag = "Users",
    summary = "Short description of all users",
    responses(
        (status = 200, description = "List users", body = [UserListItemDTO])
    )
)]
pub async fn all_users(State(state): State<AppState>) -> Result<Json<Vec<UserListItemDTO>>, ApiError> {
    Ok(Json(service::get_all(&state).await?))
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    tag = "Users",
    summary = "Detailed description of specific user",
    responses(
        (status = 200, description = "Get user by id", body = [UserDetailsDTO])
    )
)]
pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<UserDetailsDTO>, ApiError> {
    Ok(Json(service::get_user_details(&state, id).await?))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/all", get(all_users))
        .route("/user/{id}", get(get_user))
}
