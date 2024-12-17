// wyjście

use std::fmt;
use crate::config;

enum WiadomościBłędach {
    PróbaOdczytaniaLinii(),
    PrzekroczonaIlośćPrób(),
}

impl fmt::Display for WiadomościBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościBłędach::PróbaOdczytaniaLinii() => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH(0)),
        }
    }
}
