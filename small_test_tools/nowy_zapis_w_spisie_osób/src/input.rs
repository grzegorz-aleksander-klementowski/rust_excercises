// wejście

use std::io;

trait Wejście {
    fn czytnik_zapisków(&self) -> Option<String>;
}

struct WejścieŁańcucha;

impl<'a> Wejście for WejścieŁańcucha {
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
                    eprintln!("Tried to read line {}: {}", próby, err);
                }
            }
        }
        eprintln!("Exeed number of attemps. Unuble to read line. Program exit…");
        None
    }
}
