use super::chat::Chat;
use std::sync::{Arc, Weak};

pub struct User {
    name: String,
    platform: String, // Текущая платформа (клиент) пользователя - например, cli, gui или web
    chat: Weak<Chat>,
}

impl User {
    fn new(name: String) -> Self {
        Self {
            name: name,
            platform: String::new(),
            chat: Weak::new(),
        }
    }

    fn join(&mut self, chat: &Arc<Chat>) -> Result<(), String> {
        if let Some(cur_chat) = self.chat.upgrade() {
            return Err(format!("Already connected to chat: \"{}\"", cur_chat.name));
        }

        self.chat = Arc::downgrade(chat);
        Ok(())
    }

    fn leave(&mut self) -> Result<(), String> {
        if let None = self.chat.upgrade() {
            return Err(format!("Not connected to a chat"));
        }

        self.chat = Weak::new();
        Ok(())
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.platform == other.platform
    }
}

impl Eq for User {}

impl std::hash::Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.platform.hash(state);
    }
}
