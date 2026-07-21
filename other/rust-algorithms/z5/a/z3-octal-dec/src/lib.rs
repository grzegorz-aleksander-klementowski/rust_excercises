//use ;

pub fn octal_into_u8(octal: &str) -> Option<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_zero() {
        assert_eq!(octal_into_u8("0"), Some(0));
    }

    #[test]
    fn parses_single_digit() {
        assert_eq!(octal_into_u8("7"), Some(7));
    }

    #[test]
    fn parses_multiple_digits() {
        assert_eq!(octal_into_u8("123"), Some(83));
    }

    #[test]
    fn parses_with_leading_zeros() {
        assert_eq!(octal_into_u8("000123"), Some(83));
    }

    #[test]
    fn parses_max_u8() {
        // 377(oct) = 255(dec)
        assert_eq!(octal_into_u8("377"), Some(255));
    }

    #[test]
    fn rejects_first_value_above_u8() {
        // 400(oct) = 256(dec)
        assert_eq!(octal_into_u8("400"), None);
    }

    #[test]
    fn rejects_large_overflow() {
        assert_eq!(octal_into_u8("777"), None);
    }

    #[test]
    fn rejects_digit_8() {
        assert_eq!(octal_into_u8("18"), None);
    }

    #[test]
    fn rejects_digit_9() {
        assert_eq!(octal_into_u8("129"), None);
    }

    #[test]
    fn rejects_letters() {
        assert_eq!(octal_into_u8("12a"), None);
    }

    #[test]
    fn rejects_special_characters() {
        assert_eq!(octal_into_u8("12_3"), None);
    }

    #[test]
    fn rejects_negative_sign() {
        assert_eq!(octal_into_u8("-7"), None);
    }

    #[test]
    fn rejects_plus_sign() {
        assert_eq!(octal_into_u8("+7"), None);
    }

    #[test]
    fn rejects_empty_string() {
        assert_eq!(octal_into_u8(""), None);
    }

    #[test]
    fn rejects_whitespace() {
        assert_eq!(octal_into_u8(" 7"), None);
        assert_eq!(octal_into_u8("7 "), None);
    }
}
