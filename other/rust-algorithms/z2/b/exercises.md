# Zestaw 2b

1. Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.

2. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

   ```
   fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
   ```

   Każde losowanie oczywiście zmienia też ziarno.
   Możesz wybrać któryś z: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)


---

## EN:

# Set 2b

1. Write a three-argument function that rearranges the values of its arguments (for reference: type i32) so that they are ordered in non-descending order.

2. Create a pseudo-random number generator whose seed will be stored outside and given in the first parameter, and the range of random numbers will be given in the second and third parameters.

   ```
   fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
   ```

   Each draw, of course, also changes the seed.
   You can choose one of: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)
