use std::{alloc::GlobalAlloc, fmt::Display, io};

use async_trait::*;
use chrono::{DateTime, Utc};
use cqrs_es::persist::*;
use cqrs_es::*;
use postgres_es::PostgresEventRepository;
use postgres_es::PostgresViewRepository;
use serde::*;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

use cqrs_es::{CqrsFramework, EventStore, View};
use sqlx::PgPool;
use tokio;

//*** [EVENTS] ***\\
#[derive(Debug, serde::Deserialize)]
pub enum InventoryCommand {
    RegisterProduct {
        product_id: String,
    },
    ReceiveStock {
        quantity: f64,
    },
    ShipStock {
        quantity: f64,
    },
    IssueInvoice {
        invoice_number: String,
        total_amount: f64,
    },
}

/// *Metadata* (It sents the CQRS command with metadata)
pub async fn process_command<ES>(
    cqrs: CqrsFramework<Inventory, ES>,
    aggregate_id: &str,
    command: InventoryCommand,
) -> Result<(), AggregateError<InventoryError>>
where
    ES: cqrs_es::EventStore<Inventory>,
{
    // zbieramy proste metadane – tu tylko znacznik czasu
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("time".to_string(), Utc::now().to_rfc3339());

    // wykonujemy komendę z dołączonymi metadanymi
    cqrs.execute_with_metadata(aggregate_id, command, metadata)
        .await
}

//*** Log Qiery ***\\
struct SimpleLoggingQuerry {}
#[async_trait]
impl Query<Inventory> for SimpleLoggingQuerry {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Inventory>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InventoryEvent {
    RegisteredProduct {
        product_id: String,
    },

    StockReceived {
        quantity: f64,
        stock_level: f64,
    },

    StockShipped {
        quantity: f64,
        stock_level: f64,
    },

    InvoiceIssued {
        invoice_number: String,
        total_amount: f64,
        stock_level: f64,
    },
}

//*** [DOMAIN] ***\\
impl DomainEvent for InventoryEvent {
    fn event_type(&self) -> String {
        match self {
            InventoryEvent::RegisteredProduct { .. } => "RegisteredProduct".to_string(),
            InventoryEvent::StockReceived { .. } => "StockReceived".to_string(),
            InventoryEvent::StockShipped { .. } => "StockShipped".to_string(),
            InventoryEvent::InvoiceIssued { .. } => "InvoiceIssued".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string() // Jeśli w przyszłości będziesz potrzebował wersjonować zdarzenia,
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Inventory {
    opened: bool,
    stock_level: f64,
}

//*** [EXTERNAL SERVEICES] ***\\
pub struct InventoryServices;

impl InventoryServices {
    async fn ship_stock_from_warehouse(
        &self,
        warehouse_id: f64,
        quantity: f64,
    ) -> Result<(), StockError> {
        Ok(())
    }

    async fn validate_invoice(&self, product: &str, invoice: &str) -> Result<(), InvoiceError> {
        Ok(())
    }
}

//*** [AGGREGATE] ***\\
#[async_trait]
impl Aggregate for Inventory {
    type Command = InventoryCommand;
    type Event = InventoryEvent;
    type Services = InventoryServices;
    type Error = InventoryError;

    fn aggregate_type() -> String {
        "Product".to_string()
    }
    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            InventoryCommand::RegisterProduct { product_id } => {
                if self.opened {
                    // register the product if already was registered
                    return Err(InventoryError::from("Already registered"));
                }
                Ok(vec![InventoryEvent::RegisteredProduct { product_id }])
            }
            InventoryCommand::ReceiveStock { quantity } => {
                let stock_level = self.stock_level + quantity;
                Ok(vec![InventoryEvent::StockReceived {
                    quantity,
                    stock_level,
                }])
            }
            InventoryCommand::ShipStock { quantity } => {
                let stock_level = self.stock_level - quantity;
                if stock_level < 0_f64 {
                    return Err(InventoryError::from("Stock is not enought"));
                }
                Ok(vec![InventoryEvent::StockShipped {
                    quantity,
                    stock_level,
                }])
            }
            InventoryCommand::IssueInvoice {
                invoice_number,
                total_amount,
            } => {
                let stock_level = self.stock_level - total_amount;
                if stock_level < 0.0 {
                    return Err(InventoryError::from("Stock is not enough to issue invoice"));
                }
                Ok(vec![InventoryEvent::InvoiceIssued {
                    invoice_number,
                    total_amount,
                    stock_level,
                }])
            }
            _ => Ok(vec![]),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            InventoryEvent::RegisteredProduct { .. } => self.opened = true,
            InventoryEvent::StockReceived {
                quantity,
                stock_level,
            } => self.stock_level = stock_level,
            InventoryEvent::StockShipped {
                quantity,
                stock_level,
            } => self.stock_level = stock_level,
            InventoryEvent::InvoiceIssued {
                invoice_number,
                total_amount,
                stock_level,
            } => self.stock_level = stock_level,
        }
    }
}

//*** [ERRORS] ***\\
#[derive(Debug)]
pub struct InventoryError(String);
pub struct StockError;
pub struct InvoiceError;

impl Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InventoryError {}

impl From<&str> for InventoryError {
    fn from(message: &str) -> Self {
        InventoryError(message.to_string())
    }
}

#[tokio::test]
async fn test_event_store() {
    let event_store = cqrs_es::mem_store::MemStore::<Inventory>::default();
    let query = SimpleLoggingQuerry {};
    let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], InventoryServices);

