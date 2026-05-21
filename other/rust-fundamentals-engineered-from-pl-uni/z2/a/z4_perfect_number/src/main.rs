// 4. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.
//
// 4. Write a function that answers the question whether its argument is a perfect number.

use askusr::AskForData;

fn is_perfect_num(n: usize) -> bool {
    if n == 0 {
        return false;
    }

    let divisors_sum: usize = (1..n).filter(|x| n.is_multiple_of(*x)).sum();

    divisors_sum == n
}

fn main() {
    let n: usize = AskForData::ask_for_data("Check a number if it's the Perfect Number: ");
    if is_perfect_num(n) {
        println!("The number is the Perfect Number! ");
    } else {
        println!("The number is NOT the Perfect Number! ");
    }
}
