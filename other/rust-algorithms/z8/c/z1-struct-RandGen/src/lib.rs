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

// INFO: Random number generator struct.
pub struct RandGen {
    seed: i64,
}

impl RandGen {
    // INFO: Constructor
    fn new(seed: i64) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::RandGen;

    #[test]
    fn new_create_instance() {
        let cases = [(0.0, RandGen { seed: 0 })];
    }
}
