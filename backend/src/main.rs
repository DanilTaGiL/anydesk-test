mod apidoc;
mod features;
mod app_state;
mod api_error;

use crate::apidoc::ApiDoc;
use crate::features::{tasks, users};
use axum::{Router, serve, http};
use axum::http::Method;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /* envs */
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    /* state */
    let state = app_state::AppState::new(&database_url).await?;

    /* logs */
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info")) // RUST_LOG=debug cargo run
        .with(tracing_subscriber::fmt::layer())
        .init();

    /* routing */
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<http::HeaderValue>()?)
        .allow_origin("http://localhost".parse::<http::HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .merge(users::handler::router())
        .merge(tasks::handler::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(state.clone());

    /* listener and start up */
    let listener = TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("Failed to bind");
    tracing::info!("🚀  Server listening on {}", listener.local_addr().unwrap());

    serve(listener, app).await.expect("Server error");
    Ok(())
}
