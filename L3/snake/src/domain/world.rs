use crate::domain::apple::Apple;
use crate::domain::snake::Snake;

pub struct World {
    // Конфигурация
    pub size: (usize, usize), // (0, 0) - (size.x, size.y)
    pub update_time: u64,     // ms. частота обновления мира
    pub spawn_time: u64,      // ms. частота появления яблок

    // Объекты
    pub snakes: Vec<Snake>,
    pub apples: Vec<Apple>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            update_time: 200,
            spawn_time: 1000,
            snakes: Vec::new(),
            apples: Vec::new(),
        }
    }
}
