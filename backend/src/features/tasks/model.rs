use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskCreate {
    pub title: String,
    pub category: TaskCategory,
    pub description: String,
    pub creator_id: Uuid,
    pub assigned_to: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct TaskListItem {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskDetail {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
    pub description: String,
    pub creator_id: Uuid,
    pub assigned_to: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskCategory {
    Bug,
    Task,
}
