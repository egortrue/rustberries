// Не сохраняемое хранилище в памяти

use crate::domain::{chat::Chat, message::Message, user::User};
use crate::repository::ChatRepository;
use dashmap::DashMap;
use std::net::SocketAddr;
use std::{collections::LinkedList, sync::Arc};
use uuid::Uuid;

#[derive(Default)]
pub struct State {
    chats: DashMap<Uuid, Arc<Chat>>,
    users: DashMap<Uuid, Box<User>>,
    messages: DashMap<Uuid, LinkedList<Message>>, // uuid for chat
}

#[axum::async_trait]
impl ChatRepository for State {
    async fn user_login(&self, username: String, address: SocketAddr) -> Result<Uuid, String> {
        if let Some(_) = self.users.iter().find(|el| el.value().name() == username) {
            return Err(format!("The user already logged in!"));
        }

        let id = Uuid::new_v4();
        let user = Box::new(User::new(username, address));
        self.users.insert(id, user);
        Ok(id)
    }

    async fn chat_create(
        &self,
        chatname: String,
        password: Option<String>,
    ) -> Result<Uuid, String> {
        if let Some(_) = self.chats.iter().find(|el| el.value().name() == chatname) {
            return Err(format!("The chat already exists with name: {chatname}"));
        }

        let id = Uuid::new_v4();
        let chat = Arc::new(Chat::new(chatname, password));
        self.chats.insert(id, chat);
        self.messages.insert(id, LinkedList::new());
        Ok(id)
    }

    async fn chat_list(&self) -> Vec<(Uuid, Arc<Chat>)> {
        self.chats
            .iter()
            .map(|el| (el.key().clone(), Arc::clone(el.value())))
            .collect()
    }

    async fn join(&self, user: &Uuid, chat: &Uuid, password: Option<String>) -> Result<(), String> {
        // Лок чата
        let chat_lock = match self.chats.get_mut(chat) {
            Some(entry) => entry,
            None => return Err("Chat not found".to_string()),
        };

        // Лок пользователя
        let mut user_lock = match self.users.get_mut(user) {
            Some(entry) => entry,
            None => return Err("User not found".to_string()),
        };

        // Доп. проверка на валидность пользователя
        if user_lock.value().is_subscribed() {
            return Err("User already subscribed".to_string());
        }

        // (atomic++) + получение ресивера канала для подписки пользователя
        let channel = match chat_lock.value().subscribe(password) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };

        // Подписка пользователя на канал (хранение ресивера)
        match user_lock.value_mut().subscribe(channel) {
            Ok(_) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    async fn leave(&self, user: &Uuid, chat: &Uuid) -> Result<(), String> {
        // Лок чата
        let chat_lock = match self.chats.get(chat) {
            Some(entry) => entry,
            None => return Err("Chat not found".to_string()),
        };

        // Лок пользователя
        let mut user_lock = match self.users.get_mut(user) {
            Some(entry) => entry,
            None => return Err("User not found".to_string()),
        };

        // (atomic--)
        match chat_lock.value().unsubscribe() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // Отписка пользователя от канала (дроп рисивера)
        match user_lock.value_mut().unsubscribe() {
            Ok(_) => Ok(()),
            Err(e) => return Err(e),
        }
    }

    async fn send(&self, user: &Uuid, chat: &Uuid, text: String) -> Result<(), String> {
        // Лок чата
        let chat_lock = match self.chats.get(chat) {
            Some(entry) => entry,
            None => return Err("Chat not found".to_string()),
        };

        // Лок сообщений
        let mut messages_lock = match self.messages.get_mut(chat) {
            Some(entry) => entry,
            None => return Err("Messages for chat not found".to_string()),
        };

        // Лок пользователя
        let user_lock = match self.users.get(user) {
            Some(entry) => entry,
            None => return Err("User not found".to_string()),
        };

        // Формирование и отправка сообщения
        let message = Message {
            content: text,
            author: user_lock.value().name().to_string(),
        };
        match chat_lock.value().send(&message) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        messages_lock.value_mut().push_back(message);

        Ok(())
    }

    async fn messages(&self, user: &Uuid, chat: &Uuid) -> Result<Vec<Message>, String> {
        // Лок пользователя
        let _ = match self.users.get(user) {
            Some(entry) => entry,
            None => return Err("User not found".to_string()),
        };

        // Лок сообщений
        let messages_lock = match self.messages.get(chat) {
            Some(entry) => entry,
            None => return Err("Messages for chat not found".to_string()),
        };

        Ok(messages_lock.value().iter().cloned().collect())
    }
}
