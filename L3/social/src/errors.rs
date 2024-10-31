use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = std::result::Result<T, ServerError>;

pub enum ServerError {
    // 4XX
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    // 5XX
    InternalError(String),
    DatabaseError(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerError::BadRequest(error) => (StatusCode::BAD_REQUEST, format!("{error}")),
            ServerError::NotFound(error) => (StatusCode::NOT_FOUND, format!("{error}")),
            ServerError::Unauthorized(error) => (StatusCode::UNAUTHORIZED, format!("{error}")),
            ServerError::InternalError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
            }
            ServerError::DatabaseError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
            }
        }
        .into_response()
    }
}
