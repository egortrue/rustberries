use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub likes: u32,
}

impl Post {
    pub fn new(author: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author,
            content,
            likes: 0,
        }
    }
}
