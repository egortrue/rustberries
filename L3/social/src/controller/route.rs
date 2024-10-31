// Обработчики HTTP-запросов

use crate::{
    controller::{
        dto::{
            RequestAddPost, RequestDeletePost, RequestGetPost, RequestLikePost, RequestLogin,
            RequestRegister,
        },
        token::Token,
    },
    domain::{post::Post, user::User},
    errors::ErrorKind,
    repository::SocialRepository,
};
use axum::{
    extract::{Path, Request, State},
    http::{self, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::Utc;
use log::{error, info};
use std::sync::Arc;

/// Основной маппинг эндпоинтов
pub fn create_router(state: Arc<dyn SocialRepository>) -> Router {
    Router::new()
        // Эндпоинты требующие токена
        .route("/posts", get(get_posts).post(add_post))
        .route("/posts/:id", get(get_post).delete(delete_post))
        .route("/posts/:id/likes", post(like_post))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            with_authorize_token,
        ))
        // Базовые эндпоинты
        .route("/register", post(register))
        .route("/login", post(login))
        // Настройка
        .layer(middleware::from_fn(with_logger))
        .with_state(state)
}

/// Мидлваер для логирования
async fn with_logger(request: Request, next: Next) -> impl IntoResponse {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let status = response.status();
    match status {
        StatusCode::OK => info!("{method} {uri} -> {status}"),
        StatusCode::CREATED => info!("{method} {uri} -> {status}"),
        _ => error!("{method} {uri} -> {status}"),
    };
    response
}

// Мидлваер для обработки заголовка с токеном
pub async fn with_authorize_token(
    State(state): State<Arc<dyn SocialRepository>>,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    let auth_header = request.headers_mut().get(http::header::AUTHORIZATION);

    // Чтение токена
    let auth_header = match auth_header {
        Some(header) => match header.to_str() {
            Ok(auth_header) => auth_header,
            Err(_) => {
                return ErrorKind::Unauthorized("Empty header is not allowed".into())
                    .into_response()
            }
        },
        None => {
            return ErrorKind::Unauthorized("Please add the JWT token to the header".into())
                .into_response()
        }
    };

    // Расшифровка
    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());
    let token = match Token::decode(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(error) => return error.into_response(),
    };

    // Время жизни токена
    if Utc::now().timestamp() as usize > token.claims.exp {
        return ErrorKind::Unauthorized("Token has been expired".into()).into_response();
    }

    // Чтение пользователя и передача в дальнейший запрос
    let user = match state.get_user(&token.claims.username).await {
        Ok(user) => user,
        Err(error) => return error.into_response(),
    };
    request.extensions_mut().insert(user);

    next.run(request).await
}

/// Запись пользователя в БД
pub async fn register(
    State(state): State<Arc<dyn SocialRepository>>,
    Json(body): Json<RequestRegister>,
) -> impl IntoResponse {
    let user = User::new(body.username, body.password);
    match state.add_user(&user).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(error) => error.into_response(),
    }
}

/// Получение авторизационного токена
pub async fn login(
    State(state): State<Arc<dyn SocialRepository>>,
    Json(body): Json<RequestLogin>,
) -> impl IntoResponse {
    let user_db = match state.get_user(&body.username).await {
        Ok(user_db) => user_db,
        Err(error) => return error.into_response(),
    };

    if !user_db.verify_password(&body.password) {
        return ErrorKind::BadRequest("Wrong password".into()).into_response();
    }

    match Token::encode(&body.username) {
        Ok(token) => Json(token).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_posts(State(state): State<Arc<dyn SocialRepository>>) -> impl IntoResponse {
    match state.get_posts().await {
        Ok(posts) => Json(posts).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_post(
    State(state): State<Arc<dyn SocialRepository>>,
    Path(path): Path<RequestGetPost>,
) -> impl IntoResponse {
    match state.get_post(&path.id).await {
        Ok(post) => Json(post).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn add_post(
    State(state): State<Arc<dyn SocialRepository>>,
    Extension(user): Extension<User>,
    Json(body): Json<RequestAddPost>,
) -> impl IntoResponse {
    let post = Post::new(user.username, body.content);
    match state.add_post(&post).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn delete_post(
    State(state): State<Arc<dyn SocialRepository>>,
    Extension(user): Extension<User>,
    Path(path): Path<RequestDeletePost>,
) -> impl IntoResponse {
    let post = match state.get_post(&path.id).await {
        Ok(post) => post,
        Err(error) => return error.into_response(),
    };

    if post.author != user.username {
        return ErrorKind::Unauthorized("You're not the author".into()).into_response();
    }

    match state.delete_post(&path.id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn like_post(
    State(state): State<Arc<dyn SocialRepository>>,
    Extension(user): Extension<User>,
    Path(path): Path<RequestLikePost>,
) -> impl IntoResponse {
    let liked = match state.get_likes(&path.id).await {
        Ok(liked) => liked,
        Err(error) => return error.into_response(),
    };

    if liked.contains(&user.username) {
        return ErrorKind::BadRequest("Yor already liked this post".into()).into_response();
    }

    if let Err(error) = state.like_post(&path.id).await {
        return error.into_response();
    };

    if let Err(error) = state.add_likes(&path.id, &user.username).await {
        return error.into_response();
    }

    StatusCode::OK.into_response()
}
