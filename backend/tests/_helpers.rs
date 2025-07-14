use backend::{app_state::AppState, create_router};
use tokio::{net::TcpListener, sync::oneshot};

const DB_URL: &str = "postgres://anydesk:mysecretpassword@localhost:5432/anydesk";

pub async fn spawn_app() -> (String, oneshot::Sender<()>) {
    // 1. Connect to DB
    let state = AppState::new(DB_URL).await.unwrap();
    let app   = create_router(state);

    // 2. Listen any free port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr     = listener.local_addr().unwrap();

    // 3. Graceful-shutdown
    let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(async { let _ = stop_rx.await; });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server error: {e}");
        }
    });

    (format!("http://{addr}"), stop_tx)
}
