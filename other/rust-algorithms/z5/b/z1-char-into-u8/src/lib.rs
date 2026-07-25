/*
[PL]
1. Napisz funkcję

   ```
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   która zwraca wartość cyfry dziesiętnej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą.

[EN]
1. Write:

   ```rust
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   It returns the value of a decimal digit character, or a textual error description if it is not a digit.
 */

fn char_into_u8_digit(c: char) -> Result<u8, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    /* fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    } */
}