    let aggregate_id = "aggregate_instance_A";

    // deposit 7000 zł
    cqrs.execute(
        aggregate_id,
        InventoryCommand::ReceiveStock { quantity: 7000_f64 },
    )
    .await
    .unwrap();

    // issue invoice for 2500.00 2500.00 zł
    cqrs.execute(
        aggregate_id,
        InventoryCommand::IssueInvoice {
            invoice_number: "PL_KUBEX_7226542/06/25.".to_string(),
            total_amount: 2500.00,
        },
    )
    .await
    .unwrap();
}

#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type AccountTestFramework = TestFramework<Inventory>;

    #[test]
    fn test_stock_received() {
        let previous = InventoryEvent::StockReceived {
            quantity: 200.0,
            stock_level: 200.0,
        };
        let expected = InventoryEvent::StockReceived {
            quantity: (200.0),
            stock_level: (400.0),
        };

        AccountTestFramework::with(InventoryServices)
            .given(vec![previous])
            .when(InventoryCommand::ReceiveStock { quantity: 200.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_ship_stock_not_enought_stock() {
        AccountTestFramework::with(InventoryServices)
            .given_no_previous_events()
            .when(InventoryCommand::ShipStock { quantity: 550.0 })
            .then_expect_error_message(&InventoryError::from("Stock is not enought").to_string());
    }
}

// *** Building views ***
// Single ledger entry in db operation logs (potem dobuduję wpisy (albo rozbuduje?))
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub kind: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
}

/// New ledger`kind` with`amount` and timestampt
impl LedgerEntry {
    pub fn new(kind: &str, amount: f64) -> Self {
        Self {
            kind: kind.to_string(),
            amount,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InventoryView {
    pub ledger: Vec<LedgerEntry>,
    pub stock_level: f64,
}

// updateing the views
impl cqrs_es::View<Inventory> for InventoryView {
    fn update(&mut self, event: &cqrs_es::EventEnvelope<Inventory>) {
        match &event.payload {
            InventoryEvent::RegisteredProduct { product_id } => {
                self.ledger.push(LedgerEntry::new("register", 0.0));
            }
            InventoryEvent::StockReceived {
                quantity,
                stock_level,
            } => {
                self.ledger.push(LedgerEntry::new("receive", *quantity));
                self.stock_level = *stock_level;
            }
            InventoryEvent::StockShipped {
                quantity,
                stock_level,
            } => {
                self.ledger.push(LedgerEntry::new("ship", *quantity));
                self.stock_level = *stock_level;
            }
            InventoryEvent::InvoiceIssued {
                total_amount,
                stock_level,
                ..
            } => {
                self.ledger.push(LedgerEntry::new("invoice", *total_amount));
                self.stock_level = *stock_level;
            }
        }
    }
}

/// It set the connection with PostgreSQL and back PersistedEventStore<postgres_es::PostgresEventRepository, Inventory>
pub async fn configure_repo(
) -> cqrs_es::persist::PersistedEventStore<postgres_es::PostgresEventRepository, Inventory> {
    let conn_str = "postgresql://test_user:pass@localhost:5432/kubex_warehouse_stocks";
    let pool = postgres_es::default_postgress_pool(conn_str).await;
    let repo = postgres_es::PostgresEventRepository::new(pool.clone());
    cqrs_es::persist::PersistedEventStore::<
        postgres_es::PostgresEventRepository,
        Inventory
    >::new_event_store(repo) // :contentReference[oaicite:0]{index=0}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // memory_store for in RAM memory storage
    // let memory_store = mem_store::MemStore::<Inventory>::default();
    // let store_for_view = memory_store.clone();

    // persistent_store is for pernament save in DB (using instead of in RAM memory_store
    let command_store = configure_repo().await;
    let view_store = configure_repo().await;

    // It's querry that log every event
    let logger = SimpleLoggingQuerry {};

    // Building CQRS Framework
    let cqrs = CqrsFramework::new(command_store, vec![Box::new(logger)], InventoryServices);

    let aggregate_id = "product-123";

    // sending a command
    cqrs.execute(
        aggregate_id,
        InventoryCommand::RegisterProduct {
            product_id: aggregate_id.to_string(),
        },
    )
    .await?;
    cqrs.execute(
        aggregate_id,
        InventoryCommand::ReceiveStock { quantity: 150.0 },
    )
    .await?;
    cqrs.execute(aggregate_id, InventoryCommand::ShipStock { quantity: 40.0 })
        .await?;
    cqrs.execute(
        aggregate_id,
        InventoryCommand::IssueInvoice {
            invoice_number: "INV-2025-0001".to_string(),
            total_amount: 30.0,
        },
    )
    .await?;

    // 5) rebuilding from event store
    let mut view = InventoryView::default();
    let events = view_store.load_events(aggregate_id).await?; // ładuje Vec<EventEnvelope<Inventory>> :contentReference[oaicite:0]{index=0}
    for env in events {
        view.update(&env);
    }

    // 6) Wypisujemy finalny stan
    println!("\n=== Finalny InventoryView dla '{}' ===", aggregate_id);
    println!("Stock level: {}", view.stock_level);
    for entry in view.ledger {
        println!("- [{:?}] {}: {}", entry.timestamp, entry.kind, entry.amount);
    }

    Ok(())
}

// TO DO:
// - secure duble registration
// - add interface for executing command and query read data
// - add more test
// - split the main file into multible segragate files
