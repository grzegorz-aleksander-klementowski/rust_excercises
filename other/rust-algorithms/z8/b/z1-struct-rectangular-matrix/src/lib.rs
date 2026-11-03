/*
[PL]
# Zestaw 8b

1. Zdefiniuj typ strukturalny (wraz z implementacją metod) przeznaczony do przechowywania macierzy prostokątnej wartości typu `f64`. Weź pod uwagę następujące możliwości:

   * konstruktor `new(wysokosc, szerokosc, wypelniacz) -> Macierz`;
   * konstruktor `zerowa(wysokosc, szerokosc) -> Macierz`;
   * konstruktor `jednostkowa(wysokosc) -> Macierz`;
   * akcesor `macierz.element(indeks_wiersza, indeks_kolumny) -> f64`;
   * mutator `macierz.zmien_element(indeks_wiersza, indeks_kolumny, nowa_wartosc)`;
   * konstruktor `suma(macierz1, macierz2) -> Option<Macierz>`;
   * metoda `macierz.wyswietl()`.

   Napisz testy.

[EN]

# Set 8b

1. Define a struct, together with its methods, for storing a rectangular matrix of `f64` values. Include:

   * `new(height, width, filler) -> Matrix`;
   * `zero(height, width) -> Matrix`;
   * `identity(height) -> Matrix`;
   * `matrix.element(row_index, column_index) -> f64`;
   * `matrix.change_element(row_index, column_index, new_value)`;
   * `sum(matrix1, matrix2) -> Option<Matrix>`;
   * `matrix.display()`.

   Write tests.
 */

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
struct Matrix {
    height: usize,         // usize because of indexing
    width: usize,          // usize because of indexing
    filler: Vec<Vec<i32>>, // vec and reasonable number type in it
}

impl Matrix {
    #[allow(dead_code)]
    fn new(height: usize, width: usize, filler: i32) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn zero(height: usize, width: usize) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    fn identity(height: usize) -> Self {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn constuctor_new_with_filler() {
        let cases = [
            (
                (100_usize, 100_usize, 100_i32),
                Matrix {
                    height: 100,
                    width: 100,
                    filler: 100,
                },
            ), // Normal case
            (
                (10_usize, 20_usize, 30_i32),
                Matrix {
                    height: 10,
                    width: 20,
                    filler: 30,
                },
            ), // A different input case
            (
                (0_usize, 0_usize, 0_i32),
                Matrix {
                    height: 0,
                    width: 0,
                    filler: 0,
                },
            ), // Zero case
            (
                (1_usize, 1_usize, -1),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: -1,
                },
            ), // Negative case
            (
                (1_000_000_000_usize, 1_000_000_000_usize, 1_000_000_000_i32),
                Matrix {
                    height: 1_000_000_000,
                    width: 1_000_000_000,
                    filler: 1_000_000_000,
                },
            ), // A big number case
            (
                (1_000_000_000_usize, 1_000_000_000_usize, -1_000_000_000),
                Matrix {
                    height: 1_000_000_000,
                    width: 1_000_000_000,
                    filler: -1_000_000_000,
                },
            ), // A negative big number case
            (
                (usize::MAX, usize::MAX, i32::MAX),
                Matrix {
                    height: usize::MAX,
                    width: usize::MAX,
                    filler: i32::MAX,
                },
            ), // A MAX number
            (
                (usize::MAX, usize::MAX, i32::MIN),
                Matrix {
                    height: usize::MAX,
                    width: usize::MAX,
                    filler: i32::MIN,
                }, // A MIN number
            ), // A mix number MAX, MAX and MIN
            (
                (usize::MIN, usize::MIN, i32::MAX),
                Matrix {
                    height: 0,
                    width: 0,
                    filler: i32::MAX, // The constructor still assign the value
                }, // A MIN number
            ), // A mix number MIN, MIN and MAX
        ];

        for (input, expected) in cases {
            let actual = Matrix::new(input.0, input.1, input.2);
            assert_eq!(
                actual, expected,
                "The error with input: {input:?}, got: {actual:?}."
            )
        }
    }

    #[test]
    fn zero_with_diffent_values_and_sizes() {
        let cases = [
            (
                [0, 0],
                Matrix {
                    height: 0,
                    width: 0,
                    filler: 0,
                },
            ), // All dimensions are zero
            (
                [10, 0],
                Matrix {
                    height: 10,
                    width: 0,
                    filler: 0,
                },
            ), // Only first demension zero
            (
                [0, 10],
                Matrix {
                    height: 0,
                    width: 10,
                    filler: 0,
                },
            ), // Only second demension zero
            (
                [usize::MAX, usize::MAX],
                Matrix {
                    height: usize::MAX,
                    width: usize::MAX,
                    filler: 0,
                },
            ), // Max
            (
                [10, 20],
                Matrix {
                    height: 10,
                    width: 20,
                    filler: 0,
                },
            ), // Diffrent valuse
            (
                [1_000_000_000, 2_000_000_000],
                Matrix {
                    height: 1_000_000_000,
                    width: 2_000_000_000,
                    filler: 0,
                },
            ), // A big number
        ];
    }
}
