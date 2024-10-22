// Хранилища данных и взаимодействие с ними

pub mod dashmap;

use crate::domain::{chat::Chat, message::Message};
use std::{net::SocketAddr, sync::Arc};
use uuid::Uuid;

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait ChatRepository: Sync + Send {
    async fn user_login(&self, name: String, address: SocketAddr) -> Result<Uuid, String>;
    async fn chat_create(&self, name: String, password: Option<String>) -> Result<Uuid, String>;
    async fn chat_list(&self) -> Vec<(Uuid, Arc<Chat>)>;
    async fn join(&self, user: &Uuid, chat: &Uuid, password: Option<String>) -> Result<(), String>;
    async fn leave(&self, user: &Uuid, chat: &Uuid) -> Result<(), String>;
    async fn send(&self, user: &Uuid, chat: &Uuid, text: String) -> Result<(), String>;
    async fn messages(&self, user: &Uuid, chat: &Uuid) -> Result<Vec<Message>, String>;
}
