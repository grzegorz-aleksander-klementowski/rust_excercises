use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kierunek {
    Gora,
    Prawo,
    Dol,
    Lewo,
}

impl Kierunek {
    fn obrot_w_prawo(&self) -> Kierunek {
        match self {
            Kierunek::Gora => Kierunek::Prawo,
            Kierunek::Prawo => Kierunek::Dol,
            Kierunek::Dol => Kierunek::Lewo,
            Kierunek::Lewo => Kierunek::Gora,
        }
    }
    
    fn wektor_ruchu(&self) -> (isize, isize) {
        match self {
            Kierunek::Gora => (-1, 0),
            Kierunek::Prawo => (0, 1),
            Kierunek::Dol => (1, 0),
            Kierunek::Lewo => (0, -1),
        }
    }
}

fn main() {
    // Wczytaj mapę
    let plik = File::open("./input").expect("Nie udało się otworzyć pliku z mapą.");
    let czytnik = BufReader::new(plik);

    let mut mapa: Vec<String> = Vec::new();
    for linia in czytnik.lines() {
        let linia = linia.expect("Błąd odczytu linii z pliku.");
        mapa.push(linia);
    }

    let wysokosc = mapa.len();
    let szerokosc = if wysokosc > 0 { mapa[0].len() } else { 0 };

    // Znajdź strażnika i kierunek
    let mut pozycja_straznika: (isize, isize) = (-1, -1);
    let mut kierunek_straznika = Kierunek::Gora; // domyślnie, ustawiony będzie później
    
    'szukaj: for (r, wiersz) in mapa.iter().enumerate() {
        for (c, ch) in wiersz.chars().enumerate() {
            match ch {
                '^' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Gora;
                    break 'szukaj;
                }
                'v' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Dol;
                    break 'szukaj;
                }
                '<' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Lewo;
                    break 'szukaj;
                }
                '>' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Prawo;
                    break 'szukaj;
                }
                _ => {}
            }
        }
    }

    if pozycja_straznika == (-1, -1) {
        eprintln!("Nie znaleziono strażnika na mapie.");
        return;
    }

    // Zbiór odwiedzonych pól
    let mut odwiedzone: HashSet<(isize, isize)> = HashSet::new();
    odwiedzone.insert(pozycja_straznika);

    // Funkcja sprawdzająca czy pole jest przeszkodą lub poza mapą
    let czy_przeszkoda = |r: isize, c: isize| {
        if r < 0 || c < 0 || r >= wysokosc as isize || c >= szerokosc as isize {
            // Poza mapą traktujmy jak przeszkodę aby wywołać obrót lub zakończenie ruchu
            return true;
        }
        mapa[r as usize].chars().nth(c as usize).map(|ch| ch == '#').unwrap_or(true)
    };

    // Symulacja ruchu
    loop {
        let (dr, dc) = kierunek_straznika.wektor_ruchu();
        let przed_r = pozycja_straznika.0 + dr;
        let przed_c = pozycja_straznika.1 + dc;

        // Sprawdź czy przed nami jest przeszkoda lub wyjście poza mapę
        if przed_r < 0 || przed_c < 0 || przed_r >= wysokosc as isize || przed_c >= szerokosc as isize {
            // Jeśli wyjście poza mapę - kończymy
            break;
        }
        
        if czy_przeszkoda(przed_r, przed_c) {
            // Jeśli jest przeszkoda przed nami, skręcamy w prawo
            kierunek_straznika = kierunek_straznika.obrot_w_prawo();
        } else {
            // Możemy iść do przodu
            pozycja_straznika = (przed_r, przed_c);
            odwiedzone.insert(pozycja_straznika);
            // Sprawdź czy po ruchu nie opuściliśmy mapy
            if pozycja_straznika.0 < 0 || pozycja_straznika.1 < 0 ||
               pozycja_straznika.0 >= wysokosc as isize || pozycja_straznika.1 >= szerokosc as isize {
                break;
            }
        }
    }

    // Wynik
    let liczba_odwiedzonych = odwiedzone.len();
    println!("{}", liczba_odwiedzonych);
}

