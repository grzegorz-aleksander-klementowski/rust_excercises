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

/// Converts an ASCII digit character into its numeric value.
///
/// # Errors
///
/// Returns an error if `c` is not an ASCII digit between `'0'` and `'9'`.
pub fn char_into_u8_digit(c: char) -> Result<u8, String> {
    let n = c as u8;
    if (b'0'..(b'0' + 10)).contains(&n) {
        Ok(n - b'0')
    } else {
        Err(format!("{c} is not a <0, 9> digit."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zwraca_wartosc_dla_0() {
        assert_eq!(char_into_u8_digit('0'), Ok(0));
    }

    #[test]
    fn zwraca_wartosc_dla_5() {
        assert_eq!(char_into_u8_digit('5'), Ok(5));
    }

    #[test]
    fn zwraca_wartosc_dla_9() {
        assert_eq!(char_into_u8_digit('9'), Ok(9));
    }

    #[test]
    fn blad_dla_litery() {
        assert!(char_into_u8_digit('a').is_err());
    }

    #[test]
    fn blad_dla_bialego_znaku() {
        assert!(char_into_u8_digit(' ').is_err());
    }

    #[test]
    fn blad_dla_znaku_specjalnego() {
        assert!(char_into_u8_digit('@').is_err());
    }

    #[test]
    fn blad_dla_cyfry_unicode() {
        // Arabska cyfra ٣ (U+0663)
        assert!(char_into_u8_digit('٣').is_err());
    }
}
