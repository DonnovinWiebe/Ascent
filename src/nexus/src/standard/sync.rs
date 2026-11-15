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
    /// Gets an immutable reference to the `Client`'s `Bank`.
    fn bank(&self) -> &Bank;

    /// Logs a good server interaction.
    fn log_good(&mut self);
    
    /// Logs a server error.
    fn log_error(&mut self, error: Schrod<()>);

    /// Logs unauthorized access to the server.
    fn log_unauthorized(&mut self);

    /// Logs that a next request that has to be made based on some prior `Interaction`.
    fn log_next_request(&mut self, request: ClientRequest);
    
    /// Starts a merge conflint resolution.
    fn log_start_merge_conflict(&mut self, server_ledger: Vec<Transaction>);

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
    Perform { action: BankAction, starting_client_revision: Uuid, resulting_client_revision: Uuid },

    /// A command to overwrite server data for the attached reason.
    ForceUpdate { updated_ledger: Vec<Transaction>, updated_currency_exchange: CurrencyExchange, updated_tag_registry: TagRegistry, updated_revision: Uuid },

    /// A request to get the last revision on the server for the following reason.
    RequestLastRevision { context: Context },

    /// A request to get the entire ledger of the server for the following reason.
    RequestLedger { context: Context },
}
impl ComPrint for ClientRequest {
    fn com_type(&self) -> String {
        match self {
            ClientRequest::Ping => "[Ping]".to_string(),
            ClientRequest::Perform { action, starting_client_revision, resulting_client_revision } => "[Perform]".to_string(),
            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_revision } => "[Force update]".to_string(),
            ClientRequest::RequestLastRevision { context } => "[Request last revision]".to_string(),
            ClientRequest::RequestLedger { context } => "[Request ledger]".to_string(),
        }
    }
    
    fn com_details(&self) -> String {
        match self {
            ClientRequest::Ping => "[Ping]".to_string(),
            ClientRequest::Perform { action, starting_client_revision, resulting_client_revision } => format!("{} - {}", self.com_type(), action.com_type()),
            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_revision } => "[Force update]".to_string(),
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
    SendLastRevision { server_revision: Uuid, context: Context },

    /// Sends back the entire ledger on the server for the following reason.
    SendLedger { ledger: Vec<Transaction>, context: Context },
}
impl ComPrint for ServerResponse {
    fn com_type(&self) -> String {
        match self {
            ServerResponse::Pong => "[Pong]".to_string(),
            ServerResponse::Approve => "[Approve]".to_string(),
            ServerResponse::Error { log } => "[Error]".to_string(),
            ServerResponse::SendLastRevision { server_revision, context } => "[Send last revision]".to_string(),
            ServerResponse::SendLedger { ledger, context } => "[Send ledger]".to_string(),
        }
    }

    fn com_details(&self) -> String {
        match self {
            ServerResponse::Pong => "[Pong]".to_string(),
            ServerResponse::Approve => "[Approve]".to_string(),
            ServerResponse::Error { log } => "[Error]".to_string(),
            ServerResponse::SendLastRevision { server_revision, context } => format!("{} - {}", self.com_type(), context.com_type()),
            ServerResponse::SendLedger { ledger, context } => format!("{} - {}", self.com_type(), context.com_type()),
        }
    }
}



