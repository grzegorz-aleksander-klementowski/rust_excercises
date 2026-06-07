/*
# Zestaw 4a

1. Napisz funkcję o nagłówku

   ```
   fn liczba_wystapien(napis: ..., znak: ...) -> ...
   ```

   która liczy i zwróci ile jest danych znaków w danym napisie.

1. Write a function about the header

   ```
   fn number_of_occurrences(string: ..., sign: ...) -> ...
   ```

   which counts and returns how many characters there are in a given string.
 */

fn number_of_occurrences(string: &str, sign: char) -> usize {
    string.chars().filter(|&c| c == sign).count()
}

fn main() {
    let s = "Hello, world!";
    println!("s");
    let n = number_of_occurrences(s, 'o');
    println!("{n}");
}
