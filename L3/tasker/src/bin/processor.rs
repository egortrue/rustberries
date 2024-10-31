use notify::{RecursiveMode, Watcher};
use serde_json::json;
use std::{io::BufRead, path::Path, sync::mpsc, time::Duration};
use tasker::{Task, TaskResult};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().expect(".env file not found");
    let input_file = std::env::var("INPUT_FILE").expect("ENV variable not found: INPUT_FILE");
    let output_file = std::env::var("OUTPUT_FILE").expect("ENV variable not found: OUTPUT_FILE");

    // Поток уведомлений об изменениях
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    // Отправитель уведомлений об изменениях
    let mut watcher = notify::recommended_watcher(tx).expect("Failed create watcher");
    watcher
        .watch(Path::new(&input_file), RecursiveMode::Recursive)
        .expect("Failed to start watch");

    // Исполнитель
    for event in rx {
        let ouput_file_async = output_file.clone();

        tokio::spawn(async move {
            if let Ok(event) = event {
                if event.kind.is_modify() {
                    let input_file = event.paths.get(0).unwrap();

                    // Чтение измененного файла
                    let content = tokio::fs::read(input_file).await.unwrap();
                    let line = match content.lines().last() {
                        Some(line) => line,
                        None => return,
                    };

                    // Парисинг задачи
                    let task: Task = match serde_json::from_str(&line.unwrap()) {
                        Ok(task) => task,
                        Err(_) => return,
                    };

                    // Вывод
                    let result: TaskResult = process(task).await;
                    let content = json!(result).to_string() + "\n";
                    let mut file = OpenOptions::new()
                        .append(true)
                        .open(&ouput_file_async)
                        .await
                        .unwrap();

                    let _ = file.write(content.as_bytes()).await;
                }
            }
        });
    }
}

async fn process(task: Task) -> TaskResult {
    let start = tokio::time::Instant::now();
    let output = match task.r#type {
        tasker::TaskType::SLEEP => sleep(task.data).await,
    };
    TaskResult {
        task,
        output,
        elapsed: start.elapsed().as_millis() as usize,
    }
}

async fn sleep(secs: usize) -> String {
    tokio::time::sleep(Duration::from_secs(secs as u64)).await;
    "".to_owned()
}
