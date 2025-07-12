use axum::{routing::{get}, Json, Router, response::IntoResponse};
use uuid::Uuid;
use super::model::*;

/// GET /users
pub async fn list_users() -> impl IntoResponse {
    Json(vec![
        UserListItem {
            id: Uuid::new_v4(),
            full_name: "Danil Suiagin".into(),
            role: UserRole::Admin,
        },
    ])
}

/// GET /user/{id}
pub async fn get_user(axum::extract::Path(id): axum::extract::Path<Uuid>) -> impl IntoResponse {
    Json(UserDetail {
        id,
        full_name: "Danil Suiagin".into(),
        headline: "SMTH ABOUT DANIL".into(),
        role: UserRole::Admin,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/user/{id}", get(get_user))
}
