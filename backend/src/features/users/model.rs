use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct UserListItem {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Serialize, ToSchema)]
pub struct UserDetail {
    pub id: Uuid,
    pub full_name: String,
    pub headline: String,
    pub role: UserRole,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    Support,
}