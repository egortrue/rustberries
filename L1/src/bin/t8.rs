// L1.8

use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn use_hashmap() {
    let hashmap = Arc::new(Mutex::new(HashMap::new()));
    let mut handlers = vec![];

    for id in 0..5 {
        let hashmap_thread = Arc::clone(&hashmap);
        let handler = thread::spawn(move || {
            let mut hashmap = hashmap_thread.lock().unwrap();
            hashmap.insert(format!("thread #{id}"), id);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap()
    }

    println!("{:?}", hashmap.lock().unwrap());
}

fn use_dashmap() {
    let dashmap = Arc::new(DashMap::new());
    let mut handlers = vec![];

    for id in 0..5 {
        let dashmap_thread = Arc::clone(&dashmap);
        let handler = thread::spawn(move || {
            dashmap_thread.insert(format!("thread #{id}"), id);
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap()
    }

    println!("{:?}", dashmap);
}

fn main() {
    use_hashmap();
    use_dashmap();
}
