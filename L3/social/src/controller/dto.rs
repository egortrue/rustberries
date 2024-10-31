// Data Transfer Objects - объекты сериализации запросов

use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestRegister {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestLogin {
    pub username: String,
    pub password: String,
}
