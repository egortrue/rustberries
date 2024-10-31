use serde_json::json;
use std::ops::Rem;
use tasker::Task;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Загрузка переменных из .env-файла
    dotenv::dotenv().expect(".env file not found");
    let input_file = std::env::var("INPUT_FILE").expect("ENV variable not found: INPUT_FILE");
    let timeout = std::env::var("CREATOR_TIMEOUT")
        .expect("ENV variable not found: CREATOR_TIMEOUT")
        .parse::<u64>()
        .unwrap();

    loop {
        let input_file_async = input_file.clone();
        tokio::spawn(async move {
            // Генерация задачи
            let task = Task {
                id: Uuid::new_v4(),
                r#type: tasker::TaskType::SLEEP,
                data: rand::random::<usize>().rem(10),
            };

            // Запись задачи
            let content = json!(task).to_string() + "\n";
            let mut file = tokio::fs::OpenOptions::new()
                .append(true)
                .open(input_file_async.as_str())
                .await
                .unwrap();
            let _ = file.write(content.as_bytes()).await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(timeout)).await;
    }
}
