use crate::api_error::ApiError;
use crate::features::users::model::{UserDAO, UserDetailsDTO, UserListItemDTO, UserRole};
use crate::features::users::repo::MockUsersRepo;
use crate::features::users::service::{get_all, get_user_details};
use serde_json::Value;
use uuid::Uuid;

/* Unit tests for model */
#[test]
fn list_dto_full_name_ok() {
    let dao = UserDAO {
        id: Uuid::nil(),
        first_name: "Alice".into(),
        last_name: "Smith".into(),
        role: UserRole::ADMIN,
        ..Default::default()
    };
    let dto: UserListItemDTO = dao.into();
    assert_eq!(dto.full_name, "Alice Smith");
}

#[test]
fn details_dto_with_empty_headline() {
    let dao = UserDAO {
        id: Uuid::nil(),
        first_name: "Bob".into(),
        last_name: "Jones".into(),
        role: UserRole::DEVELOPER,
        headline: None,
    };
    let dto: UserDetailsDTO = dao.into();
    assert_eq!(dto.headline, "");
}

#[test]
fn list_dto_camel_case_keys() {
    let dto = UserListItemDTO {
        id: Uuid::nil(),
        full_name: "X".into(),
        role: UserRole::SUPPORT,
    };
    let json: Value = serde_json::to_value(&dto).unwrap();
    assert!(json.get("fullName").is_some());
    assert!(json.get("full_name").is_none());
}

#[test]
fn user_role_in_uppercase() {
    let j = serde_json::to_string(&UserRole::ADMIN).unwrap();
    assert_eq!(j, r#""ADMIN""#);

    let role: UserRole = serde_json::from_str(&j).unwrap();
    assert_eq!(role, UserRole::ADMIN);
}

/* Unit tests for service */
#[tokio::test]
async fn details_returns_not_found() {
    let mut mock = MockUsersRepo::new();
    mock.expect_get_by_id()
        .return_once(|_id| Ok(None));

    let result = get_user_details(&mock, Uuid::nil()).await;
    assert!(matches!(result, Err(ApiError::NotFound)));
}

#[tokio::test]
async fn details_happy_path() {
    let mut mock = MockUsersRepo::new();
    mock.expect_get_by_id()
        .return_once(|_| Ok(Some(UserDAO {
            id: Uuid::nil(),
            first_name: "A".into(),
            last_name:  "B".into(),
            headline: None,
            role: UserRole::ADMIN,
        })));

    let dto = get_user_details(&mock, Uuid::nil()).await.unwrap();
    assert_eq!(dto.first_name, "A");
    assert_eq!(dto.last_name, "B");
}

#[tokio::test]
async fn list_maps_to_dto() {
    let mut mock = MockUsersRepo::new();
    mock.expect_get_all()
        .return_once(|| Ok(vec![UserDAO {
            id: Uuid::nil(),
            first_name: "X".into(),
            last_name:  "Y".into(),
            headline: None,
            role: UserRole::SUPPORT,
        }]));

    let list = get_all(&mock).await.unwrap();
    assert_eq!(list[0].full_name, "X Y");
}
