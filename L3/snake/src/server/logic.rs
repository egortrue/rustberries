use app::domain::{collider::Collider, world::World};
use std::{
    env,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub fn run(world: Arc<RwLock<World>>) {
    spawn_apples(world.clone());
    check_collisions(world.clone());
}

fn spawn_apples(world: Arc<RwLock<World>>) {
    let timeout = Duration::from_millis(env::var("APPLE_SPAWN_TIME").unwrap().parse().unwrap());
    let apples_limit = env::var("APPLE_SPAWN_LIMIT")
        .unwrap_or("40".to_string())
        .parse::<usize>()
        .expect("ENV not found: APPLE_SPAWN_LIMIT");
    thread::spawn(move || loop {
        thread::sleep(timeout);
        let mut world = world.write().unwrap();
        if world.apples.len() < apples_limit {
            world.spawn_apple();
        }
    });
}

fn check_collisions(world: Arc<RwLock<World>>) {
    let timeout = Duration::from_millis(env::var("WORLD_UPDATE_TIME").unwrap().parse().unwrap());

    let mut apples_destroy = vec![];
    thread::spawn(move || loop {
        thread::sleep(timeout);
        let mut world = world.write().unwrap();

        for snake in world.snakes.iter() {
            for (index, apple) in world.apples.iter().enumerate() {
                if snake.collide_with(Collider::Apple(apple.position)) {
                    apples_destroy.push(index);
                }
            }
        }

        for index in apples_destroy.iter() {
            world.apples.remove(*index);
        }
        apples_destroy.clear();
    });
}
