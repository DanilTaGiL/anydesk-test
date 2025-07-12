use crate::api_error::ApiError;
use crate::app_state::AppState;
use crate::features::tasks::model::{
    TaskCreateDTO, TaskDetailsDTO, TaskListItemDTO, TaskUpdateDTO,
};
use crate::features::tasks::repo;

pub async fn get_all_tasks(state: &AppState) -> Result<Vec<TaskListItemDTO>, ApiError> {
    let tasks = repo::get_all_tasks(&state.db).await?;
    Ok(tasks.into_iter().map(Into::into).collect())
}

pub async fn get_task(state: &AppState, id: i32) -> Result<TaskDetailsDTO, ApiError> {
    match repo::get_task_by_id(&state.db, id).await? {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}

pub async fn create_task(state: &AppState, dto: TaskCreateDTO) -> Result<TaskDetailsDTO, ApiError> {
    let dao = repo::create_task(&state.db, dto).await?;
    Ok(dao.into())
}

pub async fn update_task(
    state: &AppState,
    id: i32,
    dto: TaskUpdateDTO,
) -> Result<TaskDetailsDTO, ApiError> {
    match repo::update_task(&state.db, id, dto).await? {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}

pub async fn delete_task(state: &AppState, id: i32) -> Result<(), ApiError> {
    let affected = repo::delete_task(&state.db, id).await?;
    if affected == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(())
    }
}
