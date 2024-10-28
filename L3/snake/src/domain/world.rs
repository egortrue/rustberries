use crate::domain::apple::Apple;
use crate::domain::snake::Snake;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct World {
    // Конфигурация
    pub size: (usize, usize), // Размер мира (включая границы) ((0, 0) - (x, y))
    pub update_time: u64,     // Частота обновления мира (мс)
    pub spawn_time: u64,      // Частота появления яблок (мс)

    // Объекты
    pub snakes: Vec<Snake>,
    pub apples: Vec<Apple>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            update_time: 166,
            spawn_time: 3000,
            snakes: Vec::new(),
            apples: Vec::new(),
        }
    }

    pub fn spawn_snake(&mut self, snake: Snake) {
        self.snakes.push(snake);
    }

    pub fn kill_snake(&mut self, snake: &Snake) {
        if let Some((index, _)) = self
            .snakes
            .iter()
            .enumerate()
            .find(|(_, el)| el.username == snake.username)
        {
            self.snakes.swap_remove(index);
        }
    }

    pub fn spawn_apple(&mut self) {
        self.apples.push(Apple::new(
            (thread_rng().gen::<usize>() % self.size.0).clamp(2, self.size.0 - 2),
            (thread_rng().gen::<usize>() % self.size.1).clamp(2, self.size.0 - 2),
        ));
    }
}
