use notify::{event, RecursiveMode, Watcher};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};
use tasker::{Task, TaskResult};

#[tokio::main]
async fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().expect(".env file not found");

    // Настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init();

    // Поток уведомлений об изменениях
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    // Отправитель уведомлений об изменениях
    notify::recommended_watcher(tx)
        .expect("Failed create watcher using 'notify'")
        .watch(Path::new("./workdir"), RecursiveMode::Recursive)
        .expect("Failed to start watch using 'notify'");

    // Исполнитель
    for event in rx {
        tokio::spawn(async {
            if let Ok(event) = event {
                if event.kind.is_create() {
                    for path in event.paths {
                        let task = String::new();
                        tokio::fs::read_to_string(&path);
                        let task: Task = match serde_json::from_str(&task) {
                            Ok(task) => task,
                            Err(_) => continue,
                        };

                        let result: TaskResult = process(task);
                        tokio::fs::remove_file(&path);
                    }
                }
            }
        });
    }
}

fn process(task: Task) -> TaskResult {
    let output = String::new();
    TaskResult { task, output }
}
