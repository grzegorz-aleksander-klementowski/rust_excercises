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

fn na_rzymskie(liczba: usize) -> String {
    todo!()
}

fn main() {
    println!("Hello, world!");
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
