# Set 1a

> Note: the “data” in the tasks below are not read from anywhere. They are fixed as variables in the program source; only the results are displayed.

1. Write a program that displays whether a given year is a leap year.
2. Write a program that displays the number of days in a month from its number and the year number.
3. Write a program converting a temperature in degrees Celsius to degrees Fahrenheit: `F = 32 + 9/5 C`.
4. Write the inverse conversion.
5. Given two valid times within one day, each as integer hours, minutes, and seconds, display their difference in the same form, with minutes and seconds in `[0; 59]`.
6. Compute the factorial of a given number in two ways: with a `while`/`loop` loop and with a `for` loop.
7. Display the digits of a given integer in reverse order.
8. Compute the sum of the digits of a given integer.
9. Find all Pythagorean triples whose values are no greater than a given value. Assume `0 < a < b < c`. Solve it twice: with a `while`/`loop` loop and with a `for` loop.

---

# Set 2a

1. Display a table of visible ASCII characters with their codes, from 33 to 126.
2. Write a function which, for a positive integer `n`, returns the iteration number at which the Collatz problem reaches one (for example, for `n = 12`, return `9`).
3. Write a function that determines whether its argument is an Armstrong number.
4. Write a function that determines whether its argument is a perfect number.
5. Write a function that displays the prime factorisation of a given number.
6. Write:

   ```rust
   fn pow_mod(x: u128, n: u128, p: u128) -> u128
   ```

   It must compute `(xⁿ) % p` correctly for the largest possible inputs.

   * Hint 1: use the property of the remainder of a product, namely multiplication modulo `p`.
   * Hint 2: for time optimisation, use fast exponentiation.

---

# Set 2b

1. Write a three-argument function which rearranges its `i32` arguments in non-decreasing order.

