// wyjście

use std::{ io, fmt, fs, env };
use std::path::{ Path, PathBuf };
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
    PotwierdzenieZapisaniaPliku,
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
            WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[6]),
            WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku => write!(f, "{}", config::ZAWARTOŚCI_WIADOMOŚCI[7])
        }
    }
}

// format ZapiskiOsobowe to final cvf card
impl fmt::Display for ZapiskiOsobowe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}{}{}{}{}{}", 
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

        let mut ścieszka_do_przechowalni_styczności: PathBuf = PathBuf::new();
        if let Some(home_dir) = env::home_dir() {
            ścieszka_do_przechowalni_styczności.push(home_dir);
            ścieszka_do_przechowalni_styczności.push(".QuickCVF");
            if !ścieszka_do_przechowalni_styczności.exists() {
                match fs::create_dir(&ścieszka_do_przechowalni_styczności) {
                    Ok(())      => {
                        print!("{}", WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku);
                        ()
                    },
                    Err(błąd)   => {
                        eprintln!("{}", WiadomościoBłędach::NieMożnaUtworzyćMiejsca(błąd));
                        ścieszka_do_przechowalni_styczności = env::current_dir().expect("Nie udało się odnaleźć miejsca, w którym jest działalnik. ");
                    }
                }
            }
        } else { 
            eprintln!("{}", WiadomościoBłędach::NieznaleziomoMisjcaDomowego);
            ścieszka_do_przechowalni_styczności = env::current_dir().expect("Nie udało się odnaleźć miejsca, w którym jest działalnik. ");
        }



        let nagłówek_fn_do_nazwy_pliku: String = self.nagłówek_fn.trim().chars().skip(3).collect();
        let nagłówek_n_do_nazwy_pliku: String  = self.nagłówek_n.trim().chars().skip(2).collect();
        let nazwa_pliku: String = format!("styczność_{}_{}.vcf", nagłówek_fn_do_nazwy_pliku, nagłówek_n_do_nazwy_pliku);
        let pełna_ścieszka_do_styczności = ścieszka_do_przechowalni_styczności.join(nazwa_pliku);
        let wynik_z_zapiania_pliku: Result<(), io::Error> = fs::write(pełna_ścieszka_do_styczności, zawartość_do_pliku);
        match wynik_z_zapiania_pliku {
            Ok(())      => println!("{}", WiadomościDoUżytkownika::PotwierdzenieZapisaniaPliku),
            Err(błąd)   => eprintln!("{}", WiadomościoBłędach::NiepomyślnieZapisanoPlik(błąd)),
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
    NieznaleziomoMisjcaDomowego,
    NieMożnaUtworzyćMiejsca(io::Error),
}

// Dipley trait for displaying messages about ERRORS

impl<'a> fmt::Display for WiadomościoBłędach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WiadomościoBłędach::PróbaOdczytaniaLinii(err)   => write!(f, "{}: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[0], err),
            WiadomościoBłędach::PrzekroczonaIlośćPrób       => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[1]),
            WiadomościoBłędach::WiadomośćSprawdzająca       => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[2]),
            WiadomościoBłędach::NiepomyślnieZapisanoPlik(err)=>write!(f, "{} Błąd: {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[3], err),
            WiadomościoBłędach::NieznaleziomoMisjcaDomowego => write!(f, "{}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[4]),
            WiadomościoBłędach::NieMożnaUtworzyćMiejsca(err) => write!(f, "{} {}", config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[5], err),
        }
    }
}
