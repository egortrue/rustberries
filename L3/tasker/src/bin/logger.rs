use notify::{RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc};

#[tokio::main]
async fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().expect(".env file not found");
    let input_file = std::env::var("INPUT_FILE").expect("ENV variable not found: INPUT_FILE");
    let output_file = std::env::var("OUTPUT_FILE").expect("ENV variable not found: OUTPUT_FILE");

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init();

    // Поток уведомлений об изменениях
    let (tx_input, rx_input) = mpsc::channel::<notify::Result<notify::Event>>();
    let (tx_output, rx_output) = mpsc::channel::<notify::Result<notify::Event>>();

    // Отправитель уведомлений об изменениях
    let mut watcher_input = notify::recommended_watcher(tx_input).expect("Failed create watcher");
    watcher_input
        .watch(Path::new(&input_file), RecursiveMode::NonRecursive)
        .expect("Failed to start watch");

    // Отправитель уведомлений об изменениях
    let mut watcher_output = notify::recommended_watcher(tx_output).expect("Failed create watcher");
    watcher_output
        .watch(Path::new(&output_file), RecursiveMode::NonRecursive)
        .expect("Failed to start watch");

    let mut handler = tokio::task::JoinSet::new();

    handler.spawn(async move {
        for _ in rx_output {
            log::info!("Finished a task");
        }
    });

    handler.spawn(async move {
        for _ in rx_input {
            log::info!("Added new task");
        }
    });

    handler.join_all().await;
}
