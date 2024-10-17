use super::message::Message;
use std::collections::LinkedList;
use tokio::sync::broadcast;

pub struct Chat {
    pub name: String,
    password: Option<String>,
    messages: LinkedList<Message>,
    channel: (broadcast::Sender<Message>, broadcast::Receiver<Message>),
}

impl Chat {
    pub fn new(name: String, password: Option<String>) -> Self {
        Self {
            name,
            password,
            messages: LinkedList::new(),
            channel: broadcast::channel(100),
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
