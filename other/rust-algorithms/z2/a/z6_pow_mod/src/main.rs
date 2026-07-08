// 6. Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (xⁿ) % p w taki sposób, by działało to poprawnie dla jak największych danych.
//
//     * Wskazówka 1: skorzystaj z własności reszty z dzielenia dla iloczynu (czy też inaczej: iloczynu modulo).
//     * Wskazówka 2: w celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.
//
// 6. Write a function pow_mod(x: u128, n: u128, p: u128) -> u128 that will calculate (xⁿ) % p in such a way that it works correctly for the largest possible data.
//
//    * Tip 1: use the property of the remainder of division for the product (or: modulo product).
//    * Tip 2: For possible time optimization, use the fast exponentiation algorithm.

use askusr::AskForData;

trait ToPower {
    fn pow_mod(self, n: Self, p: Self) -> Self;
}

impl ToPower for u128 {
    fn pow_mod(self, n: Self, p: Self) -> Self {
        // The wrong way example:
        // self.pow(n as u32) % p

        let mut result: Self = 1;
        for _ in 0..n {
            result = (result * self) % p;
        }

        result
    }
}

fn main() {
    println!("Enter the numbers you want to calculate xⁿ % p");
    let x: u128 = AskForData::ask_for_data("Enter the x: ");
    let n: u128 = AskForData::ask_for_data("Enter the n: ");
    let p: u128 = AskForData::ask_for_data("Enter the p: ");
    println!("{}", x.pow_mod(n, p));
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_small_power() {
        assert_eq!(u128::pow_mod(2, 10, 1000), 24);
    }
    #[test]
    fn test_fermat_case() {
        assert_eq!(u128::pow_mod(5, 117, 19), 1);
    }
    #[test]
    fn test_zero_exponent() {
        assert_eq!(u128::pow_mod(10, 0, 17), 1);
    }
    #[test]
    fn test_zero_base() {
        assert_eq!(u128::pow_mod(0, 10, 17), 0);
    }
    #[test]
    fn test_mod_one() {
        assert_eq!(u128::pow_mod(42, 42, 1), 0);
    }
    #[test]
    fn test_large_numbers() {
        assert_eq!(
            u128::pow_mod(123_456_789, 123_456, 1_000_000_007),
            116_286_100
        );
    }
}
