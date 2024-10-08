// Хранилища данных и взаимодействие с ними

use crate::domain::{Chat, User};
use std::sync::{Arc, RwLock};

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait ChatRepository: Sync + Send {
    async fn join(&self, user: &Arc<User>, chat: &Arc<Chat>);
}

/// Non-persitant хранилище в памяти
pub mod local {
    use crate::domain::{Chat, User};
    use crate::repository::ChatRepository;
    use std::{
        collections::LinkedList,
        sync::{Arc, RwLock},
    };

    pub struct State {
        users: RwLock<LinkedList<User>>,
        chats: RwLock<LinkedList<Chat>>,
    }

    impl Default for State {
        fn default() -> Self {
            Self {
                users: RwLock::new(LinkedList::new()),
                chats: RwLock::new(LinkedList::new()),
            }
        }
    }

    #[axum::async_trait]
    impl ChatRepository for State {
        async fn join(&self, user: &Arc<User>, chat: &Arc<Chat>) {
            todo!()
        }
    }
}

/// Persistant хранилище в памяти (Redis)
pub mod redis {}
