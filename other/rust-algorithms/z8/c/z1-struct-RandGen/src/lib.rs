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
#[derive(Debug, PartialEq)]
pub struct RandGen {
    seed: i64,
}

impl RandGen {
    // INFO: Constructor
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }

    pub fn gen_range(min: i64, max: i64) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::RandGen;

    #[test]
    fn new_creates_instance() {
        let cases = [
            (0, RandGen { seed: 0 }),
            (1, RandGen { seed: 1 }),
            (100, RandGen { seed: 100 }),
            (i64::MAX, RandGen { seed: i64::MAX }),
            (i64::MIN, RandGen { seed: i64::MIN }),
        ];

        cases
            .iter()
            .for_each(|(input, expected)| assert_eq!(RandGen::new(*input), *expected));
    }
}
