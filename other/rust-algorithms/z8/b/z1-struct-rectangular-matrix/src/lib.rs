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

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Matrix {
    height: usize,         // usize because of indexing
    width: usize,          // usize because of indexing
    filler: Vec<Vec<f64>>, // vec and reasonable number type in it
}

impl Matrix {
    #[allow(dead_code)]
    fn new(height: usize, width: usize, filler: f64) -> Self {
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
    fn new_creates_matrix_with_requested_dimensions_and_filler() {
        let cases = [
            (
                (1_usize, 1_usize, 1.0_f64),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
            ), // Normal case
            (
                (3_usize, 3_usize, 1.0_f64),
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.0, 1.0, 1.0],
                        vec![1.0, 1.0, 1.0],
                        vec![1.0, 1.0, 1.0],
                    ],
                },
            ), // Equal numbers of rows and columns
            (
                (1_usize, 2_usize, 0.0_f64),
                Matrix {
                    height: 1,
                    width: 2,
                    filler: vec![vec![0.0, 0.0]],
                },
            ), // A single row and two columns
            (
                (2_usize, 1_usize, 0.0_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.0], vec![0.0]],
                },
            ), // Two rows and a single column
            (
                (2_usize, 1_usize, 1.5_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![1.5], vec![1.5]],
                },
            ), // Fraction filler
            (
                (2_usize, 1_usize, 0.9_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.9], vec![0.9]],
                },
            ), // Fraction 0.9 filler
            (
                (2_usize, 1_usize, 0.1_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.1], vec![0.1]],
                },
            ), // Fraction 0.1 filler
            (
                (2_usize, 1_usize, 0.99999999_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.99999999], vec![0.99999999]],
                },
            ), // Fraction 0.99999999 filler
            (
                (2_usize, 1_usize, 0.000000001_f64),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.000000001], vec![0.000000001]],
                },
            ), // Fraction 0.000000001 filler
            (
                (0_usize, 2_usize, 1.0_f64),
                Matrix {
                    height: 0,
                    width: 2,
                    filler: vec![],
                },
            ), // Zero height and width 2
            (
                (2_usize, 0_usize, 1.0_f64),
                Matrix {
                    height: 2,
                    width: 0,
                    filler: vec![vec![], vec![]],
                },
            ), // Height 2 and zero width
            (
                (0_usize, 0_usize, 0.0_f64),
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
            ), // Zero case
            (
                (1_usize, 1_usize, -1.0_f64),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![-1.0]],
                },
            ), // Negative filler case
            (
                (5_usize, 3_usize, 7.0_f64),
                Matrix {
                    height: 5,
                    width: 3,
                    filler: vec![
                        vec![7.0, 7.0, 7.0],
                        vec![7.0, 7.0, 7.0],
                        vec![7.0, 7.0, 7.0],
                        vec![7.0, 7.0, 7.0],
                        vec![7.0, 7.0, 7.0],
                    ],
                },
            ), // More rows than columns
            (
                (3_usize, 5_usize, -4.0_f64),
                Matrix {
                    height: 3,
                    width: 5,
                    filler: vec![
                        vec![-4.0, -4.0, -4.0, -4.0, -4.0],
                        vec![-4.0, -4.0, -4.0, -4.0, -4.0],
                        vec![-4.0, -4.0, -4.0, -4.0, -4.0],
                    ],
                },
            ), // More columns than rows and a negative filler
            (
                (2_usize, 2_usize, 1_000_000_000.0_f64),
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![
                        vec![1_000_000_000.0, 1_000_000_000.0],
                        vec![1_000_000_000.0, 1_000_000_000.0],
                    ],
                },
            ), // A big number filler
            (
                (2_usize, 2_usize, -1_000_000_000.0_f64),
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![
                        vec![-1_000_000_000.0, -1_000_000_000.0],
                        vec![-1_000_000_000.0, -1_000_000_000.0],
                    ],
                },
            ), // A negative big number filler
            (
                (1, 1, f64::MAX),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![f64::MAX]],
                },
            ), // Largest finite f64 filler
            (
                (1, 1, f64::MIN),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![f64::MIN]],
                },
            ), // Most negative finite f64 filler
            (
                (1, 1, f64::INFINITY),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![f64::INFINITY]],
                },
            ), // Positive infinity filler
            (
                (1, 1, f64::NEG_INFINITY),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![f64::NEG_INFINITY]],
                },
            ), // Negative infinity filler
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
    fn zero_with_different_dimensions() {
        let cases = [
            (
                [0, 0],
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
            ), // All dimensions are zero – empty
            (
                [2, 0],
                Matrix {
                    height: 2,
                    width: 0,
                    filler: vec![vec![], vec![]],
                },
            ), // Include height but width is zero
            (
                [0, 2],
                Matrix {
                    height: 0,
                    width: 2,
                    filler: vec![],
                },
            ), // Zero height and width 2.
            (
                [1, 1],
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
            ), // A single column and a single row
            (
                [5, 1],
                Matrix {
                    height: 5,
                    width: 1,
                    filler: vec![vec![0.0], vec![0.0], vec![0.0], vec![0.0], vec![0.0]],
                },
            ), // many rows and a single column
            (
                [1, 5],
                Matrix {
                    height: 1,
                    width: 5,
                    filler: vec![vec![0.0, 0.0, 0.0, 0.0, 0.0]],
                },
            ), // many columns and a single row
            (
                [2, 2],
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![0.0, 0.0], vec![0.0, 0.0]],
                },
            ), // Square matrix filled with zeroes
            (
                [5, 3],
                Matrix {
                    height: 5,
                    width: 3,
                    filler: vec![
                        vec![0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0],
                    ],
                },
            ), // More rows than columns
            (
                [3, 5],
                Matrix {
                    height: 3,
                    width: 5,
                    filler: vec![
                        vec![0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0],
                    ],
                },
            ), // More columns than rows
        ];
        for (input, expected) in cases {
            let actual = Matrix::zero(input[0], input[1]);
            assert_eq!(
                actual, expected,
                "err: The input: {input:?}, got: {actual:?}."
            )
        }
    }

    #[test]
    fn identity_construct_square_matrix_of_requested_dimension() {
        let cases = [
            (
                0,
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
            ), // Zero/empty
            (
                1,
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
            ), // A single identity (1x1)
            (
                2,
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 0.0], vec![0.0, 1.0]],
                },
            ), // Square matrix identity (2x2)
            (
                3,
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.0, 0.0, 0.0],
                        vec![0.0, 1.0, 0.0],
                        vec![0.0, 0.0, 1.0],
                    ],
                },
            ), // 3 dimension identity (3x3)
            (
                10,
                Matrix {
                    height: 10,
                    width: 10,
                    filler: vec![
                        vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
                        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                    ],
                },
            ), // A big number identity (10x10)
        ];

        for (input, expected) in cases {
            let actual = Matrix::identity(input);
            assert_eq!(
                actual, expected,
                "err: input: {input:?} \nthe result: {actual:?} \n got: {expected:?}"
            )
        }
    }
}
