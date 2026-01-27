# Event Sourcing Example

Implementing the Event Sourcing and CQRS patterns with RustAPI for audit trails and complex domain logic.

> 📖 **Related**: [Domain-Driven Design patterns](https://tuntii.github.io/RustAPI/)

## Overview

This example demonstrates:

- **Event Sourcing** — Store all changes as events, not just current state
- **CQRS** — Command Query Responsibility Segregation
- **Aggregate Reconstruction** — Rebuild state from event history
- **Domain Events** — Type-safe event definitions
- **Concurrent Event Store** — Thread-safe with DashMap

## Prerequisites

- Rust 1.70+
- Understanding of [crud-api](../crud-api/) patterns
- Basic knowledge of Event Sourcing concepts

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| Event Sourcing | Immutable event log |
| CQRS | Separate read/write models |
| DashMap | Concurrent state storage |
| Domain Events | Typed event variants |
| Aggregate Pattern | State reconstruction |

## Quick Start

```bash
# Run the example
cargo run -p event-sourcing

# Server starts at http://127.0.0.1:8080
```

## The Domain: Bank Account

This example models a simple bank account with:
- Account opening
- Deposits
- Withdrawals

All operations are stored as events, enabling full audit trail and state reconstruction.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/accounts/{id}` | Execute command (open/deposit/withdraw) |
| GET | `/accounts/{id}` | Get current account state |
| GET | `/accounts/{id}/events` | Get event history |

## Testing

### Open Account

```bash
curl -X POST http://127.0.0.1:8080/accounts/acc-001 \
  -H "Content-Type: application/json" \
  -d '{
    "type": "OpenAccount",
    "owner": "Alice",
    "initial_balance": 1000.0
  }'
```

### Deposit Money

```bash
curl -X POST http://127.0.0.1:8080/accounts/acc-001 \
  -H "Content-Type: application/json" \
  -d '{
    "type": "Deposit",
    "amount": 500.0
  }'
```

### Withdraw Money

```bash
curl -X POST http://127.0.0.1:8080/accounts/acc-001 \
  -H "Content-Type: application/json" \
  -d '{
    "type": "Withdraw",
    "amount": 200.0
  }'
```

### Get Account State

```bash
curl http://127.0.0.1:8080/accounts/acc-001

# Response:
# {
#   "id": "acc-001",
#   "owner": "Alice",
#   "balance": 1300.0,
#   "version": 3
# }
```

### Get Event History

```bash
curl http://127.0.0.1:8080/accounts/acc-001/events

# Response:
# [
#   {"type": "AccountOpened", "owner": "Alice", "initial_balance": 1000.0},
#   {"type": "MoneyDeposited", "amount": 500.0},
#   {"type": "MoneyWithdrawn", "amount": 200.0}
# ]
```

## Code Walkthrough

### 1. Domain Events

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum BankEvent {
    AccountOpened { owner: String, initial_balance: f64 },
    MoneyDeposited { amount: f64 },
    MoneyWithdrawn { amount: f64 },
}
```

Events are immutable facts about what happened. The `#[serde(tag = "type")]` adds a discriminator for JSON.

### 2. Commands

```rust
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum BankCommand {
    OpenAccount { owner: String, initial_balance: f64 },
    Deposit { amount: f64 },
    Withdraw { amount: f64 },
}
```

Commands express intent — they may be rejected if business rules are violated.

### 3. Aggregate

```rust
#[derive(Debug, Clone, Default)]
struct BankAccount {
    id: String,
    owner: String,
    balance: f64,
    version: u64,  // Event version for optimistic concurrency
}

impl BankAccount {
    fn apply(&mut self, event: &BankEvent) {
        match event {
            BankEvent::AccountOpened { owner, initial_balance } => {
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
```

### 4. Event Store

```rust
#[derive(Clone)]
struct EventStore {
    events: Arc<DashMap<String, Vec<BankEvent>>>,
}

impl EventStore {
    async fn append(&self, aggregate_id: &str, event: BankEvent) {
        self.events
            .entry(aggregate_id.to_string())
            .or_default()
            .push(event);
    }

    async fn load(&self, aggregate_id: &str) -> Option<BankAccount> {
        let events = self.events.get(aggregate_id)?;
        
        let mut account = BankAccount {
            id: aggregate_id.to_string(),
            ..Default::default()
        };
        
        // Reconstruct state by replaying events
        for event in events.iter() {
            account.apply(event);
        }
        
        Some(account)
    }
}
```

### 5. Command Handler

```rust
async fn handle_command(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(cmd): Json<BankCommand>,
) -> Result<Json<BankAccount>, ApiError> {
    let mut account = state.event_store.load(&id).await
        .unwrap_or_default();
    
    // Validate command and produce event
    let event = match cmd {
        BankCommand::Withdraw { amount } => {
            if account.balance < amount {
                return Err(ApiError::bad_request("Insufficient funds"));
            }
            BankEvent::MoneyWithdrawn { amount }
        }
        // ... other commands
    };
    
    // Store event
    state.event_store.append(&id, event.clone()).await;
    
    // Apply to get new state
    account.apply(&event);
    
    Ok(Json(account))
}
```

## Key Concepts

### Event Sourcing Benefits

| Benefit | Description |
|---------|-------------|
| **Audit Trail** | Complete history of all changes |
| **Time Travel** | Reconstruct state at any point |
| **Debugging** | See exactly what happened |
| **Event Replay** | Rebuild read models, fix bugs |
| **Analytics** | Rich historical data |

### CQRS Pattern

```
┌─────────────┐     ┌─────────────┐
│   Command   │────▶│   Events    │
│   Handler   │     │   (Write)   │
└─────────────┘     └──────┬──────┘
                           │
                           ▼
┌─────────────┐     ┌─────────────┐
│    Query    │◀────│  Projector  │
│   Handler   │     │   (Read)    │
└─────────────┘     └─────────────┘
```

### State Reconstruction

```rust
// Events are the source of truth
let events = vec![
    BankEvent::AccountOpened { owner: "Alice", initial_balance: 100.0 },
    BankEvent::MoneyDeposited { amount: 50.0 },
    BankEvent::MoneyWithdrawn { amount: 30.0 },
];

// Current state = fold(events)
let account = events.iter().fold(BankAccount::default(), |mut acc, e| {
    acc.apply(e);
    acc
});
// balance = 120.0
```

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.2" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
dashmap = "5.5"
uuid = { version = "1", features = ["v4"] }
```

## Production Considerations

### Event Storage

For production, replace DashMap with a proper event store:
- PostgreSQL with event table
- EventStoreDB
- Apache Kafka

### Snapshots

For aggregates with many events, use snapshots:

```rust
struct Snapshot {
    aggregate_id: String,
    version: u64,
    state: BankAccount,
}

// Load from snapshot + recent events only
fn load_with_snapshot(id: &str) -> BankAccount {
    let snapshot = get_latest_snapshot(id);
    let events = get_events_since(id, snapshot.version);
    
    let mut account = snapshot.state;
    for event in events {
        account.apply(&event);
    }
    account
}
```

### Event Versioning

Handle schema changes with upcasting:

```rust
enum BankEventV1 { ... }
enum BankEventV2 { ... }  // New version

fn upcast(v1: BankEventV1) -> BankEventV2 {
    // Transform old events to new format
}
```

## Next Steps

- **[microservices](../microservices/)** — Distributed architecture
- **[proof-of-concept](../proof-of-concept/)** — Full application

## Related Documentation

- [LEARNING_PATH.md](../LEARNING_PATH.md) — Learning progression
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
