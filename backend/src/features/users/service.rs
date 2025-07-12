use uuid::Uuid;
use crate::{app_state::AppState, features::users::{model::*, repo}};
use crate::api_error::ApiError;

pub async fn users_list(state: &AppState) -> anyhow::Result<Vec<UserListItemDTO>> {
    let all = repo::get_all(&state.db).await?;
    Ok(all.into_iter().map(Into::into).collect())
}

pub async fn get_user_details(state: &AppState, id: Uuid) -> Result<UserDetailsDTO, ApiError> {
    let result = repo::get_by_id(&state.db, id).await?;
    match result {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}
