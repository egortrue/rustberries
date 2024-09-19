// L1.5
// Используем простейший крейт ctrlc для обработки и flume для каналов
// Решение: try_recv в каждом потоке через отдельный канал остановки

use std::{sync::Arc, thread, time};

fn main() {
    // Два канала: один для работы воркеров, второй для отправки сигнала остановки
    let (tx_work, rx_work) = flume::unbounded();
    let (tx_terminate, rx_terminate) = flume::unbounded();

    // Создание воркеров
    let workers = 4;
    let tx_work_s = Arc::new(tx_work);
    let rx_terminate_s = Arc::new(rx_terminate);
    for worker in 0..workers {
        let tx_work_t = Arc::clone(&tx_work_s);
        let rx_terminate_t = Arc::clone(&rx_terminate_s);

        // Бесконечный работяга
        thread::spawn(move || loop {
            // Попытаемся считать сигнал остановки
            if let Ok(_) = rx_terminate_t.try_recv() {
                // Если получили сигнал остановки, говорим "пока пока"
                tx_work_t.send(format!("w{worker:<3}: bye bye!")).unwrap();
                break;
            }

            // Продолжаем работу
            tx_work_t.send(format!("w{worker}: working...")).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        });
    }
    drop(tx_work_s); // сбрасываем лишний источник

    // Отслеживание сигнала остановки (запускается в отдельном потоке)
    // Проблема?? - надо знать сколько воркеров, чтобы всем отправить дубли сигнала
    ctrlc::set_handler(move || {
        for _ in 0..workers {
            tx_terminate.send(()).unwrap();
        }
    })
    .unwrap();

    // Чтение результатов воркеров
    for result in rx_work {
        println!("{result}");
    }

    println!("BYE BYE, WORLD!");

    /*
    Пример вывода:

    w1: working...
    w0: working...
    w0: working...
    w3: working...
    w1: working...
    w2: working...  <---- CTRL-C
    w1: bye bye!
    w3: bye bye!
    w0: bye bye!
    w2: bye bye!
    BYE BYE, WORLD!

    */
}
