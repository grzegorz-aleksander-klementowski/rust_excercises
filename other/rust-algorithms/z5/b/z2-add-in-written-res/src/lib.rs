/*
[PL]
2. Napisz funkcję o nagłówku
   ```
   fn add_by_hand(a: &str, b: &str) -> Result<String, String>
   ```

   która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu).

   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

[EN]
2. Write:

   ```rust
   fn add_by_hand(a: &str, b: &str) -> Result<String, String>
   ```

   It adds two natural numbers supplied as decimal strings, which need not be valid and may be empty. Return either the string result or an error description. Use written addition because the numbers may be arbitrarily large.

   **Note:** Use the preceding function and the `?` operator.
 */

use z1_char_into_u8::char_into_u8_digit;

pub fn add_by_hand(a: &str, b: &str) -> Result<String, String> {
    println!("a-str: {a} | b-str: {b}");

    // Check if the arguments are empty.
    if a.is_empty() || b.is_empty() {
        // Return negative result (ERR)
        return Err("String is empty!".to_string());
    }

    // Initialize vars for fitted strings (used later in the addition loop).
    let mut a_fitted = String::new();
    let mut b_fitted = String::new();

    // #Fitting the strings to be used in the loop addition.
    // Fullfilling the shorter string of digits with zeros.
    let a_len = a.len();
    let b_len = b.len();
    if a_len > b_len {
        let mut diff_a_b = a_len - b_len;
        while diff_a_b == 0 {
            b_fitted.push('0');
            diff_a_b -= 1;
        }
    }
    if a_len < b_len {
        let mut diff_a_b = b_len - a_len;
        while diff_a_b == 0 {
            a_fitted.push('0');
            diff_a_b -= 1;
        }
    }

    // Testing lenths!
    a_fitted.push_str(a);
    b_fitted.push_str(b);
    let a_len = a_fitted.len();
    let b_len = b_fitted.len();
    if a_fitted != b_fitted {
        eprintln!("Wrong lenth! a_fitted: {a_len}, b_fitted: {b_len}.");
        return Err(format!(
            "Wrong lenth! a_fitted: {a_len}, b_fitted: {b_len} "
        ));
    } else {
        println!("`a` and `b` are equal: \n{a_fitted}\n{b_fitted}");
    }

    // Initialize result variable.
    let mut res = String::new();
    // Initialize „carry” to hold the rest from the addition.
    let mut carry = 0;
    // The addition loop where the calculation is done.
    for (char_a, char_b) in a.chars().rev().zip(b.chars().rev()) {
        println!("a: {char_a} | b: {char_b}");
        // Converting the characters digits from the strings into numbers (by the function used in
        // the previous exercise). Return negative result in a case of failing (ERR).
        let num_a = char_into_u8_digit(char_a)?;
        let num_b = char_into_u8_digit(char_b)?;

        // Add two digits and take the carry from it.
        let addition = carry + num_a + num_b;
        carry += addition % 10;

        // Transform the result into a character and push it into the result
        let addition_char = (addition + b'0') as char;
        res.push(addition_char);
    }

    // Return the possitive result (OK)
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dodaje_male_liczby() {
        assert_eq!(add_by_hand("2", "3"), Ok("5".to_string()));
    }

    #[test]
    fn dodaje_zero() {
        assert_eq!(add_by_hand("0", "0"), Ok("0".to_string()));
    }

    #[test]
    fn dodaje_z_przeniesieniem() {
        assert_eq!(add_by_hand("9", "1"), Ok("10".to_string()));
    }

    #[test]
    fn dodaje_wiele_przeniesien() {
        assert_eq!(add_by_hand("999999", "1"), Ok("1000000".to_string()));
    }

    #[test]
    fn dodaje_liczby_roznej_dlugosci() {
        assert_eq!(add_by_hand("123", "98765"), Ok("98888".to_string()));
    }

    #[test]
    fn dodaje_bardzo_duze_liczby() {
        assert_eq!(
            add_by_hand("123456789123456789123456789", "987654321987654321987654321"),
            Ok("1111111111111111111111111110".to_string())
        );
    }

    #[test]
    fn pierwszy_argument_pusty() {
        assert!(add_by_hand("", "123").is_err());
    }

    #[test]
    fn drugi_argument_pusty() {
        assert!(add_by_hand("123", "").is_err());
    }

    #[test]
    fn oba_argumenty_puste() {
        assert!(add_by_hand("", "").is_err());
    }

    #[test]
    fn pierwszy_argument_nieprawidlowy() {
        assert!(add_by_hand("12a3", "456").is_err());
    }

    #[test]
    fn drugi_argument_nieprawidlowy() {
        assert!(add_by_hand("123", "45x6").is_err());
    }

    #[test]
    fn oba_argumenty_nieprawidlowe() {
        assert!(add_by_hand("abc", "xyz").is_err());
    }

    #[test]
    fn spacje_sa_nieprawidlowe() {
        assert!(add_by_hand("123 ", "456").is_err());
        assert!(add_by_hand("123", " 456").is_err());
    }

    #[test]
    fn znak_minus_jest_nieprawidlowy() {
        assert!(add_by_hand("-1", "2").is_err());
    }

    #[test]
    fn znak_plus_jest_nieprawidlowy() {
        assert!(add_by_hand("+1", "2").is_err());
    }

    #[test]
    fn zera_wiodace_sa_poprawne() {
        assert_eq!(add_by_hand("000123", "000077"), Ok("200".to_string()));
    }
}
