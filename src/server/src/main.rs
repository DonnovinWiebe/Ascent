use nexus::standard::sync::ServerController;
use tracing_subscriber::EnvFilter;

use crate::auth::validate_token;

pub mod auth;

#[tokio::main]
async fn main() {
    // setting up logging
    let tracing_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();
    tracing::info!("Initializing server...");
    
    // setting up network listening
    let port = "3587";
    let listener = ServerController::create_listener(port).await;
    let router = ServerController::create_router();

    // validating the token
    validate_token();

    // serving
    tracing::info!("Serving on internal port: {}", port);
    axum::serve(listener, router).await.expect("This should not crash. - main()");
}