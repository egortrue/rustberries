use app::domain::{snake::Snake, world::World};
use std::{
    sync::{Arc, RwLock},
    time::{self, Duration},
};

pub fn run(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) {
    let mut update_timer = time::Instant::now();
    let update_time = Duration::from_millis(world.read().unwrap().update_time);

    // Получение данных о мире
    let mut spawn_timer = time::Instant::now();
    let spawn_time = Duration::from_millis(world.read().unwrap().spawn_time);
    std::thread::spawn(move || loop {
        if spawn_timer.elapsed() > spawn_time {
            let mut world = world.write().unwrap();
            world.spawn_apple();
            spawn_timer = time::Instant::now();
        }
    });

    // Отправка данных
    std::thread::spawn(move || loop {
        if update_timer.elapsed() > update_time {
            let mut snake = snake.write().unwrap();

            // // Передвижение
            // snake.moving();
            // for (i, apple) in world.apples.iter().enumerate() {
            //     if snake.collide_with([apple.position]) {
            //         snake.grow();
            //         world.apples.remove(i);
            //         break;
            //     }
            // }

            // // Обновление мира
            // world.snakes.pop();
            // world.snakes.push(snake.clone());

            update_timer = time::Instant::now();
        }
    });
}
