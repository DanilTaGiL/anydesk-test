use crate::api_error::ApiError;
use crate::features::users::repo::UsersRepo;
use crate::features::users::model::*;
use uuid::Uuid;

pub async fn get_all<R: UsersRepo>(repo: &R) -> anyhow::Result<Vec<UserListItemDTO>> {
    let all = repo.get_all().await?;
    Ok(all.into_iter().map(Into::into).collect())
}

pub async fn get_user_details<R: UsersRepo>(
    repo: &R,
    id: Uuid,
) -> Result<UserDetailsDTO, ApiError> {
    match repo.get_by_id(id).await? {
        Some(dao) => Ok(dao.into()),
        None      => Err(ApiError::NotFound),
    }
}
