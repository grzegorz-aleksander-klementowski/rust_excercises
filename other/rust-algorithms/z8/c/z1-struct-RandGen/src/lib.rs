/*
# [PL]
## Zestaw 8c

1. Oprogramuj typ:

   ```rust
   struct RandGen {
       seed: i64,
   }
   ```

   Powinien on umożliwiać utworzenie generatora przez `RandGen::new(seed)` oraz losowanie liczb przez `gen_range(min, max)`. Dwa generatory z takim samym ziarnem powinny zwracać identyczne kolejne wartości, a wynik `gen_range(3, 15)` musi należeć do przedziału od `3` do `15` włącznie.

# [EN]
## # Set 8c

1. Implement:

   ```rust
   struct RandGen {
       seed: i64,
   }
   ```

   It must allow construction with `RandGen::new(seed)` and number generation with `gen_range(min, max)`. Two generators with the same seed must produce the same sequence, and `gen_range(3, 15)` must return a value from `3` through `15`, inclusive.
 */

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
