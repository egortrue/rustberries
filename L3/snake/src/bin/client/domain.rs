use std::{collections::LinkedList, time::Duration};

pub struct World {
    pub update_time: u64, // in ms -> FPS
    pub spawn_time: u64,  // apple spawn
    pub snake: Snake,
    pub enemies: Vec<Snake>,
    pub apples: Vec<Apple>,
}

pub struct Apple {
    pub position: (usize, usize),
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    pub username: String,
    pub color: [u8; 3],
    pub positions: LinkedList<(usize, usize)>,
    pub direction: Direction,
}

impl Snake {
    pub fn moving(&mut self) {
        let head = self.positions.front().unwrap();
        match self.direction {
            Direction::UP => self.positions.push_front((head.0, head.1 - 1)),
            Direction::DOWN => self.positions.push_front((head.0, head.1 + 1)),
            Direction::LEFT => self.positions.push_front((head.0 - 1, head.1)),
            Direction::RIGHT => self.positions.push_front((head.0 + 1, head.1)),
        }
        self.positions.pop_back();
    }

    pub fn grow(&mut self) {
        self.positions
            .push_back(self.positions.back().unwrap().clone())
    }

    pub fn check_collision(&self, object: impl IntoIterator<Item = (usize, usize)>) -> bool {
        let head = *self.positions.front().unwrap();
        for collider in object {
            if head == collider {
                return true;
            }
        }
        false
    }
}
