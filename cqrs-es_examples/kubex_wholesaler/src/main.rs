use async_trait::*;
use cqrs_es::*;
use serde::*;
use tokio::*;

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

impl DomainEvent for InventoryEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            InventoryEvent::RegisteredProduct { .. } => todo!(),
            InventoryEvent::StockReceived { .. } => todo!(),
            InventoryEvent::StockShipped { .. } => todo!(),
            InventoryEvent::InvoiceIssued { .. } => todo!(),
        };
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

//*** [ERRORS] ***\\

#[derive(Debug)]
pub struct InventoryError(String);

fn main() {
    println!("Hello, world!");
}
