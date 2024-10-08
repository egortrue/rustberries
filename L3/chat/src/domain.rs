// Элементы бизнес логики

use std::{
    collections::LinkedList,
    sync::{Arc, Weak},
};
use tokio::sync::broadcast;
use uuid::Uuid;

pub struct User {
    name: String,
    platform: String, // Текущая платформа (клиент) пользователя - например, cli, gui или web
    chat: Weak<Chat>,
}

pub struct Chat {
    name: String,
    password: Option<String>,
    messages: LinkedList<Message>,
    users: LinkedList<Weak<User>>,
    channel: (broadcast::Sender<Message>, broadcast::Receiver<Message>),
}

#[derive(Clone)]
pub struct Message {
    pub content: String,
    pub author: Uuid,
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

impl Chat {
    fn new(name: String, password: Option<String>) -> Self {
        Self {
            name: name,
            password: password,
            messages: LinkedList::new(),
            users: LinkedList::new(),
            channel: broadcast::channel(100),
        }
    }

    fn join(&mut self, user: &Arc<User>) -> Result<(), String> {
        self.users.push_back(Arc::downgrade(user));
        Ok(())
    }
}
