/*
[PL]
2. Napisz funkcję o nagłówku
   ```
   fn add_by_hand(a: &str, b: &str) -> Result<String, String>
   ```

   która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu).

   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

[EN]
2. Write:

   ```rust
   fn add_by_hand(a: &str, b: &str) -> Result<String, String>
   ```

   It adds two natural numbers supplied as decimal strings, which need not be valid and may be empty. Return either the string result or an error description. Use written addition because the numbers may be arbitrarily large.

   **Note:** Use the preceding function and the `?` operator.
 */

pub fn add_by_hand(a: &str, b: &str) -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dodaje_male_liczby() {
        assert_eq!(add_by_hand("2", "3"), Ok("5".to_string()));
    }

    #[test]
    fn dodaje_zero() {
        assert_eq!(add_by_hand("0", "0"), Ok("0".to_string()));
    }

    #[test]
    fn dodaje_z_przeniesieniem() {
        assert_eq!(add_by_hand("9", "1"), Ok("10".to_string()));
    }

    #[test]
    fn dodaje_wiele_przeniesien() {
        assert_eq!(add_by_hand("999999", "1"), Ok("1000000".to_string()));
    }

    #[test]
    fn dodaje_liczby_roznej_dlugosci() {
        assert_eq!(add_by_hand("123", "98765"), Ok("98888".to_string()));
    }

    #[test]
    fn dodaje_bardzo_duze_liczby() {
        assert_eq!(
            add_by_hand("123456789123456789123456789", "987654321987654321987654321"),
            Ok("1111111111111111111111111110".to_string())
        );
    }

    #[test]
    fn pierwszy_argument_pusty() {
        assert!(add_by_hand("", "123").is_err());
    }

    #[test]
    fn drugi_argument_pusty() {
        assert!(add_by_hand("123", "").is_err());
    }

    #[test]
    fn oba_argumenty_puste() {
        assert!(add_by_hand("", "").is_err());
    }

    #[test]
    fn pierwszy_argument_nieprawidlowy() {
        assert!(add_by_hand("12a3", "456").is_err());
    }

    #[test]
    fn drugi_argument_nieprawidlowy() {
        assert!(add_by_hand("123", "45x6").is_err());
    }

    #[test]
    fn oba_argumenty_nieprawidlowe() {
        assert!(add_by_hand("abc", "xyz").is_err());
    }

    #[test]
    fn spacje_sa_nieprawidlowe() {
        assert!(add_by_hand("123 ", "456").is_err());
        assert!(add_by_hand("123", " 456").is_err());
    }

    #[test]
    fn znak_minus_jest_nieprawidlowy() {
        assert!(add_by_hand("-1", "2").is_err());
    }

    #[test]
    fn znak_plus_jest_nieprawidlowy() {
        assert!(add_by_hand("+1", "2").is_err());
    }

    #[test]
    fn zera_wiodace_sa_poprawne() {
        assert_eq!(add_by_hand("000123", "000077"), Ok("200".to_string()));
    }
}
