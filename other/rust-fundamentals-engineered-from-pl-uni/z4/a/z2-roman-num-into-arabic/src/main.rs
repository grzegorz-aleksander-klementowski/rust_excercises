/* 2. Napisz funkcję o nagłówku

  ```
  fn rzymskie(napis: ...) -> ...
  ```

  która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:

  ```
  rzymskie("III") == 3
  rzymskie("IX") == 9
  rzymskie("XIX") == 19
  rzymskie("MCMX") == 1910
  ```

2. Write a function about the header

   ```
   fn roman(inscription: ...) -> ...
   ```

   which, for a string representing a number in Roman writing (we assume its correctness), returns the number represented by the string. Examples:

   ```
   Roman("III") == 3
   Roman("IX") == 9
   Roman("XIX") == 19
   Roman("MCMX") == 1910
   ```

*/

fn roman(inscription: &str) -> usize {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

//TDD way
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_iii() {
        let roman_no = "III";
        assert_eq!(roman(roman_no), 3);
    }

    #[test]
    fn test_roman_vi() {
        let roman_no = "vi";
        assert_eq!(roman(roman_no), 6);
    }

    #[test]
    fn test_roman_ix() {
        let roman_no = "IX";
        assert_eq!(roman(roman_no), 9);
    }

    #[test]
    fn test_roman_xix() {
        let roman_no = "XIX";
        assert_eq!(roman(roman_no), 19);
    }

    #[test]
    fn test_roman_mcmx() {
        let roman_no = "MCMX";
        assert_eq!(roman(roman_no), 1910);
    }

    #[test]
    fn test_roman_empty() {
        let roman_no = "";
        assert_eq!(roman(roman_no), 0);
    }

    #[test]
    fn test_roman_lower_case() {
        let roman_no = "iv";
        assert_eq!(roman(roman_no), 4);
    }
}
