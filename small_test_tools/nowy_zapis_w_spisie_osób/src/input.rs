// wejście

use std::io;
use crate::output;

trait Wejście {
    fn czytnik_zapisków(&self) -> Option<String>;
}

struct WejścieŁańcucha;

impl Wejście for WejścieŁańcucha {
    fn czytnik_zapisków(&self) -> Option<String> {
        // return Some(String::from("sprawdzian"))
        while let próby = 0 <= 3 {
            let mut wejście = String::new();
            match io::stdin().read_line(&mut wejście) {
                Ok(wejście)   => {
                    Some(wejście)
                }
                Err(err)       => {
                    próby += 1;
                    eprintln!("{} {}: {}", output::WiadomościoBłędach::PróbaOdczytaniaLinii, próby, err);
                }
            }
        }
        eprintln!("{}", output::WiadomościoBłędach::PrzekroczonaIlośćPrób);
        None
    }
}
