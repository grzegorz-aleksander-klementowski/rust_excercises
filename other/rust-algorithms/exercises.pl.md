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

---

# Zestaw 8a

1. Zdefiniuj typ strukturalny do przechowywania koloru w modelu RGB z dokładnością do 8 bitów na składową. Udostępnij operacje potrzebne, aby działał program testujący korzystający z konstruktorów `from_3u8`, `from_percent`, `gray`, `white` i `black`, porównywania kolorów, metody `invert` oraz metod `intensity`, `as_rgb_u8tuple` i `as_cmy_u8tuple`.

   Program testujący powinien poprawnie obsługiwać kolory szary `(127, 127, 127)`, szary utworzony z wartości procentowej `50.0`, fiolet `(100, 35, 120)`, biały `(255, 255, 255)` i czarny `(0, 0, 0)`.

---

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

---

# Zestaw 8c

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

---

# Zestaw 8d

1. Zaprojektuj typ `Towar` (wraz z ewentualnymi typami pomocniczymi), który służy do przechowywania następujących cech towaru w pewnej hurtowni:

   * opis/nazwa;
   * jednostka: sztuki, litry lub kilogramy;
   * waga jednostkowa w kilogramach;
   * wymagane warunki przechowywania: zamrażarka, chłodziarka albo warunki normalne.

   Konstruktor (lub konstruktory) nie powinien dopuszczać niedodatnich wag, a dla jednostki „kilogramy” powinien wymuszać wagę jednostkową równą `1.0`.

2. Zaprojektuj typ `Zamowienie` (wraz z ewentualnymi typami pomocniczymi), który przechowuje listę towarów wraz z ilością każdego z nich. Potrzebne są:

   * konstruktor tworzący puste zamówienie;
   * metoda zwracająca sumaryczną wagę zamówienia;
   * metoda zwracająca sumaryczną wagę elementów zamówienia wymagających danego sposobu przechowywania;
   * metoda dodająca towar w podanej ilości, przy czym ilość musi być dodatnia, dla jednostki „sztuki” musi być całkowita, a istniejący już identyczny towar zwiększa swoją ilość zamiast tworzyć nowy element.

---

# Zestaw 9a

1. Stwórz generyczny typ `Urna` (na bazie konkretnego typu z zestawu 8c), który może przechowywać dane dowolnego — w miarę możliwości — typu i losować z nich za pomocą generatora `RandGen` z zestawu 8c. Zachowaj zachowanie metod `new`, `doloz`, `rozmiar`, `losuj_z_us` oraz `losuj_bez_us` z poprzedniego zestawu.

2. Dopisz analogiczne testy dla typów konkretnych:

   * `Urna<i32>`;
   * `Urna<bool>`;
   * `Urna<String>`;
   * `Urna<Moneta>`, gdzie `Moneta` jest własnym typem wyliczeniowym o dwóch wartościach: `Orzel` i `Reszka`.

---

# Zestaw 9b

*Zadania autorstwa mgra Piotra Koseli.*

1. Stwórz strukturę `Date` oraz zaimplementuj dla niej metody:

   ```rust
   fn to_string(&self) -> String
   fn from_3(day: u8, month: Month, year: u16) -> Date
   fn from_string(string: &str, delim: char) -> Date
   ```

2. Stwórz strukturę `Time` oraz zaimplementuj dla niej metody analogiczne jak dla struktury `Date`.

3. Zmodyfikuj strukturę `Date`, aby mogła opcjonalnie przechowywać również czas. Dodaj metody:

   ```rust
   fn has_time(&self) -> bool
   fn set_time(&mut self, time: Time)
   fn clear_time(&mut self)
   ```

4. Zaimplementuj cechy `PartialOrd`, `Ord`, `PartialEq` i `Eq` dla zmodyfikowanej struktury `Date`, zgodnie ze zdroworozsądkowym poczuciem czasu.

5. Stwórz strukturę `Task` z polami `name: String`, `description: String`, `priority: Priority` (warianty `Low`, `Medium`, `High`) oraz `due: Date`.

