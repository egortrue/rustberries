// Обработчики HTTP-запросов
use crate::controller::dto::{RequestChatCreate, RequestChatJoin, ResponseChatInfo};
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
use serde_json::json;
use std::sync::Arc;

/// Основной маппинг эндпоинтов
pub fn create_router(state: Arc<dyn ChatRepository>) -> Router {
    Router::new()
        // Основные эндпоинты
        .route("/join", post(join))
        .route("/leave", post(leave))
        .route("/send", post(send))
        .route("/messages", get(messages))
        // Дополнительные эндпоинты
        .route("/create", post(create))
        .route("/list", get(list))
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
    let body = response.body();
    match status {
        StatusCode::OK => info!("{method} {uri} -> {status}"),
        _ => error!("{method} {uri} -> {status}"),
    };

    response
}

/// Создание чат-комнаты
pub async fn create(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatCreate>,
) -> impl IntoResponse {
    if let Err(e) = state.create(body.name, body.password).await {
        (StatusCode::CONFLICT, e)
    } else {
        (StatusCode::OK, String::new())
    }
}

/// Получение списка чат-комнат
pub async fn list(State(state): State<Arc<dyn ChatRepository>>) -> impl IntoResponse {
    let mut result = vec![];
    for chat in state.list().await {
        result.push(ResponseChatInfo {
            name: chat.name.clone(),
            users: chat.users.load(std::sync::atomic::Ordering::Relaxed),
            private: chat.password.is_some(),
        })
    }

    Json(json!(result))
}

/// Индентификация и подключение пользователя
pub async fn join(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatJoin>,
) -> impl IntoResponse {
    let mut a = format!("{} connected to {}", body.username, body.chat);
    if let Some(password) = body.password {
        a += format!(" with password {password}").as_str();
    }
    a
}

///
pub async fn leave(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatCreate>,
) -> impl IntoResponse {
    todo!()
}

pub async fn send(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatCreate>,
) -> impl IntoResponse {
    todo!()
}

pub async fn messages(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatCreate>,
) -> impl IntoResponse {
    todo!()
}
