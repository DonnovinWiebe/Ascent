use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::standard::{bank::BankAction, transaction::Transaction};

pub const DEFAULT_ROUTE: &'static str = "/sync";


pub trait ComPrint {
    fn com_type(&self) -> String;
    fn com_details(&self) -> String;
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Request {
    Ping,
    Perform { action: BankAction, starting_version: Uuid, resulting_version: Uuid },
    ForceUpdate { updated_ledger: Vec<Transaction>, updated_version: Uuid, context: Context },
    RequestLastRevision { context: Context },
    RequestLedger { context: Context },
}
impl ComPrint for Request {
    fn com_type(&self) -> String {
        match self {
            Request::Ping => "[Ping]".to_string(),
            Request::Perform { action, starting_version, resulting_version } => "[Perform]".to_string(),
            Request::ForceUpdate { updated_ledger, updated_version, context } => "[Force update]".to_string(),
            Request::RequestLastRevision { context } => "[Request last revision]".to_string(),
            Request::RequestLedger { context } => "[Request ledger]".to_string(),
        }
    }
    
    fn com_details(&self) -> String {
        match self {
            Request::Ping => "[Ping]".to_string(),
            Request::Perform { action, starting_version, resulting_version } => format!("{} - {}", self.com_type(), action.com_type()),
            Request::ForceUpdate { updated_ledger, updated_version, context } => format!("{} - {}", self.com_type(), context.com_type()),
            Request::RequestLastRevision { context } => format!("{} - {}", self.com_type(), context.com_type()),
            Request::RequestLedger { context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Pong,
    Approve,
    SendLastRevision { current_revision: Uuid, context: Context },
    SendLedger { ledger: Vec<Transaction>, context: Context },
}
impl ComPrint for Response {
    fn com_type(&self) -> String {
        match self {
            Response::Pong => "[Pong]".to_string(),
            Response::Approve => "[Approve]".to_string(),
            Response::SendLastRevision { current_revision, context } => "[Send last revision]".to_string(),
            Response::SendLedger { ledger, context } => "[Send ledger]".to_string(),
        }
    }

    fn com_details(&self) -> String {
        match self {
            Response::Pong => "[Pong]".to_string(),
            Response::Approve => "[Approve]".to_string(),
            Response::SendLastRevision { current_revision, context } => format!("{} - {}", self.com_type(), context.com_type()),
            Response::SendLedger { ledger, context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Context {
    Audit,
    VersionMismatch,
    IncompatibleHistories,
    MergeConflict,
    MergeConflictResolution,
    Unexpected,
    Unauthorized,
}
impl ComPrint for Context {
    fn com_type(&self) -> String {
        match self {
            Context::Audit => "[Audit]".to_string(),
            Context::VersionMismatch => "[Version mismatch]".to_string(),
            Context::IncompatibleHistories => "[Incompatible histories]".to_string(),
            Context::MergeConflict => "[Merge conflict]".to_string(),
            Context::MergeConflictResolution => "[Merge conflict resolution]".to_string(),
            Context::Unexpected => "[Unexpected]".to_string(),
            Context::Unauthorized => "[Unauthorized]".to_string(),
        }
    }

    fn com_details(&self) -> String { self.com_type() }
}



pub mod ClientController {
    use serde_json::json;
    use crate::standard::sync::{Request, Response};

    pub async fn send(server_address: &str, request: Request) -> Result<Response, reqwest::Error> {
        reqwest::Client::new()
                .post(server_address)
                .json(&json!(request))
                .send().await?
                .error_for_status()?
                .json::<Response>().await
    }
}



pub mod ServerController {
    use axum::{Json, Router, routing::post};
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use crate::standard::sync::{DEFAULT_ROUTE, Request, Response};

    pub fn create_router() -> Router {
        Router::new()
            .layer(TraceLayer::new_for_http())
            .route(DEFAULT_ROUTE, post(handle_request))
    }

    pub async fn create_listener(port: &str) -> TcpListener {
        TcpListener::bind(&format!("0.0.0.0:{}", port))
            .await
            .expect("Fatal error! - ServerController::create_listener()")
    }

    pub async fn handle_request(Json(request): Json<Request>) -> Json<Response> {
        Json(Response::Pong)
    }
}