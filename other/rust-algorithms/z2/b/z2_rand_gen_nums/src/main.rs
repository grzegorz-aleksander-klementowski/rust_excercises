// 2. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.
//
//     ```
//     fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
//     ```
//     Każde losowanie oczywiście zmienia też ziarno.
//     Możesz wybrać któryś z: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)

// 2. Create a pseudo-random number generator whose seed will be stored outside and given in the first parameter, and the range of random numbers will be given in the second and third parameters.
//
//    ```
//    fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
//    ```
//
//    Each draw, of course, also changes the seed.
//    You can choose one of: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)

use askusr::AskForData;

fn rand(seed: &mut u128, min_rand: u128, max_rand: u128) -> u128 {
    // seed0 → start point
    // modulo → max_rand
    // end untill it repeats
    // bottom row → min_rand
    // multiplier I chose by myself
    // min_rand → increment

    let multiplier: u128 = 1_664_525;
    let increment: u128 = 1_013_904_223;
    let modulus: u128 = 2u128.pow(32);

    // `u128` don't allow num < 0
    assert!(
        (0 == multiplier) || (multiplier > modulus) || (increment > modulus),
        "Incorrect internal value"
    );

    // The LCG formula (Linear Congruential Generator)
    *seed = (multiplier * *seed + increment) % modulus;
    // Fit to the user range
    (*seed % (max_rand - min_rand + 1)) + min_rand
}

fn main() {
    let mut seed: u128 = AskForData::ask_for_data("Provide the seed to generate a number: ");
    let min_rand: u128 =
        AskForData::ask_for_data("Provide the minimal number to generate a number: ");
    let max_rand: u128 =
        AskForData::ask_for_data("Provide the maximal number to generate a number: ");

    // Stop program if user input incorrect values and print info.
    assert!(
        max_rand <= min_rand,
        "Incorrect the user range input. The end range should be larger than the start range."
    );

    let mut lcg_len = 0;

    let lcg0 = seed;
    let mut lcg;

    println!("Random numbers: ");
    loop {
        lcg = rand(&mut seed, min_rand, max_rand);
        print!("{lcg}  ");
        lcg_len += 1;
        if lcg0 == seed {
            break;
        }
    }

    // (I know the lenght of the LCG is out of task, but I did it.)
    println!(
        "\nThe lenght of the LCG (linear congruential generator) with seed {seed}, min num {min_rand}, max num {max_rand} is {lcg_len}"
    );
}
