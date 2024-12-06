use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let sciezka = "./input";
    let plik = File::open(&sciezka)?;
    let czytnik = BufReader::new(plik);

    let mut linie = czytnik.lines().map(|l| l.unwrap());
    
    let mut reguly: Vec<(i32, i32)> = Vec::new();
    let mut aktualizacje: Vec<Vec<i32>> = Vec::new();

    loop {
        let linia = match linie.next() {
            Some(l) => l,
            None => break, // brak danych
        };
        let linia = linia.trim();
        if linia.is_empty() {
            break;
        }
        if let Some(idx) = linia.find('|') {
            let lewa = &linia[..idx];
            let prawa = &linia[idx+1..];
            let x: i32 = lewa.trim().parse().expect("Błąd parsowania liczby");
            let y: i32 = prawa.trim().parse().expect("Błąd parsowania liczby");
            reguly.push((x,y));
        }
    }

    for linia in linie {
        let linia = linia.trim();
        if linia.is_empty() {
            continue;
        }
        let strony: Vec<i32> = linia.split(',')
            .map(|elem| elem.trim().parse().expect("Błąd parsowania strony"))
            .collect();
        aktualizacje.push(strony);
    }
    let mut suma_srodkowych = 0;

    'outer: for aktualizacja in &aktualizacje {
        // Mapa: strona -> indeks w aktualizacji
        let mut indeksy = std::collections::HashMap::new();
        for (i, &strona) in aktualizacja.iter().enumerate() {
            indeksy.insert(strona, i);
        }
        for &(x,y) in &reguly {
            if let (Some(ix), Some(iy)) = (indeksy.get(&x), indeksy.get(&y)) {
                // Reguła: x musi być przed y
                if ix >= iy {
                    // Jeśli nie jest spełnione, pomijamy tę aktualizację
                    continue 'outer;
                }
            }
        }
        let n = aktualizacja.len();
        let srodkowy_indeks = n/2; 
        let srodkowa_strona = aktualizacja[srodkowy_indeks];
        suma_srodkowych += srodkowa_strona;
    }

    println!("{}", suma_srodkowych);

    Ok(())
}

