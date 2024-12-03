use std::fs::File;
use std::io::{self, BufRead };

fn main() {
    let ścieżka_do_pomieszanej_pamięci: &str = "./zapiski/pomieszana_pamięć";
    let pomieszana_pamięć: File = File::open(ścieżka_do_pomieszanej_pamięci).expect("Nie można odnaleźć pomiedzanej pamięci. Powinno znajdować sie w ./zapiski/pomieszana_pamięć");
    let czytnik_pomieszanej_pamięci: io::BufReader<File> = io::BufReader::new(pomieszana_pamięć);

    // wzorzec: „mut(X,Y)”
    let początkowy_wzorzec: &str = "mut(";
    let końcowy_znak: char = ')';

    for (liczba_porządkowa, wiersz) in czytnik_pomieszanej_pamięci.lines().enumerate() {
        let wiersz: String = wiersz.expect("Nie udało się wczytać wiersza");
        println!("Linia: {}", liczba_porządkowa + 1);

        if let Some(wskaźnik_zaczynający_początkowy_wzorzec) = wiersz.find(początkowy_wzorzec) {
            let wskaźnik_końcowy_początkowego_wzorca: usize = wskaźnik_zaczynający_początkowy_wzorzec + początkowy_wzorzec.len();
            //for znak in wiersz[wskaźnik_zaczynający_początkowy_wzorzec..wskaźnik_zaczynający_początkowy_wzorzec+4].chars() {
                //if znak.is_digit
            }
        }
    }

}
