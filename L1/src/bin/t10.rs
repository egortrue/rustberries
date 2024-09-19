// L1.10

use core::time;
use std::thread;

const N: usize = 50;

fn main() {
    let (tx1, rx1) = flume::unbounded();
    let (tx2, rx2) = flume::unbounded();

    // Первый поток - умножает
    thread::spawn(move || loop {
        if let Ok(number) = rx1.recv() {
            tx2.send(number * number).unwrap();
        } else {
            break;
        }
    });

    // Второй поток - печатает
    thread::spawn(move || loop {
        if let Ok(number) = rx2.recv() {
            println!("{number}");
        } else {
            break;
        }
    });

    // Пишем в первый поток
    for number in 1..=N {
        tx1.send(number).unwrap();
        thread::sleep(time::Duration::from_millis(250));
    }
}
