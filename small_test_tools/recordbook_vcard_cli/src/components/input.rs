// wejście

use crate::{output, ZapiskiOsobowe};
use std::io;

pub trait Wejście {
    fn czytnik_zapisków(&self) -> Result<String, output::WiadomościoBłędach>;
    fn pobierz_zapisek_z_próbami(&self) -> String;
}

impl Wejście for ZapiskiOsobowe {
    fn czytnik_zapisków(&self) -> Result<String, output::WiadomościoBłędach> {
        // check if hte program can read the line up to 3 times before exit in case of constant failing
        let mut zapisek = String::new();
        match io::stdin().read_line(&mut zapisek) {
            Ok(_) => Ok(zapisek),
            Err(err) => Err(output::WiadomościoBłędach::PróbaOdczytaniaLinii(err)),
        }
    }

    fn pobierz_zapisek_z_próbami(&self) -> String {
        let mut wynik_z_zapisku: Result<String, output::WiadomościoBłędach>;

        let mut próby: usize = 0;
        while próby <= 3 {
            wynik_z_zapisku = self.czytnik_zapisków();

            match wynik_z_zapisku {
                Ok(zapisek) => return zapisek,
                Err(błąd) => {
                    próby += 1;
                    eprintln!("{błąd}");
                }
            }
        }
        eprintln!("{}", output::WiadomościoBłędach::PrzekroczonaIlośćPrób);
        std::process::exit(1);
    }
}
