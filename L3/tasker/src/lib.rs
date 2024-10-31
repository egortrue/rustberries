use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum TaskType {
    SLEEP,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub r#type: TaskType,
    pub data: usize,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResult {
    pub task: Task,
    pub elapsed: usize,
    pub output: String,
}
