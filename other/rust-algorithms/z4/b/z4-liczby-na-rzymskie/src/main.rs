/*
4. Napisz funkcję o nagłówku

   ```
   fn na_rzymskie(liczba: ...) -> ...
   ```

   która dla danej liczby całkowitej zwraca jej zapis rzymski. Przykłady:

   ```
   na_rzymskie(3) == "III"
   na_rzymskie(9) == "IX"
   na_rzymskie(19) == "XIX"
   na_rzymskie(1910) == "MCMX"
   ```
 */

use askusr::AskForData;

fn na_rzymskie(liczba: u32) -> String {
    // Checking if the entry number is correct or not.
    assert!(liczba != 0, "The number must be grater than zero.");
    assert!(
        liczba < 4000,
        "The program isn't desgned for the number larger than 3999."
    );

    // Separete the whole number into pieces.
    let mut number = liczba;
    let mut number_arr: [u32; 4] = [0, 0, 0, 0];
    for i in number_arr.iter_mut().rev() {
        *i = number % 10;
        number /= 10;
    }

    // Define the roman numerals in an array. In the end is 2x 'M' char, thus the last „level” of
    // the roman numerals (thousands) are build only from 'M' characters, and the ”handreds level”
    // include 'M' chars too.
    let roman_numerals_arr = [
        ['M', 'M', 'M'], // Thousands
        ['M', 'D', 'C'], // Hundreds
        ['C', 'L', 'X'], // Tens
        ['X', 'V', 'I'], // Ones
    ];
    // Define the levels of the roman numerals. For example: `I`, `V`, `X` is the lowest level.

    // The variable for the result.
    let mut res = String::new();
    // The loop iterate over the separated input number and the roman numerals. The roman numerals
    // are taken in number of 3 because at least 3 numbers are needed to creat a single arabic number (ei. IV (I and V) or IX (I and X) – so we need I, V, X to buld a number from 1–9. It's similar for tens (X, L, C) and hundred (X, D, M).
    for (roman_numerals, arabic_number) in roman_numerals_arr.iter().zip(number_arr) {
        // Calculate hundreds, tens, ones
        let mut counter = arabic_number;
        match arabic_number {
            0..=3 => {
                while counter > 0 {
                    res.push(roman_numerals[2]);
                    counter -= 1;
                }
            }
            4 => {
                res.push(roman_numerals[2]);
                res.push(roman_numerals[1]);
            }
            5..=8 => {
                res.push(roman_numerals[1]);
                while counter - 5 > 0 {
                    res.push(roman_numerals[2]);
                    counter -= 1;
                }
            }
            9 => {
                res.push(roman_numerals[2]);
                res.push(roman_numerals[0]);
            }
            _ => {
                unreachable!("digit must be between 0 and 9");
            }
        }
    }

    res
}

fn main() {
    let number: u32 =
        AskForData::ask_for_data("Napisz numer, który chcesz zamienić na liczbę rzymską: ");
    println!("{}", na_rzymskie(number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_numbers() {
        assert_eq!(na_rzymskie(1), "I");
        assert_eq!(na_rzymskie(2), "II");
        assert_eq!(na_rzymskie(3), "III");
    }

    #[test]
    fn subtractive_notation() {
        assert_eq!(na_rzymskie(4), "IV");
        assert_eq!(na_rzymskie(9), "IX");
        assert_eq!(na_rzymskie(40), "XL");
        assert_eq!(na_rzymskie(90), "XC");
        assert_eq!(na_rzymskie(400), "CD");
        assert_eq!(na_rzymskie(900), "CM");
    }

    #[test]
    fn assignment_examples() {
        assert_eq!(na_rzymskie(19), "XIX");
        assert_eq!(na_rzymskie(1910), "MCMX");
    }

    #[test]
    fn mixed_numbers() {
        assert_eq!(na_rzymskie(58), "LVIII");
        assert_eq!(na_rzymskie(99), "XCIX");
        assert_eq!(na_rzymskie(444), "CDXLIV");
        assert_eq!(na_rzymskie(944), "CMXLIV");
        assert_eq!(na_rzymskie(1994), "MCMXCIV");
        assert_eq!(na_rzymskie(2024), "MMXXIV");
        assert_eq!(na_rzymskie(3999), "MMMCMXCIX");
    }
}
