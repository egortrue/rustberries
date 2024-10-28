mod socket;

use app::domain::world::World;
use std::sync::{Arc, RwLock};

fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().ok();

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init();

    // Создание мира
    let world = Arc::new(RwLock::new(World::new(40, 40)));

    // Запуск
    socket::run(world);
}
