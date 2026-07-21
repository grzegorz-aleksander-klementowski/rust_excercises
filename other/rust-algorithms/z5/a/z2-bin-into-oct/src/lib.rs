// Change the name for better understanding from `2_sys_value`
pub fn eight_b_bin_into_decimal(bin: &str) -> Option<u8> {
    // Validate the string if it's empty, when we know it is lower than 8 bits.
    if bin.is_empty() {
        // Input is empty.
        return None;
    }

    // Short the bit string to the first 0. If it start with '0', then the program is looking for
    // bit '1'. If it found, then it split the string into two string, to create new 'bin' string
    // from the second part (and cutting off the rest (zeros)).
    let mut bin = bin;
    if bin.starts_with('0') {
        match bin.find(|c| c != '0') {
            Some(idx) => bin = bin.split_at(idx).1,
            None => return Some(0),
        };
    }

    // Set the binary lenth for futher calculation of the conversion
    let bin_len = (bin.len() - 1) as u32;
    if bin_len >= 8 {
        // The input is too long.
        return None;
    }

    let mut res = 0u8;
    for (index, bit) in bin.chars().enumerate() {
        if bit == '1' {
            // If the char is '1', then convert into decimal number.
            let i = index as u32;
            res += 2u8.pow(bin_len - i);
            // If the char is '0' – do nothing.
        } else if bit == '0' {
            // Do nothing
        } else {
            // If it's not '1' or '0' (so the invalid character) – return 'None'.
            return None;
        }
    }

    Some(res)
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
