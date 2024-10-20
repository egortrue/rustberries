// Хранилища данных и взаимодействие с ними

pub mod dashmap;

use crate::domain::{chat::Chat, user::User};
use std::sync::Arc;
use uuid::Uuid;

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait ChatRepository: Sync + Send {
    async fn user_login(&self, name: String) -> Result<Uuid, String>;
    async fn chat_create(&self, name: String, password: Option<String>) -> Result<(), String>;
    async fn chat_list(&self) -> Vec<(Uuid, Arc<Chat>)>;
    async fn join(&self, user: &Uuid, chat: &Uuid, password: Option<String>) -> Result<(), String>;
    async fn leave(&self, uset: &Uuid, chat: &Uuid) -> Result<(), String>;
}
