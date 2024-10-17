/// Data Transfer Objects - объекты сериализации запросов
use serde::{Deserialize, Serialize};

/// Данные для создания комнаты
#[derive(Deserialize)]
pub struct RequestCreateChat {
    pub name: String,
    pub password: Option<String>,
}

/// Данные для подключения к комнате
#[derive(Deserialize)]
pub struct RequestJoinChat {
    pub username: String,
    pub chat: String,
    pub password: Option<String>,
}
