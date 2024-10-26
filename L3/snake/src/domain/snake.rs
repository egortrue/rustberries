use std::collections::LinkedList;

#[derive(Clone)]
pub enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Snake {
    pub username: String,
    pub color: [u8; 3],
    pub score: usize,
    pub direction: SnakeDirection,
    pub positions: LinkedList<(usize, usize)>,
}

impl Snake {
    pub fn spawn(username: String, color: [u8; 3]) -> Self {
        let head: (usize, usize) = (15, 15);
        Self {
            username,
            color,
            score: 0,
            direction: SnakeDirection::RIGHT,
            positions: LinkedList::from([
                (head.0, head.1),
                (head.0 - 1, head.1),
                (head.0 - 2, head.1),
            ]),
        }
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

    pub fn collide_with(&self, object: impl IntoIterator<Item = (usize, usize)>) -> bool {
        let head = *self.positions.front().unwrap();
        for collider in object {
            if head == collider {
                return true;
            }
        }
        false
    }
}
