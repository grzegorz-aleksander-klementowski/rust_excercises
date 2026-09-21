[PL]

# Zestaw 5b

Do wyboru: zadania 1+2 lub 3+4. (Oczywiście warto zrobić wszystkie.)

1. Napisz funkcję

   ```
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   która zwraca wartość cyfry dziesiętnej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą.

2. Napisz funkcję o nagłówku

   ```
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
   ```

   która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu).

   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

3. Napisz funkcję

   ```
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   która zwraca wartość cyfry rzymskiej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą rzymską (jednym z: I V X L C D M).

4. Napisz funkcję o nagłówku

   ```
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności; ponadto pusty ciąg także jest niepoprawny) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu.

   Błędy mogą być trojakie — niewłaściwa cyfra (o tym powiadomi nas poprzednia funkcja pomocnicza); pusty napis; niewłaściwa kolejność cyfr.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

---

[EN]


# Set 5b

Choose tasks 1+2 or 3+4; completing all tasks is encouraged.

1. Write:

   ```rust
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   It returns the value of a decimal digit character, or a textual error description if it is not a digit.

2. Write:

   ```rust
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
   ```

   It adds two natural numbers supplied as decimal strings, which need not be valid and may be empty. Return either the string result or an error description. Use written addition because the numbers may be arbitrarily large.

   **Note:** Use the preceding function and the `?` operator.

3. Write:

   ```rust
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   It returns the value of a Roman numeral character, or a textual error description if the character is not one of `I V X L C D M`.

4. Write:

   ```rust
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   It receives a possibly invalid Roman-numeral string; an empty string is invalid too. Return the represented number or an error description. Errors include an invalid digit, an empty string, and invalid digit order.

   **Note:** Use the preceding function and the `?` operator.

