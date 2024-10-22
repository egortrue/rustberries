use super::message::Message;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::broadcast::{self, Receiver, Sender};

pub struct Chat {
    name: String,
    password: Option<String>,
    users: AtomicUsize, // https://doc.rust-lang.org/stable/nomicon/atomics.html#relaxed
    channel: Sender<Message>,
}

impl Chat {
    pub fn new(name: String, password: Option<String>) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            name,
            password,
            users: AtomicUsize::new(0),
            channel: tx,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_private(&self) -> bool {
        self.password.is_some()
    }

    pub fn users(&self) -> usize {
        self.users.load(Ordering::Relaxed)
    }

    /// Получение ресивера канала и обновление атомарного счетчика (+1)
    pub fn subscribe(&self, password: Option<String>) -> Result<Receiver<Message>, String> {
        if self.password.is_some() && self.password != password {
            return Err("Wrong password for subscribe".to_string());
        }
        self.users.fetch_add(1, Ordering::Relaxed);
        Ok(self.channel.subscribe())
    }

    /// Обновление атомарного счетчика (-1)
    pub fn unsubscribe(&self) -> Result<(), String> {
        // Должны загрузить счетчик и проверить его на 0 прежде чем убавить его
        if self.users.load(Ordering::Acquire) == 0 {
            return Err("No users to unsubscribe".to_string());
        }
        self.users.fetch_sub(1, Ordering::Release);
        Ok(())
    }

    /// Отправка сообщения всем ресиверам
    pub fn send(&self, message: &Message) -> Result<(), String> {
        match self.channel.send(message.clone()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
