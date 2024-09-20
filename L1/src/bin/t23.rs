// L1.23

use geometry::Point;

// инкапсуляция в модуле
mod geometry {
    pub struct Point {
        x: f64,
        y: f64,
    }

    // Два конструктора на выбор
    impl Point {
        pub fn new() -> Self {
            Point { x: 0.0, y: 0.0 }
        }

        pub fn from(x: f64, y: f64) -> Self {
            Point { x, y }
        }

        pub fn distance(&self, other: &Self) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }
}

fn main() {
    let point1 = Point::from(3.0, 4.0);
    let point2 = Point::new();

    println!("distance = {}", point1.distance(&point2));
    assert_eq!(point1.distance(&point2), 5.0);
    assert_eq!(point1.distance(&point2), point2.distance(&point1));
}
