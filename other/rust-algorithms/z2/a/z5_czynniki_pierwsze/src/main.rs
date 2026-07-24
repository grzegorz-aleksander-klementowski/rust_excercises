// 5. Napisz funkcję, która wyświetla rozkład podanej liczby na czynniki pierwsze.
// 5. Write a function that displays the prime factorization of a given number.

use askusr::AskForData;

trait PrimeFactorization {
    fn prime_factorization(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl PrimeFactorization for usize {
    fn prime_factorization(&self) -> Vec<Self> {
        let mut v_prime_n: Vec<Self> = Vec::new();
        if *self == 0 {
            eprintln!("0 has no prime factorization!");
            return v_prime_n;
        }
        let mut n = *self;
        let mut check_div = 2;

        while n > 1 {
            if n.is_multiple_of(check_div) {
                v_prime_n.push(check_div);
                n /= check_div;
            } else {
                check_div += 1;
            }
        }

        v_prime_n
    }
}

fn main() {
    let n: usize =
        AskForData::ask_for_data("Enter the number to show the prime factorization of it: ");

    println!(
        "The prime factorization of number {n}:\n{:?}",
        n.prime_factorization()
    );
}
