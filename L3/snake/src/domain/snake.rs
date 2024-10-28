use super::collider::Collider;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::LinkedList;

#[derive(Clone, Serialize, Deserialize)]
pub enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Snake {
    alive: bool,
    pub username: String,
    pub color: [u8; 3],
    pub score: usize,
    pub direction: SnakeDirection,
    pub positions: LinkedList<(usize, usize)>,
}

impl Snake {
    fn new(username: String, color: [u8; 3], position: (usize, usize)) -> Self {
        Self {
            alive: false,
            username,
            color,
            score: 0,
            direction: SnakeDirection::RIGHT,
            positions: LinkedList::from([
                (position.0, position.1),
                (position.0 - 1, position.1),
                (position.0 - 2, position.1),
            ]),
        }
    }

    pub fn random() -> Self {
        let mut random = thread_rng();
        Self::new(
            format!("user{}", random.gen::<u16>()),
            [random.gen::<u8>(), random.gen::<u8>(), random.gen::<u8>()],
            (
                (random.gen::<usize>() % 10).clamp(5, 10),
                (random.gen::<usize>() % 10).clamp(5, 10),
            ),
        )
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn alive(&mut self) {
        self.alive = true;
        self.score = 0;
        self.positions = Snake::random().positions;
        self.direction = SnakeDirection::RIGHT;
    }

    pub fn die(&mut self) {
        self.alive = false;
    }

    pub fn moving(&mut self) {
        let head: &(usize, usize) = self.positions.front().unwrap();
        let new_head = match self.direction {
            SnakeDirection::UP => (head.0, head.1 - 1),
            SnakeDirection::DOWN => (head.0, head.1 + 1),
            SnakeDirection::LEFT => (head.0 - 1, head.1),
            SnakeDirection::RIGHT => (head.0 + 1, head.1),
        };

        self.positions.push_front(new_head);
        self.positions.pop_back();
    }

    pub fn grow(&mut self) {
        self.score += 1;
        self.positions
            .push_back(self.positions.back().unwrap().clone())
    }

    pub fn collide_with(&self, object: Collider) -> bool {
        let head = *self.positions.front().unwrap();
        match object {
            Collider::Apple(apple) => apple == head,
            Collider::Snake(snake) => snake.contains(&head),
            Collider::World(world) => {
                if head.0 == 0 || head.0 == world.0 - 1 || head.1 == 1 || head.1 == world.1 - 1 {
                    true
                } else {
                    false
                }
            }
        }
    }
}
