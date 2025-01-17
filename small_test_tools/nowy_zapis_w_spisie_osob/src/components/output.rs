// wyjście

use std::fmt;
use std::io;
use crate::{ config, ZapiskiOsobowe }; // config file include arrays with content of error messages

// Enum for VCF cards headers
pub enum NagłówkiVCF {
    BeginVcard,
    FN,
    N, 
    ORG,
    EMAIL,
    TEL,
    EndVcard,
}

impl fmt::Display for NagłówkiVCF {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NagłówkiVCF::BeginVcard => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[0]),
            NagłówkiVCF::FN         => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[1]),
            NagłówkiVCF::N          => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[2]),
            NagłówkiVCF::ORG        => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[3]),
            NagłówkiVCF::EMAIL      => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[4]),
            NagłówkiVCF::TEL        => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[5]),
            NagłówkiVCF::EndVcard   => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[6]),
        }
    }
}

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

impl fmt::Display for ZapiskiOsobowe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}", 
               &self.zaczynajka_karty_vcf, 
               NagłówkiVCF::FN,     &self.nagłówek_fn,
               NagłówkiVCF::N,      &self.nagłówek_n,
               NagłówkiVCF::ORG,    &self.nagłówek_zrzeszenie,
               NagłówkiVCF::EMAIL,  &self.nagłówek_poczta,
               NagłówkiVCF::TEL,    &self.nagłówek_dalnomównik,
               &self.kończajka_karty_vcf
               ) 
    }
}

// Enum for error messages
#[derive(Debug)]
pub enum WiadomościoBłędach {
    PróbaOdczytaniaLinii(io::Error),
    PrzekroczonaIlośćPrób,
    WiadomośćSprawdzająca,
}

// Dipley trait for displaying messages about ERRORS

impl<'a> fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii(err)   => write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], err),
            WiadomościoBłędach::PrzekroczonaIlośćPrób       => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1]),
            WiadomościoBłędach::WiadomośćSprawdzająca       => write!(f, "{}", "Wiadomość sprawdzona"),
        }
    }
}
