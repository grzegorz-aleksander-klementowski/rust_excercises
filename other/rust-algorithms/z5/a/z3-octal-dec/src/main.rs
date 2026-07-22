/*
[PL]
3. Napisz funkcję o nagłówku

   ```
   fn wartosc_syst8(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest `None`.

   **Uwaga 1:** Funkcje te należy zbudować z funkcji zadań poprzednich.
   **Uwaga 2:** Zrób dwie wersje tej funkcji — pierwszą bez znaku zapytania, a drugą ze znakiem zapytania.

[EN]
3. Write a function about the header

   ```
   fn syst_value8(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in octal - provided it fits in eight bits. If not (or the notation contains a non-octal character or the parameter is empty), the result is `None`.

   **Note 1:** These functions should be built from the functions of the previous tasks.
   **Note 2:** Make two versions of this function - the first without the question mark and the second with the question mark.
 */

use std::env;
fn main() {
    let mut args = env::args();
    let _program = args.next();
    let Some(octal) = args.next() else {
        println!("Wrong argument!");
        return;
    };

    println!("{:?}", z3_octal_dec::octal_into_u8(&octal));
}
