pub fn replace_syst8_with_syst2(octal: &str) -> Option<String> {
    // Check if the string is empty or has got a '8' or '9' character.
    if octal.is_empty() {
        return None;
    }

    let mut res = String::new();
    // „Bit weights” thus these are successive powers of two.
    let bit_weights_arr: [u8; 3] = [4, 2, 1];
    for c in octal.chars() {
        // Check if the number is in the corract range of the character.
        if !('0'..='7').contains(&c) {
            return None;
        }
        // If everything is all right – we can move on.
        // Transfor the num character to u8 number with „quick” method
        let mut eight_sys_num = (c as u8) - b'0';
        // We endure the number is within the 0-7 range
        // Do the calculation if the number is correct (so, non-numbers aren't allowed too!).
        for bit_weight in bit_weights_arr {
            // If the number is equal or lower we add '1', if not – '0'.
            if eight_sys_num >= bit_weight {
                // Calculate the substration of 8sys number minut bit weight – then the rest of
                // 8sys number is what we have to substract further. If the operatiion is
                // possible then we push '1' caracter to the result. If not – we push '0'.
                eight_sys_num -= bit_weight;
                res.push('1');
            } else {
                res.push('0');
            }
        }
    }

    // Now, we have to check if there are zeros of the beggining of the string. First – we look for
    // the index where to cut (instead of cut every time if it found, in matter of optimalisation).
    if res.starts_with('0') {
        let mut possible_zero_index = 0;
        for (n, c) in res.chars().enumerate() {
            if c == '0' {
                possible_zero_index = n;
            } else if c == '1' {
                break;
            }
        }
        res = res.split_off(possible_zero_index + 1);
        // Check if it not cut everything (in case if the number was only zeros).
        if res.is_empty() {
            res.push('0');
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::replace_syst8_with_syst2;

    #[test]
    fn converts_single_digits() {
        assert_eq!(replace_syst8_with_syst2("0"), Some("0".to_string()));
        assert_eq!(replace_syst8_with_syst2("1"), Some("1".to_string()));
        assert_eq!(replace_syst8_with_syst2("2"), Some("10".to_string()));
        assert_eq!(replace_syst8_with_syst2("3"), Some("11".to_string()));
        assert_eq!(replace_syst8_with_syst2("4"), Some("100".to_string()));
        assert_eq!(replace_syst8_with_syst2("5"), Some("101".to_string()));
        assert_eq!(replace_syst8_with_syst2("6"), Some("110".to_string()));
        assert_eq!(replace_syst8_with_syst2("7"), Some("111".to_string()));
    }

    #[test]
    fn converts_multi_digit_numbers() {
        assert_eq!(replace_syst8_with_syst2("10"), Some("1000".to_string()));
        assert_eq!(replace_syst8_with_syst2("17"), Some("1111".to_string()));
        assert_eq!(replace_syst8_with_syst2("20"), Some("10000".to_string()));
        assert_eq!(replace_syst8_with_syst2("77"), Some("111111".to_string()));
        assert_eq!(replace_syst8_with_syst2("123"), Some("1010011".to_string()));
        assert_eq!(
            replace_syst8_with_syst2("777"),
            Some("111111111".to_string())
        );
    }

    #[test]
    fn removes_leading_zeros() {
        assert_eq!(replace_syst8_with_syst2("0001"), Some("1".to_string()));
        assert_eq!(replace_syst8_with_syst2("0010"), Some("1000".to_string()));
        assert_eq!(replace_syst8_with_syst2("0007"), Some("111".to_string()));
    }

    #[test]
    fn zero_is_always_single_zero() {
        assert_eq!(replace_syst8_with_syst2("00"), Some("0".to_string()));
        assert_eq!(replace_syst8_with_syst2("000"), Some("0".to_string()));
        assert_eq!(replace_syst8_with_syst2("000000"), Some("0".to_string()));
    }

    #[test]
    fn converts_large_numbers() {
        assert_eq!(
            replace_syst8_with_syst2("12345670"),
            Some("1010011100101110111000".to_string())
        );

        assert_eq!(
            replace_syst8_with_syst2("77777777"),
            Some("111111111111111111111111".to_string())
        );
    }

    #[test]
    fn rejects_empty_string() {
        assert_eq!(replace_syst8_with_syst2(""), None);
    }

    #[test]
    fn rejects_invalid_characters() {
        assert_eq!(replace_syst8_with_syst2("8"), None);
        assert_eq!(replace_syst8_with_syst2("9"), None);
        assert_eq!(replace_syst8_with_syst2("128"), None);
        assert_eq!(replace_syst8_with_syst2("7a"), None);
        assert_eq!(replace_syst8_with_syst2("abc"), None);
        assert_eq!(replace_syst8_with_syst2("-1"), None);
        assert_eq!(replace_syst8_with_syst2("1 "), None);
        assert_eq!(replace_syst8_with_syst2(" 1"), None);
    }
}
