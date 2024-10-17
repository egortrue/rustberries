// Хранилища данных и взаимодействие с ними

use crate::domain::{chat::Chat, user::User};
use std::sync::{Arc, RwLock};

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait ChatRepository: Sync + Send {
    async fn create(&self, name: String, password: Option<String>) -> Result<(), String>;
    async fn list(&self) -> Vec<String>;
    async fn join(&self, user: &Arc<User>, chat: &Arc<Chat>) -> Result<(), String>;
}

/// Не сохраняемое хранилище в памяти
pub mod local {
    use crate::domain::{chat::Chat, user::User};
    use crate::repository::ChatRepository;
    use dashmap::{DashMap, DashSet};
    use std::sync::Arc;

    pub type State = DashMap<Chat, DashSet<Arc<User>>>;

    #[axum::async_trait]
    impl ChatRepository for State {
        async fn create(&self, name: String, password: Option<String>) -> Result<(), String> {
            if self.iter().find(|el| el.key().name == name).is_some() {
                return Err(format!("Chat already exists with name: {}", name));
            }

            self.insert(Chat::new(name, password), DashSet::new());
            Ok(())
        }

        async fn list(&self) -> Vec<String> {
            let mut result = vec![];
            for el in self.iter() {
                result.push(el.key().name.clone());
            }
            result
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
