// L1.17

use std::{
    ops::AddAssign,
    sync::{Arc, Mutex},
    thread, time,
};

struct Counter {
    value: Mutex<usize>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    fn increment(&self) {
        self.value.lock().unwrap().add_assign(1);
    }

    fn get(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

fn main() {
    let counter = Arc::new(Counter::new());

    let counter1 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            counter1.increment();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        for _ in 0..10 {
            counter2.increment();
            thread::sleep(time::Duration::from_millis(300));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", counter.get());
    assert_eq!(counter.get(), 20);
}
