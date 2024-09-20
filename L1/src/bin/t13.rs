// L1.13

use std::io::{self, BufRead};

// Простой и не оптимальный вариант >_<
// Читаем все - сортируем - убираем дубли
fn simple() -> Vec<String> {
    let mut lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines.sort();
    lines.dedup();
    lines
}

// Работает в случае если дубликаты идут подряд
// Зато с O(1) по времени и памяти
fn duplicated() {
    let mut last = String::new();

    io::stdin().lock().lines().for_each(|line| {
        let line = line.unwrap();
        if line != last {
            println!("{line}");
        }
        last = line;
    });
}

fn main() {
    let lines = simple();
    for line in lines {
        println!("{line}")
    }

    println!("============================");

    duplicated();
}
