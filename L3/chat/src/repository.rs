// Хранилища данных и взаимодействие с ними

use crate::domain::{chat::Chat, user::User};
use std::sync::{Arc, RwLock};

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait ChatRepository: Sync + Send {
    async fn create(&self, name: String, password: Option<String>) -> Result<(), String>;
    async fn list(&self) -> Vec<Arc<Chat>>;
    async fn join(&self, user: &Arc<User>, chat: &Arc<Chat>) -> Result<(), String>;
}

/// Не сохраняемое хранилище в памяти
pub mod local {
    use crate::domain::{chat::Chat, user::User};
    use crate::repository::ChatRepository;
    use dashmap::DashMap;
    use std::collections::LinkedList;
    use std::sync::Arc;

    pub type State = DashMap<Arc<Chat>, LinkedList<Arc<User>>>;

    #[axum::async_trait]
    impl ChatRepository for State {
        async fn create(&self, name: String, password: Option<String>) -> Result<(), String> {
            if self.iter().find(|el| el.key().name == name).is_some() {
                return Err(format!("Chat already exists with name: {name}"));
            }

            self.insert(Arc::new(Chat::new(name, password)), LinkedList::new());
            Ok(())
        }

        async fn list(&self) -> Vec<Arc<Chat>> {
            self.iter().map(|el| Arc::clone(el.key())).collect()
        }

        async fn join(&self, user: &Arc<User>, chat: &Arc<Chat>) -> Result<(), String> {
            todo!()
        }
    }
}

/// Сохраняемое хранилище в памяти (Redis)
pub mod redis {}

/// Сохраняемое хранилище в отдельной БД (Postgres)
pub mod postgres {}
