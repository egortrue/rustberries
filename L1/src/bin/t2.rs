// L1.2

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

    // Создаем потоки, разделив данные по-ровну
    let mut handlers = vec![];
    for i in 0..threads {
        let handler = thread::spawn(move || {
            let start = 1 + (i * elements_per_thread); // начиная с 1
            let stop = 1 + (i + 1) * elements_per_thread; // не включительно

            for element in start..stop {
                println!("{}", element * element);
            }
        });
        handlers.push(handler);
    }

    // Считаем остатки (их мало) в основном потоке
    let remains_start = N - elements_remain;
    let remains_stop = N;
    for element in remains_start..remains_stop {
        println!("{}", element * element);
    }

    // Ждем остальные потоки
    for handler in handlers {
        handler.join().unwrap();
    }
}
