// L2.7
// Разделяем файл по кускам (используя mutex и seek) и обрабатываем каждый кусок в отдельном потоке
// Основной поток объединяет результаты и выводит JSON в консоль

/*

Usage: t7.exe [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -t, --threads <THREADS>  Кол-во поток для выполнения задачи [default: 1]
  -h, --help               Print help

*/

use clap::Parser;
use core::str;
use serde_json::json;
use std::{
    collections::HashMap,
    fs,
    io::{Read, Seek, SeekFrom},
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread, time,
};

#[derive(Parser)]
struct Args {
    file: String,

    /// Кол-во поток для выполнения задачи
    #[clap(short, long, default_value = "1")]
    threads: usize,
}

fn main() {
    let start = time::Instant::now();
    let args = Args::parse();
    let mut counter: HashMap<char, usize> = HashMap::with_capacity(26 * 2);

    // Открываем файл только на чтение и получаем его размер из метаданных (без чтения)
    let file = Arc::new(Mutex::new(
        fs::File::open(&args.file).expect("Couldn't open the file"),
    ));
    let bytes_count = file.lock().unwrap().metadata().unwrap().len() as usize;
    let bytes_per_thread = bytes_count / args.threads;
    let mut bytes_remains = bytes_count % args.threads;

    if args.threads > 1 {
        // Создаем канал
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(tx);

        // Создаем потоки
        for i in 0..args.threads {
            let file_thread = Arc::clone(&file);
            let tx_thread = Arc::clone(&tx);
            thread::spawn(move || {
                // Чтение части файла
                let string = read_partition(&file_thread, i * bytes_per_thread, bytes_per_thread);

                // Создание и отправка локального счетчика букв
                let mut counter_thread: HashMap<char, usize> = HashMap::with_capacity(26 * 2);
                count_letters(&mut counter_thread, &string);
                tx_thread.send(counter_thread).expect("Failed to send data");
            });
        }
        drop(tx); // сбрасываем лишний источник

        // Получим ответ от потоков и объеденим все счетчики
        for local_counter in rx {
            for (k, v) in local_counter.iter() {
                if counter.contains_key(k) {
                    *counter.get_mut(k).unwrap() += v;
                } else {
                    counter.insert(*k, *v);
                }
            }
        }
    } else {
        bytes_remains = bytes_count;
    }

    // Прочитаем остатки байт в основном потоке
    let string = read_partition(&file, bytes_count - bytes_remains, bytes_remains);
    count_letters(&mut counter, &string);
    let elapsed = start.elapsed();

    // Вывод результатов
    let result = json!({
        "elapsed": format!("{:?}", elapsed),
        "result": counter,
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

/// Чтение области файла с диска в оперативную память
fn read_partition(file: &Arc<Mutex<fs::File>>, start: usize, size: usize) -> String {
    let mut buffer = vec![0; size];
    let mut file_mutex = file.lock().unwrap();
    file_mutex.seek(SeekFrom::Start(start as u64)).unwrap();
    file_mutex.read_exact(&mut buffer).unwrap();
    drop(file_mutex);

    // Конвертация байт в utf8 строку
    str::from_utf8(&buffer).expect("Invalid UTF-8").to_string()
}

fn count_letters(counter: &mut HashMap<char, usize>, string: &str) {
    for ch in string.chars() {
        if !ch.is_ascii_alphabetic() {
            continue;
        }

        if counter.contains_key(&ch) {
            *counter.get_mut(&ch).unwrap() += 1;
        } else {
            counter.insert(ch, 1);
        }
    }
}
