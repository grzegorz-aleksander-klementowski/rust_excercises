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
    height: i32,
    width: i32,
    filler: i32,
}

impl Matrix {
    #[allow(dead_code)]
    fn new(height: i32, width: i32, filler: i32) -> Self {
        todo!()
    }
}

mod tests {
    use super::*;

    #[test]
    fn constuctor_new_with_filler() {
        let cases = [
            (
                [100, 100, 100],
                Matrix {
                    height: 100,
                    width: 100,
                    filler: 100,
                },
            ), // Normal case
            (
                [10, 20, 30],
                Matrix {
                    height: 10,
                    width: 20,
                    filler: 30,
                },
            ), // A different input case
            (
                [0, 0, 0],
                Matrix {
                    height: 0,
                    width: 0,
                    filler: 0,
                },
            ), // Zero case
            (
                [-1, -1, -1],
                Matrix {
                    height: -1,
                    width: -1,
                    filler: -1,
                },
            ), // Necative case
            (
                [1_000_000_000, 1_000_000_000, 1_000_000_000],
                Matrix {
                    height: 1_000_000_000,
                    width: 1_000_000_000,
                    filler: 1_000_000_000,
                },
            ), // A big numbe case
            (
                [-1_000_000_000, -1_000_000_000, -1_000_000_000],
                Matrix {
                    height: -1_000_000_000,
                    width: -1_000_000_000,
                    filler: -1_000_000_000,
                },
            ), // A negative big number case
            (
                [i32::MAX, i32::MAX, i32::MAX],
                Matrix {
                    height: i32::MAX,
                    width: i32::MAX,
                    filler: i32::MAX,
                },
            ), // A MAX number
            (
                [i32::MIN, i32::MIN, i32::MIN],
                Matrix {
                    height: i32::MIN,
                    width: i32::MIN,
                    filler: i32::MIN,
                }, // A MIN number
            ),
            (
                [i32::MAX, i32::MIN, i32::MAX],
                Matrix {
                    height: i32::MAX,
                    width: i32::MIN,
                    filler: i32::MAX,
                }, // A MIN number
            ), // A mix number MAX and MIN
        ];

        for (input, expected) in cases {
            let actual = Matrix::new(input[0], input[1], input[2]);
            assert_eq!(
                actual, expected,
                "The error with input: {input:?}, got: {actual:?}."
            )
        }
    }
}
