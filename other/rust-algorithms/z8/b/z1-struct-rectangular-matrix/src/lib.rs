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
    fn constructor_new_with_filler() {
        let cases = [
            (
                (1_usize, 1_usize, 1_i32),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![1]],
                },
            ), // Normal case
            (
                (3_usize, 3_usize, 1_i32),
                Matrix {
                    height: 3,
                    width: 3,
                    filler: vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
                },
            ), // Normal case – same amount of more columns and rows
            (
                (1_usize, 2_usize, 0_i32),
                Matrix {
                    height: 1,
                    width: 2,
                    filler: vec![vec![0, 0]],
                },
            ), // A single row and two columns
            (
                (2_usize, 1_usize, 0_i32),
                Matrix {
                    height: 2,
                    width: 1,
                    filler: vec![vec![0], vec![0]],
                },
            ), // Two rows and a single column
            (
                (0_usize, 2_usize, 1_i32),
                Matrix {
                    height: 0,
                    width: 2,
                    filler: vec![],
                },
            ), // Zero height and include width
            (
                (2_usize, 0_usize, 1_i32),
                Matrix {
                    height: 2,
                    width: 0,
                    filler: vec![vec![], vec![]],
                },
            ), // Include height and zero width
            (
                (0_usize, 0_usize, 0_i32),
                Matrix {
                    height: 0,
                    width: 0,
                    filler: vec![],
                },
            ), // Zero case
            (
                (1_usize, 1_usize, -1),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![-1]],
                },
            ), // Negative filler case
            (
                (5_usize, 3_usize, 7),
                Matrix {
                    height: 5,
                    width: 3,
                    filler: vec![
                        vec![7, 7, 7],
                        vec![7, 7, 7],
                        vec![7, 7, 7],
                        vec![7, 7, 7],
                        vec![7, 7, 7],
                    ],
                },
            ), // More rows than columns
            (
                (3_usize, 5_usize, -4),
                Matrix {
                    height: 3,
                    width: 5,
                    filler: vec![
                        vec![-4, -4, -4, -4, -4],
                        vec![-4, -4, -4, -4, -4],
                        vec![-4, -4, -4, -4, -4],
                    ],
                },
            ), // More columns than rows and a negative filler
            (
                (2_usize, 2_usize, 1_000_000_000_i32),
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![
                        vec![1_000_000_000, 1_000_000_000],
                        vec![1_000_000_000, 1_000_000_000],
                    ],
                },
            ), // A big number filler
            (
                (2_usize, 2_usize, -1_000_000_000_i32),
                Matrix {
                    height: 2,
                    width: 2,
                    filler: vec![
                        vec![-1_000_000_000, -1_000_000_000],
                        vec![-1_000_000_000, -1_000_000_000],
                    ],
                },
            ), // A negative big number filler
            (
                (1, 1, i32::MAX),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![i32::MAX]],
                },
            ), // A MAX number filler
            (
                (1, 1, i32::MIN),
                Matrix {
                    height: 1,
                    width: 1,
                    filler: vec![vec![i32::MIN]],
                },
            ), // A MIN number filler
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
