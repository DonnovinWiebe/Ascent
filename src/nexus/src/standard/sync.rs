use std::sync::Arc;
use schrod::Schrod;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::standard::{bank::{Bank, BankAction, CurrencyExchange, TagRegistry}, transaction::Transaction};

pub type BankInstance = Arc<Mutex<Bank>>;
pub const DEFAULT_ROUTE: &'static str = "/sync";



/// Defines what a client needs to be able to do.
pub trait Client {
    /// Logs unauthorized access to the server.
    fn log_unauthorized(&mut self);

    /// Logs a server error.
    fn log_error(&mut self, error: Schrod<()>);

    /// Starts a merge conflint resolution.
    fn log_merge_conflict(&mut self);

    /// Logs that a next request that has to be made based on some prior `Interaction`.
    fn log_next_request(&mut self, request: ClientRequest);
}



/// Defines what communications need to be able to print nicely.
pub trait ComPrint {
    /// Gets just the basic event name.
    fn com_type(&self) -> String;

    // Gets the event name and some relevant details.
    fn com_details(&self) -> String;
}



/// Defines the kinds of requests the client can make to the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClientRequest {
    /// Just a basic ping.
    Ping,

    /// A basic request to perform a given action.
    Perform { action: BankAction, starting_version: Uuid, resulting_version: Uuid },

    /// A command to overwrite server data for the attached reason.
    ForceUpdate { updated_ledger: Vec<Transaction>, updated_currency_exchange: CurrencyExchange, updated_tag_registry: TagRegistry, updated_version: Uuid, context: Context },

    /// A request to get the last revision on the server for the following reason.
    RequestLastRevision { context: Context },

    /// A request to get the entire ledger of the server for the following reason.
    RequestLedger { context: Context },
}
impl ComPrint for ClientRequest {
    fn com_type(&self) -> String {
        match self {
            ClientRequest::Ping => "[Ping]".to_string(),
            ClientRequest::Perform { action, starting_version, resulting_version } => "[Perform]".to_string(),
            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => "[Force update]".to_string(),
            ClientRequest::RequestLastRevision { context } => "[Request last revision]".to_string(),
            ClientRequest::RequestLedger { context } => "[Request ledger]".to_string(),
        }
    }
    
