// 3. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.
//
// 3. Write a function that answers the question whether its argument is an Armstrong number.

use gak_core::ask_for_data::AskForData;

fn is_armstrong(n: usize) -> bool {
    let mut v_num: Vec<usize> = Vec::new();

    let mut n_len = 0;
    let mut num_to_div = n;

    while num_to_div != 0 {
        n_len += 1;

        v_num.push(num_to_div % 10);
        num_to_div /= 10;
    }

    let calculations: usize = v_num.iter().map(|n| n.pow(n_len)).sum();
    calculations == n
}

fn main() {
    let n: usize = AskForData::ask_for_data("Check a number if it's an Armstrong number: ");
    if is_armstrong(n) {
        println!("The number is the Armstrong number! ");
    } else {
        println!("The number is NOT the Armstrong number! ");
    }
}
