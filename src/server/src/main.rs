use std::sync::Arc;

use nexus::standard::{bank::Bank, sync::ServerController};
use schrod::Schrod::{Fail, Pass};
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

use crate::{auth::validate_token, init::load_bank};

pub mod auth;
pub mod init;

#[tokio::main]
async fn main() {
    // setting up logging
    let tracing_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));
    tracing_subscriber::fmt()
        .with_env_filter(tracing_filter)
        .init();
    tracing::info!("Initializing server...");

    // loading the bank
    let bank_result = load_bank();
    let bank = match bank_result {
        Pass(bank) => bank,
        Fail(_) => {
            tracing::error!("Failed to load Bank!");
            for er in &bank_result.results() { tracing::error!(er); }
            Bank::default()
        },
    };
    
    // setting up network listening
    let port = "3587";
    let listener = ServerController::create_listener(port).await;
    let router = ServerController::create_router(Arc::new(Mutex::new(bank)));

    // validating the token
    validate_token();

    // serving
    tracing::info!("Serving on internal port: {}", port);
    axum::serve(listener, router).await.expect("This should not crash. - main()");
}