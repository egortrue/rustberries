use app::domain::{collider::Collider, snake::Snake, world::World};
use std::{
    env,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

pub fn run(snake: Arc<RwLock<Snake>>, world: Arc<RwLock<World>>) {
    let timeout = Duration::from_millis(
        env::var("WORLD_UPDATE_TIME")
            .expect("ENV not found: WORLD_UPDATE_TIME")
            .parse()
            .unwrap(),
    );

    thread::spawn(move || loop {
        thread::sleep(timeout);
        if !snake.read().unwrap().is_alive() {
            continue;
        }

        let mut snake = snake.write().unwrap();
        let world = world.read().unwrap();
        snake.moving();

        if snake.collide_with(Collider::World(world.size)) {
            snake.die();
            continue;
        }

        for enemy in world.snakes.iter() {
            if snake.username != enemy.username {
                if snake.collide_with(Collider::Snake(enemy.positions.clone())) {
                    snake.die();
                    continue;
                }
            }
        }

        for apple in world.apples.iter() {
            if snake.collide_with(Collider::Apple(apple.position)) {
                snake.grow();
            }
        }
    });
}
