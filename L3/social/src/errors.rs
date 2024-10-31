use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = std::result::Result<T, ErrorKind>;

pub enum ErrorKind {
    // 4XX
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Conflict(String),
    // 5XX
    UndefinedError(String),
    SerializeError(String),
    JwtError(jsonwebtoken::errors::Error),
    DbConnectionError(deadpool_postgres::PoolError),
    PostgresError(tokio_postgres::error::DbError),
}

impl IntoResponse for ErrorKind {
    fn into_response(self) -> axum::response::Response {
        match self {
            // 4XX
            ErrorKind::BadRequest(error) => (StatusCode::BAD_REQUEST, error),
            ErrorKind::NotFound(error) => (StatusCode::NOT_FOUND, error),
            ErrorKind::Unauthorized(error) => (StatusCode::UNAUTHORIZED, error),
            ErrorKind::Conflict(error) => (StatusCode::CONFLICT, error),

            // 5XX
            ErrorKind::UndefinedError(error) => {
                log::error!("Undefined error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, format!(""))
            }
            ErrorKind::SerializeError(error) => {
                log::error!("Serialize error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, format!(""))
            }
            ErrorKind::JwtError(error) => {
                log::error!("JWT error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, format!(""))
            }
            ErrorKind::DbConnectionError(error) => {
                log::error!("Database is not available: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, format!(""))
            }
            ErrorKind::PostgresError(error) => {
                log::error!("Invalid postgresql request: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, format!(""))
            }
        }
        .into_response()
    }
}
