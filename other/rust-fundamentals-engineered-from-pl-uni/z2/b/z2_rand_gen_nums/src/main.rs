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
    todo!()
}

fn main() {
    let seed: usize = AskForData::ask_for_data("Provide the seed to generate a number: ");
    let min_rand: usize = AskForData::ask_for_data("Provide the seed to generate a number: ");
    let max_rand: usize = AskForData::ask_for_data("Provide the seed to generate a number: ");
    println!("to–do: {seed}");
}
