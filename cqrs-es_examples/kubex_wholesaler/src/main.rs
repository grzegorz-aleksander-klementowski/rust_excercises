use std::{fmt::Display, io};

use async_trait::*;
use cqrs_es::*;
use serde::*;

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

#[derive(Serialize, Deserialize, Default)]
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
            InventoryCommand::ReceiveStock { quantity } => {
                let stock_level = self.stock_level + quantity;
                Ok(vec![InventoryEvent::StockReceived {
                    quantity,
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
}

fn main() {
    println!("Hello, world!");
}
