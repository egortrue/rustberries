mod app;
mod domain;

use app::App;
use domain::{Apple, Snake, World};
use rand::Rng;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
    time::{self, Duration},
};

fn main() {
    let mut random = rand::thread_rng();
    let world = World {
        update_time: 200,
        spawn_time: 1000,
        snake: Snake {
            username: format!("user{}", random.gen::<u16>()),
            color: [random.gen::<u8>(), random.gen::<u8>(), random.gen::<u8>()],
            positions: LinkedList::from([(10, 10), (9, 10), (8, 10)]),
            direction: domain::Direction::RIGHT,
        },
        enemies: vec![],
        apples: vec![],
    };

    // Таймеры
    let mut update_timer = time::Instant::now();
    let mut spawn_timer = time::Instant::now();
    let update_time = Duration::from_millis(world.update_time);
    let spawn_time = Duration::from_millis(world.spawn_time);

    // Синх
    let world = Arc::new(Mutex::new(world));
    let world_update = world.clone();

    std::thread::spawn(move || loop {
        if update_timer.elapsed() > update_time {
            let mut world = world_update.lock().unwrap();
            world.snake.moving();

            for (i, apple) in world.apples.iter().enumerate() {
                if (world.snake.check_collision([apple.position])) {
                    world.snake.grow();
                    world.apples.remove(i);
                    break;
                }
            }

            update_timer = time::Instant::now();
        }

        if spawn_timer.elapsed() > spawn_time {
            let mut world = world_update.lock().unwrap();
            let mut random = rand::thread_rng();
            world.apples.push(Apple {
                position: (random.gen::<usize>() % 30, random.gen::<usize>() % 30),
            });

            spawn_timer = time::Instant::now();
        }
    });

    App::run(&world);
}
