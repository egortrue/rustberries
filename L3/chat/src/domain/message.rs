use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Message {
    pub content: String,
    pub author: String,
}
