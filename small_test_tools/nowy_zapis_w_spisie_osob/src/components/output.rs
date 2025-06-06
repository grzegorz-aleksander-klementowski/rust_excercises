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
            NagłówkiVCF::BeginVcard => {
                write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[0])
            }
            NagłówkiVCF::FN => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[1]),
            NagłówkiVCF::N => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[2]),
            NagłówkiVCF::ORG => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[3]),
            NagłówkiVCF::EMAIL => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[4]),
            NagłówkiVCF::TEL => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[5]),
            NagłówkiVCF::EndVcard => write!(f, "{}", config::NAGŁÓWKI_ZAPISKÓW_OSOBOWYCH_VCF[6]),
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
            WiadomościDoUżytkownika::Przywitanie => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[0])
            }
            WiadomościDoUżytkownika::ZapytanieOImię => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[1])
            }
            WiadomościDoUżytkownika::ZapytanieONazwisko => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[2])
            }
            WiadomościDoUżytkownika::ZapytanieOZrzeszenie => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[3])
            }
            WiadomościDoUżytkownika::ZapytanieOPocztę => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[4])
            }
            WiadomościDoUżytkownika::ZapytanieONumerDalnomównika => {
                write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[5])
            }
            WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku => {
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
        let nazwa_pliku: String =
            format!("zapisek.cvf" /*, &self.nagłówek_fn, &self.nagłówek_n*/);
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

impl<'a> fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii(err) => {
                write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], err)
            }
            WiadomościoBłędach::PrzekroczonaIlośćPrób => {
                write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1])
            }
            WiadomościoBłędach::WiadomośćSprawdzająca => {
                write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[2])
            }
            WiadomościoBłędach::NiepomyślnieZapisanoPlik(err) => write!(
                f,
                "{} Błąd: {}",
                config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[3],
                err
            ),
        }
    }
}
