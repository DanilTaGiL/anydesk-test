use uuid::Uuid;
use crate::{app_state::AppState, features::users::{model::*, repo}};

pub async fn users_list(state: &AppState) -> anyhow::Result<Vec<UserListItemDTO>> {
    let all = repo::get_all(&state.db).await?;
    Ok(all.into_iter().map(Into::into).collect())
}

pub async fn get_user_details(state: &AppState, id: Uuid) -> anyhow::Result<UserDetailsDTO> {
    let dao = repo::get_by_id(&state.db, id).await?;
    Ok(dao.into())
}
