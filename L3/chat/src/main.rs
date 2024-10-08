// L3.3

use repository::ChatRepository;
use std::{error::Error, sync::Arc};

mod controller;
mod domain;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .init();

    // Настройка хранилища
    let repository: Arc<dyn ChatRepository> = Arc::new(repository::local::State::default());

    // Настройка контроллера
    let socket = format!("localhost:{}", 80);
    let listener = tokio::net::TcpListener::bind(&socket).await?;
    let controller = controller::create_router(Arc::clone(&repository));

    // Запуск
    axum::serve(listener, controller).await?;
    Ok(())
}
