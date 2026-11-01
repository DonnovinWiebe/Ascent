use nexus::standard::sync::ServerController;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let tracing_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();
    let listener = ServerController::create_listener("3000").await;
    let router = ServerController::create_router();
    axum::serve(listener, router).await.expect("This should not crash. - main()");
}