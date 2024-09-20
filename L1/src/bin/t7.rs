// L1.7

use std::{sync::Arc, thread, time};

#[tokio::main]
async fn main() {
    thread_close_channel();
    println!("=================================");
    task_close_channel().await;
    println!("=================================");
    task_cancel_token().await;
}

fn thread_close_channel() {
    let (tx, rx) = flume::unbounded();
    let rx = Arc::new(rx);
    let mut handlers = vec![];

    // Получатели (потоки)
    for _ in 0..3 {
        let rx_thread = Arc::clone(&rx);
        let handler = thread::spawn(move || loop {
            if let Ok(data) = rx_thread.recv() {
                println!("{data}");
            } else {
                break;
            }
        });
        handlers.push(handler)
    }

    // Отправитель
    for data in 0..8 {
        tx.send(data).unwrap();
        tx.send(data).unwrap();
        thread::sleep(time::Duration::from_millis(250));
    }

    // Закрываем поток со стороны отправителя
    drop(tx);

    // Ждем пока получатели поймут что поток закрыт и завершаться
    for handler in handlers {
        handler.join().unwrap();
    }
}

async fn task_close_channel() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let mut handlers = tokio::task::JoinSet::new();

    // Отправители (асинхронные таски)
    for _ in 0..3 {
        let tx = tx.clone();
        handlers.spawn(async move {
            for data in 0.. {
                if let Ok(_) = tx.send(data) {
                    continue;
                } else {
                    break;
                }
            }
        });
    }

    // Получатель
    let start = time::Instant::now();
    while time::Instant::now() - start < time::Duration::from_secs(5) {
        let data = rx.recv().await.unwrap();
        println!("{data}");
        thread::sleep(time::Duration::from_millis(250));
    }

    // Закрываем поток со стороны получателя
    rx.close();

    // Ждем пока отправители поймут что поток закрыт и завершаться
    handlers.join_all().await;
}

async fn task_cancel_token() {
    let token = tokio_util::sync::CancellationToken::new();
    let mut handlers = tokio::task::JoinSet::new();

    // Вокреры (асинхронные таски)
    for _ in 0..3 {
        let token = token.clone();
        handlers.spawn(async move {
            for data in 0.. {
                tokio::select! {
                    _ = token.cancelled() => {
                        break;
                    }
                    _ = tokio::time::sleep(std::time::Duration::from_millis(250)) => {
                        println!("working... {data}");
                    }
                }
            }
        });
    }

    // Останавливаем работяг
    thread::sleep(time::Duration::from_secs(5));
    token.cancel();

    // Ждем пока воркеры доделают работу
    handlers.join_all().await;
}
