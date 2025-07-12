use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use uuid::Uuid;

/* Database model */
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDAO {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub headline: Option<String>,
    pub role: UserRole,
}

/* DTO models */
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserListItemDTO {
    pub id: Uuid,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDetailsDTO {
    pub id: Uuid,
    pub full_name: String,
    pub headline: String,
    pub role: UserRole,
}

/* enums */
#[derive(Serialize, Deserialize, ToSchema, Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    ADMIN,
    SUPPORT,
    DEVELOPER
}

/* Converters */
impl From<UserDAO> for UserListItemDTO {
    fn from(dao: UserDAO) -> Self {
        Self {
            id: dao.id,
            full_name: format!("{} {}", dao.first_name, dao.last_name),
            role: dao.role,
        }
    }
}

impl From<UserDAO> for UserDetailsDTO {
    fn from(dao: UserDAO) -> Self {
        Self {
            id: dao.id,
            full_name: format!("{} {}", dao.first_name, dao.last_name),
            headline: dao.headline.unwrap_or_default(),
            role: dao.role,
        }
    }
}
