use super::model::*;
use crate::api_error::ApiError;
use crate::app_state::AppState;
use crate::features::users::service;
use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/users",
    tag = "Users",
    responses(
        (status = 200, description = "List users", body = [UserListItemDTO])
    )
)]
pub async fn users_list(State(state): State<AppState>) -> Result<Json<Vec<UserListItemDTO>>, ApiError> {
    let result = service::users_list(&state).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    tag = "Users",
    responses(
        (status = 200, description = "Get user by id", body = [UserDetailsDTO])
    )
)]
pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<UserDetailsDTO>, ApiError> {
    let result = service::get_user_details(&state, id).await?;
    Ok(Json(result))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", get(users_list))
        .route("/user/{id}", get(get_user))
}
