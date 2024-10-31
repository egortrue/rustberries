// L3.3

mod controller;
mod domain;
mod errors;
mod repository;

use repository::SocialRepository;
use std::{env, error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().expect(".env file not found");

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .init();

    // Настройка хранилища
    let repository: Arc<dyn SocialRepository> = Arc::new(repository::postgres::State::default());

    // Настройка контроллера
    let socket = env::var("SERVER_SOCKET").expect("ENV variable not found: SERVER_SOCKET");
    let listener = tokio::net::TcpListener::bind(&socket).await?;
    let controller = controller::route::create_router(Arc::clone(&repository));

    // Запуск
    axum::serve(listener, controller).await?;
    Ok(())
}
