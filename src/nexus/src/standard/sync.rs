use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::standard::{bank::BankAction, transaction::Transaction};

pub const DEFAULT_ROUTE: &'static str = "/sync";



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Request {
    Ping,
    Perform { action: BankAction, starting_version: Uuid, resulting_version: Uuid },
    ForceUpdate { updated_ledger: Vec<Transaction>, updated_version: Uuid, context: Context },
    RequestLastRevision { context: Context },
    RequestLedger { context: Context },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Pong,
    Approve,
    SendLastRevision { current_revision: Uuid, context: Context },
    SendLedger { ledger: Vec<Transaction>, context: Context },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Context {
    Audit,
    VersionMismatch,
    IncompatibleHistories,
    MergeConflict,
    MergeConflictResolution,
    Unexpected,
}



pub struct ClientController {
    server_address: String,
}
impl ClientController {
    pub fn new(server_address: String) -> ClientController {
        ClientController { server_address }
    }

    pub async fn send(&self, request: Request) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
                .post(&self.server_address)
                .json(&json!(request))
                .send().await?
                .error_for_status()?
                .json::<Response>().await
    }
}



pub mod ServerController {
    use axum::{Json, Router, routing::post};
    use tokio::net::TcpListener;
    use crate::standard::sync::{DEFAULT_ROUTE, Response};

    pub fn create_router() -> Router { Router::new().route(DEFAULT_ROUTE, post(handle_request)) }

    pub async fn create_listener(port: String) -> TcpListener {
        TcpListener::bind(&format!("0.0.0.0:{}", port))
            .await
            .expect("Fatal error! - ServerController:create_listener()")
    }

    pub async fn handle_request() -> Json<Response> { Json(Response::Pong) }
}