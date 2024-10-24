// Обработчики HTTP-запросов

use crate::controller::dto::{
    RequestChatCreate, RequestChatJoin, RequestChatLeave, RequestMessageList, RequestMessageSend,
    RequestUserLogin, ResponseChatCreate, ResponseChatInfo, ResponseUserLogin,
};
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
        .route("/leave", post(leave))
        .route("/send", post(send))
        .route("/messages", get(messages))
        // Дополнительные эндпоинты
        .route("/login", post(login))
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
    match status {
        StatusCode::OK => info!("{method} {uri} -> {status}"),
        _ => error!("{method} {uri} -> {status}"),
    };

    response
}

/// Создание чата
pub async fn create(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatCreate>,
) -> impl IntoResponse {
    match state.chat_create(body.name.clone(), body.password).await {
        Ok(id) => Json(ResponseChatCreate { id }).into_response(),
        Err(e) => (StatusCode::CONFLICT, e).into_response(),
    }
}

/// Получение списка чатов
pub async fn list(State(state): State<Arc<dyn ChatRepository>>) -> impl IntoResponse {
    let mut result = vec![];
    for (id, chat) in state.chat_list().await {
        result.push(ResponseChatInfo {
            id,
            name: chat.name().to_string(),
            users: chat.users(),
            private: chat.is_private(),
        })
    }

    Json(result)
}

/// Регистрация пользователя и получение уникального индентификатора
pub async fn login(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestUserLogin>,
) -> impl IntoResponse {
    let socket = match tokio::net::lookup_host(body.address).await {
        Ok(mut el) => el.next().unwrap(),
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    match state.user_login(body.username.clone(), socket).await {
        Ok(id) => Json(ResponseUserLogin { id }).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

/// Подключение пользователя к чата
pub async fn join(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatJoin>,
) -> impl IntoResponse {
    match state.join(&body.user, &body.chat, body.password).await {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

/// Отключение пользователя от чата
pub async fn leave(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestChatLeave>,
) -> impl IntoResponse {
    match state.leave(&body.user, &body.chat).await {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

/// Отправка пользователем сообщения в чат
pub async fn send(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestMessageSend>,
) -> impl IntoResponse {
    match state.send(&body.user, &body.chat, body.text).await {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

/// Получение пользователем всех сообщений в чате
pub async fn messages(
    State(state): State<Arc<dyn ChatRepository>>,
    Json(body): Json<RequestMessageList>,
) -> impl IntoResponse {
    match state.messages(&body.user, &body.chat).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
