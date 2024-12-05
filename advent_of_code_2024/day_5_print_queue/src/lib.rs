use std::io::{ self, BufRead }; 
use std::fs::File;

const ŚCIEŻKA_DO_POLECEŃ_DRUKARKI: &str = "./input";


struct polecenia_do_drukarki {
    ścieżka: &'static str,
    czytnik_poleceń: io::BufReader<File>,
}

impl polecenia_do_drukarki {
    fn nowy ( ścieżka: &'static str ) -> Self {
        let polecenia: File = File::open(ścieżka).expect("Nie można odnaleźć poleceń. Nazwa zapisku winna nosić nazwę „input”");
        let czytnik_poleceń: io::BufReader<File> = io::BufReader::new(polecenia);

        Self {
            ścieżka,
            czytnik_poleceń
        }
    }

    fn ilość_zestawień_stron(&self) {

    }
}
