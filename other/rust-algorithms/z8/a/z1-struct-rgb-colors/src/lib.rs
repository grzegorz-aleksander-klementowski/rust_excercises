/*
[PL]
Zestaw 8a

1. Zdefiniuj typ strukturalny do przechowywania koloru w modelu RGB z dokładnością do 8 bitów na składową. Udostępnij operacje potrzebne, aby działał program testujący korzystający z konstruktorów `from_3u8`, `from_percent`, `gray`, `white` i `black`, porównywania kolorów, metody `invert` oraz metod `intensity`, `as_rgb_u8tuple` i `as_cmy_u8tuple`.

   Program testujący powinien poprawnie obsługiwać kolory szary `(127, 127, 127)`, szary utworzony z wartości procentowej `50.0`, purple `(100, 35, 120)`, biały `(255, 255, 255)` i czarny `(0, 0, 0)`.

[EN]
# Set 8a

1. Define a struct for storing an RGB colour with 8 bits of precision per component. Provide the operations required by a test program using the `from_3u8`, `from_percent`, `gray`, `white`, and `black` constructors, colour comparison, the `invert` method, and the `intensity`, `as_rgb_u8tuple`, and `as_cmy_u8tuple` methods.

   The test program should correctly handle the grey colour `(127, 127, 127)`, grey created from `50.0` percent, violet `(100, 35, 120)`, white `(255, 255, 255)`, and black `(0, 0, 0)`.
 */

// The RGB color struct
#[derive(Debug, PartialEq, Eq)]
struct RgbColor(u8, u8, u8);
impl RgbColor {
    // Constructor from 8-bit value
    fn from_3u8(red: u8, green: u8, blue: u8) -> Self {
        todo!()
    }

    // Constructor from percent value
    fn from_percent(percent_red: f32, percent_green: f32, percent_blue: f32) -> Self {
        todo!()
    }

    // Constructor set gray
    fn gray() -> Self {
        todo!()
    }

    // Constructor set white
    fn white() -> Self {
        todo!()
    }

    // Constructor set black
    fn black() -> Self {
        todo!()
    }

    // Constructor set black
    fn purple() -> Self {
        todo!()
    }

    // Invert the RGB colors
    fn invert(&mut self) {
        todo!()
    }

    // Define intensity of the RGB color
    fn intensity(&self) -> f32 {
        todo!()
    }

    // Convert the color into u8 tuple
    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        todo!()
    }

    // Convert the color into u8 tuple of CMY color
    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test all of the color constructors
    #[test]
    fn constructor_from_u8_tuple() {
        // Test constructor from u8 values
        let test_rgb_color: [u8; 3] = [177, 177, 177];
        let expected = RgbColor(test_rgb_color[0], test_rgb_color[1], test_rgb_color[2]);
        let actual = RgbColor::from_3u8(test_rgb_color[0], test_rgb_color[1], test_rgb_color[2]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn constructor_from_percent_values() {
        // Test constructor from percent values (50.0 → 50% → gray walue)
        let test_rgb_color: [f32; 3] = [50.0, 50.0, 50.0];
        let expected = RgbColor(127, 127, 127);
        let actual =
            RgbColor::from_percent(test_rgb_color[0], test_rgb_color[1], test_rgb_color[2]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn constructor_name_color() {
        // Test set collor – the array of the case tuples pair (actual, expecter)
        let cases: [(fn() -> RgbColor, RgbColor); 4] = [
            (RgbColor::gray, RgbColor(127, 127, 127)), // Test the gray color constructor
            (RgbColor::white, RgbColor(255, 255, 255)), // Test the gray color constructor
            (RgbColor::black, RgbColor(0, 0, 0)),      // Test the black color constructor
            (RgbColor::purple, RgbColor(100, 35, 120)), // Test the purple color constructor
        ];

        // Read the cases and test them
        for case in cases {
            let expected = case.1;
            let actual = case.0();

            assert_eq!(actual, expected);
        }
    }
}
