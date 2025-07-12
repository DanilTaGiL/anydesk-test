use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct TaskListItem {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDetail {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
    pub description: String,
    pub creator_id: Uuid,
    pub assigned_to: Uuid,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskCategory {
    Bug,
    Task,
}
