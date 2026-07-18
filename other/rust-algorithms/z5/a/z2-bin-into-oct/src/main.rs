/*
[PL]
2. Napisz funkcję o nagłówku

   ```
   fn eight_b_bin_into_decimal(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest `None`.


[EN]
2. Write a function about the header

   ```
   fn 2_sys_value(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in binary - provided it fits in eight bits. If not (or the notation contains a non-binary character or the parameter is empty), the result is `None`.
 */

// Change the name for better understanding from `2_sys_value`
fn eight_b_bin_into_decimal(bin: &str) -> Option<u8> {
    // Validate the string
    if bin.is_empty() {
        return None;
    }
    // Set the binary lenth for futher calculation of the conversion
    let bin_len = (bin.len() - 1) as u32;
    if bin_len >= 9 {
        return None;
    }
    let mut res = 0u8;
    for (index, bit) in bin.chars().enumerate() {
        if bit == '1' {
            // If the char is '1', then convert into octal number.
            let i = index as u32;
            println!("i: {i}");
            println!("bin_len: {bin_len}");
            res = 2u8.pow(bin_len - i);
        } else if bit == '0' {
            // Do nothing
        } else {
            return None;
        }
    }

    Some(res)
}

fn main() {
    println!(
        "Example 10000, res: {:?}",
        eight_b_bin_into_decimal("10000")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_input_str() {
        assert_eq!(eight_b_bin_into_decimal(""), None);
        assert_eq!(eight_b_bin_into_decimal("abc"), None);
        assert_eq!(eight_b_bin_into_decimal("10c"), None);
        assert_eq!(eight_b_bin_into_decimal("02"), None);
        assert_eq!(eight_b_bin_into_decimal("102"), None);
    }

    #[test]
    fn parses_zero() {
        assert_eq!(eight_b_bin_into_decimal("0"), Some(0));
    }

    #[test]
    fn parses_one() {
        assert_eq!(eight_b_bin_into_decimal("1"), Some(1));
    }

    #[test]
    fn parses_typical_binary_values() {
        assert_eq!(eight_b_bin_into_decimal("10"), Some(2));
        assert_eq!(eight_b_bin_into_decimal("101"), Some(5));
        assert_eq!(eight_b_bin_into_decimal("101010"), Some(42));
        assert_eq!(eight_b_bin_into_decimal("10000000"), Some(128));
    }

    #[test]
    fn parses_largest_u8_value() {
        assert_eq!(eight_b_bin_into_decimal("11111111"), Some(255));
    }

    #[test]
    fn accepts_leading_zeroes() {
        assert_eq!(eight_b_bin_into_decimal("00000101"), Some(5));
        assert_eq!(eight_b_bin_into_decimal("000000000"), Some(0));
        assert_eq!(eight_b_bin_into_decimal("000000001"), Some(1));
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(eight_b_bin_into_decimal(""), None);
    }

    #[test]
    fn rejects_value_larger_than_u8() {
        assert_eq!(eight_b_bin_into_decimal("100000000"), None); // 256
        assert_eq!(eight_b_bin_into_decimal("111111111"), None); // 511
    }

    #[test]
    fn rejects_non_binary_digits() {
        assert_eq!(eight_b_bin_into_decimal("2"), None);
        assert_eq!(eight_b_bin_into_decimal("10201"), None);
        assert_eq!(eight_b_bin_into_decimal("123"), None);
    }

    #[test]
    fn rejects_letters_and_symbols() {
        assert_eq!(eight_b_bin_into_decimal("abc"), None);
        assert_eq!(eight_b_bin_into_decimal("10a01"), None);
        assert_eq!(eight_b_bin_into_decimal("-101"), None);
        assert_eq!(eight_b_bin_into_decimal("+101"), None);
        assert_eq!(eight_b_bin_into_decimal("10_01"), None);
    }

    #[test]
    fn rejects_whitespace() {
        assert_eq!(eight_b_bin_into_decimal(" "), None);
        assert_eq!(eight_b_bin_into_decimal(" 101"), None);
        assert_eq!(eight_b_bin_into_decimal("101 "), None);
        assert_eq!(eight_b_bin_into_decimal("10 1"), None);
        assert_eq!(eight_b_bin_into_decimal("\n101"), None);
    }

    #[test]
    fn rejects_overflow_even_with_valid_binary_characters() {
        let very_large_binary = "1".repeat(1_000);

        assert_eq!(eight_b_bin_into_decimal(&very_large_binary), None);
    }
}
