// wyjście

use crate::{config, ZapiskiOsobowe};
use std::fmt;
use std::fs;
use std::io; // config file include arrays with content of error messages

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
            Self::BeginVcard => {
                write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[0])
            }
            Self::FN => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[1]),
            Self::N => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[2]),
            Self::ORG => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[3]),
            Self::EMAIL => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[4]),
            Self::TEL => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[5]),
            Self::EndVcard => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[6]),
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
    PotwierdzenieZapisaniaPliku,
}

impl fmt::Display for WiadomościDoUżytkownika {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Przywitanie => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[0])
            }
            Self::ZapytanieOImię => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[1])
            }
            Self::ZapytanieONazwisko => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[2])
            }
            Self::ZapytanieOZrzeszenie => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[3])
            }
            Self::ZapytanieOPocztę => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[4])
            }
            Self::ZapytanieONumerDalnomównika => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[5])
            }
            Self::PotwierdzenieZapisaniaPliku => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[6])
            }
        }
    }
}

// format ZapiskiOsobowe to final cvf card
impl fmt::Display for ZapiskiOsobowe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}{}{}{}{}{}",
            &self.zaczynajka_karty_vcf,
            &self.nagłówek_fn,
            &self.nagłówek_n,
            &self.nagłówek_zrzeszenie,
            &self.nagłówek_poczta,
            &self.nagłówek_dalnomównik,
            &self.kończajka_karty_vcf
        )
    }
}

// Traits for output
pub trait Wyjście {
    fn wyjście_do_pliku_cvf(&self, zawartość_do_pliku: String);
}

impl Wyjście for ZapiskiOsobowe {
    fn wyjście_do_pliku_cvf(&self, zawartość_do_pliku: String) {
        let nazwa_pliku: String = "zapisek.cvf".to_string();
        let wynik_z_zapiania_pliku: Result<(), io::Error> =
            fs::write(nazwa_pliku, zawartość_do_pliku);
        match wynik_z_zapiania_pliku {
            Ok(()) => println!("{}", WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku),
            Err(błąd) => eprintln!("{}", WiadomościoBłędach::NiepomyślnieZapisanoPlik(błąd)),
        }
    }
}

// Enum for error messages
#[derive(Debug)]
pub enum WiadomościoBłędach {
    PróbaOdczytaniaLinii(io::Error),
    PrzekroczonaIlośćPrób,
    WiadomośćSprawdzająca,
    NiepomyślnieZapisanoPlik(io::Error),
}

// Dipley trait for displaying messages about ERRORS

impl fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PróbaOdczytaniaLinii(err) => {
                write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], err)
            }
            Self::PrzekroczonaIlośćPrób => {
                write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1])
            }
            Self::WiadomośćSprawdzająca => {
                write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[2])
            }
            Self::NiepomyślnieZapisanoPlik(err) => write!(
                f,
                "{} Błąd: {}",
                config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[3],
                err
            ),
        }
    }
}
