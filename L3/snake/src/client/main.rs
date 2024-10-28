mod gui;
mod logic;
mod socket;

use app::domain::{snake::Snake, world::World};
use std::sync::{Arc, RwLock};

fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().ok();

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init();

    // Локальная змея (для локального изменения и отправки на сервер)
    let snake = Arc::new(RwLock::new(Snake::random()));

    // Локальная копия мира (отрисовка) (включает в себя локальную змею после старта игры)
    let world = Arc::new(RwLock::new(World::default()));

    // Подключение к сокетам для обновления данных
    socket::run(snake.clone(), world.clone());

    // Внитриигровая логика со стороны клиента
    logic::run(snake.clone(), world.clone());

    // GUI-клиент для отрисовки
    gui::run(snake.clone(), world.clone());
}
