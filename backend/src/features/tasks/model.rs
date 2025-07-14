use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

/* Database model */
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct TaskDAO {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
    pub creator_id: Uuid,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
}

/* DTO models */
// create / update
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreateDTO {
    pub title: String,
    pub category: TaskCategory,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub assigned_to: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateDTO {
    pub title: Option<String>,
    pub category: Option<TaskCategory>,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
}

// outbound
#[derive(Serialize, ToSchema)]
pub struct TaskListItemDTO {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDetailsDTO {
    pub id: i32,
    pub title: String,
    pub category: TaskCategory,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub assigned_to: Option<Uuid>,
}

/* enums */
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, ToSchema, Type)]
#[sqlx(type_name = "task_category", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskCategory {
    BUG,
    TASK,
    RESEARCH,
}

/* Converters */
impl From<TaskDAO> for TaskListItemDTO {
    fn from(x: TaskDAO) -> Self {
        Self {
            id: x.id,
            title: x.title,
            category: x.category,
        }
    }
}
impl From<TaskDAO> for TaskDetailsDTO {
    fn from(x: TaskDAO) -> Self {
        Self {
            id: x.id,
            title: x.title,
            category: x.category,
            description: x.description,
            creator_id: x.creator_id,
            assigned_to: x.assigned_to,
        }
    }
}
