mod features;

use crate::features::{tasks, users};
use axum::{Router, serve};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    /* ───── Логирование ───── */
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info")) // RUST_LOG=debug cargo run
        .with(tracing_subscriber::fmt::layer())
        .init();

    /* ───── Роуты ───── */
    let app = Router::new()
        .merge(users::router())
        .merge(tasks::router());

    /* ───── Слушатель и запуск ───── */
    let listener = TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("Failed to bind");
    tracing::info!("🚀  Server listening on {}", listener.local_addr().unwrap());

    serve(listener, app).await.expect("Server error");
}
