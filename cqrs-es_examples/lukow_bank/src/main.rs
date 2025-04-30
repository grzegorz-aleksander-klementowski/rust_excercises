use async_trait::*;
use cqrs_es::*;
use serde::*;
use tokio::*;

#[derive(Debug, serde::Deserialize)]
pub enum BankAccountCommand {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64 },
    WriteCheck { check_number: String, amount: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },

    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },

    CustomerWithDrewCash {
        amount: f64,
        balance: f64,
    },

    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            BankAccountEvent::AccountOpened { account_id } => todo!(),
            BankAccountEvent::CustomerDepositedMoney { amount, balance } => todo!(),
            BankAccountEvent::CustomerWithDrewCash { amount, balance } => todo!(),
            BankAccountEvent::CustomerWroteCheck { check_number, amount, balance } => todo!(),
        }
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
