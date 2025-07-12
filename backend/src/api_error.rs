use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

// to prevent errors leaking
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self, "request failed");
        (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
    }
}
