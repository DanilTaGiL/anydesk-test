mod api_error;

use backend::{ApiDoc, app_state, create_router};
use tokio::net::TcpListener;
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
    let app = create_router(state.clone())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    /* listener and start up */
    let listener = TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("Failed to bind");
    tracing::info!("🚀  Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("Server error");
    Ok(())
}
