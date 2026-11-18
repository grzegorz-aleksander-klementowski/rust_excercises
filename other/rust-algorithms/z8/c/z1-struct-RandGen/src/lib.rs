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

#[derive(Debug)]
pub enum RandGenError {
    InvalidInput,
    Overflow,
    Internal,
}

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

    // INFO: Linear Congruential Generator (LCG).
    pub fn gen_range(&mut self, min: i64, max: i64) -> Result<i64, RandGenError> {
        if min <= max {
            // INFO: Parameters based on the Numerical Recipes configuration.
            let multiplier: i64 = 1_664_525;
            let increment: i64 = 1_013_904_223;
            let modulus: i64 = 2i64.pow(32);

            // WARN: LCG condition (for educational purpose)
            if multiplier != 0 && multiplier <= modulus && increment <= modulus {
                // The LCG formula (Linear Congruential Generator)
                self.seed = (multiplier
                    .checked_mul(self.seed)
                    .ok_or(RandGenError::Overflow)?
                    .checked_add(increment)
                    .ok_or(RandGenError::Overflow)?)
                .rem_euclid(modulus);

                // Fit to the range and return
                Ok((self.seed.rem_euclid(
                    max.checked_sub(min)
                        .ok_or(RandGenError::Overflow)?
                        .checked_add(1)
                        .ok_or(RandGenError::Overflow)?,
                ))
                .checked_add(min)
                .ok_or(RandGenError::Overflow)?)
            } else {
                Err(RandGenError::Internal) // Return internal error
            }
        } else {
            Err(RandGenError::InvalidInput) // Return invalid input error
        }
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

    #[test]
    fn gen_range_returns_value_within_inclusive_range() {
        let test_seed = [
            RandGen { seed: 0 },
            RandGen { seed: 42 },
            RandGen { seed: -42 },
        ];

        let inputs = [
            [0, 0],
            [1, 1],
            [0, 10],
            [3, 7],
            [3, 15],
            [0, 2],
            [0, 5],
            [10, 15],
            [-10, 15],
            [-150, -100],
            [1, 1_000_000],
            [i64::MAX, i64::MAX],
            [i64::MIN, i64::MIN],
        ];

        for mut seed in test_seed {
            for [min, max] in inputs {
                let actual = seed.gen_range(min, max).unwrap_or_else(|_| panic!("err: Unexpected error during number generation to the range max: {max} and min: {min}. Seed: {seed:?}"));

                assert!(
                    (min..=max).contains(&actual),
                    "err: inputs: min: {min}, max: {max}\ngot: {actual:?}"
                );
            }
        }
    }

    #[test]
    fn gen_range_returns_error() {
        let test_seed = [
            RandGen { seed: 0 },
            RandGen { seed: 42 },
            RandGen { seed: -42 },
        ];

        let inputs = [
            [10, 0],
            [7, 3],
            [1, 0],
            [0, -1],
            [10, -15],
            [-10, -15],
            [1_000_000, 1],
            [i64::MAX, i64::MIN],
        ];

        for mut seed in test_seed {
            for [min, max] in inputs {
                let actual = seed.gen_range(min, max).is_err();
                assert!(
                    actual,
                    "fail: expected the error fir the range min: {min} and max: {max}"
                );
            }
        }
    }
}
