// Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana.
//
//    Zakładamy, że 0 < a < b < c. Zadanie zrób na dwa sposoby — z użyciem pętli while/loop oraz z użyciem pętli for.
//    9. Write a program that finds all Pythagorean triples with values not greater than the given value.

//   We assume that 0 < a < b < c. Do the task in two ways - using a while/loop loop and using a for loop.

use std::ops::RangeInclusive;

#[derive(std::fmt::Debug)]
pub struct PythagoreanTriangle {
    a: usize,
    b: usize,
    c: usize,
}

// Constructor
impl PythagoreanTriangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
}

pub trait GeometricCondition {
    //fn find_pythagorean_triples(self) -> Self;
    fn satisfies_condition(&self) -> bool;
    fn find_conditions_in_range(range: RangeInclusive<usize>) -> Vec<Self>
    where
        Self: Sized;
}

impl GeometricCondition for PythagoreanTriangle {
    // Exists because in sake of „checking” and writting formula (currently rechecks ordering is unnecessarily)
    fn satisfies_condition(&self) -> bool {
        ((self.a * self.a) + (self.b * self.b) == (self.c * self.c))
            && ((self.a < self.b) && (self.b < self.c))
    }

    // Range as Range typ in the sake of a nice interface
    // Produce candidates
    fn find_conditions_in_range(range: RangeInclusive<usize>) -> Vec<Self> {
        let mut triangles_in_range_list: Vec<Self> = Vec::new();

        let mut range_from = *range.start();
        let range_to = *range.end();
        if range_from == 0 {
            range_from = 1;
        }

        for a in range_from..=range_to - 2 {
            let a_sqr = a * a;

            for b in a + 1..=range_to - 1 {
                let b_sqr = b * b;

                let c = ((a_sqr + b_sqr) as f32).sqrt() as usize;
                // let c = c_sqr.sqrt();

                let p_triangle = Self::new(a, b, c);

                // verify invariants
                if p_triangle.satisfies_condition() {
                    triangles_in_range_list.push(p_triangle);
                }
            }
        }
        triangles_in_range_list
    }
}
