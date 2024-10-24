mod app;
mod domain;

use app::App;
use domain::{Snake, World};
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
    time::{self, Duration},
};

fn main() {
    let world = World {
        snake: Snake {
            color: egui::Color32::BLUE.to_array(),
            positions: LinkedList::from([(10, 10), (9, 10), (8, 10), (7, 10), (6, 10)]),
            direction: domain::Direction::RIGHT,
        },
        enemies: vec![],
        apples: vec![],
    };

    let world = Arc::new(Mutex::new(world));
    let world_update = world.clone();
    let mut deltatime = time::Instant::now();
    std::thread::spawn(move || loop {
        // 3 FPS
        if deltatime.elapsed() > Duration::from_millis(250) {
            world_update.lock().unwrap().snake.moving();
            deltatime = time::Instant::now();
        }
    });

    App::run(&world);
}
