use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io::Write;

#[allow(clippy::large_stack_frames)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // *** C++ sytle PASJA INFORMATYKI EXAMPLE ***

    const MAKS: usize = 69253;

    let mut fib: [BigUint; MAKS] = std::array::from_fn(|_| BigUint::zero());

    print!("Ile liczb fibonacciego chcemy wyznaczyć? ");
    std::io::stdout().flush()?;
    let mut odstojnia = String::new();
    std::io::stdin().read_line(&mut odstojnia)?;
    let n = odstojnia.trim().parse::<usize>()?;
    fib[0] = BigUint::one();
    fib[1] = BigUint::one();

    let mut i: usize = 2;
    while i < n {
        fib[i] = &fib[i - 1] + &fib[i - 2];
        i += 1;
    }

    // Print all numbers:
    /* let mut i: usize = 0;
    while i < n {
        println!("Liczba wyrazu {}: {}", i + 1, fib[i]);
        i += 1;
    } */

    // Print the last number
    //println!("Ostatnia liczba Fibonacciego {n}: {}", fib[n - 1]);
    //Złota liczba (gold number)
    let precision = 50u32;
    let num = &fib[n - 1] * BigUint::from(10u32).pow(precision);
    let gold_number = num / &fib[n - 2];
    println!("Złota liczba (gold number): {gold_number}");

    Ok(())
}
