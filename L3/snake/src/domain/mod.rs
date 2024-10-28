pub mod apple;
pub mod snake;
pub mod world;

use apple::Apple;
use snake::Snake;
use world::World;

pub enum Collider {
    World(World),
    Apple(Apple),
    Snake(Snake),
}
