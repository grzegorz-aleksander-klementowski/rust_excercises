use std::io::{ self, BufRead }; 
use std::fs::File;

const ŚCIEŻKA_DO_POLECEŃ_DRUKARKI: &str = "./input";

trait Czytnik<T> {
    fn czytnik_zapisku(&self) -> io::BufReader<File>;
}

struct polecenia_do_drukarki {
    ścieżka: &'static str,
    czytnik_poleceń: io::BufReader<File>,
}

impl Czytnik<File> for polecenia_do_drukarki {
    fn czytnik_zapisku(&self) -> io::BufReader<File> {
        let zapisek: File = File::open(self.ścieżka).expect("Nie można odnaleźć poleceń. Nazwa zapisku winna nosić nazwę „input”");
        let czytnik_zapisku: io::BufReader<File> = io::BufReader::new(zapisek);
        return czytnik_zapisku;
    }


}
impl polecenia_do_drukarki {
    fn nowy ( ścieżka: &'static str ) -> Self {
        let czytnik_poleceń = Czytnik::czytnik_zapisku(ścieżka);
        Self {
            ścieżka,
            czytnik_poleceń
        }
    }
}
