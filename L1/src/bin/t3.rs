// L1.3

use std::sync::{mpsc, Arc};
use std::thread;

static N: usize = 500_000;

fn main() {
    // Подготовка
    let mut threads = thread::available_parallelism().unwrap().get();
    if threads > N {
        threads = N;
    }
    let elements_per_thread = N / threads;
    let elements_remain = N % threads;

    // Создаем канал
    let (tx, rx) = mpsc::channel();
    let txs = Arc::new(tx);

    // Создаем потоки отправки локальной суммы, разделив данные по-ровну
    for i in 0..threads {
        let tx_thread = Arc::clone(&txs);
        thread::spawn(move || {
            let start = 1 + (i * elements_per_thread); // начиная с 1
            let stop = 1 + (i + 1) * elements_per_thread; // не включительно (N + 1)

            let mut local_sum = 0;
            for element in start..stop {
                local_sum += element * element;
            }

            tx_thread.send(local_sum).unwrap();
        });
    }
    drop(txs); // сбрасываем лишний источник

    let mut result = 0;

    // Посчитаем остатки
    let remains_start = N - elements_remain;
    let remains_stop = N;
    for element in remains_start..remains_stop {
        result += element * element;
    }

    // Получим ответ от потоков
    for thread_result in rx {
        result += thread_result;
    }

    println!("{result}");
}
