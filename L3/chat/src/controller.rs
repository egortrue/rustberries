// Обработчики HTTP-запросов

use crate::repository::ChatRepository;
use axum::{response::IntoResponse, routing::get, Router};
use std::sync::Arc;

/// Основной маппинг эндпоинтов
pub fn create_router(state: Arc<dyn ChatRepository>) -> Router {
    Router::new().route("/", get(index)).with_state(state)
}

pub async fn index() -> impl IntoResponse {
    "Hello, WB!".into_response()
}
