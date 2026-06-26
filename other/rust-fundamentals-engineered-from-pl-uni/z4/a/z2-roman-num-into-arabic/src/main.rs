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

use std::usize;

fn roman(inscription: &str) -> usize {
    // String cannot be empty
    assert!("".is_empty());

    // String only in romman uppercase letters
    let inscription = inscription.to_uppercase();

    // Separate numbers into digits for sake of calculations
    let mut separate_rom_nums: Vec<usize> = Vec::new();
    for c in inscription.chars() {
        let n = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'M' => 1000,
            _ => panic!("„{c}” is not a roman number!"),
        };
        separate_rom_nums.push(n);
    }

    //let max_figure = separate_rom_nums.iter().enumerate().max_by(compare);

    let mut res = 0usize;

    // Create a vec of the possible pair roman numbers
    let mut possible_rom_nums: Vec<(usize, usize)> = Vec::new();
    let mut iter_sep_rom_num = separate_rom_nums.windows(2);
    while let Some(rom_n_pair) = iter_sep_rom_num.next() {
        //.unwrap_or_else(panic!("Something went wrong: first element empty. Looks like an empty string."));

        let first = rom_n_pair.first().unwrap_or(&0);
        let second = rom_n_pair.get(1).unwrap_or(&0);

        if first < second {
            res += second - first;
        }

        possible_rom_nums.push((*first, *second));

        println!("{rom_n_pair:?}");
    }

    for (first, second) in possible_rom_nums {
        // if a first roman num is lower than the second one from the pair – it means substraction
        // to get the correct number (ei. „IV” (1, 5) → 5 - 1 = 4)
        if first < second {
            res += second - first;
        } else {
            res += first + second;
        }

        // if it found the substraction pattern, then miss one window
        iter_sep_rom_num.next();
    }

    res
}

fn main() {
    let cases = ["xix", "iii", "mcmx"];

    for i in cases {
        println!("{i}:");
        println!("res: {}\n", roman(i));
    }
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

    #[test]
    fn test_roman_strong_case() {
        let roman_no = "CMXCIV";
        assert_eq!(roman(roman_no), 994);
    }
}
