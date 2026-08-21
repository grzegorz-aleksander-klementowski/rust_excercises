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
    // Return the error if the argument string is empty
    if napis.is_empty() {
        return Err("The string argument is empty! Nothing to convert. ".to_string());
    }
    println!("Napis: {napis}");
    // Result variable
    let mut res: u128 = 0;

    // Check if the roman figure occur more than 3 times. In case of more than 3 – return an error
    for roman_fig in ['M', 'D', 'C', 'L', 'X', 'V', 'I'] {
        let r_fig_conuter = napis.chars().filter(|c| *c == roman_fig).count();
        if r_fig_conuter > 3 {
            return Err(format!(
                "`{roman_fig}` char occur more than 3 times in the string function argument."
            ));
        }
    }

    // Array of roman figures
    let roman_figues_val = [1000, 500, 100, 50, 10, 1];
    // Convert string into a vector.
    let mut vec_converted_num: Vec<u128> = Vec::new();
    for c in napis.chars() {
        // Converting the roman figure into a number
        let converted_number = convert_roman_fig_into_number(c)? as u128;
        vec_converted_num.push(converted_number);
    }

    // # Check invalid digit order
    for n in vec_converted_num.windows(2) {
        println!("\n");
        println!("Obliczam {n:?}");
        // take two numbers from the vector. Checking if the first one is grater than the lower one
        // (like 1000(M) and 100(M)) or same (100 and 100 (CC)) in case of being multuple of 10. IN
        // case being multiple by 5 (5, 50) – chechking correctness of neighbord numbers (IX – 1 and 10)
        if n[0] >= n[1]
        /* && (Some(n) == n.last()) */
        {
            // In this case we ca add toghether
            println!("=== DODAJE ===");
            println!("Wynik: {res}. Dodaje: {}", n[0]);
            res += n[0];
            println!("Nowy wynik: {res}");
        }
        // If the first element is lower than the second, and is 1 or to power of 10, and
        // is not 50 or 500, then we can add to the result.
        // Also, check incorrect figure when it doesn't fit the range ofthe neightbord number (can't be „XD” but can be „XL”. `500` is out of range if it is next to `10`.
        else if (n[0] < n[1])
            && ((n[0].is_multiple_of(10) || n[0] == 1) && (n[0] != 50 && n[0] != 500))
            && (n[0] * n[0] == n[1] || n[0] + 9 == 10)
        {
            res -= n[0];
        }
    }
    // If loop finish without an error – we can add the last element of the vec
    // If there is no last element – return the critical error (it should not happend)
    println!("Dodaje ostatni element: {:?}", vec_converted_num.last());
    res += vec_converted_num.last().unwrap();
    println!("Otrzymano ostateczny wynik: {res}");

    // Return the result
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::number_into_roman_numeral;

    #[test]
    fn single_roman_digit() {
        assert_eq!(number_into_roman_numeral("I"), Ok(1));
        assert_eq!(number_into_roman_numeral("V"), Ok(5));
        assert_eq!(number_into_roman_numeral("X"), Ok(10));
        assert_eq!(number_into_roman_numeral("L"), Ok(50));
        assert_eq!(number_into_roman_numeral("C"), Ok(100));
        assert_eq!(number_into_roman_numeral("D"), Ok(500));
        assert_eq!(number_into_roman_numeral("M"), Ok(1000));
    }

    #[test]
    fn valid_multiple_digits() {
        assert_eq!(number_into_roman_numeral("II"), Ok(2));
        assert_eq!(number_into_roman_numeral("III"), Ok(3));
        assert_eq!(number_into_roman_numeral("VII"), Ok(7));
        assert_eq!(number_into_roman_numeral("XX"), Ok(20));
        assert_eq!(number_into_roman_numeral("LX"), Ok(60));
        assert_eq!(number_into_roman_numeral("CXI"), Ok(111));
        assert_eq!(number_into_roman_numeral("CMXC"), Ok(990));
        assert_eq!(number_into_roman_numeral("MDCLXVI"), Ok(1666));
    }

    #[test]
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
    fn valid_digits_must_be_in_non_increasing_order() {
        assert_eq!(number_into_roman_numeral("MDC"), Ok(1600));
        assert_eq!(number_into_roman_numeral("CLX"), Ok(160));
        assert_eq!(number_into_roman_numeral("XVI"), Ok(16));
    }

    #[test]
    fn error_can_occur_after_valid_prefix() {
        assert!(number_into_roman_numeral("MDCZ").is_err());
        assert!(number_into_roman_numeral("XD").is_err());
    }
}
