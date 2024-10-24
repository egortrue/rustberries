use std::collections::LinkedList;

pub struct World {
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
    pub color: [u8; 4],
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
}
