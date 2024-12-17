// wyjście

use std::fmt;
use crate::config; // config file include arrays with content of error messages

// Enum for error messages
pub enum WiadomościoBłędach {
    PróbaOdczytaniaLinii,
    PrzekroczonaIlośćPrób,
}

// Dipley trait for displaying messages about ERRORS
impl fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0]),
            WiadomościoBłędach::PrzekroczonaIlośćPrób => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1]),
        }
    }
}
