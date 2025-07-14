pub mod apidoc;
pub mod app_state;
pub mod api_error;
pub mod features;

pub use apidoc::ApiDoc;

pub fn create_router(state: app_state::AppState) -> axum::Router {
    use axum::{http, Router};
    use axum::http::Method;
    use tower_http::cors::{Any, CorsLayer};
    use crate::features::{tasks, users};

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>().unwrap())
        .allow_origin("http://localhost".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .merge(users::handler::router())
        .merge(tasks::handler::router())
        .layer(cors)
        .with_state(state)
}