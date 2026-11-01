use nexus::standard::sync::ServerController;

#[tokio::main]
async fn main() {
    let listener = ServerController::create_listener("3000").await;
    let router = ServerController::create_router();
    axum::serve(listener, router).await.unwrap();
}