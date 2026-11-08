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

use std::process::Output;

#[derive(Debug, PartialEq)]
enum MatrixError {
    IndexOutOfBounds,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Matrix {
    height: usize,         // usize because of indexing
    width: usize,          // usize because of indexing
    filler: Vec<Vec<f64>>, // vec and reasonable number type in it
}

impl Matrix {
    #[allow(dead_code)]
    pub fn new(height: usize, width: usize, filler: f64) -> Self {
        let row = vec![filler; width];
        Self {
            height,
            width,
            filler: vec![row; height],
        }
    }

    #[allow(dead_code)]
    pub fn zero(height: usize, width: usize) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn identity(height: usize) -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn element(&self, row_index: usize, column_index: usize) -> Option<f64> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn change_element(
        &mut self,
        row_index: usize,
        column_index: usize,
        new_value: f64,
    ) -> Result<(), MatrixError> {
        todo!()
    }

    #[allow(dead_code)]
    fn sum(matrix1: &Self, matrix2: &Self) -> Option<Self> {
        todo!()
    }

    #[allow(dead_code)]
    fn display(&self) {
        todo!()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
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

    #[test]
    fn access_element_in_matrix_by_two_dimensions() {
        // The structure of the cases: (Matrix, [usize; 2], Option<f64>)
        let cases = [
            (
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
                [0_usize, 0_usize],
                None,
            ), // Accessing [0, 0] in an empty matrix returns `None`.
            (
                Matrix {
                    height: 1,
                    width: 0,
                    filler: vec![vec![]],
                },
                [0, 0],
                None,
            ), // A matrix with zero width has no valid columns.
            (
                Matrix {
                    height: 0,
                    width: 1,
                    filler: vec![],
                },
                [0_usize, 0_usize],
                None,
            ), // A matrix with zero height has no valid rows.
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
                [0, 1],
                None,
            ), // The column 1 is out of range in 1x1 dimensions
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
                [1, 0],
                None,
            ), //  Row 1 is out of range for a 1x1 matrix.
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
                [0, 0],
                Some(1.0_f64),
            ), // Matrix with the only element in a 1x1 matrix is at 0x0 and contains 1.0.
            (
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![1.0], vec![0.0]],
                },
                [0, 1],
                None,
            ), // Column 1 is out of range for a 2x1 matrix.
            (
                Matrix {
                    height: 1,
                    width: 2,
                    filler: vec![vec![1.0, 0.0]],
                },
                [1, 0],
                None,
            ), // Row 1 is out of range for a 1x2 matrix.
            (
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![1.0], vec![0.0]],
                },
                [1, 0],
                Some(0.0), // Point to the correct number in 2x1 dimensions
            ),
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ],
                },
                [0, 0],
                Some(1.0), // 3x3 dimensions point correctly to 1.0
            ),
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ],
                },
                [1, 1],
                Some(5.0), // 3x3 dimensions point correctly to 5.0
            ),
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ],
                },
                [2, 2],
                Some(9.0), // 3x3 dimensions: Row 2, column 2 contains 9.0
            ),
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.1, 2.2, 3.3],
                        vec![4.4, 5.5, 6.6],
                        vec![7.7, 8.8, 9.9],
                    ],
                },
                [2, 0],
                Some(7.7), // 3x3 dimensions filled with float numbers, point correctly to 7.7
            ),
            (
                Matrix {
                    height: 3,
                    width: 4,
                    filler: vec![
                        vec![1.1, 2.2, 3.3, 0.01],
                        vec![4.4, 5.5, 6.6, 0.02],
                        vec![7.7, 8.8, 9.9, 0.03],
                    ],
                },
                [2, 3],
                Some(0.03), // 3x4 dimensions filled with float numbers, point correctly to 0.03
            ),
            (
                Matrix {
                    height: 3,
                    width: 4,
                    filler: vec![
                        vec![1.1, 2.2, 3.3, 0.01],
                        vec![4.4, 5.5, 6.6, 0.02],
                        vec![7.7, 8.8, 9.9, 0.03],
                    ],
                },
                [3, 2],
                None, // 3x4 dimensions filled with float numbers, point incorrectly,
                      // return `None`
            ),
        ];

        for (access_object, input, expected) in cases {
            let actual = access_object.element(input[0], input[1]);
            assert_eq!(
                actual, expected,
                "err: \n- object: {access_object:?}; \n- input: {input:?} \n- got: {actual:?}"
            );
        }
    }

    #[test]
    fn change_element_updates_the_selected_cell() {
        /* The array cases inlude types:
        - Matrix (input),
        - [usize; 2] ([row_index, column_index),
        - f64 (new_value),
        - Matrix (expected output).
        - Result<(), MatrixError> (expected result)*/

        let cases = [
            (
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
                [0_usize, 0_usize],
                1.0,
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
                Err(MatrixError::IndexOutOfBounds),
            ), // An invalid 0x0 matrix: changing [0, 0] to `1.0`, returns an error
            (
                Matrix {
                    height: 1,
                    width: 0,
                    filler: vec![vec![]],
                },
                [0_usize, 0_usize],
                1.0,
                Matrix {
                    height: 1,
                    width: 0,
                    filler: vec![vec![]],
                },
                Err(MatrixError::IndexOutOfBounds),
            ), // An invalid 1x0 matrix: changing [0.0] to 1.0 returns an error.
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
                [1_usize, 0_usize],
                1.0,
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
                Err(MatrixError::IndexOutOfBounds),
            ), // A 1x1 matrix: changing [1, 0] to `1.0` returns an out-of-bounds error.
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
                [0, 1],
                1.0,
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
                Err(MatrixError::IndexOutOfBounds),
            ), // A 1x1 matrix changing [0, 1] to `1.0` returns an out-of-bounds error.
            (
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 1.0], vec![1.0, 1.0]],
                },
                [1, 2],
                0.2,
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 1.0], vec![1.0, 1.0]],
                },
                Err(MatrixError::IndexOutOfBounds),
            ), // A 2x2 matrix: changing [1, 2] to `0.2` returns an out-of-bounds error.
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.0]],
                },
                [0_usize, 0_usize],
                1.0,
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
                Ok(()),
            ), // A 1x1 matrix: changing `[0,0]` to `1.0` successds.
            (
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
                },
                [0, 0],
                5.0,
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![5.0, 2.0], vec![3.0, 4.0]],
                },
                Ok(()),
            ), // A 2x2 matrix: changing `[0,0]` to `5.0` cucceeds.
            (
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
                },
                [1, 1],
                0.01,
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![1.0, 2.0], vec![3.0, 0.01]],
                },
                Ok(()),
            ), // A 2x2 matrix: changing `[1,1]` to `0.01` cucceeds.
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.5, 2.5, 3.5],
                        vec![4.5, 5.5, 6.5],
                        vec![7.5, 8.5, 9.5],
                    ],
                },
                [1, 1],
                0.0,
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.5, 2.5, 3.5],
                        vec![4.5, 0.0, 6.5],
                        vec![7.5, 8.5, 9.5],
                    ],
                },
                Ok(()),
            ), // A 3x3 matrix: changing `[1,1]` to `0.0` succeeds.
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.5, 2.5, 3.5],
                        vec![4.5, 5.5, 6.5],
                        vec![7.5, 8.5, 9.5],
                    ],
                },
                [2, 2],
                0.0,
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.5, 2.5, 3.5],
                        vec![4.5, 5.5, 6.5],
                        vec![7.5, 8.5, 0.0],
                    ],
                },
                Ok(()),
            ), // A 3x3 matrix: changing `[2,2]` to `0.0` succeeds.
            (
                Matrix {
                    height: 3,
                    width: 2,
                    filler: vec![vec![1.5, 2.5], vec![4.5, 5.5], vec![7.5, 8.5]],
                },
                [2, 1],
                0.0,
                Matrix {
                    height: 3,
                    width: 2,
                    filler: vec![vec![1.5, 2.5], vec![4.5, 5.5], vec![7.5, 0.0]],
                },
                Ok(()),
            ), // A 3x2 matrix: changing `[2,1]` to `0.0` succeeds.
            (
                Matrix {
                    height: 3,
                    width: 2,
                    filler: vec![vec![0.0, 2.5], vec![4.5, 5.5], vec![7.5, 8.5]],
                },
                [2, 1],
                0.0,
                Matrix {
                    height: 3,
                    width: 2,
                    filler: vec![vec![0.0, 2.5], vec![4.5, 5.5], vec![7.5, 0.0]],
                },
                Ok(()),
            ), // A 3x2 matrix: changing `[2, 1]` to its existing value `0.0`, succeeds.
        ];

        for (object_input, index, new_value, object_output, result) in cases {
            let mut object = object_input;
            let actual = object.change_element(index[0], index[1], new_value);
            assert_eq!((actual, object), (result, object_output));
        }
    }

    #[test]
    fn sum_of_two_matrices_returns_some_for_matching_dimensions() {
        /* Each case contains:
         * - [Matrix; 2] (the two input matrices);
         * - Option(Matrix) (the expected result);
         * */

        let cases = [
            (
                [
                    Matrix {
                        height: 1,
                        width: 1,
                        filler: vec![vec![1.0]],
                    },
                    Matrix {
                        height: 1,
                        width: 1,
                        filler: vec![vec![2.0]],
                    },
                ],
                Some(Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![3.0]],
                }),
            ), // Adding two 1x1 matrices returns `Some(Matrix)`.
            (
                [
                    Matrix {
                        height: 1,
                        width: 1,
                        filler: vec![vec![0.00001]],
                    },
                    Matrix {
                        height: 1,
                        width: 1,
                        filler: vec![vec![0.00002]],
                    },
                ],
                Some(Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![0.00003]],
                }),
            ), // Adding two floats in 1x1 matrices returns `Some(Matrix)`.
            (
                [
                    Matrix {
                        height: 2,
                        width: 2,
                        filler: vec![vec![2.0, 2.0], vec![2.0, 2.0]],
                    },
                    Matrix {
                        height: 2,
                        width: 2,
                        filler: vec![vec![3.0, 3.0], vec![3.0, 3.0]],
                    },
                ],
                Some(Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![5.0, 5.0], vec![5.0, 5.0]],
                }),
            ), // Adding two 2x2 matrices returns `Some(Matrix)`.
            (
                [
                    Matrix {
                        height: 3,
                        width: 2,
                        filler: vec![vec![2.0, 2.0], vec![2.0, 2.0], vec![2.0, 2.0]],
                    },
                    Matrix {
                        height: 3,
                        width: 2,
                        filler: vec![vec![3.0, 3.0], vec![3.0, 3.0], vec![3.0, 3.0]],
                    },
                ],
                Some(Matrix {
                    height: 3,
                    width: 2,
                    filler: vec![vec![5.0, 5.0], vec![5.0, 5.0], vec![5.0, 5.0]],
                }),
            ), // Adding two 3x2 matrices returns `Some(Matrix)`.
            (
                [
                    Matrix {
                        height: 2,
                        width: 3,
                        filler: vec![vec![2.0, 2.0, 2.0], vec![2.0, 2.0, 2.0]],
                    },
                    Matrix {
                        height: 2,
                        width: 3,
                        filler: vec![vec![3.0, 3.0, 3.0], vec![3.0, 3.0, 3.0]],
                    },
                ],
                Some(Matrix {
                    height: 2,
                    width: 3,
                    filler: vec![vec![5.0, 5.0, 5.0], vec![5.0, 5.0, 5.0]],
                }),
            ), // Adding two 2x3 matrices returns `Some(Matrix)`.
            (
                [
                    Matrix {
                        height: 3,
                        width: 3,
                        filler: vec![
                            vec![1.5, 2.5, 3.5],
                            vec![4.5, 5.5, 6.5],
                            vec![7.5, 8.5, 9.5],
                        ],
                    },
                    Matrix {
                        height: 3,
                        width: 3,
                        filler: vec![
                            vec![9.5, 8.5, 7.5],
                            vec![6.5, 5.5, 4.5],
                            vec![3.5, 2.5, 1.5],
                        ],
                    },
                ],
                Some(Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![11.0, 11.0, 11.0],
                        vec![11.0, 11.0, 11.0],
                        vec![11.0, 11.0, 11.0],
                    ],
                }),
            ), // Adding two 3x3 matrices returns `Some(Matrix)`.
        ];

        for ([input1, input2], expected) in cases {
            let actual = Matrix::sum(&input1, &input2);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn sum_of_two_matrices_returns_none_with_different_dimensions() {
        /* Each case contains:
         * - [Matrix; 2] (the two input matrices);
         * - Option(Matrix) (the expected result);
         * */

        let cases = [
            (
                [
                    Matrix {
                        height: 0,
                        width: 0,
                        filler: vec![],
                    },
                    Matrix {
                        height: 0,
                        width: 0,
                        filler: vec![],
                    },
                ],
                None,
            ), // Adding two 0x0 matrices returns `None`.
            (
                [
                    Matrix {
                        height: 1,
                        width: 2,
                        filler: vec![vec![0.00001, 0.00001]],
                    },
                    Matrix {
                        height: 2,
                        width: 1,
                        filler: vec![vec![0.00002], vec![0.00002]],
                    },
                ],
                None,
            ), // Adding two 1x2 and 2x1 matrices with floats returns `None`.
            (
                [
                    Matrix {
                        height: 2,
                        width: 1,
                        filler: vec![vec![2.0], vec![2.0]],
                    },
                    Matrix {
                        height: 1,
                        width: 2,
                        filler: vec![vec![3.0, 3.0]],
                    },
                ],
                None,
            ), // Adding two 1x2 and 2x1 matrices returns `None`.
            (
                [
                    Matrix {
                        height: 3,
                        width: 3,
                        filler: vec![
                            vec![1.5, 2.5, 3.5],
                            vec![4.5, 5.5, 6.5],
                            vec![7.5, 8.5, 9.5],
                        ],
                    },
                    Matrix {
                        height: 3,
                        width: 2,
                        filler: vec![vec![9.5, 8.5], vec![6.5, 5.5], vec![3.5, 2.5]],
                    },
                ],
                None,
            ), // Adding two 3x3 to 3x2 matrices returns `None`.
            (
                [
                    Matrix {
                        height: 3,
                        width: 2,
                        filler: vec![vec![9.5, 8.5], vec![6.5, 5.5], vec![3.5, 2.5]],
                    },
                    Matrix {
                        height: 3,
                        width: 3,
                        filler: vec![
                            vec![1.5, 2.5, 3.5],
                            vec![4.5, 5.5, 6.5],
                            vec![7.5, 8.5, 9.5],
                        ],
                    },
                ],
                None,
            ), // Adding two 2x3 to 3x3 matrices returns `None`.
        ];

        for ([input1, input2], expected) in cases {
            let actual = Matrix::sum(&input1, &input2);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn format_matrix_as_string() {
        /* Each case contains:
         * - Matrix (input);
         * - String (output);*/

        let cases = [
            (
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1.0]],
                },
                "┌─┐\n│1│\n└─┘",
            ), // Format 1x1 matrix.
            (
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0.5], vec![0.5]],
                },
                "┌───┐\n│0.5│\n│0.5│\n└───┘",
            ), // Format 2x1 matrix.
            (
                Matrix {
                    height: 1,
                    width: 2,
                    filler: vec![vec![2.2, 4.1]],
                },
                "┌────────┐\n│2.2  4.1│\n└────────┘",
            ), // Format 1x2 matrix.
            (
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![200.555, 1.1], vec![0.55, 1.11]],
                },
                "┌─────────────┐\n│200.555  1.1 │\n│0.55     1.11│\n└─────────────┘",
            ), // Format 2x2 matrix with different numbers length.
            (
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![vec![200.555, -1.1], vec![-0.55, -1.11]],
                },
                "┌──────────────┐\n│200.555  -1.1 │\n│-0.55    -1.11│\n└──────────────┘",
            ), // Format 2x2 matrix with different numbers length and negative numbers.
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![1.9, 2.9, 3.9],
                        vec![4.9, 5.9, 6.9],
                        vec![7.9, 8.9, 9.9],
                    ],
                },
                "┌─────────────┐\n│1.9  2.9  3.9│\n│4.9  5.9  6.9│\n│7.9  8.9  9.9│\n└─────────────┘",
            ), // Format 3x3 matrix.
            (
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![
                        vec![11.99, 22.99, 33.99],
                        vec![44.99, 55.99, 66.99],
                        vec![77.99, 88.99, 99.99],
                    ],
                },
                "┌───────────────────┐\n│11.99  22.99  33.99│\n│44.99  55.99  66.99│\n│77.99  88.99  99.99│\n└───────────────────┘",
            ), // Format 3x3 matrix and longer numbers.
        ];

        for (input, expected) in cases {
            let actual = input.to_string();
            assert_eq!(actual, expected);
        }
    }
}
