// wyjście

use std::fmt;
use crate::config; // config file include arrays with content of error messages

// Enum for error messages
pub enum WiadomościoBłędach<'a> {
    PróbaOdczytaniaLinii(&'a usize),
    PrzekroczonaIlośćPrób,
}

// Dipley trait for displaying messages about ERRORS
impl<'a> fmt::Display for WiadomościoBłędach<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii(&usize) => write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], usize),
            WiadomościoBłędach::PrzekroczonaIlośćPrób => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1]),
        }
    }
}
