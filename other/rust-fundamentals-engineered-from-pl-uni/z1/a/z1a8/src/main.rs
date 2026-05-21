// 8. Napisz program, który oblicza sumę cyfr danej liczby całkowitej.
// 8. Write a program that calculates the sum of the digits of a given integer

trait DigitsSum {
    fn sum_of_digits(self) -> Self;
}

impl DigitsSum for isize {
    fn sum_of_digits(self) -> Self {
        let mut num = self.abs();
        let mut result = 0;
        while num > 0 {
            result += num % 10;
            num /= 10;
        }

        result
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Can't read line.");
    let num: isize = buf.trim().parse().expect("Can't parse.");
    let num = num.sum_of_digits();
    println!("{num}");
}
