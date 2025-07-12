use uuid::Uuid;
use crate::{app_state::AppState, features::users::{model::*, repo}};
use crate::api_error::ApiError;

pub async fn get_all(state: &AppState) -> anyhow::Result<Vec<UserListItemDTO>> {
    let all = repo::get_all_users(&state.db).await?;
    Ok(all.into_iter().map(Into::into).collect())
}

pub async fn get_user_details(state: &AppState, id: Uuid) -> Result<UserDetailsDTO, ApiError> {
    match repo::get_user_by_id(&state.db, id).await? {
        Some(dao) => Ok(dao.into()),
        None => Err(ApiError::NotFound),
    }
}
