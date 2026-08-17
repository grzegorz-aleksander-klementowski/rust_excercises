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
   fn value_roman_figure(c: char) -> Result<u16, String>
   ```

   It returns the value of a Roman numeral character, or a textual error description if the character is not one of `I V X L C D M`.
 */

pub fn value_roman_figure(c: char) -> Result<u16, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_roman_numerals() {
        assert_eq!(value_roman_figure('I'), Ok(1));
        assert_eq!(value_roman_figure('V'), Ok(5));
        assert_eq!(value_roman_figure('X'), Ok(10));
        assert_eq!(value_roman_figure('L'), Ok(50));
        assert_eq!(value_roman_figure('C'), Ok(100));
        assert_eq!(value_roman_figure('D'), Ok(500));
        assert_eq!(value_roman_figure('M'), Ok(1000));
    }

    #[test]
    fn incorrect_character_returns_error() {
        assert!(value_roman_figure('A').is_err());
    }

    #[test]
    fn lowercase_liter_returns_error() {
        assert!(value_roman_figure('i').is_err());
    }

    #[test]
    fn special_character_returns_error() {
        assert!(value_roman_figure('?').is_err());
    }
}
