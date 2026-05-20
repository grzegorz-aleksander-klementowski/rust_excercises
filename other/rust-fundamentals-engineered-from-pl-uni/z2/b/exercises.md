# Zestaw 2b

1. Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.

2. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

   ```
   fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
   ```

   Każde losowanie oczywiście zmienia też ziarno.
   Możesz wybrać któryś z: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)
