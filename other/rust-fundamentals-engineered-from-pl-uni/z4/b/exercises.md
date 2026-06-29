-- English translation below --

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
    ENGLISH TRANSLATION
---


# Set 4b

1. Write a function about the header

   ```
   fn every_second_character(string: ...) -> ...
   ```

   which will return a string containing every second character from the given string.

2. Define a function with a header

   ```
   fn encrypt(string: ..., key: ...) -> ...
   ```

   which for a given string will return the same string encrypted with a simple reversal cipher - the key determines the length of the reversed fragments. Examples:

   ```
   encrypt(Aladdin, 2) == lAdayn
   encrypt(Aladdin, 3) == alAnyd
   encrypt(Aladdin, 4) == dalAyn
   encrypt(Aladdin, 5) == yndalA
   encrypt(goat, 3) == zoka
   encrypt(blood sausage, 3) == sakanazk
   encrypt(cat Mruczek", 2) == "ok tmCruzek"

3. Write a language function that receives the name and surname as two string parameters and returns a string made from the first letter of the name, a dot, a space and the surname, with the first letter of the name and surname being uppercase and the remaining letters lowercase.

   Tip: use the `to_lowercase` and `to_uppercase` methods.

4. Write a function about the header

   ```
   fn to_roman(number: ...) -> ...
   ```

   which, for a given integer, returns its Roman notation. Examples:

   ```
   na_roman(3) == III
   na_roman(9) == IX
   na_rzymskie(19) == XIX
   na_roman(1910) == MCMX
   ```

5. Write a function about the header

   ```
   fn add_written(a: ..., b: ...) -> ...
   ```

   which will add the two (assumed correct) integers given in the arguments as decimal strings - and return the result as a string as well. Note: addition must be done in writing, as the numbers can be arbitrarily large. Examples:

   ```
   add_pisemnie(1, 3) == 4
   add_pisemnie(1, 3) == 4
   add_pisemnie(8, 3) == 11
   add_pisemnie(10, 23) == 33
   add_pisemnie(1, 0) == 1
   add_written(11, 00) == 11
   add_written(131, 9000) == 10131
   add_written(998, 7) == 1005
   add_pisemnie(24872947, 294729478) == 319602425
   add_pisemnie(592472987429874982471852, 6782893629472904209740298) == 12707623503770844037158880
   ```
