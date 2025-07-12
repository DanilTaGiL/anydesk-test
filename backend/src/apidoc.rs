use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::users::handler::users_list,
        crate::features::users::handler::get_user,
        crate::features::tasks::handler::list_tasks,
        crate::features::tasks::handler::get_task,
        crate::features::tasks::handler::create_task,
    ),
    components(
        schemas(
            crate::features::users::model::UserListItemDTO,
            crate::features::users::model::UserDetailsDTO,
            crate::features::users::model::UserRole,
            crate::features::tasks::model::TaskListItem,
            crate::features::tasks::model::TaskDetail,
            crate::features::tasks::model::TaskCreate,
            crate::features::tasks::model::TaskCategory,
        )
    ),
    info(
        title = "Test assignment for AnyDesk",
        version = "0.1.0",
        description = "Test assignment for AnyDesk"
    )
)]
pub struct ApiDoc;
