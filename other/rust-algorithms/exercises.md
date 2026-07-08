# Zestaw 1a

> Uwaga: „dane” w poniższych zadaniach nie są nigdzie pobierane — są ustalane na sztywno jako pewne zmienne w tekście programu. Wyniki są natomiast wyświetlane.

1. Napisz program, który wyświetla informację o przestępności danego roku.
2. Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.
3. Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita

   ```
   F = 32 + 9/5 C
   ```
4. I odwrotnie.
5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).
6. Napisz program, który oblicza silnię dla danej liczby. Zadanie zrób na dwa sposoby — z użyciem pętli while/loop oraz z użyciem pętli for.
7. Napisz program, który wyświetla cyfry danej liczby całkowitej (od końca).
8. Napisz program, który oblicza sumę cyfr danej liczby całkowitej.
9. Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana.

   Zakładamy, że 0 < a < b < c. Zadanie zrób na dwa sposoby — z użyciem pętli while/loop oraz z użyciem pętli for.

---

# Zestaw 2a

1. Wyświetl tabelę widzialnych znaków ASCII wraz z kodami (od 33 do 126).
2. Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji, w której osiągamy jedynkę w problemie Collatza (np. dla n = 12 wynikiem jest 9).
3. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą Armstronga.
4. Napisz funkcję, która odpowiada na pytanie, czy jej argument jest liczbą doskonałą.
5. Napisz funkcję, która wyświetla rozkład podanej liczby na czynniki pierwsze.
6. Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (xⁿ) % p w taki sposób, by działało to poprawnie dla jak największych danych.

   * Wskazówka 1: skorzystaj z własności reszty z dzielenia dla iloczynu (czy też inaczej: iloczynu modulo).
   * Wskazówka 2: w celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.

---

# Zestaw 2b

1. Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.

2. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

   ```
   fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
   ```

   Każde losowanie oczywiście zmienia też ziarno.
   Możesz wybrać któryś z: [https://en.wikipedia.org/wiki/Linear_congruential_generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)

---

# Zestaw 3a

1. Napisz funkcję

   ```
   fn met_newt(f: fn(f64) -> f64, fprim: fn(f64) -> f64, x0: f64, eps: f64, n: u128) -> f64
   ```

   realizującą znajdowanie przybliżonego miejsca zerowego metodą Newtona (przy założeniu, że funkcje w parametrach spełniają odpowiednie założenia: druga jest pochodną pierwszej) — w czterech wersjach:

   * iteracyjnej z pętlą loop (z ewentualnymi break continue return);
   * iteracyjnej z pętlą while (bez żadnych break continue return);
   * rekurencyjnej;
   * iteracyjnej z pętlą for (z ewentualnymi break continue return).

---

# Zestaw 4a

1. Napisz funkcję o nagłówku

   ```
   fn liczba_wystapien(napis: ..., znak: ...) -> ...
   ```

   która liczy i zwróci ile jest danych znaków w danym napisie.

2. Napisz funkcję o nagłówku

   ```
   fn rzymskie(napis: ...) -> ...
   ```

   która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:

   ```
   rzymskie("III") == 3
   rzymskie("IX") == 9
   rzymskie("XIX") == 19
   rzymskie("MCMX") == 1910
   ```

---

# Zestaw 4b

1. Napisz funkcję o nagłówku

   ```
   fn co_drugi_znak(napis: ...) -> ...
   ```

   która zwróci napis zawierający co drugi znak z danego napisu.

2. Zdefiniuj funkcję o nagłówku

   ```
   fn szyfruj(napis: ..., klucz: ...) -> ...
   ```

   która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:

   ```
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

3. Napisz funkcję językową, która otrzymuje w dwóch parametrach napisowych imię i nazwisko, a zwraca napis powstały z pierwszej litery imienia, kropki, spacji i nazwiska, przy czym w wyniku pierwsza litera imienia i nazwiska mają być duże, pozostałe małe.

   Wskazówka: użyj metod `to_lowercase` oraz `to_uppercase`.

4. Napisz funkcję o nagłówku

   ```
   fn na_rzymskie(liczba: ...) -> ...
   ```

   która dla danej liczby całkowitej zwraca jej zapis rzymski. Przykłady:

   ```
   na_rzymskie(3) == "III"
   na_rzymskie(9) == "IX"
   na_rzymskie(19) == "XIX"
   na_rzymskie(1910) == "MCMX"
   ```

5. Napisz funkcję o nagłówku

   ```
   fn dodaj_pisemnie(a: ..., b: ...) -> ...
   ```

   która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis. Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykłady:

   ```
   dodaj_pisemnie("1", "3") == "4"
   dodaj_pisemnie("1", "3") == "4"
   dodaj_pisemnie("8", "3") == "11"
   dodaj_pisemnie("10", "23") == "33"
   dodaj_pisemnie("1", "0") == "1"
   dodaj_pisemnie("11", "00") == "11"
   dodaj_pisemnie("131", "9000") == "10131"
   dodaj_pisemnie("998", "7") == "1005"
   dodaj_pisemnie("24872947", "294729478") == "319602425"
   dodaj_pisemnie("592472987429874982471852", "6782893629472904209740298") == "12707623503770844037158880"
   ```

---

# Zestaw 5a

1. Napisz funkcję o nagłówku

   ```
   fn zamien_syst8_na_syst2(z: &str) -> Option<String>
   ```

   zamieniającą zapis liczby całkowitej bez znaku w systemie ósemkowym na zapis w systemie dwójkowym. Wynik ma być najkrótszy możliwy, niepusty. Wynik `None` ma oznaczać wystąpienie w parametrze z niedozwolonego znaku (spoza cyfr ósemkowych) lub pusty napis w parametrze.

2. Napisz funkcję o nagłówku

   ```
   fn wartosc_syst2(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie dwójkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra dwójkowa lub parametr jest pusty), to wynikiem jest `None`.

3. Napisz funkcję o nagłówku

   ```
   fn wartosc_syst8(z: &str) -> Option<u8>
   ```

   obliczającą wartość całkowitą bez znaku zapisaną w systemie ósemkowym — pod warunkiem, że mieści się na ośmiu bitach. Jeśli nie (lub w zapisie występuje znak inny niż cyfra ósemkowa lub parametr jest pusty), to wynikiem jest `None`.

   **Uwaga 1:** Funkcje te należy zbudować z funkcji zadań poprzednich.
   **Uwaga 2:** Zrób dwie wersje tej funkcji — pierwszą bez znaku zapytania, a drugą ze znakiem zapytania.

---

# Zestaw 5b

Do wyboru: zadania 1+2 lub 3+4. (Oczywiście warto zrobić wszystkie.)

1. Napisz funkcję

   ```
   fn wartosc_cyfry(c: char) -> Result<u8, String>
   ```

   która zwraca wartość cyfry dziesiętnej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą.

2. Napisz funkcję o nagłówku

   ```
   fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
   ```

   która doda dwie liczby naturalne podane w argumentach jako napisy w zapisie dziesiętnym (niekoniecznie poprawne; puste napisy także uznajemy za niepoprawne) — i zwróci wynik również jako napis (lub napisowy opis błędu).

   Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

3. Napisz funkcję

   ```
   fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
   ```

   która zwraca wartość cyfry rzymskiej podanej jako znak — albo opis tekstowy błędu, jeśli znak nie jest cyfrą rzymską (jednym z: I V X L C D M).

4. Napisz funkcję o nagłówku

   ```
   fn rzymskie(napis: &str) -> Result<u128, String>
   ```

   która dla napisu reprezentującego liczbę w zapisie rzymskim (nie zakładamy jego poprawności; ponadto pusty ciąg także jest niepoprawny) zwraca liczbę reprezentowaną przez ów napis lub napisowy opis błędu.

   Błędy mogą być trojakie — niewłaściwa cyfra (o tym powiadomi nas poprzednia funkcja pomocnicza); pusty napis; niewłaściwa kolejność cyfr.

   **Uwaga:** użyj funkcji z poprzedniego zadania i operatora `?`.

---

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