/// Defines reasons for some requests being made.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Context {
    /// A basic check to ensure consitency.
    SyncCheck,

    /// There was a revision mismatch between the client and server.
    RevisionMismatch,

    /// The client has updated data compared to the server.
    UpdateServerData,

    /// There is a data mismatch between the client and server that has to be reconciled.
    MergeConflict,

    /// This is a resolution to a merge conflict.
    MergeConflictResolution,
}
impl ComPrint for Context {
    fn com_type(&self) -> String {
        match self {
            Context::SyncCheck => "[Audit]".to_string(),
            Context::RevisionMismatch => "[Revision mismatch]".to_string(),
            Context::UpdateServerData => "[Update server data]".to_string(),
            Context::MergeConflict => "[Merge conflict]".to_string(),
            Context::MergeConflictResolution => "[Merge conflict resolution]".to_string(),
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
    Error { log: Schrod<()> },

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
                    Err(err) => return Interaction::Error { log: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") }
                };

        // checks if the response is unauthorized
        if reqwest_response.status() == reqwest::StatusCode::UNAUTHORIZED { return Interaction::Unauthorized }

        // checks if the response has any error
        let reqwest_response = match reqwest_response.error_for_status() {
            Ok(reqwest_response) => reqwest_response,
            Err(err) => return Interaction::Error { log: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") },
        };

        // returns a good response
        match reqwest_response.json::<ServerResponse>().await {
            Ok(response) => Interaction::Good { response: response },
            Err(err) => Interaction::Error { log: Schrod::from_result(Err(err), "Server interaction failed!", "ClientControll::send()") }
        }
    }

    /// Handles an `Interaction`.
    pub fn handle_interaction<A: Client>(app: &mut A, interaction: Interaction) {
        match interaction {
            // the interaction between the client and server was good
            Interaction::Good { response } => {
                match &response {
                    // got a pong
                    ServerResponse::Pong => {
                        tracing::info!("Pong");
                        app.log_good();
                    },

                    // last request was approved
                    ServerResponse::Approve => {
                        tracing::info!("The server accepted the last request.");
                        app.log_good();
                    },

                    // the server had an error parsing the last request
                    ServerResponse::Error { log } => {
                        tracing::error!("The server sent an error!");
                        app.log_error(log.clone());
                    },

                    // the server sent it's current revision for one of the following reasons:
                    // sync check: a routine check to make sure the server and client are on the same revision
                    // revision mismatch: some action on the server was denied due to mismatched versions
                    ServerResponse::SendLastRevision { server_revision, context } => {
                        app.log_good();
                        
                        match context {
                            Context::SyncCheck => {
                                let comparison_result = VersionComparisonResult::compare(*server_revision, app.bank().get_history());
                                match comparison_result {
                                    VersionComparisonResult::Matching => { tracing::info!("Checkup passed with matching histories between the client and server.") },

                                    VersionComparisonResult::Ahead => {
                                        tracing::info!("The client has data ahead of the server's.");
                                        let ledger = app.bank().get_ledger_copy();
                                        let currency_exchange = app.bank().currency_exchange.clone();
                                        let tag_registry = app.bank().tag_registry.clone();
                                        let revision = app.bank().get_last_revision();
                                        app.log_next_request(ClientRequest::ForceUpdate {
                                            updated_ledger: ledger,
                                            updated_currency_exchange: currency_exchange,
                                            updated_tag_registry: tag_registry,
                                            updated_revision: revision,
                                        });
                                    },

                                    VersionComparisonResult::Behind => {
                                        tracing::warn!("The client's data is behind that of the server!");
                                        app.log_next_request(ClientRequest::RequestLedger { context: Context::MergeConflict });
                                    }
                                }
                            },

                            Context::RevisionMismatch => {
                                let comparison_result = VersionComparisonResult::compare(*server_revision, app.bank().get_history());
                                match comparison_result {
                                    VersionComparisonResult::Matching => { tracing::warn!("Bad interaction: {}", response.com_details()); },

                                    VersionComparisonResult::Ahead => {
                                        tracing::info!("The client has data ahead of the server's.");
                                        let ledger = app.bank().get_ledger_copy();
                                        let currency_exchange = app.bank().currency_exchange.clone();
                                        let tag_registry = app.bank().tag_registry.clone();
                                        let revision = app.bank().get_last_revision();
                                        app.log_next_request(ClientRequest::ForceUpdate {
                                            updated_ledger: ledger,
                                            updated_currency_exchange: currency_exchange,
                                            updated_tag_registry: tag_registry,
                                            updated_revision: revision,
                                        });
                                    },

                                    VersionComparisonResult::Behind => {
                                        tracing::warn!("The client's data is behind that of the server!");
                                        app.log_next_request(ClientRequest::RequestLedger { context: Context::MergeConflict });
                                    }
                                }
                            },

                            // no other context is allowed
                            _ => { tracing::warn!("Bad interaction: {}", response.com_details()); },
                        }
                    },
                    
                    // the server sent it's ledger for one of the following reasons:
                    // merge conflict: a merge conflict has started
                    ServerResponse::SendLedger { ledger, context } => {
                        app.log_good();
                        
                        match context {
                            Context::MergeConflict => { app.log_start_merge_conflict(ledger.clone()); },
                            
                            // no other context is allowed
                            _ => { tracing::warn!("Bad interaction: {}", response.com_details()); },
                        }
                    },
                }
            },

            // the interaction was unauthorized
            Interaction::Unauthorized => {
                tracing::error!("Unauthorized server access!");
                app.log_unauthorized();
            },

            // there was an error interacting with the server
            Interaction::Error { log } => {
                tracing::error!("There was an error interacting with the server!");
                app.log_error(log.clone());
            }
        }
    }
}



pub mod ServerController {
    use axum::{Json, Router, extract::{Request, State}, middleware::{self, Next}, response::Response, routing::post};
    use axum::http::header;
    use reqwest::StatusCode;
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use crate::{data::{save_data::{SaveData, SaveDataType}, tooling::Platforms}, standard::sync::{BankInstance, ComPrint, Context, DEFAULT_ROUTE, ClientRequest, ServerResponse}};

    pub fn create_router(bank: BankInstance, token: String) -> Router {
        Router::new()
            .route(DEFAULT_ROUTE, post(handle_request))
            .layer(middleware::from_fn_with_state(token, require_token))
            .layer(TraceLayer::new_for_http())
            .with_state(bank)
    }

    pub async fn create_listener(port: &str) -> TcpListener {
        TcpListener::bind(&format!("0.0.0.0:{}", port))
            .await
            .expect("Fatal error! - ServerController::create_listener()")
    }

    async fn require_token(State(token): State<String>, request: Request, next: Next) -> Result<Response, StatusCode> {
        let provided = request
            .headers()
            .get(header::AUTHORIZATION) // yields "Bearer `token`"
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(normalize_token);
    
        if provided.as_deref() == Some(token.as_str()) { Ok(next.run(request).await) }
        else {
            tracing::warn!("Rejected unauthorized request!");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
    
    fn normalize_token(dirty_token: &str) -> String {
        dirty_token
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_uppercase()
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

            ClientRequest::Perform { action, starting_client_revision, resulting_client_revision } => {
                let server_revision = bank.get_last_revision();

                // the client and server revisions match (or the server has no transactions)
                if server_revision == starting_client_revision || bank.get_ledger().is_empty() {
                    tracing::info!("Revisions match.");
                    let action_result = bank.perform(action, Some(resulting_client_revision));

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
                    let response = ServerResponse::SendLastRevision { server_revision: bank.get_last_revision(), context: Context::RevisionMismatch };
                    tracing::info!("Replying: {}", response.com_details());
                    return Json(response)
                }
            },

            ClientRequest::ForceUpdate { updated_ledger, updated_currency_exchange, updated_tag_registry, updated_revision } => {
                bank.force_update(updated_ledger, updated_currency_exchange, updated_tag_registry, updated_revision);
                let response = ServerResponse::Approve;
                tracing::info!("Replying: {}", response.com_details());
                return Json(response)
            },

            ClientRequest::RequestLastRevision { context } => {
                let response = ServerResponse::SendLastRevision { server_revision: bank.get_last_revision(), context };
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