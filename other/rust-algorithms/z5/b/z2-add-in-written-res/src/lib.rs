/*
[PL]
2. Napisz funkcję o nagłówku
   ```
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
   ```

   która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu).

   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

[EN]
2. Write:

   ```rust
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
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
    fn test_example() {
        let result = add_by_hand("2", "2");
        assert_eq!(result, Ok("4".to_string()));
    }
}
