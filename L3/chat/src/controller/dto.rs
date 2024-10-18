/// Data Transfer Objects - объекты сериализации запросов
use serde::{Deserialize, Serialize};

/// Данные для создания комнаты
#[derive(Deserialize)]
pub struct RequestChatCreate {
    pub name: String,
    pub password: Option<String>,
}

/// Данные для подключения к комнате
#[derive(Deserialize)]
pub struct RequestChatJoin {
    pub username: String,
    pub chat: String,
    pub password: Option<String>,
}

/// Визуальные данные о комнате
#[derive(Serialize)]
pub struct ResponseChatInfo {
    pub name: String,
    pub users: usize,  // кол-во пользователей
    pub private: bool, // используется пароль?
}
