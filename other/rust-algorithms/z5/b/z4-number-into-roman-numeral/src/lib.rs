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
    println!("\n\ninfo: new case – {napis}\n");
    // Return the error if the argument string is empty
    if napis.is_empty() {
        return Err("The string argument is empty! Nothing to convert. ".to_string());
    }
    // Result variable
    let mut res: u128 = 0;

    // Check if the roman figure occur more than 3 times in a window. In case of more than 3 – return an error
    let vec_napis: Vec<char> = napis.chars().collect();
    for c_roman_fig in ['M', 'D', 'C', 'L', 'X', 'V', 'I'] {
        if vec_napis
            .windows(4)
            .any(|win| win.iter().all(|&c| c == c_roman_fig))
        {
            return Err(format!(
                "`{c_roman_fig}` char occur more than 3 times in the string function argument ({napis})."
            ));
        }
    }

    // Convert string into a vector.
    let mut vec_converted_num: Vec<u128> = Vec::new();
    for c in napis.chars() {
        // Converting the roman figure into a number AND it does the VALIDATION of the roman figure.
        let converted_number = convert_roman_fig_into_number(c)? as u128;
        vec_converted_num.push(converted_number);
    }

    // Create variables needed for checking invalid digit order loop below.
    // carry – needed to check if rest any number after adding toghether numbers.
    // windows_vec_conv_num – take 3 number in loop in a time
    // windows_vec_conv_num_len – save the lenth of the window.
    // previous_element – save the previous number (e.i. „I”) to avoid comparing situation, whem are added two correct number (e.i.: „IV”), while the previous one was not correct comparing to the currect pair number (e.i.: „IIV”).
    let mut carry = 0;
    let windows_vec_conv_num = vec_converted_num.windows(2);
    let windows_vec_conv_num_len = windows_vec_conv_num.len();
    let mut previous_element = 0;
    println!("info: set vector coverted number lenth: {windows_vec_conv_num_len}");
    // # Check invalid digit order
    // The loop take the window and enumerate it for to check the last number
    for (e, n) in windows_vec_conv_num.enumerate() {
        println!("info: set enumeration in loop: {e}");
        println!("info: seach in the window: {n:?}");
        // take two numbers from the vector. Checking if the first one is grater than the lower one
        // (like 1000(M) and 100(M)) or same (100 and 100 (CC)) in case of being multuple of 10. In
        // case of being multiple by 5 (5, 50) – chechking correctness of neighbord numbers (IX – 1 and 10)
        if n[0] >= n[1] {
            // In this case we ca add toghether
            println!("info: add n[0] ({}) to result ({res}).", n[0]);
            res += n[0];
            println!("info: now result: {res}");

            // If subtraction was't allowed before – now it is after adding a number to the result.
            if carry != 0 {
                println!(
                    "info: carry found: {carry}. substract carry ({carry}) from result ({res})."
                );
                res -= carry;
                println!("info: now result: {res}");
                carry = 0;
            }
        }
        // If the first element is lower than the second, and is 1 or to power of 10, and
        // is not 50 or 500, then we can add to the result.
        else if (n[0] < n[1])
            && ((n[0].is_multiple_of(10) || n[0] == 1) && (n[0] != 50 && n[0] != 500))
            // Also, check incorrect figure when it doesn't fit the range ofthe neightbord number (can't be „XD” but can be „XL”. `500` is out of range if it is next to `10`.
            // - n[0].pow(2) == n[1] → in case if X/C has to be next to C/M;
            // - n[0] + 9 == n[1] → case of `I` and `X`;
            // - n[0] * 2 == n[1] → case of  V/L/D has to be next to I/X/C/D
            && (n[0] * 10 == n[1] || n[0] + 9 == n[1] || n[0] * 5 == n[1])
        {
            // Check if will not
            if res != 0 {
                if n[0] == previous_element
                    || ([5, 50, 500].contains(&n[1]) && n[1] == previous_element)
                    || n[0] == n[1]
                {
                    return Err(format!(
                        "error: invalid roman numeral sequences ({})",
                        napis
                    ));
                } else {
                    println!(
                        "info: The previous element ({previous_element}) correct – not same as {}",
                        n[0]
                    );
                }
                println!("info: substract n[0] ({}) from result ({res}).", n[0]);
                res -= n[0];
                println!("info: now result: {res}");
            } else {
                carry = n[0];
                println!("info: saved carry: {carry}");
            }
        }
        // Cach the case for 5, 50, and 500
        else if n[0] >= n[1] && ([5, 50, 500].contains(&n[0]) && [5, 50, 500].contains(&n[1])) {
            return Err(format!("error: {} and {} cannot be toghether", n[0], n[1]));
        } else {
            // if any case wasn't included in the above algorithm – it means
            // something is wrong.
            return Err(format!(
                "error: In roman numerials, the number :{:?} and {:?} cannot be stay toghether and be calculated as well.",
                n[0], n[1]
            ));
        }

        if e == windows_vec_conv_num_len {
            println!("info: last one");
        } else {
            println!("info: doesn't find the last element. ");
            println!(
                "info: currect element: {e}. last one (lenth of vec): {windows_vec_conv_num_len}"
            );
        }

        // save the last element for the next loop
        previous_element = n[0];
        println!("info: save the last element for the next loop: {previous_element}");
    }
    // If loop finish without an error – we can add the last element of the vec
    // If there is no last element – return the critical error (it should not happend)
    println!("info: the loop finished.");
    println!(
        "info: the last element of the vector of numbers ({}) added to result ({res}).",
        vec_converted_num.last().unwrap()
    );
    res += vec_converted_num.last().unwrap();
    println!("info: final result: {res}");

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
    #[test]
    fn rejects_invalid_roman_numeral_sequences() {
        // small | small | big → invalid
        assert!(number_into_roman_numeral("IIV").is_err());
        // big-halg | small | big-half
        assert!(number_into_roman_numeral("VIV").is_err());
        // small | big | small → invalid
        assert!(number_into_roman_numeral("IXIX").is_err());
        assert!(number_into_roman_numeral("XXC").is_err());
        assert!(number_into_roman_numeral("CCD").is_err());
        assert!(number_into_roman_numeral("XXXX").is_err());

        // V, L and D cannot be repeated.
        assert!(number_into_roman_numeral("VV").is_err());
        assert!(number_into_roman_numeral("LL").is_err());
        assert!(number_into_roman_numeral("DD").is_err());
    }

    #[test]
    fn accepts_repeated_digits_when_the_overall_numeral_is_valid() {
        // X occurs four times in total, but never more than three times consecutively.
        assert_eq!(number_into_roman_numeral("XXXIX"), Ok(39));

        // M occurs four times in total, but the numeral itself is valid.
        assert_eq!(number_into_roman_numeral("MMMCM"), Ok(3900));
    }
}
