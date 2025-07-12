use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,

    #[error(transparent)]
    DB(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// to prevent errors leaking
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self, "request failed");

        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            ApiError::Internal(_) | ApiError::DB(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}
