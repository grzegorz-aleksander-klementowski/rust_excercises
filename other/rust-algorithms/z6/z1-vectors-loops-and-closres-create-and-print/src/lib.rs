/* [PL]
Zadania wykonaj w dwóch wersjach — przy użyciu pętli oraz bez ich użycia (z iteratorami zamiast tego).

1. Utwórz (i wyświetl) wektor zawierający:

   * małe litery alfabetu angielskiego;
   * kwadraty 10 kolejnych liczb całkowitych począwszy od 1;
   * 10 kolejnych potęg dwójki;
   * odwrotności wszystkich liczb od 1 do 20;
   * liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4.

[EN]
Complete every task twice: once with loops and once without loops, using iterators instead.

1. Create and display vectors containing:

   * lowercase English alphabet letters;
   * squares of ten consecutive integers starting from 1;
   * ten consecutive powers of two;
   * reciprocals of every number from 1 to 20;
   * numbers from 1 to 100 divisible by 3 but not by 4.
 */

// it creates and display the alphabet using closure
#[warn(dead_code)]
fn alphabet_by_loop() -> [char; 26] {
    // The result of the function
    let mut alphabet = ['_'; 26];
    // The loop securly input the alphabet letters into the result array.
    for (alphabet_letter, byte_letter) in alphabet.iter_mut().zip(b'a'..=b'z') {
        *alphabet_letter = char::from(byte_letter);
    }

    // print because the exercise want to display as well
    println!("{alphabet:?}");
    alphabet
}

// it creates and display the alphabet using closure
#[warn(dead_code)]
fn alphabet_by_closure() -> [char; 26] {
    let mut alphabet = ['_'; 26];

    alphabet
        .iter_mut()
        .zip(b'a'..=b'z')
        .for_each(|(alphabet_letter_slot, letter_byte)| {
            *alphabet_letter_slot = char::from(letter_byte)
        });

    alphabet
}

#[cfg(test)]
mod tests {
    use crate::{alphabet_by_closure, alphabet_by_loop};

    #[test]
    fn test_alphabet() {
        // Test the function using `for` loop
        assert_eq!(
            [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            alphabet_by_loop()
        );

        // Test the function using the closure
        assert_eq!(
            [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            alphabet_by_closure()
        );
    }
}
