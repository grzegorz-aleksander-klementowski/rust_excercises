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
   fn convert_roman_fig_into_number(c: char) -> Result<u16, String>
   ```

   It returns the value of a Roman numeral character, or a textual error description if the character is not one of `I V X L C D M`.
 */

/// Converts a Roman numeral character into its numeric value.
///
/// # Errors
///
/// Returns an error if `c` is not one of the recognised Roman numeral
/// characters: `I`, `V`, `X`, `L`, `C`, `D`, or `M`.
pub fn convert_roman_fig_into_number(c: char) -> Result<u16, String> {
    match c {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(format!(
            "`{c}` is an invalid character. Cannot be converted into a number."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_roman_numerals() {
        assert_eq!(convert_roman_fig_into_number('I'), Ok(1));
        assert_eq!(convert_roman_fig_into_number('V'), Ok(5));
        assert_eq!(convert_roman_fig_into_number('X'), Ok(10));
        assert_eq!(convert_roman_fig_into_number('L'), Ok(50));
        assert_eq!(convert_roman_fig_into_number('C'), Ok(100));
        assert_eq!(convert_roman_fig_into_number('D'), Ok(500));
        assert_eq!(convert_roman_fig_into_number('M'), Ok(1000));
    }

    #[test]
    fn incorrect_character_returns_error() {
        assert!(convert_roman_fig_into_number('A').is_err());
    }

    #[test]
    fn lowercase_liter_returns_error() {
        assert!(convert_roman_fig_into_number('i').is_err());
    }

    #[test]
    fn special_character_returns_error() {
        assert!(convert_roman_fig_into_number('?').is_err());
    }
}
