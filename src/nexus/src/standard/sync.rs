use std::sync::Arc;
use schrod::Schrod;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::standard::{bank::{Bank, BankAction, CurrencyExchange, TagRegistry}, transaction::Transaction};

pub const DEFAULT_ROUTE: &'static str = "/sync";


pub trait ComPrint {
    fn com_type(&self) -> String;
    fn com_details(&self) -> String;
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Request {
    Ping,
    Perform { action: BankAction, starting_version: Uuid, resulting_version: Uuid },
    ForceUpdate { updated_ledger: Vec<Transaction>, updated_currency_exchange: CurrencyExchange, updated_tag_registry: TagRegistry, updated_version: Uuid, context: Context },
    RequestLastRevision { context: Context },
    RequestLedger { context: Context },
}
impl ComPrint for Request {
    fn com_type(&self) -> String {
        match self {
            Request::Ping => "[Ping]".to_string(),
            Request::Perform { action, starting_version, resulting_version } => "[Perform]".to_string(),
            Request::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => "[Force update]".to_string(),
            Request::RequestLastRevision { context } => "[Request last revision]".to_string(),
            Request::RequestLedger { context } => "[Request ledger]".to_string(),
        }
    }
    
    fn com_details(&self) -> String {
        match self {
            Request::Ping => "[Ping]".to_string(),
            Request::Perform { action, starting_version, resulting_version } => format!("{} - {}", self.com_type(), action.com_type()),
            Request::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => format!("{} - {}", self.com_type(), context.com_type()),
            Request::RequestLastRevision { context } => format!("{} - {}", self.com_type(), context.com_type()),
            Request::RequestLedger { context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Pong,
    Approve,
    Error { log: Schrod<()> },
    SendLastRevision { current_revision: Uuid, context: Context },
    SendLedger { ledger: Vec<Transaction>, context: Context },
}
impl ComPrint for Response {
    fn com_type(&self) -> String {
        match self {
            Response::Pong => "[Pong]".to_string(),
            Response::Approve => "[Approve]".to_string(),
            Response::Error { log } => "[Error]".to_string(),
            Response::SendLastRevision { current_revision, context } => "[Send last revision]".to_string(),
            Response::SendLedger { ledger, context } => "[Send ledger]".to_string(),
        }
    }

    fn com_details(&self) -> String {
        match self {
            Response::Pong => "[Pong]".to_string(),
            Response::Approve => "[Approve]".to_string(),
            Response::Error { log } => "[Error]".to_string(),
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


pub type BankInstance = Arc<Mutex<Bank>>;



pub mod ServerController {
    use axum::{Json, Router, extract::State, routing::post};
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use crate::{data::{save_data::{SaveData, SaveDataType}, tooling::Platforms}, standard::sync::{BankInstance, ComPrint, Context, DEFAULT_ROUTE, Request, Response}};

    pub fn create_router(bank: BankInstance) -> Router {
        Router::new()
            .route(DEFAULT_ROUTE, post(handle_request))
            .layer(TraceLayer::new_for_http())
            .with_state(bank)
    }

    pub async fn create_listener(port: &str) -> TcpListener {
        TcpListener::bind(&format!("0.0.0.0:{}", port))
            .await
            .expect("Fatal error! - ServerController::create_listener()")
    }

    pub async fn handle_request(State(bank): State<BankInstance>, Json(request): Json<Request>) -> Json<Response> {
        tracing::info!("Received: {}", request.com_details());
        let mut bank = bank.lock().await;
    
        match request {
            Request::Ping => {
                let response = Response::Pong;
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            Request::Perform { action, starting_version, resulting_version } => {
                let server_revision = bank.get_last_revision();

                // the client and server versions match (or the server has no transactions)
                if server_revision == starting_version || bank.get_ledger().is_empty() {
                    tracing::info!("Versions match.");
                    let action_result = bank.perform(action, Some(resulting_version));

                    // the action failed on the server's end
                    if action_result.is_fail() {
                        tracing::error!("Failed to apply BankAction!");
                        let response = Response::Error { log: action_result };
                        tracing::info!("Replying: {}", response.com_details());
                        return Json(response)
                    }

                    // the action succeeded
                    tracing::info!("Successfully applied BankAction.");

                    // saving
                    let save_data = SaveData::BankData {
                        history: bank.get_history(),
                        transactions: bank.get_ledger_copy(),
                        currency_exchange: bank.currency_exchange.clone(),
                        tag_registry: bank.tag_registry.clone()
                    };
                    let save_result = save_data.save(SaveDataType::BankData, Platforms::Server);

                    // saving failed
                    if save_result.is_fail() {
                        tracing::error!("Failed to save!");
                        let response = Response::Error { log: save_result };
                        tracing::info!("Replying: {}", response.com_details());
                        return Json(response)
                    }

                    // saving succeeded
                    tracing::info!("Saved.");

                    // returning a success
                    let response = Response::Approve;
                    tracing::info!("Replying: {}", response.com_details());
                    return Json(response)
                }

                // the client and server versions do not match
                else {
                    tracing::warn!("Versions do not match!");
                    let response = Response::SendLastRevision { current_revision: bank.get_last_revision(), context: Context::IncompatibleHistories };
                    tracing::info!("Replying: {}", response.com_details());
                    return Json(response)
                }
            },

            Request::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => {
                bank.force_update(updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version);
                let response = Response::Approve;
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            Request::RequestLastRevision { context } => {
                let response = Response::SendLastRevision { current_revision: bank.get_last_revision(), context };
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            Request::RequestLedger { context } => {
                let response = Response::SendLedger { ledger: bank.get_ledger_copy(), context };
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },
        }
    }
}