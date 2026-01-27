//! # Event Sourcing Example
//!
//! Implementing Event Sourcing and CQRS patterns with RustAPI.
//!
//! ## Demonstrates
//! - Event sourcing with immutable event log
//! - CQRS (Command Query Responsibility Segregation)
//! - Aggregate pattern with state reconstruction
//! - Concurrent event store with `DashMap`
//!
//! ## Run
//! ```bash
//! cargo run -p event-sourcing
//! ```
//!
//! ## Test
//! ```bash
//! curl -X POST http://127.0.0.1:8080/accounts/acc-001 \
//!   -H "Content-Type: application/json" \
//!   -d '{"type": "OpenAccount", "owner": "Alice", "initial_balance": 1000.0}'
//! ```
//!
//! ## Cookbook
//! <https://tuntii.github.io/RustAPI/>

use dashmap::DashMap;
use rustapi_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

// --- Domain Events ---
#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
#[serde(tag = "type")]
enum BankEvent {
    AccountOpened { owner: String, initial_balance: f64 },
    MoneyDeposited { amount: f64 },
    MoneyWithdrawn { amount: f64 },
}

// --- Commands ---
#[derive(Debug, Deserialize, Schema)]
#[serde(tag = "type")]
enum BankCommand {
    OpenAccount { owner: String, initial_balance: f64 },
    Deposit { amount: f64 },
    Withdraw { amount: f64 },
}

// --- Aggregate ---
#[derive(Debug, Clone, Default, Serialize, Schema)]
struct BankAccount {
    id: String,
    owner: String,
    balance: f64,
    version: u64,
}

impl BankAccount {
    fn apply(&mut self, event: &BankEvent) {
        match event {
            BankEvent::AccountOpened {
                owner,
                initial_balance,
            } => {
                self.owner = owner.clone();
                self.balance = *initial_balance;
            }
            BankEvent::MoneyDeposited { amount } => {
                self.balance += amount;
            }
            BankEvent::MoneyWithdrawn { amount } => {
                self.balance -= amount;
            }
        }
        self.version += 1;
    }
}

// --- Event Store ---
#[derive(Clone)]
struct EventStore {
    events: Arc<DashMap<String, Vec<BankEvent>>>,
}

impl EventStore {
    fn new() -> Self {
        Self {
            events: Arc::new(DashMap::new()),
        }
    }

    async fn append(&self, aggregate_id: &str, event: BankEvent) {
        let mut events = self.events.entry(aggregate_id.to_string()).or_default();
        events.push(event);
    }

    async fn load(&self, aggregate_id: &str) -> Option<BankAccount> {
        // If no events exist, return None
        if !self.events.contains_key(aggregate_id) {
            return None;
        }

        let events = self.events.get(aggregate_id).unwrap();
        let mut account = BankAccount {
            id: aggregate_id.to_string(),
            ..Default::default()
        };

        for event in events.iter() {
            account.apply(event);
        }

        Some(account)
    }
}

// --- Handlers ---

#[derive(Clone)]
struct AppState {
    event_store: EventStore,
}

async fn handle_command(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(cmd): Json<BankCommand>,
) -> Result<Json<BankAccount>, ApiError> {
    // Check existence
    let mut account = if let Some(acc) = state.event_store.load(&id).await {
        acc
    } else {
        // If not found, only OpenAccount is valid which we strictly support via POST /accounts usually,
        // but here we support specific ID opening too if command is OpenAccount
        if matches!(cmd, BankCommand::OpenAccount { .. }) {
            BankAccount {
                id: id.clone(),
                ..Default::default()
            }
        } else {
            return Err(ApiError::not_found("Account not found"));
        }
    };

    // Process logic
    let event = match cmd {
        BankCommand::OpenAccount {
            owner,
            initial_balance,
        } => {
            if account.version > 0 {
                return Err(ApiError::bad_request("Account already exists"));
            }
            BankEvent::AccountOpened {
                owner,
                initial_balance,
            }
        }
        BankCommand::Deposit { amount } => {
            if amount <= 0.0 {
                return Err(ApiError::bad_request("Invalid amount"));
            }
            BankEvent::MoneyDeposited { amount }
        }
        BankCommand::Withdraw { amount } => {
            if amount <= 0.0 {
                return Err(ApiError::bad_request("Invalid amount"));
            }
            if account.balance < amount {
                return Err(ApiError::bad_request("Insufficient funds"));
            }
            BankEvent::MoneyWithdrawn { amount }
        }
    };

    // Persist
    state.event_store.append(&id, event.clone()).await;

    // Apply to return latest state
    account.apply(&event);

    Ok(Json(account))
}

async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BankAccount>, ApiError> {
    // This is the "Query" side - practically a projection on the fly
    match state.event_store.load(&id).await {
        Some(acc) => Ok(Json(acc)),
        None => Err(ApiError::not_found("Account not found")),
    }
}

async fn create_account(
    State(state): State<AppState>,
    Json(cmd): Json<BankCommand>,
) -> Result<Json<BankAccount>, ApiError> {
    // Only OpenAccount is valid here
    if !matches!(cmd, BankCommand::OpenAccount { .. }) {
        return Err(ApiError::bad_request(
            "Only OpenAccount command allowed here",
        ));
    }

    let id = Uuid::new_v4().to_string();
    handle_command(State(state), Path(id), Json(cmd)).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        event_store: EventStore::new(),
    };

    println!("Event Sourcing Demo running on :3000");
    RustApi::new()
        .state(state)
        .route("/accounts", post(create_account))
        .route("/accounts/:id/command", post(handle_command))
        .route("/accounts/:id", get(get_account))
        .run("0.0.0.0:3000")
        .await
        .unwrap();
}
