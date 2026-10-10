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

use std::ops::RangeInclusive;

// === SUBEXERCISE 1: lowercase English alphabet letters === \\
// it creates and display the alphabet using closure created as ARRAY
#[allow(dead_code)]
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

// it creates and display the alphabet using closure created as ARRAY
#[allow(dead_code)]
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

// === SUBEXERCISE 2: squares of ten consecutive integers starting from 1 === \\
// by loop
#[allow(dead_code)]
fn sqr_from_1_to_10_as_vec_by_loop() -> Vec<u32> {
    let mut sqr_nums: Vec<u32> = Vec::new();
    let mut i: u32 = 1;
    while i <= 10 {
        // The square of the loop index
        let sqr_idx = i.pow(2);
        // Push it into the vector result
        sqr_nums.push(sqr_idx);
        println!("info: now sqr_nums vector is {sqr_nums:?}");

        // Increment up to ten
        i += 1;
    }
    // Print it as it was said in the exercise content
    println!("{sqr_nums:?}");
    // return result
    sqr_nums
}

// by closure
#[allow(dead_code)]
fn sqr_from_1_to_10_as_vec_by_closure() -> Vec<u32> {
    let range_num: RangeInclusive<u32> = 1..=10;
    let sqr_nums: Vec<u32> = range_num.map(|num| num.pow(2)).collect();

    println!("{sqr_nums:?}");
    // return result
    sqr_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet() {
        let expected: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        // Test the function using `for` loop
        assert_eq!(expected, alphabet_by_loop());

        // Test the function using the closure
        assert_eq!(expected, alphabet_by_closure());
    }

    #[test]
    fn test_squr_to_ten() {
        let expected = vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100];

        // Test the function using `for` loop
        assert_eq!(expected, sqr_from_1_to_10_as_vec_by_loop());

        // Test the function using the closure
        assert_eq!(expected, sqr_from_1_to_10_as_vec_by_closure());
    }
}
