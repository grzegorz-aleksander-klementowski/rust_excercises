# [PL]
## Zestaw 8c

1. Oprogramuj typ:

   ```rust
   struct RandGen {
       seed: i64,
   }
   ```

   Powinien on umożliwiać utworzenie generatora przez `RandGen::new(seed)` oraz losowanie liczb przez `gen_range(min, max)`. Dwa generatory z takim samym ziarnem powinny zwracać identyczne kolejne wartości, a wynik `gen_range(3, 15)` musi należeć do przedziału od `3` do `15` włącznie.

2. Stwórz typ `Urna`, który może przechowywać znaki i losować z nich za pomocą generatora `RandGen`. Powinien udostępniać konstruktor przyjmujący generator oraz metody `doloz`, `rozmiar`, `losuj_z_us` i `losuj_bez_us`.

   * Losowanie z usunięciem z pustej urny zwraca `None`.
   * Po włożeniu znaków `'a'`, `'b'`, `'c'`, `'d'` rozmiar urny wynosi `4`.
   * `losuj_z_us` zwraca element i zmniejsza rozmiar urny, a `losuj_bez_us` zwraca element bez zmiany rozmiaru.

# [EN]
## # Set 8c

1. Implement:

   ```rust
   struct RandGen {
       seed: i64,
   }
   ```

   It must allow construction with `RandGen::new(seed)` and number generation with `gen_range(min, max)`. Two generators with the same seed must produce the same sequence, and `gen_range(3, 15)` must return a value from `3` through `15`, inclusive.

2. Create an `Urna` type that stores characters and draws from them using `RandGen`. It must provide a constructor accepting the generator and the methods `doloz`, `rozmiar`, `losuj_z_us`, and `losuj_bez_us`.

   * Drawing with removal from an empty urn returns `None`.
   * After adding `'a'`, `'b'`, `'c'`, and `'d'`, its size is `4`.
   * `losuj_z_us` returns an item and decreases the size; `losuj_bez_us` returns an item without changing the size.


