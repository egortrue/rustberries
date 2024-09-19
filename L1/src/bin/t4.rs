// L1.4
// Использовал flume так как интерфейс тот же что и у mpsc

use std::{env, sync::Arc, thread, time::Duration};

fn main() {
    // Парсинг агрумента кол-ва воркеров
    let args: Vec<String> = env::args().collect();
    let workers_input = args
        .get(1)
        .expect("No argument for workers count: *.exe <count>");
    let workers = workers_input
        .parse::<usize>()
        .expect("Argument for workers count is not a number: *.exe <count>");

    // Создание канала
    let (tx, rx) = flume::unbounded();
    let rxs = Arc::new(rx);

    // Бесконечные потоки чтения
    for worker in 0..workers {
        let rx_thread = Arc::clone(&rxs);
        thread::spawn(move || loop {
            // Не очень понятно "произвольные данные"
            // Видимо, поскольку нет никакого механизма синхронизации
            // то поток прочитает первое что пришло на момент его активации
            // - это и является произвольными данными
            println!("w{worker:<3} {}", rx_thread.recv().unwrap());
        });
    }

    // Бесконечный поток записи
    for data in 1.. {
        tx.send(data).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}
