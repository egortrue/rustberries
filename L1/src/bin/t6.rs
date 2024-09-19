// L1.6
// Регистрируем общее время работы, включая создание потоков и их исполнение

use std::{thread, time};

static N: u64 = 5;

fn main() {
    let start = time::Instant::now();

    // Два канала: один для работы воркеров, второй для отправки сигнала остановки
    let (tx_work, rx_work) = flume::unbounded();
    let (tx_terminate, rx_terminate) = flume::unbounded();

    // Бесконечный отправитель
    thread::spawn(move || loop {
        if let Ok(_) = rx_terminate.try_recv() {
            break;
        }
        tx_work.send("work").unwrap();
    });

    // Бесконечный приемник
    thread::spawn(move || loop {
        match rx_work.recv() {
            Ok(_) => continue,
            Err(_) => break,
        }
    });

    // Простой вариант, но возмножно не самый точный, так как ожидание работает только в рамках одного потока
    // thread::sleep(time::Duration::from_secs(N));

    // Используем сравнение времени, чтобы включить в работу исполнение других потоков и время на их создание
    let mut current = time::Instant::now();
    while current.duration_since(start) < time::Duration::from_secs(N) {
        current = time::Instant::now();
    }

    // Вывод в миллисек
    println!("{:?}ms", current.duration_since(start).as_millis());

    // Отправляем сигнал завершения отправителю
    tx_terminate.send(()).unwrap();
}
