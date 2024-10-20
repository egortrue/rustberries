use super::message::Message;
use tokio::sync::broadcast;

pub struct User {
    name: String,
    channel: Option<broadcast::Receiver<Message>>,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            channel: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_subscribed(&self) -> bool {
        self.channel.is_some()
    }

    pub fn subscribe(&mut self, receiver: broadcast::Receiver<Message>) -> Result<(), &str> {
        if self.channel.is_some() {
            return Err("User already subscribed!");
        }
        self.channel = Some(receiver);
        Ok(())
    }

    pub fn unsubscribe(&mut self) -> Result<(), &str> {
        if self.channel.is_none() {
            return Err("User already unsubscribed!");
        }
        self.channel = None;
        Ok(())
    }
}
