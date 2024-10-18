use super::message::Message;
use std::{
    collections::LinkedList,
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::sync::broadcast;

pub struct Chat {
    pub name: String,
    pub users: AtomicUsize,
    pub password: Option<String>,
    pub messages: LinkedList<Message>,
    pub channel: (
        broadcast::Sender<Message>,
        Arc<broadcast::Receiver<Message>>,
    ),
}

impl Chat {
    pub fn new(name: String, password: Option<String>) -> Self {
        let (tx, rx) = broadcast::channel(100);
        Self {
            name,
            password,
            users: AtomicUsize::new(0),
            messages: LinkedList::new(),
            channel: (tx, Arc::new(rx)),
        }
    }
}

impl PartialEq for Chat {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Chat {}

impl std::hash::Hash for Chat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
