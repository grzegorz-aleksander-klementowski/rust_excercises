// wejście

use std::io;
use crate::output;

trait Wejście {
    fn czytnik_zapisków(&self) -> Option<String>;
}

struct WejścieŁańcucha;

impl Wejście for WejścieŁańcucha {
    fn czytnik_zapisków(&self) -> Option<String> {
        // check if hte program can read the line up to 3 times before exit
        let mut próby: usize = 0;
        while próby >= 3 {
            let mut zapisek = String::new();
            match io::stdin().read_line(&mut zapisek) {
                Ok(_)   => return Some(zapisek),
                Err(err)       => {
                    próby += 1;
                    eprintln!("{} {}: {}", output::WiadomościoBłędach::PróbaOdczytaniaLinii(&próby), próby, err);
                }
            }
        }
        eprintln!("{}", output::WiadomościoBłędach::PrzekroczonaIlośćPrób);
        None
    }
}
