/* 1. Napisz funkcję o nagłówku

  ```
  fn co_drugi_znak(napis: ...) -> ...
  ```

  która zwróci napis zawierający co drugi znak z danego napisu.
*/

fn co_drugi_znak(napis: &str) -> String {
    if napis.len() <= 1 {
        return String::from(napis);
    }

    // First I transfer to String type to have the data on heap instead of stack. Then transfer
    // into iter chars. Then enumerate to idicate which one is even or not, thus I can know which
    // one is the second one number. I filter it with map to get drop – result, then collect it.

    napis
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c)
        .collect()
}

fn main() {
    let co_drugi = co_drugi_znak("Witaj Świecie!");
    println!("{co_drugi}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(co_drugi_znak(""), "");
    }

    #[test]
    fn test_one_character() {
        assert_eq!(co_drugi_znak("a"), "a");
    }

    #[test]
    fn test_two_characters() {
        assert_eq!(co_drugi_znak("ab"), "a");
    }

    #[test]
    fn test_even_length() {
        assert_eq!(co_drugi_znak("abcdef"), "ace");
    }

    #[test]
    fn test_odd_length() {
        assert_eq!(co_drugi_znak("abcdefg"), "aceg");
    }

    #[test]
    fn test_spaces() {
        assert_eq!(co_drugi_znak("a b c d"), "abcd");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(co_drugi_znak("!@#$%^"), "!#%");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(co_drugi_znak("123456789"), "13579");
    }
}