2. Create a pseudorandom-number generator whose seed is stored externally and passed as its first parameter; the second and third parameters specify the generated-number range.

   ```rust
   fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
   ```

   Each draw must change the seed. You may choose a [linear congruential generator](https://en.wikipedia.org/wiki/Linear_congruential_generator).

---

# Set 3a

1. Write:

   ```rust
   fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
   ```

   It approximates a root using Newton’s method, assuming that the supplied functions meet the required conditions and the second is the derivative of the first. Create four versions:

   * iterative, using `loop` and optional `break`, `continue`, or `return`;
   * iterative, using `while` without `break`, `continue`, or `return`;
   * recursive;
   * iterative, using `for` and optional `break`, `continue`, or `return`.

---

# Set 4a

1. Write:

   ```rust
   fn liczba_wystapien(napis: ..., znak: ...) -> ...
   ```

   It counts and returns the occurrences of the specified character in the specified string.

2. Write:

   ```rust
   fn rzymskie(napis: ...) -> ...
   ```

   It receives a valid Roman-numeral string and returns its numeric value. Examples:

   ```rust
   rzymskie("III") == 3
   rzymskie("IX") == 9
   rzymskie("XIX") == 19
   rzymskie("MCMX") == 1910
   ```

---

# Set 4b

1. Write:

   ```rust
   fn co_drugi_znak(napis: ...) -> ...
   ```

   It returns a string containing every second character of the input.

2. Define:

   ```rust
   fn szyfruj(napis: ..., klucz: ...) -> ...
   ```

   It encrypts a string with a simple reversal cipher: the key specifies the length of reversed fragments. Examples:

   ```rust
   szyfruj("Aladyn", 2) == "lAdayn"
   szyfruj("Aladyn", 3) == "alAnyd"
   szyfruj("Aladyn", 4) == "dalAyn"
   szyfruj("Aladyn", 5) == "yndalA"
   szyfruj("koza", 3) == "zoka"
   szyfruj("kaszanka", 3) == "sakanazk"
   szyfruj("kot Mruczek", 9) == "zcumM tok"
   szyfruj("kot Mruczek", 1) == "kot Mruczek"
   szyfruj("kot Mruczek", 2) == "ok tmCruzek"
   ```

3. Write a function taking first name and surname as string parameters. It returns the initial, a full stop, a space, and the surname. In the result, the initial and the first surname letter are uppercase; all remaining letters are lowercase.

   Hint: use `to_lowercase` and `to_uppercase`.

4. Write:

   ```rust
   fn na_rzymskie(liczba: ...) -> ...
   ```

   It returns the Roman-numeral representation of an integer. Examples: `3 -> "III"`, `9 -> "IX"`, `19 -> "XIX"`, `1910 -> "MCMX"`.

5. Write:

   ```rust
   fn dodaj_pisemnie(a: ..., b: ...) -> ...
   ```

   It adds two valid decimal integers supplied as strings and returns a string. Perform written addition because the integers may be arbitrarily large.

---

# Set 5a

1. Write:

   ```rust
   fn zamien_syst8_na_syst2(z: &str) -> Option<String>
   ```

   It converts an unsigned octal-integer representation to binary. The result must be the shortest possible non-empty representation. Return `None` for an empty input or a non-octal character.

2. Write:

   ```rust
   fn wartosc_syst2(z: &str) -> Option<u8>
   ```

   It computes an unsigned binary integer, provided it fits in eight bits. Return `None` otherwise, for an invalid character, or for an empty input.

3. Write:

   ```rust
   fn wartosc_syst8(z: &str) -> Option<u8>
   ```

   It computes an unsigned octal integer, provided it fits in eight bits. Return `None` otherwise, for an invalid character, or for an empty input.

   **Note 1:** Build these functions from previous-task functions.
   **Note 2:** Create two versions: one without and one with the `?` operator.

---

# Set 5b

Choose tasks 1+2 or 3+4; completing all tasks is encouraged.

1. Write:

   ```rust
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   It returns the value of a decimal digit character, or a textual error description if it is not a digit.

2. Write:

   ```rust
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
   ```

   It adds two natural numbers supplied as decimal strings, which need not be valid and may be empty. Return either the string result or an error description. Use written addition because the numbers may be arbitrarily large.

   **Note:** Use the preceding function and the `?` operator.

3. Write:

   ```rust
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   It returns the value of a Roman numeral character, or a textual error description if the character is not one of `I V X L C D M`.

4. Write:

   ```rust
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   It receives a possibly invalid Roman-numeral string; an empty string is invalid too. Return the represented number or an error description. Errors include an invalid digit, an empty string, and invalid digit order.

   **Note:** Use the preceding function and the `?` operator.

---

# Set 6a

Complete every task twice: once with loops and once without loops, using iterators instead.

1. Create and display vectors containing:

   * lowercase English alphabet letters;
   * squares of ten consecutive integers starting from 1;
   * ten consecutive powers of two;
   * reciprocals of every number from 1 to 20;
   * numbers from 1 to 100 divisible by 3 but not by 4.

2. Write functions which, for a given vector of strings, return:

   * strings shorter than four characters;
   * strings containing neither `a` nor `A`;
   * strings containing digits;
   * the same strings reversed;
   * strings containing a doubled letter, for example `inny`, `pizza`, `brutto`, `lekki`, or `dzienny`.

3. Write:

   ```rust
   fn indeksy(tablica: ..., element: &str) -> ...
   ```

   It returns a vector of zero-based indices at which the specified element occurs in the given array.

---

# Set 8a

1. Define a struct for storing an RGB colour with 8 bits of precision per component. Provide the operations required by a test program using the `from_3u8`, `from_percent`, `gray`, `white`, and `black` constructors, colour comparison, the `invert` method, and the `intensity`, `as_rgb_u8tuple`, and `as_cmy_u8tuple` methods.

   The test program should correctly handle the grey colour `(127, 127, 127)`, grey created from `50.0` percent, violet `(100, 35, 120)`, white `(255, 255, 255)`, and black `(0, 0, 0)`.

---

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

---

# Set 8c

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

---

# Set 8d

1. Design a `Towar` type, with any auxiliary types required, for storing warehouse-product data:

   * description/name;
   * unit: items, litres, or kilograms;
   * unit weight in kilograms;
   * required storage conditions: freezer, refrigerator, or normal conditions.

   Its constructor or constructors must reject non-positive weights and, for the “kilograms” unit, enforce a unit weight of `1.0`.

2. Design a `Zamowienie` type, with any auxiliary types required, that stores products and their quantities. It needs:

   * a constructor for an empty order;
   * a method returning the total order weight;
   * a method returning the total weight of items requiring a specified storage condition;
   * a method adding a product in the specified quantity. The quantity must be positive, must be integral for the “items” unit, and adding an identical existing product must increase its quantity rather than add a new entry.

---

# Set 9a

1. Create a generic `Urna` type, based on the concrete type from Set 8c, which can store data of any feasible type and draw from it using the `RandGen` generator from Set 8c. Preserve the behaviour of `new`, `doloz`, `rozmiar`, `losuj_z_us`, and `losuj_bez_us`.

2. Add equivalent tests for:

   * `Urna<i32>`;
   * `Urna<bool>`;
   * `Urna<String>`;
   * `Urna<Moneta>`, where `Moneta` is a custom enum with the values `Orzel` and `Reszka`.

---

# Set 9b

*Tasks by Piotr Kosela, MSc.*

1. Create a `Date` structure and implement:

   ```rust
   fn to_string(&self) -> String
   fn from_3(day: u8, month: Month, year: u16) -> Date
   fn from_string(string: &str, delim: char) -> Date
   ```

2. Create a `Time` structure and implement methods analogous to those of `Date`.

3. Modify `Date` so that it can optionally store a time as well. Add:

   ```rust
   fn has_time(&self) -> bool
   fn set_time(&mut self, time: Time)
   fn clear_time(&mut self)
   ```

4. Implement `PartialOrd`, `Ord`, `PartialEq`, and `Eq` for the modified `Date`, in accordance with a common-sense notion of time.

5. Create a `Task` structure with `name: String`, `description: String`, `priority: Priority` (`Low`, `Medium`, `High`), and `due: Date` fields.

6. Implement sensible `PartialOrd` and `PartialEq` behaviour for `Task`.

---

# Set 9c

1. Traditional playing cards are divided into the suits spades, hearts, diamonds, and clubs. Design an enum representing them, ordered as in bridge: spades, hearts, diamonds, clubs. Derive the appropriate traits and order the variants correctly.

2. Design a type for storing the following possible errors:

   * no error;
   * invalid file format;
   * a missing file, including its name;
   * a file that is too large, including its current and maximum size.

   Also implement `pokaz_komunikat`, which displays a complete message for the specified error, including its data.

---

# Set 10a

1. Create an `enum Gatunek` with several book genres. Then create a `Ksiazka` structure holding title, author, genre, and page count. Store books in a vector and implement a method that filters books of a specified genre with more than 300 pages.

2. Implement a simple issue-tracking system. Each issue has a unique identifier, title, and status: `Otwarte`, `Przetwarzane`, or `Zamkniete(Rezultat)`, where the result can be positive or negative. Store issues in a vector. Add methods to change the status and list only issues with a given status.

3. Define `enum PaymentMethod { Cash, Card, Transfer }` and a `Transaction` structure with `amount: f64` and `method: PaymentMethod` fields. Implement a function summing all transactions of a specified type from an array or vector.

4. Implement a simple airline-reservation system. Each reservation has a passenger, flight number, travel class (economy, business, first), and status (reserved, cancelled). `SystemRezerwacji` stores `Vec<Rezerwacja>` and provides methods to:

   * add a reservation;
   * cancel reservations by passenger surname;
   * count reservations with a given class and status;
   * list passengers in a given class, sorted alphabetically.

5. Design a `DanePogodowe` structure containing a location (`String`), consecutive observation day (`u32`), temperature (`f32`), and weather conditions (sun, clouds, rain, snow). Implement `DziennikPogodowy`, which stores data from multiple days and provides methods returning the average temperature for a specified weather type, the most frequent weather type, and sunny days with temperature above `30°C`.

---

# Set 10b

1. Implement the traditional Polish game of twenty-one ("oczko") — first for one player, then for multiple players. Your implementation should:

   * design suitable types for card suits, card values, cards, a deck, a hand, a result, and other required entities;
   * display them appropriately;
   * handle errors appropriately;
   * use your own pseudorandom-number implementation, while preparing the application for later replacement of that generator;
   * be readable and convenient to use.

---

# Set 13a

1. Create a library project containing an `ulamek` module that implements an `Ulamek` type representing a common fraction. Implement at least:

   * a `new(numerator, denominator)` constructor;
   * `as_f64()`, `licznik()`, and `mianownik()` accessors;
   * the `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=`, `==`, and `!=` operators as traits;
   * a converting `from_str(fraction_as_string)` constructor.

2. The module must also contain a comprehensive test submodule. At minimum, test conversion of `3/4` to `0.75`, `1/3 + 1/2 == 5/6`, failure on a zero denominator, equivalence of `1/-3` and `-2/6`, correct parsing of `"1/-3"`, `"-2/6"`, `"13"`, and `"-26/-2"`, as well as invalid strings such as `"x/-3"` and `"1/3/5"`.

---

# Set 14a

1. Extend the Set 10b task with:

   * automated unit tests;
   * the external `rand` crate from [crates.io](https://crates.io/crates/rand).

---

# Set 15a

Implement two versions of a `stos` module containing a generic `Stos` type that implements a stack for types with suitably selected traits, represented as a singly linked list. It needs the methods `new`, `is_empty`, `top`, `pop`, and `push`.

Use suitable result types that account for possible failures, and write extensive tests in a submodule; they must pass for both versions.

1. In the first version, use `Box` pointers.
2. In the second version, for volunteers, use raw pointers and unsafe code.

---

# Examination — additional tasks

1. Write a function with a header of your own design:

   ```rust
   fn najwiecej_wystapien(ss: &Vec<String>, c: char) -> ...
   ```

   It should return the number of occurrences of `c` in the string from the vector that contains more occurrences than every other string. If no such string exists, return `None`.

2. Write a function with a header of your own design which, for a given vector of integers, returns the first duplicated value. For example, for `[2, 1, 3, 5, 3, 2]`, the result is `3`. If the vector contains no duplicates, report the absence of a result in a clear, idiomatic Rust way.

3. Write:

   ```rust
   fn iloczyn_kolejnych(a: u128, b: u128) -> u128
   ```

   It computes the product of all integers in the closed interval `[a; b]`; assume that the result fits in `u128`. If `a > b`, treat the interval as empty and decide what result should be returned. Examples:

   ```rust
   iloczyn_kolejnych(3, 3) == 3
   iloczyn_kolejnych(0, 400) == 0
   iloczyn_kolejnych(3, 5) == 60
   ```

   Note: using a loop, recursion, or a `return` statement significantly lowers the grade for this task.

4. Write:

   ```rust
   fn pora_dnia(godzina: u8, minuta: u8) -> Result<String, String>
   ```

   It accepts a time of day as an hour and minute in 24-hour notation and returns a textual description: `"noc"`, `"rano"`, `"popoludnie"`, or `"wieczor"`; for invalid data it returns an error description. Assume that:

   * morning starts at `6:30`, inclusive, and ends at `12:00`, inclusive;
   * evening starts at `18:15`, inclusive, and ends at `23:10`, inclusive;
   * afternoon is between morning and evening;
   * the rest is night.
