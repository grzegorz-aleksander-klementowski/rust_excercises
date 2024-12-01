use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Ścieżka do pliku wejściowego
    let sciezka_pliku = "input";

    // Otwieranie pliku
    let plik = File::open(sciezka_pliku)?;
    let czytnik = io::BufReader::new(plik);

    // Inicjalizacja wektorów
    let mut lista_lewa = Vec::new();
    let mut lista_prawa = Vec::new();

    // Odczyt pliku linia po linii
    for linia in czytnik.lines() {
        let linia = linia?;
        if let Some((lewa, prawa)) = linia.split_once(' ') {
            lista_lewa.push(lewa.trim().parse::<i32>().unwrap());
            lista_prawa.push(prawa.trim().parse::<i32>().unwrap());
        }
    }

    // Sortowanie obu list
    lista_lewa.sort();
    lista_prawa.sort();

    // Obliczanie sumy różnic
    let suma_roznic: i32 = lista_lewa
        .iter()
        .zip(lista_prawa.iter())
        .map(|(lewa, prawa)| (lewa - prawa).abs())
        .sum();

    // Wynik
    println!("Suma różnic: {}", suma_roznic);

    Ok(())
}

