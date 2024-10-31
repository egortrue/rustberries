// Хранилища данных и взаимодействие с ними

pub mod postgres;

use uuid::Uuid;

use crate::{
    domain::{post::Post, user::User},
    errors::Result,
};

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait SocialRepository: Sync + Send {
    async fn add_user(&self, user: &User) -> Result<()>;
    async fn get_user(&self, username: &String) -> Result<User>;
    async fn add_post(&self, post: &Post) -> Result<()>;
    async fn get_posts(&self) -> Result<Vec<Post>>;
    async fn get_post(&self, id: &Uuid) -> Result<Post>;
    async fn delete_post(&self, id: &Uuid) -> Result<()>;
    async fn like_post(&self, id: &Uuid) -> Result<()>;
    async fn add_likes(&self, post: &Uuid, username: &String) -> Result<()>;
    async fn get_likes(&self, post: &Uuid) -> Result<Vec<String>>;
}
