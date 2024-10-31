// Data Transfer Objects - объекты сериализации запросов

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RequestRegister {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestAddPost {
    pub content: String,
}

#[derive(Deserialize)]
pub struct RequestGetPost {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct RequestDeletePost {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct RequestLikePost {
    pub id: Uuid,
}
