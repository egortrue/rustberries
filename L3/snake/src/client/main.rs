mod gui;

use app::domain::{apple::Apple, snake::Snake, world::World};
use gui::Client;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::{self, Duration},
};

fn main() {
    let mut random = rand::thread_rng();

    let username = format!("user{}", random.gen::<u16>());
    let color = [random.gen::<u8>(), random.gen::<u8>(), random.gen::<u8>()];
    let snake = Snake::spawn(username, color);

    let world = World {
        size: (100, 100),
        update_time: 200,
        spawn_time: 2500,
        snakes: vec![],
        apples: vec![],
    };

    // Таймеры
    let mut update_timer = time::Instant::now();
    let mut spawn_timer = time::Instant::now();
    let update_time = Duration::from_millis(world.update_time);
    let spawn_time = Duration::from_millis(world.spawn_time);

    // Синх
    let snake_updater = Arc::new(Mutex::new(snake));
    let snake_mover = snake_updater.clone();
    let world_draw = Arc::new(Mutex::new(world));
    let world_updater = world_draw.clone();

    std::thread::spawn(move || loop {
        if update_timer.elapsed() > update_time {
            let mut snake = snake_mover.lock().unwrap();
            let mut world = world_updater.lock().unwrap();

            // Передвижение
            snake.moving();
            for (i, apple) in world.apples.iter().enumerate() {
                if snake.collide_with([apple.position]) {
                    snake.grow();
                    world.apples.remove(i);
                    break;
                }
            }

            // Обновление мира
            world.snakes.pop();
            world.snakes.push(snake.clone());

            update_timer = time::Instant::now();
        }

        if spawn_timer.elapsed() > spawn_time {
            let mut world = world_updater.lock().unwrap();
            let mut random = rand::thread_rng();
            world.apples.push(Apple {
                position: (random.gen::<usize>() % 30, random.gen::<usize>() % 30),
            });

            spawn_timer = time::Instant::now();
        }
    });

    Client::run(snake_updater, world_draw);
}
