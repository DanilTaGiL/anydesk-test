use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::users::handler::all_users,
        crate::features::users::handler::get_user,
        crate::features::tasks::handler::all_tasks,
        crate::features::tasks::handler::get_task,
        crate::features::tasks::handler::create_task,
        crate::features::tasks::handler::update_task,
        crate::features::tasks::handler::delete_task,
    ),
    components(
        schemas(
            crate::features::users::model::UserListItemDTO,
            crate::features::users::model::UserDetailsDTO,
            crate::features::users::model::UserRole,
            crate::features::tasks::model::TaskListItemDTO,
            crate::features::tasks::model::TaskDetailsDTO,
            crate::features::tasks::model::TaskCreateDTO,
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
