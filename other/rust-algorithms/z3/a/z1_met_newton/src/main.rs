/* 1. Napisz funkcję

```
fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
```

realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona (przy założeniu, że funkcje w parametrach spełniają odpowiednie założenia: druga jest pochodną pierwszej) — w czterech wersjach:

* iteracyjnej z pętlą loop (z ewentualnymi break continue return);
* iteracyjnej z pętlą while (bez żadnych break continue return);
* rekurencyjnej;
* iteracyjnej z pętlą for (z ewentualnymi break continue return).

1. Write a function

   ```
   fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
   ```

   finding the approximate zero using Newton's method (assuming that the functions in the parameters meet the appropriate assumptions: the second one is a derivative of the first one) - in four versions:

   * iterative with a loop (with possible break continue return);
   * iterative with a while loop (without any break continue return);
   * recursive;
   * iterative with a for loop (with possible break continue return). */

use core::f64;

const fn quadratic_fn(x: f64) -> f64 {
    // x.powi(2) - 4.0 → clippy suggest to use ` mul_add`
    x.mul_add(x, -4.0)
}

fn fn_deriv(x: f64) -> f64 {
    2.0 * x
}

fn met_newt_for(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    if x0 == 0.0 {
        return 0.0;
    }

    let mut x = x0;
    let mut steps = 0;

    for i in 0..=n {
        if (-eps..=eps).contains(&f(x)) {
            steps = i;
            break;
        }
        x = x - (f(x) / fprim(x));
    }
    println!("Steps to find: {steps}");

    x
}

fn met_newt_recursive(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    if x0 == 0.0 {
        return 0.0;
    }

    let mut x = x0;

    // Hardcoded the maximum iterator to 100 to not chage the signature of the funtiona and
    // stick to the exercise. Normally I'd add `currect_depth` argument.
    if (n < 100) && !((-eps..=eps).contains(&f(x))) {
        x = x - (f(x) / fprim(x));
        let n = n + 1;
        x = met_newt_recursive(f, fprim, x, eps, n);
    } else {
        println!("Steps to find: {n}");
    }
    x
}

fn met_newt_while(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    if x0 == 0.0 {
        return 0.0;
    }

    let mut count = 0u128;
    let mut x = x0;

    while (count < n) && !((-eps..=eps).contains(&f(x))) {
        x = x - (f(x) / fprim(x));
        count += 1;
    }

    println!("Steps to find: {count}");
    x
}

fn met_newt_loop(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64 {
    if x0 == 0.0 {
        return 0.0;
    }

    let mut count = 0u128;
    let mut x = x0;
    loop {
        let f_x = f(x);

        // Checking if the result is close enought to the zero point
        if (-eps..=eps).contains(&f_x) {
            break;
        }
        // Check if it not exdeed the maximum value to not calculate too much
        else if count >= n {
            eprintln!("Exeeded the maximum value iterator");
            break;
        }
        // If above conditions are all right, then do the calculation

        x = x - (f_x / fprim(x));
        count += 1;
    }
    println!("Found in {count} times. ");
    x
}

fn main() {
    let x_samples = [4.0, 80.0, 1.0, 500.0, 0.1, -0.1, -4.0, -80.0, 0.0];
    for (n, x) in x_samples.iter().enumerate() {
        let n = n + 1;
        println!("\nExample with the loop loop: ");
        println!(
            "Sample {n}. Zero point: {}",
            met_newt_loop(quadratic_fn, fn_deriv, *x, 0.1, 100)
        );

        println!("\nExample with the while loop: ");
        println!("Sample {n}.");
        println!(
            "Zero point of {x}: {}",
            met_newt_while(quadratic_fn, fn_deriv, *x, 0.1, 100)
        );

        println!("\nExample with the recursive loop: ");
        println!("Sample {n}.");
        println!(
            "Zero point of {x}: {}",
            met_newt_recursive(quadratic_fn, fn_deriv, *x, 0.1, 0)
        );

        println!("\nExample with the for loop: ");
        println!("Sample {n}.");
        println!(
            "Zero point of {x}: {}",
            met_newt_for(quadratic_fn, fn_deriv, *x, 0.1, 100)
        );

        println!("\n");
    }
}
