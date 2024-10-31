use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password_hash: format!("{:x}", Sha256::digest(password)),
        }
    }

    pub fn verify_password(&self, password: &String) -> bool {
        self.password_hash == format!("{:x}", Sha256::digest(password))
    }
}
