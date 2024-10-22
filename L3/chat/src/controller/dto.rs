// Data Transfer Objects - объекты сериализации запросов

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/////////////////////////////////////////////////////////////////////////

#[derive(Deserialize)]
pub struct RequestUserLogin {
    pub username: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct ResponseUserLogin {
    pub id: Uuid,
}

/////////////////////////////////////////////////////////////////////////

/// Данные для создания комнаты
#[derive(Deserialize)]
pub struct RequestChatCreate {
    pub name: String,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseChatCreate {
    pub id: Uuid,
}

/// Данные для подключения к комнате
#[derive(Deserialize)]
pub struct RequestChatJoin {
    pub user: Uuid,
    pub chat: Uuid,
    pub password: Option<String>,
}

/// Данные для отключения пользователя от комнаты
#[derive(Deserialize)]
pub struct RequestChatLeave {
    pub user: Uuid,
    pub chat: Uuid,
}

/// Визуальные данные о комнате
#[derive(Serialize)]
pub struct ResponseChatInfo {
    pub id: Uuid,
    pub name: String,
    pub users: usize,
    pub private: bool,
}

/////////////////////////////////////////////////////////////////////////

/// Данные для отправки сообщения в комнату
#[derive(Deserialize)]
pub struct RequestMessageSend {
    pub user: Uuid,
    pub chat: Uuid,
    pub text: String,
}

/// Данные для получения списка сообщений
#[derive(Deserialize)]
pub struct RequestMessageList {
    pub user: Uuid,
    pub chat: Uuid,
}
