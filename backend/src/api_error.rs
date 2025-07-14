use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,

    #[error("referenced user does not exist")]
    ForeignKey,

    #[error(transparent)]
    DB(sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// to prevent errors leaking
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self, "request failed");

        match self {
            ApiError::ForeignKey   => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            ApiError::Internal(_) | ApiError::DB(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
            }
        }
    }
}

// Handling ForeignKey(23503) error
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db) = &e {
            if db.code() == Some(std::borrow::Cow::Borrowed("23503")) {
                return ApiError::ForeignKey;
            }
        }
        ApiError::DB(e)
    }
}
