// Хранилища данных и взаимодействие с ними

pub mod postgres;

use crate::{domain::user::User, errors::Result};

/// Основной интерфейс для работы контроллеров
#[axum::async_trait]
pub trait SocialRepository: Sync + Send {
    async fn insert_user(&self, user: &User) -> Result<()>;
    async fn get_user(&self, user: &User) -> Result<()>;
}
