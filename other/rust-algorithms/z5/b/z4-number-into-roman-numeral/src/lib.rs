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

use z3_roman_figure_into_number::convert_roman_fig_into_number;

pub fn number_into_roman_numeral(napis: &str) -> Result<u128, String> {
    println!("Napis: {napis}");
    // Result variable
    let mut res: u128 = 0;
    // Take the string into iter chars
    for c in napis.chars() {
        print!("znak: {c} ");
        let digit = (convert_roman_fig_into_number(c)?) as u128;
        println!("digit: {digit}");
        println!("Add {digit} into result ({res})");
        res += digit;
        println!("Now the result is: {res}");
    }

    // Return the result
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::number_into_roman_numeral;

    /* #[test]
    fn single_roman_digit() {
        assert_eq!(number_into_roman_numeral("I"), Ok(1));
        assert_eq!(number_into_roman_numeral("V"), Ok(5));
        assert_eq!(number_into_roman_numeral("X"), Ok(10));
        assert_eq!(number_into_roman_numeral("L"), Ok(50));
        assert_eq!(number_into_roman_numeral("C"), Ok(100));
        assert_eq!(number_into_roman_numeral("D"), Ok(500));
        assert_eq!(number_into_roman_numeral("M"), Ok(1000));
    } */

    #[test]
    fn valid_multiple_digits() {
        /*  assert_eq!(number_into_roman_numeral("II"), Ok(2));
        assert_eq!(number_into_roman_numeral("III"), Ok(3));
        assert_eq!(number_into_roman_numeral("VII"), Ok(7));
        assert_eq!(number_into_roman_numeral("XX"), Ok(20));
        assert_eq!(number_into_roman_numeral("LX"), Ok(60));
        assert_eq!(number_into_roman_numeral("CXI"), Ok(111)); */
        assert_eq!(number_into_roman_numeral("MDCLXVI"), Ok(1666));
    }

    /* #[test]
       fn digits_may_have_equal_values() {
           assert_eq!(number_into_roman_numeral("III"), Ok(3));
           assert_eq!(number_into_roman_numeral("XXX"), Ok(30));
           assert_eq!(number_into_roman_numeral("CCC"), Ok(300));
           assert_eq!(number_into_roman_numeral("MMM"), Ok(3000));
       }

       #[test]
       fn empty_string_is_invalid() {
           assert!(number_into_roman_numeral("").is_err());
       }

       #[test]
       fn invalid_character_is_rejected() {
           assert!(number_into_roman_numeral("A").is_err());
           assert!(number_into_roman_numeral("XAI").is_err());
           assert!(number_into_roman_numeral("123").is_err());
       }

       #[test]
       fn lowercase_letters_are_invalid() {
           assert!(number_into_roman_numeral("i").is_err());
           assert!(number_into_roman_numeral("xiv").is_err());
       }

       #[test]
       fn whitespace_is_invalid() {
           assert!(number_into_roman_numeral(" ").is_err());
           assert!(number_into_roman_numeral(" X").is_err());
           assert!(number_into_roman_numeral("X ").is_err());
       }

       #[test]
       fn increasing_digit_order_is_invalid() {
           assert!(number_into_roman_numeral("IV").is_err());
           assert!(number_into_roman_numeral("IX").is_err());
           assert!(number_into_roman_numeral("XL").is_err());
           assert!(number_into_roman_numeral("XC").is_err());
           assert!(number_into_roman_numeral("CD").is_err());
           assert!(number_into_roman_numeral("CM").is_err());
       }
    */
    /* #[test]
    fn invalid_order_later_in_the_number_is_rejected() {
        assert!(number_into_roman_numeral("XIV").is_err());
        assert!(number_into_roman_numeral("MXC").is_err());
        assert!(number_into_roman_numeral("DCM").is_err());
    } */

    /* #[test]
    fn valid_digits_must_be_in_non_increasing_order() {
        assert_eq!(number_into_roman_numeral("MDC"), Ok(1600));
        assert_eq!(number_into_roman_numeral("CLX"), Ok(160));
        assert_eq!(number_into_roman_numeral("XVI"), Ok(16));
    } */

    /* #[test]
    fn error_can_occur_after_valid_prefix() {
        assert!(number_into_roman_numeral("MDCZ").is_err());
    } */
}
