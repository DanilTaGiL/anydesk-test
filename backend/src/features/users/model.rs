use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserListItem {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Serialize)]
pub struct UserDetail {
    pub id: Uuid,
    pub full_name: String,
    pub headline: String,
    pub role: UserRole,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    Support,
}