6. Zaimplementuj sensownie cechy `PartialOrd` i `PartialEq` dla struktury `Task`.

---

# Zestaw 9c

1. Tradycyjne karty do gry (brydż, poker itd.) są podzielone na kolory: pik, kier, karo i trefl. Zaprojektuj typ wyliczeniowy reprezentujący te kolory, z dodatkowym warunkiem uporządkowania jak w brydżu: pik, kier, karo, trefl. Wyprowadź odpowiednie cechy i ułóż warianty we właściwej kolejności.

2. Zaprojektuj typ do przechowywania informacji o możliwych błędach:

   * brak błędu;
   * zły format pliku;
   * plik nie istnieje, wraz z nazwą pliku;
   * plik zbyt duży, wraz z aktualnym i maksymalnym rozmiarem.

   Dodatkowo zaimplementuj metodę `pokaz_komunikat`, która wyświetli pełny komunikat o podanym błędzie wraz z jego danymi.

---

# Zestaw 10a

1. Stwórz `enum Gatunek` z kilkoma gatunkami książek. Następnie stwórz strukturę `Ksiazka`, która przechowuje tytuł, autora, gatunek i liczbę stron. Przechowuj książki w wektorze i zaimplementuj metodę filtrującą książki z danego gatunku, liczące więcej niż 300 stron.

2. Zaimplementuj prosty system zgłoszeń. Każde zgłoszenie ma unikalny identyfikator, tytuł oraz status: `Otwarte`, `Przetwarzane`, `Zamkniete(Rezultat)`, gdzie rezultat może być dodatni albo negatywny. Zgłoszenia przechowuj w wektorze. Dodaj metody zmiany statusu i listowania tylko zgłoszeń o podanym statusie.

3. Zdefiniuj `enum PaymentMethod { Cash, Card, Transfer }` oraz strukturę `Transaction` z polami `amount: f64` i `method: PaymentMethod`. Zaimplementuj funkcję sumującą wszystkie transakcje danego typu z tablicy lub wektora.

4. Zaimplementuj prosty system zarządzania rezerwacjami lotniczymi. Każda rezerwacja ma pasażera, numer lotu, klasę podróży (ekonomiczna, biznesowa, pierwsza) i status (zarezerwowane, odwołane). `SystemRezerwacji` przechowuje `Vec<Rezerwacja>` i udostępnia metody:

   * dodawania rezerwacji;
   * anulowania rezerwacji na podstawie nazwiska pasażera;
   * zliczania rezerwacji w podanej klasie i statusie;
   * listowania alfabetycznie posortowanych pasażerów w podanej klasie.

5. Zaprojektuj strukturę `DanePogodowe`, zawierającą lokalizację (`String`), dzień kolejny obserwacji (`u32`), temperaturę (`f32`) i warunki pogodowe (słońce, chmury, deszcz, śnieg). Zaimplementuj `DziennikPogodowy`, który przechowuje dane z wielu dni i oferuje metody zwracające średnią temperaturę dla danego typu pogody, najczęstszy typ pogody oraz słoneczne dni z temperaturą powyżej `30°C`.

---

# Zestaw 10b

1. Zaimplementuj tradycyjną grę w oczko — najpierw dla jednej osoby, a następnie dla wielu osób. W implementacji:

   * zaprojektuj odpowiednie typy do reprezentacji kolorów kart, wartości kart, kart, talii, ręki, wyniku i innych potrzebnych rzeczy;
   * zadbaj o ich odpowiednią reprezentację na ekranie;
   * zadbaj o odpowiednią obsługę błędów;
   * skorzystaj z własnej implementacji liczb pseudolosowych — na razie przygotuj jednak aplikację do późniejszej zamiany generatora na inny;
   * zadbaj o czytelność i wygodę aplikacji.

---

# Zestaw 13a

