// 2. Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji, w której osiągamy jedynkę w problemie Collatza (np. dla n = 12 wynikiem jest 9).
//
// 2. Write a function that, for a given positive integer n, returns the number of iterations in which we reach one in the Collatz problem (e.g. for n = 12 the result is 9).

fn collatz_solver(num: usize) -> usize {
    let mut collatz_iterator = 0;
    let mut n = num;
    while n != 1 {
        // if n is even
        if n % 2 == 0 {
            // println!("it's even");
            n /= 2;
        } else {
            // println!("it's odd");
            n = (n * 3) + 1;
        }
        collatz_iterator += 1;
    }
    collatz_iterator
}

fn main() {
    let num = 12;
    println!("Collatz for 12 num is {}", collatz_solver(num));
}
