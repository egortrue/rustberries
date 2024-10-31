// Обработчики HTTP-запросов

use super::dto::{RequestLogin, RequestRegister};
use crate::{
    domain::user::{self, User},
    repository::SocialRepository,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use core::hash;
use log::{error, info, warn};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Основной маппинг эндпоинтов
pub fn create_router(state: Arc<dyn SocialRepository>) -> Router {
    Router::new()
        // Базовые эндпоинты
        .route("/register", post(register))
        .route("/login", post(login))
        // Эндпоинты требующие токена
        // .route("/posts", get(todo!()).post(todo!()))
        // .route("/posts/:id", get(todo!()).delete(todo!()))
        // .route("/posts/:id/likes", post(todo!()))
        // Настройка
        .layer(middleware::from_fn(logger))
        .with_state(state)
}

// Мидлваер для логирования
async fn logger(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let status = response.status();
    match status {
        StatusCode::OK => info!("{method} {uri} -> {status}"),
        StatusCode::CREATED => warn!("{method} {uri} -> {status}"),
        _ => error!("{method} {uri} -> {status}"),
    };
    response
}

pub async fn register(
    State(state): State<Arc<dyn SocialRepository>>,
    Json(body): Json<RequestRegister>,
) -> impl IntoResponse {
    let user = User {
        username: body.username,
        password_hash: format!("{:x}", Sha256::digest(body.password)),
    };

    match state.insert_user(&user).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(error) => error.into_response(),
    }
}

// Эндпоинт для получения авторизационного токена
pub async fn login(
    State(state): State<Arc<dyn SocialRepository>>,
    Json(body): Json<RequestLogin>,
) -> impl IntoResponse {
    let token = "token";
    Json(token)
}
