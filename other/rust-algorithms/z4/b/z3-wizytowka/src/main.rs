/* 3. Napisz funkcję `wizytówka`, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe.

  Wskazówka: użyj metod `to_lowercase` oraz `to_uppercase`.
*/

fn wizytowka(imię: &str, nazwisko: &str) -> String {
    assert!(
        !imię.is_empty() && !nazwisko.is_empty(),
        "Pola `imię` i `nazwisko` nie mogą być puste."
    );

    let pierwsza_litera_imienia = imię
        .chars()
        .next()
        .unwrap_or_default()
        .to_uppercase()
        .to_string();

    let nazwisko = nazwisko.to_lowercase();

    let mut nazwisko = nazwisko;
    let mut pierwsza_litera_nazwiska = nazwisko.remove(0).to_uppercase().to_string();

    //let nazwisko_bez_pierwszej_litery = nazwisko.to_string().remove(0);
    pierwsza_litera_nazwiska.push_str(&nazwisko);
    let nazwisko = pierwsza_litera_nazwiska.as_str();

    pierwsza_litera_imienia + ". " + nazwisko
}

fn main() {
    println!("{}", wizytowka("Grzegorz", "Klementowski"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wizytowka_poprawnie_formatuje() {
        assert_eq!(wizytowka("jan", "kowalski"), "J. Kowalski");
        assert_eq!(wizytowka("JAN", "KOWALSKI"), "J. Kowalski");
        assert_eq!(wizytowka("jAn", "kOwAlSkI"), "J. Kowalski");
        assert_eq!(wizytowka("a", "b"), "A. B");
        assert_eq!(wizytowka("żaneta", "żądło"), "Ż. Żądło");
    }
}
