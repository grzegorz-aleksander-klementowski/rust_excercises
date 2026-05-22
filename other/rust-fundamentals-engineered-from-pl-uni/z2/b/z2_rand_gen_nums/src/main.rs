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

fn rand(seed: &mut usize, min_rand: usize, max_rand: usize) -> usize {
    // seed0 → start point
    // modulo → max_rand
    // end untill it repeats
    // bottom row → min_rand
    // multiplier I chose by myself
    // min_rand → increment

    let multiplier: usize = 2;
    let modulus = 9;
    let increment = 0;

    // `usize` don't allow num < 0
    if (0 == multiplier) && (multiplier > modulus) && (increment > modulus) {
        panic!("Incorrect internal value");
    }

    let lcg = (multiplier * *seed + increment) % modulus;
    if lcg > max_rand {
        lcg % (max_rand - min_rand + 1) + min_rand
    } else {
        lcg
    }
}

fn main() {
    let mut seed: usize = AskForData::ask_for_data("Provide the seed to generate a number: ");
    let min_rand: usize =
        AskForData::ask_for_data("Provide the minimal number to generate a number: ");
    let max_rand: usize =
        AskForData::ask_for_data("Provide the maximal number to generate a number: ");

    let mut lcg_len = 0;

    let lcg0 = rand(&mut seed, min_rand, max_rand);
    let mut lcg1 = lcg0;
    let mut lcg2 = 0;

    println!("Random numbers: ");
    while lcg0 != lcg2 {
        lcg2 = rand(&mut lcg1, min_rand, max_rand);
        print!(" {lcg2} ");
        lcg_len += 1;
    }

    // (I know the lenght of the LCG is out of task, but I did it.)
    println!(
        "The lenght of the LCG (linear congruential generator) with seed {seed}, min num {min_rand}, max num {max_rand} is {lcg_len}"
    );
}
