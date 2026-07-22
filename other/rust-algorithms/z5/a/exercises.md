### English translation below

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

## English translation

# Set 5a

1. Write a function about the header

   ```
   fn replace_syst8_with_syst2(z: &str) -> Option<String>
   ```

   converting an unsigned integer in octal to binary. The result should be the shortest possible, non-empty. The result `None` is supposed to mean the occurrence of an illegal character (non-octal) in the parameter or an empty string in the parameter.

2. Write a function about the header

   ```
   fn syst_value2(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in binary - provided it fits in eight bits. If not (or the notation contains a non-binary character or the parameter is empty), the result is `None`.

3. Write a function about the header

   ```
   fn syst_value8(z: &str) -> Option<u8>
   ```

   that computes an unsigned integer value written in octal - provided it fits in eight bits. If not (or the notation contains a non-octal character or the parameter is empty), the result is `None`.

   **Note 1:** These functions should be built from the functions of the previous tasks.
   **Note 2:** Make two versions of this function - the first without the question mark and the second with the question mark.
