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
    #[allow(dead_code)]
    fn from_3u8(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }

    // Constructor from percent value
    // It represent RGB(R%, G%, B%).
    // If it is not in the percent range (100–0) then return `None`
    // The formula: `(R% * 255 / 100%) (G% * 255 / 100%) (B% * 255 / 100%)`
    #[allow(dead_code)]
    fn from_percent(percent_red: f32, percent_green: f32, percent_blue: f32) -> Option<Self> {
        if ((0.0..=100.0).contains(&percent_red))
            && ((0.0..=100.0).contains(&percent_green))
            && ((0.0..=100.0).contains(&percent_blue))
        {
            Some(Self(
                ((percent_red * 255.0) / 100.0) as u8,
                ((percent_green * 255.0) / 100.0) as u8,
                ((percent_blue * 255.0) / 100.0) as u8,
            ))
        } else {
            None
        }
    }

    // Constructor set gray
    fn gray() -> Self {
        Self(127, 127, 127)
    }

    // Constructor set white
    fn white() -> Self {
        Self(255, 255, 255)
    }

    // Constructor set black
    fn black() -> Self {
        Self(0, 0, 0)
    }

    // Constructor set black
    fn purple() -> Self {
        Self(100, 35, 120)
    }

    // Invert the RGB colors
    // Formula: (255 – R), (255 – G), (255 – B)
    #[allow(dead_code)]
    fn invert(&mut self) {
        self.0 = 255 - self.0;
        self.1 = 255 - self.1;
        self.2 = 255 - self.2;
    }

    // Define intensity of the RGB color
    // Relative luminance calculated according to the sRGB colour space
    // using the ITU-R BT.709 coefficients and linearized RGB components.
    //
    // Formula: Y = 02126R_linear + 0.7152G_linear + 0.0722B_linear
    #[allow(dead_code)]
    fn intensity(&self) -> f32 {
        // Normalization – divide each RGB component by its maximum value (255),
        // converting each component into the range [0.0, 1.0].
        // normalized_rgb variable – Include normalized RGB colors in an array (0–R, 1–G, 2–B)
        let normalized_rgb = [
            self.0 as f32 / 255.0,
            self.1 as f32 / 255.0,
            self.2 as f32 / 255.0,
        ];

        // array for the result of the linearization (0–R, 1–G, 2–B)
        let mut linearized_colors: [f32; 3] = [0.0, 0.0, 0.0];

        // Linearization uses different calculations depending on whether the normalized component is <= 0.04045 or > 0.04045.
        for (i, normalized_color) in normalized_rgb.iter().enumerate() {
            if *normalized_color <= 0.04045 {
                linearized_colors[i] = normalized_color / 12.92;
            } else {
                linearized_colors[i] = ((normalized_color + 0.055) / 1.055).powf(2.4);
            }
        }

        // linearized_colors[0] – red,
        // linearized_colors[1] – green,
        // linearized_colors[2] – blue
        // RESULT – relative luminance Y
        (0.2126 * linearized_colors[0])
            + (0.7152 * linearized_colors[1])
            + (0.0722 * linearized_colors[2])
    }

    // Convert the color into u8 tuple
    #[allow(dead_code)]
    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    // Convert the color into u8 tuple of CMY color
    #[allow(dead_code)]
    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        (255 - self.0, 255 - self.1, 255 - self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test all of the color constructors
    #[test]
    fn constructor_from_u8() {
        // Test constructor from u8 values
        let cases = [
            ([0, 0, 0], RgbColor(0, 0, 0)),             // Black
            ([127, 127, 127], RgbColor(127, 127, 127)), // Gray
            ([255, 255, 255], RgbColor(255, 255, 255)), // White
            ([100, 35, 120], RgbColor(100, 35, 120)),   // Purple
            ([12, 201, 199], RgbColor(12, 201, 199)),   // Mixed channels
        ];
        for (inputs, expected) in cases {
            let actual = RgbColor::from_3u8(inputs[0], inputs[1], inputs[2]);
            assert_eq!(actual, expected, "Failed because of: {inputs:?}");
        }
    }

    #[test]
    fn constructor_from_percent_values() {
        // Test constructor from percent values (50.0 → 50% → gray value)
        let cases = [
            ([0.0, 0.0, 0.0], Some(RgbColor(0, 0, 0))), // Black
            ([50.0, 50.0, 50.0], Some(RgbColor(127, 127, 127))), // Gray
            ([100.0, 100.0, 100.0], Some(RgbColor(255, 255, 255))), // White
            ([25.0, 50.0, 75.0], Some(RgbColor(63, 127, 191))), // Mixed channels
            ([75.0, 25.0, 50.0], Some(RgbColor(191, 63, 127))), // Mixed channels
        ];

        // Test from % collor –  the case tuples pair (actual, expecter)
        for (inputs, expected) in cases {
            let actual = RgbColor::from_percent(inputs[0], inputs[1], inputs[2]);
            assert_eq!(actual, expected, "Failed because of: {inputs:?}");
        }
    }

    #[test]
    fn constructor_from_percent_values_invalid_values() {
        let inputs = [
            [-1.0, 50.0, 50.0],              // Red negative number
            [50.0, -1.0, 50.0],              // Green negative number
            [50.0, 50.0, -1.0],              // Blue negative number
            [101.0, 50.0, 50.0],             // Red more than 100%
            [50.0, 101.0, 50.0],             // Green more than 100%
            [50.0, 50.0, 101.0],             // Blue more than 100%
            [f32::NAN, 50.0, 50.0],          // Red NAN
            [50.0, f32::NAN, 50.0],          // Green NAN
            [50.0, 50.0, f32::NAN],          // Blue NAN
            [f32::INFINITY, 50.0, 50.0],     // Red infinity
            [50.0, f32::INFINITY, 50.0],     // Green infinity
            [50.0, 50.0, f32::INFINITY],     // Blue infinity
            [f32::NEG_INFINITY, 50.0, 50.0], // Red Negative infinity
            [50.0, f32::NEG_INFINITY, 50.0], // Green Negative infinity
            [50.0, 50.0, f32::NEG_INFINITY], // Blue Negative infinity
            [75.0, 25.0, 120.0],             // More than 100%
            [101.0, 100.1, 1001.9],          // More than 100% edge cases
        ];

        // Invalid values should return `None`
        for input in inputs {
            let actual = RgbColor::from_percent(input[0], input[1], input[2]);
            let expected = None;
            assert_eq!(actual, expected, "Failed because of: {input:?}");
        }
    }

    #[test]
    fn named_constructors_create_expected_colors() {
        assert_eq!(RgbColor::gray(), RgbColor(127, 127, 127)); // The gray color constructor
        assert_eq!(RgbColor::white(), RgbColor(255, 255, 255)); // The white color constructor
        assert_eq!(RgbColor::black(), RgbColor(0, 0, 0)); // The black color constructor
        assert_eq!(RgbColor::purple(), RgbColor(100, 35, 120)); // The purple color constructor
    }

    #[test]
    fn invert_colors() {
        let cases = [
            (&mut RgbColor(0, 0, 0), &mut RgbColor(255, 255, 255)), // Black → white
            (&mut RgbColor(78, 78, 78), &mut RgbColor(177, 177, 177)), // Gray → inverse gray
            (&mut RgbColor(255, 255, 255), &mut RgbColor(0, 0, 0)), // White → black
            (&mut RgbColor(155, 220, 135), &mut RgbColor(100, 35, 120)), // Lime → purple
            (&mut RgbColor(100, 35, 120), &mut RgbColor(155, 220, 135)), // Purple → lime
            (&mut RgbColor(0, 255, 255), &mut RgbColor(255, 0, 0)), // Cyan → red
            (&mut RgbColor(255, 0, 255), &mut RgbColor(0, 255, 0)), // fuchsia → green
            (&mut RgbColor(255, 255, 0), &mut RgbColor(0, 0, 255)), // Yellow → blue
        ];

        for (input, expected) in cases {
            input.invert();
            assert_eq!(input, expected);
        }
    }

    // Relative luminance calculated according to the sRGB colour space
    // using the ITU-R BT.709 coefficients and linearized RGB components.
    #[test]
    fn intensity_returns_relative_luminance() {
        let cases = [
            (RgbColor(0, 0, 0), 0.0),
            (RgbColor(255, 255, 255), 1.0),
            (RgbColor(127, 127, 127), 0.21223076),
            (RgbColor(100, 35, 120), 0.05267455),
            (RgbColor(255, 0, 0), 0.2126),
            (RgbColor(0, 255, 0), 0.7152),
            (RgbColor(0, 0, 255), 0.0722),
        ];

        for (color, expected) in cases {
            let actual = color.intensity();
            assert!(
                (actual - expected).abs() < 1e-6,
                "intensity for {color:?}: expected {expected}, got {actual}"
            );
        }
    }

    #[test]
    fn as_rgb_u8tuple_returns_rgb_components() {
        let cases = [
            (RgbColor(0, 0, 0), (0, 0, 0)),             // Black
            (RgbColor(127, 127, 127), (127, 127, 127)), // Gray
            (RgbColor(255, 255, 255), (255, 255, 255)), // White
            (RgbColor(100, 35, 120), (100, 35, 120)),   // Purple
            (RgbColor(12, 201, 199), (12, 201, 199)),   // Mixed channels
        ];

        for (input, expected) in cases {
            let actual = input.as_rgb_u8tuple();
            assert_eq!(actual, expected, "For input: {input:?}, got: {actual:?}");
        }
    }

    #[test]
    fn as_cmy_u8tuple_returns_cmy_components() {
        let cases = [
            (RgbColor(0, 0, 0), (255, 255, 255)),
            (RgbColor(127, 127, 127), (128, 128, 128)),
            (RgbColor(255, 255, 255), (0, 0, 0)),
            (RgbColor(100, 35, 120), (155, 220, 135)),
            (RgbColor(12, 201, 199), (243, 54, 56)),
        ];

        for (input, expected) in cases {
            let actual = input.as_cmy_u8tuple();
            assert_eq!(actual, expected, "For input: {input:?}, got: {actual:?}.");
        }
    }

    #[test]
    fn different_colors_not_equal() {
        let inputs = [
            (RgbColor(100, 35, 120), RgbColor(100, 35, 121)),
            (RgbColor(255, 0, 0), RgbColor(0, 0, 0)),
            (RgbColor(0, 1, 0), RgbColor(0, 0, 0)),
        ];

        for (left, right) in inputs {
            assert_ne!(left, right, "Expected {left:?} and {right:?} to differ.");
        }
    }
}
