use serde_json::{json, Value};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::features::tasks::model::{TaskCategory, TaskCreateDTO, TaskDAO, TaskDetailsDTO, TaskListItemDTO, TaskUpdateDTO};
use crate::features::tasks::repo::MockTasksRepo;
use crate::features::tasks::service::{create_task, delete_task, get_all_tasks, get_task, update_task};

/* Unit tests for model */
#[test]
fn list_dto_mapping_ok() {
    let dao = TaskDAO {
        id: 42,
        title: "Some bug title".into(),
        category: TaskCategory::BUG,
        creator_id: Uuid::nil(),
        description: None,
        assigned_to: None,
    };
    let dto: TaskListItemDTO = dao.clone().into();
    assert_eq!(dto.id, dao.id);
    assert_eq!(dto.title, dao.title);
    assert_eq!(dto.category, dao.category);
}

#[test]
fn details_dto_mapping_ok() {
    let dao = TaskDAO {
        id: 1,
        title: "Research caching".into(),
        category: TaskCategory::RESEARCH,
        creator_id: Uuid::nil(),
        description: Some("bench Redis vs. Mem".into()),
        assigned_to: Some(Uuid::nil()),
    };
    let dto: TaskDetailsDTO = dao.clone().into();
    assert_eq!(dto.description, dao.description);
    assert_eq!(dto.assigned_to, dao.assigned_to);
}

#[test]
fn details_dto_camelcase() {
    let dto = TaskDetailsDTO {
        id: 1,
        title: "Some title".into(),
        category: TaskCategory::TASK,
        description: None,
        creator_id: Uuid::nil(),
        assigned_to: None,
    };
    let val: Value = serde_json::to_value(dto).unwrap();
    assert!(val.get("creatorId").is_some());
    assert!(val.get("creator_id").is_none());
}

#[test]
fn task_category_in_uppercase() {
    let j = serde_json::to_string(&TaskCategory::BUG).unwrap();
    assert_eq!(j, r#""BUG""#);
    let de: TaskCategory = serde_json::from_str(&j).unwrap();
    assert_eq!(de, TaskCategory::BUG);
}

#[test]
fn update_dto_optionals_none() {
    let incoming: Value = json!({ "title": "New title" });
    let dto: TaskUpdateDTO = serde_json::from_value(incoming).unwrap();
    assert_eq!(dto.title, Some("New title".into()));
    assert!(dto.category.is_none());
    assert!(dto.assigned_to.is_none());
}

/* Unit tests for service */
fn dao_stub(id: i32) -> TaskDAO {
    TaskDAO {
        id,
        title: format!("Task {id}"),
        category: TaskCategory::TASK,
        creator_id: Uuid::nil(),
        description: None,
        assigned_to: None,
    }
}

#[tokio::test]
async fn list_maps_to_dto() {
    let mut mock = MockTasksRepo::new();
    mock.expect_get_all()
        .return_once(|| Ok(vec![dao_stub(1)]));

    let list = get_all_tasks(&mock).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, 1);
    assert_eq!(list[0].title, "Task 1");
}

#[tokio::test]
async fn get_by_id_returns_not_found() {
    let mut mock = MockTasksRepo::new();
    mock.expect_get_by_id()
        .return_once(|_id| Ok(None));

    let res = get_task(&mock, 42).await;
    assert!(matches!(res, Err(ApiError::NotFound)));
}

#[tokio::test]
async fn get_happy_path() {
    let mut mock = MockTasksRepo::new();
    mock.expect_get_by_id()
        .return_once(|_id| Ok(Some(dao_stub(2))));

    let dto = get_task(&mock, 2).await.unwrap();
    assert_eq!(dto.id, 2);
    assert_eq!(dto.title, "Task 2");
}

#[tokio::test]
async fn create_task_returns_dto() {
    let mut mock = MockTasksRepo::new();
    mock.expect_create()
        .return_once(|dto: TaskCreateDTO| {
            Ok(TaskDAO {
                id: 10,
                title: dto.title,
                category: dto.category,
                creator_id: dto.creator_id,
                description: dto.description,
                assigned_to: dto.assigned_to,
            })
        });

    let dto = TaskCreateDTO {
        title: "New".into(),
        category: TaskCategory::BUG,
        description: None,
        creator_id: Uuid::nil(),
        assigned_to: None,
    };
    let result = create_task(&mock, dto).await.unwrap();
    assert_eq!(result.id, 10);
    assert_eq!(result.category, TaskCategory::BUG);
}

#[tokio::test]
async fn update_returns_not_found() {
    let mut mock = MockTasksRepo::new();
    mock.expect_update()
        .return_once(|_, _| Ok(None));

    let res = update_task(&mock, 7, TaskUpdateDTO { title: None, category: None, description: None, assigned_to: None }).await;
    assert!(matches!(res, Err(ApiError::NotFound)));
}

#[tokio::test]
async fn delete_returns_not_found() {
    let mut mock = MockTasksRepo::new();
    mock.expect_delete()
        .return_once(|_| Ok(0));

    let res = delete_task(&mock, 1).await;
    assert!(matches!(res, Err(ApiError::NotFound)));
}
