[PL]

# Zestaw 6a

Zadania wykonaj w dwóch wersjach — przy użyciu pętli oraz bez ich użycia (z iteratorami zamiast tego).

1. Utwórz (i wyświetl) wektor zawierający:

   * małe litery alfabetu angielskiego;
   * kwadraty 10 kolejnych liczb całkowitych począwszy od 1;
   * 10 kolejnych potęg dwójki;
   * odwrotności wszystkich liczb od 1 do 20;
   * liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.

2. Napisz zestaw funkcji, które dla danego wektora napisów zwrócą:

   * wektor zawierający napisy krótsze niż 4 znaki;
   * wektor napisów nie zawierających liter 'a' ani 'A';
   * wektor napisów zawierających cyfry;
   * wektor zawierający te same napisy ale odwrócone;
   * wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp).

3. Napisz funkcję

   ```
   fn indeksy(tablica: ..., element: &str) -> ...
   ```

   która zwróci wektor indeksów (licząc od zera), na których znajduje się w zadanej tablicy podany element.

---

[EN]

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
