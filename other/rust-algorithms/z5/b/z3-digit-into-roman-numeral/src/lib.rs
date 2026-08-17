/*
[PL]
3. Napisz funkcję

   ```
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   która zwraca wartość cyfry rzymskiej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą rzymską (jednym z: I V X L C D M).


[EN]
3. Write:

   ```rust
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   It returns the value of a Roman numeral character, or a textual error description if the character is not one of `I V X L C D M`.
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
