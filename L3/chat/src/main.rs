// L3.3

mod controller;
mod domain;
mod dto;
mod repository;

use repository::ChatRepository;
use std::{env, error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Загрузка переменных из .env-файла
    dotenv::dotenv()?;
    let address = env::var("ADDRESS").expect("ENV variable not found: ADDRESS");
    let port = env::var("PORT").expect("ENV variable not found: PORT");

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .init();

    // Настройка хранилища
    let repository: Arc<dyn ChatRepository> = Arc::new(repository::local::State::default());

    // Настройка контроллера
    let socket = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&socket).await?;
    let controller = controller::create_router(Arc::clone(&repository));

    // Запуск
    axum::serve(listener, controller).await?;
    Ok(())
}
