[PL]
Zestaw 8a

1. Zdefiniuj typ strukturalny do przechowywania koloru w modelu RGB z dokładnością do 8 bitów na składową. Udostępnij operacje potrzebne, aby działał program testujący korzystający z konstruktorów `from_3u8`, `from_percent`, `gray`, `white` i `black`, porównywania kolorów, metody `invert` oraz metod `intensity`, `as_rgb_u8tuple` i `as_cmy_u8tuple`.

   Program testujący powinien poprawnie obsługiwać kolory szary `(127, 127, 127)`, szary utworzony z wartości procentowej `50.0`, fiolet `(100, 35, 120)`, biały `(255, 255, 255)` i czarny `(0, 0, 0)`.

[EN]
# Set 8a

1. Define a struct for storing an RGB colour with 8 bits of precision per component. Provide the operations required by a test program using the `from_3u8`, `from_percent`, `gray`, `white`, and `black` constructors, colour comparison, the `invert` method, and the `intensity`, `as_rgb_u8tuple`, and `as_cmy_u8tuple` methods.

   The test program should correctly handle the grey colour `(127, 127, 127)`, grey created from `50.0` percent, violet `(100, 35, 120)`, white `(255, 255, 255)`, and black `(0, 0, 0)`.
