mod gui;
mod socket;

use app::domain::{snake::Snake, world::World};
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};

fn main() {
    // Локальная змея (для локального изменения и отправки на сервер)
    let snake = Arc::new(RwLock::new(Snake::new(
        format!("user{}", thread_rng().gen::<u8>()),
        [
            thread_rng().gen::<u8>(),
            thread_rng().gen::<u8>(),
            thread_rng().gen::<u8>(),
        ],
        (
            (thread_rng().gen::<usize>() % 10).clamp(2, 10),
            (thread_rng().gen::<usize>() % 10).clamp(2, 10),
        ),
    )));

    // Локальная копия мира (отрисовка) (включает в себя локальную змею после старта игры)
    let world = Arc::new(RwLock::new(World::new(40, 40)));

    // Запуск сокета для обновления данных и GUI-клиента для отрисовки
    socket::run(snake.clone(), world.clone());
    gui::run(snake.clone(), world.clone());
}
