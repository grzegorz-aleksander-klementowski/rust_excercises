/*
[PL]
4. Napisz funkcję o nagłówku

   ```
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności; ponadto pusty ciąg także jest niepoprawny) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu.

   Błędy mogą być trojakie — niewłaściwa cyfra (o tym powiadomi nas poprzednia funkcja pomocnicza); pusty napis; niewłaściwa kolejność cyfr.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

[EN]
4. Write:

   ```rust
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   It receives a possibly invalid Roman-numeral string; an empty string is invalid too. Return the represented number or an error description. Errors include an invalid digit, an empty string, and invalid digit order.

   **Note:** Use the preceding function and the `?` operator.
 */

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