    fn com_details(&self) -> String {
        match self {
            ClientRequest::Ping => "[Ping]".to_string(),
            ClientRequest::Perform { action, starting_version, resulting_version } => format!("{} - {}", self.com_type(), action.com_type()),
            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => format!("{} - {}", self.com_type(), context.com_type()),
            ClientRequest::RequestLastRevision { context } => format!("{} - {}", self.com_type(), context.com_type()),
            ClientRequest::RequestLedger { context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



/// Defines the kinds of responses the server can give to the client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerResponse {
    /// Just a basic pong.
    Pong,

    /// Approves whatever request was made.
    Approve,

    /// Sends back an error encountered while handling a request.
    Error { log: Schrod<()> },

    /// Sends back the last revision on the server for the following reason.
    SendLastRevision { current_revision: Uuid, context: Context },

    /// Sends back the entire ledger on the server for the following reason.
    SendLedger { ledger: Vec<Transaction>, context: Context },
}
impl ComPrint for ServerResponse {
    fn com_type(&self) -> String {
        match self {
            ServerResponse::Pong => "[Pong]".to_string(),
            ServerResponse::Approve => "[Approve]".to_string(),
            ServerResponse::Error { log } => "[Error]".to_string(),
            ServerResponse::SendLastRevision { current_revision, context } => "[Send last revision]".to_string(),
            ServerResponse::SendLedger { ledger, context } => "[Send ledger]".to_string(),
        }
    }

    fn com_details(&self) -> String {
        match self {
            ServerResponse::Pong => "[Pong]".to_string(),
            ServerResponse::Approve => "[Approve]".to_string(),
            ServerResponse::Error { log } => "[Error]".to_string(),
            ServerResponse::SendLastRevision { current_revision, context } => format!("{} - {}", self.com_type(), context.com_type()),
            ServerResponse::SendLedger { ledger, context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



/// Defines reasons for some requests being made.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Context {
    /// A basic check to ensure consitency.
    Checkup,

    /// There was a revision mismatch between the client and server.
    RevisionMismatch,

    /// The client has updated data compared to the server.
    Update,

    /// The histories between the client and server cannot be reconciled.
    IncompatibleHistories,
    
    /// There is a data mismatch between the client and server that has to be reconciled.
    MergeConflict,

    /// This is a resolution to a merge conflict.
    MergeConflictResolution,

    /// Some request was unexpected and cannot be processed.
    Unexpected,

    /// Some request was made without permission (bad token).
    Unauthorized,
}
impl ComPrint for Context {
    fn com_type(&self) -> String {
        match self {
            Context::Checkup => "[Audit]".to_string(),
            Context::RevisionMismatch => "[Version mismatch]".to_string(),
            Context::IncompatibleHistories => "[Incompatible histories]".to_string(),
            Context::MergeConflict => "[Merge conflict]".to_string(),
            Context::MergeConflictResolution => "[Merge conflict resolution]".to_string(),
            Context::Unexpected => "[Unexpected]".to_string(),
            Context::Unauthorized => "[Unauthorized]".to_string(),
        }
    }

    fn com_details(&self) -> String { self.com_type() }
}



/// Defines what happens when the client reads the server's response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Interaction {
    /// The interaction was successful and yielded the following response from the server.
    Good { response: ServerResponse },

    /// The interaction failed with the following error log.
    Error { error: Schrod<()> },

    /// The interaction was not authorized (bad token).
    Unauthorized,
}



/// Defines the functionality needed to run a client.
pub mod ClientController {
    use schrod::Schrod;
    use serde_json::json;
    use crate::standard::{bank::Bank, sync::{Client, ClientRequest, ComPrint, Context, Interaction, ServerResponse::{self}}, version_control::VersionComparisonResult};

    /// Sends a `ClientRequest` to the server and receives back a `ServerResponse` in the form of an `Interaction`.
    #[must_use]
    pub async fn send(server_address: &str, token: &str, request: ClientRequest) -> Interaction {
        // sending a request and getting an initial response
        let reqwest_response = match reqwest::Client::new()
                .post(server_address)
                .bearer_auth(token)
                .json(&json!(request))
                .send()
                .await
                {
                    Ok(reqwest_response) => reqwest_response,
                    // returns early if the initial response is an error
                    Err(err) => return Interaction::Error { error: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") }
                };

        // checks if the response is unauthorized
        if reqwest_response.status() == reqwest::StatusCode::UNAUTHORIZED { return Interaction::Unauthorized }

        // checks if the response has any error
        let reqwest_response = match reqwest_response.error_for_status() {
            Ok(reqwest_response) => reqwest_response,
            Err(err) => return Interaction::Error { error: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") },
        };

        // returns a good response
        match reqwest_response.json::<ServerResponse>().await {
            Ok(response) => Interaction::Good { response: response },
            Err(err) => Interaction::Error { error: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") }
        }
    }

    /// Handles an `Interaction`.
    pub fn handle_interaction<A: Client>(app: &mut A, bank: &mut Bank, interaction: Interaction) {
        match interaction {
            Interaction::Good { response } => {
                match response {
                    // got a pong
                    ServerResponse::Pong => { tracing::info!("Pong"); },

                    // last request was approved
                    ServerResponse::Approve => { tracing::info!("The server accepted the last request."); },

                    // the server had an error parsing the last request
                    ServerResponse::Error { log } => {
                        tracing::error!("The server sent an error!");
                        app.log_error(log);
                    },

                    // the server sent it's current revision for the following reason
                    ServerResponse::SendLastRevision { current_revision, context } => {
                        match context {
                            // just a regular checkup
                            Context::Checkup => {
                                let comparison_result = VersionComparisonResult::compare(current_revision, bank.get_history());
                                match comparison_result {
                                    VersionComparisonResult::Matching => { tracing::info!("Checkup passed with matching histories between the client and server.") },

                                    VersionComparisonResult::Ahead => {
                                        tracing::info!("The client has data ahead of the server's.");
                                        app.log_next_request(ClientRequest::ForceUpdate {
                                            updated_ledger: bank.get_ledger_copy(),
                                            updated_currency_exchange: bank.currency_exchange.clone(),
                                            updated_tag_registry: bank.tag_registry.clone(),
                                            updated_version: bank.get_last_revision(),
                                            context: Context::Update
                                        });
                                    },

                                    VersionComparisonResult::Behind => {
                                        tracing::warn!("The client's data is behind that of the server!");
                                        app.log_merge_conflict();
                                    }
                                }
                            },

                            // there was a version mismatch
                            Context::RevisionMismatch => {
                                let comparison_result = VersionComparisonResult::compare(current_revision, bank.get_history());
                                match comparison_result {
                                    VersionComparisonResult::Matching => { tracing::error!("Bad interaction: {}", response.com_details()); },

                                    VersionComparisonResult::Ahead => {
                                        tracing::info!("The client has data ahead of the server's.");
                                        app.log_next_request(ClientRequest::ForceUpdate {
                                            updated_ledger: bank.get_ledger_copy(),
                                            updated_currency_exchange: bank.currency_exchange.clone(),
                                            updated_tag_registry: bank.tag_registry.clone(),
                                            updated_version: bank.get_last_revision(),
                                            context: Context::Update
                                        });
                                    },

                                    VersionComparisonResult::Behind => {
                                        tracing::warn!("The client's data is behind that of the server!");
                                        app.log_merge_conflict();
                                    }
                                }
                            },

                            Context::Update => { tracing::error!("Bad interaction: {}", response.com_details()); },

                            // the versions betweent the client and server cannot be reconciled
                            Context::IncompatibleHistories => { tracing::error!("Bad interaction: {}", response.com_details()); },

                            // there is a merge conflict in progress
                            Context::MergeConflict => { tracing::error!("Bad interaction: {}", response.com_details()); },

                            // this is part of a a resolution to a merge conflict
                            Context::MergeConflictResolution => { tracing::error!("Bad interaction: {}", response.com_details()); },

                            // something in the last request was unexpected
                            Context::Unexpected => { tracing::warn!("The server was not expecting the last request...") },

                            // the last request was denied due to incorrect access (missing token)
                            Context::Unauthorized => {
                                tracing::error!("Server access denied!");
                                app.log_unauthorized();
                            },
                        }
                    },
                    
                    ServerResponse::SendLedger { ledger, context } => {
                        match context {
                            // just a regular checkup
                            Context::Checkup => {},

                            // there was a version mismatch
                            Context::RevisionMismatch => {},

                            // the versions betweent the client and server cannot be reconciled
                            Context::IncompatibleHistories => {},

                            // there is a merge conflict in progress
                            Context::MergeConflict => {},

                            // this is part of a a resolution to a merge conflict
                            Context::MergeConflictResolution => {},

                            // something in the last request was unexpected
                            Context::Unexpected => { tracing::warn!("The server was not expecting the last request...") },

                            // the last request was denied due to incorrect access (missing token)
                            Context::Unauthorized => {
                                tracing::error!("Server access denied!");
                                app.log_unauthorized();
                            },
                        }
                    },
                }
            },

            Interaction::Unauthorized {},

            Interaction::Error { error }
        }
    }
}



pub mod ServerController {
    use axum::{Json, Router, extract::State, routing::post};
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use crate::{data::{save_data::{SaveData, SaveDataType}, tooling::Platforms}, standard::sync::{BankInstance, ComPrint, Context, DEFAULT_ROUTE, ClientRequest, ServerResponse}};

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

    pub async fn handle_request(State(bank): State<BankInstance>, Json(request): Json<ClientRequest>) -> Json<ServerResponse> {
        tracing::info!("Received: {}", request.com_details());
        let mut bank = bank.lock().await;
    
        match request {
            ClientRequest::Ping => {
                let response = ServerResponse::Pong;
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            ClientRequest::Perform { action, starting_version, resulting_version } => {
                let server_revision = bank.get_last_revision();

                // the client and server versions match (or the server has no transactions)
                if server_revision == starting_version || bank.get_ledger().is_empty() {
                    tracing::info!("Versions match.");
                    let action_result = bank.perform(action, Some(resulting_version));

                    // the action failed on the server's end
                    if action_result.is_fail() {
                        tracing::error!("Failed to apply BankAction!");
                        let response = ServerResponse::Error { log: action_result };
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
                        let response = ServerResponse::Error { log: save_result };
                        tracing::info!("Replying: {}", response.com_details());
                        return Json(response)
                    }

                    // saving succeeded
                    tracing::info!("Saved.");

                    // returning a success
                    let response = ServerResponse::Approve;
                    tracing::info!("Replying: {}", response.com_details());
                    return Json(response)
                }

                // the client and server versions do not match
                else {
                    tracing::warn!("Versions do not match!");
                    let response = ServerResponse::SendLastRevision { current_revision: bank.get_last_revision(), context: Context::RevisionMismatch };
                    tracing::info!("Replying: {}", response.com_details());
                    return Json(response)
                }
            },

            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version, context } => {
                bank.force_update(updated_ledger, updated_currency_exchange, updated_tag_registry, updated_version);
                let response = ServerResponse::Approve;
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            ClientRequest::RequestLastRevision { context } => {
                let response = ServerResponse::SendLastRevision { current_revision: bank.get_last_revision(), context };
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            ClientRequest::RequestLedger { context } => {
                let response = ServerResponse::SendLedger { ledger: bank.get_ledger_copy(), context };
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },
        }
    }
}