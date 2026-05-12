// 6. Napisz program, który oblicza silnię dla danej liczby. Zadanie zrób na dwa sposoby — z użyciem pętli while/loop oraz z użyciem pętli for.
// 6. Write a program that calculates the factorial of a given number. Do the task in two ways - using a while/loop loop and using a for loop.

trait Factorial {
    fn factorial(n: usize) -> Self;
}

impl Factorial for usize {
    fn factorial(n: usize) -> Self {
        let mut count: usize = 1;
        let mut result: usize = 1;

        while count <= n {
            result *= count;
            count += 1;
        }

        result
    }
}

fn main() {
    let f = usize::factorial(3);

    println!("3! = {f}");

    let f = usize::factorial(10);
    println!("10! = {f}");
}
