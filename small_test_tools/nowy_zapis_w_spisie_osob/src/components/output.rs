// wyjście

use std::fmt;
use std::io;
use crate::config; // config file include arrays with content of error messages



// Enum for Messages for a use
pub enum WiadomościDoUżytkownika {
    Przywitanie,
    ZapytanieOImię,
    ZapytanieONazwisko,
    ZapytanieOZrzeszenie,
    ZapytanieOPocztę,
    ZapytanieONumerDalnomównika,
}

impl fmt::Display for WiadomościDoUżytkownika {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościDoUżytkownika::Przywitanie => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[0]),
            WiadomościDoUżytkownika::ZapytanieOImię => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[1]),
            WiadomościDoUżytkownika::ZapytanieONazwisko => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[2]),
            WiadomościDoUżytkownika::ZapytanieOZrzeszenie => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[3]),
            WiadomościDoUżytkownika::ZapytanieOPocztę => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[4]),
            WiadomościDoUżytkownika::ZapytanieONumerDalnomównika => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[5]),
        }
    }
}

// Enum for error messages
pub enum WiadomościoBłędach {
    PróbaOdczytaniaLinii(io::Error),
    PrzekroczonaIlośćPrób,
}

// Dipley trait for displaying messages about ERRORS
impl<'a> fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii(err) => write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], err),
            WiadomościoBłędach::PrzekroczonaIlośćPrób => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1]),
        }
    }
}
