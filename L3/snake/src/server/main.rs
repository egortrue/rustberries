mod logic;
mod socket;

use app::domain::world::World;
use std::{
    env,
    sync::{Arc, RwLock},
};

fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().ok();

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init();

    // Создание мира
    let width = env::var("WORLD_WIDTH")
        .unwrap_or("40".to_string())
        .parse::<usize>()
        .expect("ENV not found: WORLD_WIDTH");
    let height = env::var("WORLD_HEIGHT")
        .unwrap_or("40".to_string())
        .parse::<usize>()
        .expect("ENV not found: WORLD_HEIGHT");
    let world = Arc::new(RwLock::new(World::new(width, height)));

    // Запуск игровой логики и прослушивание сокетов
    logic::run(world.clone());
    socket::run(world.clone());
}