1. Utwórz projekt biblioteczny zawierający moduł `ulamek`, który implementuje typ `Ulamek` reprezentujący ułamek zwykły. Zaimplementuj co najmniej:

   * konstruktor `new(licznik, mianownik)`;
   * akcesory `as_f64()`, `licznik()` i `mianownik()`;
   * operatory `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=`, `==` i `!=` jako cechy;
   * konstruktor konwertujący `from_str(ulamek_jako_napis)`.

2. Moduł ma także zawierać wyczerpujący podmoduł testowy. Testy powinny obejmować co najmniej: konwersję `3/4` na `0.75`, dodawanie `1/3 + 1/2 == 5/6`, błąd dla zerowego mianownika, równoważność `1/-3` i `-2/6`, poprawne parsowanie `"1/-3"`, `"-2/6"`, `"13"`, `"-26/-2"` oraz błędne napisy, np. `"x/-3"` i `"1/3/5"`.

---

# Zestaw 14a

1. Uzupełnij zadanie z zestawu 10b o:

   * automatyczne testy jednostkowe;
   * użycie zewnętrznego modułu `rand` z [crates.io](https://crates.io/crates/rand).

---

# Zestaw 15a

Zaimplementuj dwie wersje modułu `stos`, zawierającego szablon typu `Stos`, implementujący stos dla typów o odpowiednio wybranych cechach, w postaci listy jednokierunkowej. Potrzebne są metody `new`, `is_empty`, `top`, `pop` i `push`.

Zadbaj o odpowiednie typy wyniku, uwzględniające możliwe niepowodzenia, oraz napisz obszerne testy jako podmoduł, które muszą przechodzić w obu wersjach.

1. W pierwszej wersji użyj wskaźników `Box`.
2. W drugiej wersji, dla chętnych, użyj surowych wskaźników i trybu niebezpiecznego.

---

# Kolokwium — zadania dodatkowe

1. Napisz funkcję o nagłówku uzupełnionym samodzielnie:

   ```rust
   fn najwiecej_wystapien(ss: &Vec<String>, c: char) -> ...
   ```

   Ma ona zwracać liczbę wystąpień znaku `c` w tym napisie z wektora, który zawiera ich więcej niż każdy pozostały napis. Jeżeli takiego napisu nie ma w wektorze, funkcja powinna zwrócić `None`.

2. Napisz funkcję z samodzielnie zaprojektowanym nagłówkiem, która dla podanego wektora liczb całkowitych zwraca pierwszą zduplikowaną liczbę. Na przykład dla danych `[2, 1, 3, 5, 3, 2]` wynikiem jest `3`. Jeżeli w wektorze nie występują żadne powtórzenia, funkcja ma w czytelny, rustowy sposób poinformować o braku wyniku.

3. Napisz funkcję:

   ```rust
   fn iloczyn_kolejnych(a: u128, b: u128) -> u128
   ```

   która oblicza iloczyn wszystkich liczb całkowitych z domkniętego przedziału `[a; b]` (zakładamy, że wynik mieści się w `u128`). Jeżeli `a > b`, uznaj przedział za pusty i samodzielnie zdecyduj, jaki wynik należy wtedy zwrócić. Przykłady:

   ```rust
   iloczyn_kolejnych(3, 3) == 3
   iloczyn_kolejnych(0, 400) == 0
   iloczyn_kolejnych(3, 5) == 60
   ```

   Uwaga: użycie pętli, rekurencji lub instrukcji `return` znacznie obniża ocenę tego zadania.

4. Napisz funkcję:

   ```rust
   fn pora_dnia(godzina: u8, minuta: u8) -> Result<String, String>
   ```

   Przyjmuje ona porę dnia w postaci godziny i minuty w notacji 24-godzinnej, a zwraca jej opis tekstowy: `"noc"`, `"rano"`, `"popoludnie"` lub `"wieczor"`; dla nieprawidłowych danych zwraca informację o błędzie. Przyjmij, że:

   * rano zaczyna się o `6:30` włącznie i kończy o `12:00` włącznie;
   * wieczór zaczyna się o `18:15` włącznie i kończy o `23:10` włącznie;
   * popołudnie jest między ranem a wieczorem;
   * reszta to noc.

---
