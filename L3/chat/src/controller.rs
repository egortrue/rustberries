// Обработчики HTTP-запросов

use crate::dto::{RequestCreateChat, RequestJoinChat};
use crate::repository::ChatRepository;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use log::{error, info};
use std::sync::Arc;

/// Основной маппинг эндпоинтов
pub fn create_router(state: Arc<dyn ChatRepository>) -> Router {
    Router::new()
        // Основные эндпоинты
        .route("/join", post(join))
        // Дополнительные эндпоинты
        .route("/create", post(create))
        .route("/list", post(list))
        // Настройка
        .layer(middleware::from_fn(logger))
        .with_state(state)
}

/// Логирование
async fn logger(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let status = response.status();
    match status {
        StatusCode::OK => info!("{method} {uri} -> {status}"),
        _ => error!("{method} {uri} -> {status}"),
    };

    response
}

pub async fn create(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestCreateChat>,
) -> impl IntoResponse {
    state.create(body.name, body.password);
}

pub async fn list(State(state): State<Arc<dyn ChatRepository>>) -> impl IntoResponse {}

/// Индентификация и подключение пользователя
pub async fn join(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestJoinChat>,
) -> impl IntoResponse {
    let mut a = format!("{} connected to {}", body.username, body.chat);
    if let Some(password) = body.password {
        a += format!(" with password {password}").as_str();
    }
    a
}
