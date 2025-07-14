use crate::api_error::ApiError;
use crate::features::tasks::model::{
    TaskCreateDTO, TaskDetailsDTO, TaskListItemDTO, TaskUpdateDTO,
};
use crate::features::tasks::repo::TasksRepo;

pub async fn get_all_tasks<R: TasksRepo>(repo: &R) -> Result<Vec<TaskListItemDTO>, ApiError> {
    let tasks = repo.get_all().await?;
    Ok(tasks.into_iter().map(Into::into).collect())
}

pub async fn get_task<R: TasksRepo>(repo: &R, id: i32) -> Result<TaskDetailsDTO, ApiError> {
    match repo.get_by_id(id).await? {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}

pub async fn create_task<R: TasksRepo>(
    repo: &R,
    dto: TaskCreateDTO,
) -> Result<TaskDetailsDTO, ApiError> {
    let dao = repo.create(dto).await?;
    Ok(dao.into())
}

pub async fn update_task<R: TasksRepo>(
    repo: &R,
    id: i32,
    dto: TaskUpdateDTO,
) -> Result<TaskDetailsDTO, ApiError> {
    match repo.update(id, dto).await? {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}

pub async fn delete_task<R: TasksRepo>(repo: &R, id: i32) -> Result<(), ApiError> {
    let affected = repo.delete(id).await?;
    if affected == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(())
    }
}
