use axum::{
    extract::Request,
    middleware::Next,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
// Информация о токене
pub struct Claims {
    pub expire: usize,
    pub username: String,
}

// Мидлваер для обработки заголовка с токеном
pub async fn with_authorize_token(req: Request, next: Next) -> impl IntoResponse {
    next.run(req).await
}
