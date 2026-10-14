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

    // The result:
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
            *alphabet_letter_slot = char::from(letter_byte);
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

        // Increment up to ten
        i += 1;
    }

    // return result
    sqr_nums
}

// by closure
#[allow(dead_code)]
fn sqr_from_1_to_10_as_vec_by_closure() -> Vec<u32> {
    let range_num: RangeInclusive<u32> = 1..=10;
    let sqr_nums: Vec<u32> = range_num.map(|num| num.pow(2)).collect();

    // return result
    sqr_nums
}

// === SUBEXERCISE 3: ten consecutive powers of two === \\
// The exercise using loop
#[allow(dead_code)]
fn ten_consecutive_powers_of_two_by_loop() -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    for i in 1..=10 {
        res.push(2_u32.pow(i));
    }

    res
}

// The exercise using closure
#[allow(dead_code)]
fn ten_consecutive_powers_of_two_by_closure() -> Vec<u32> {
    (1..=10).map(|num| 2_u32.pow(num)).collect()
}

// === SUBEXERCISE 4: reciprocals of every number from 1 to 20; === \\
// The exercise using loop
#[allow(dead_code)]
fn reciprocals_of_every_num_by_loop(range: &RangeInclusive<u16>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    for i in range.clone() {
        res.push(1.0 / f32::from(i));
    }

    res
}

// The exercise using closure
#[allow(dead_code)]
fn reciprocals_of_every_num_by_closure(range: &RangeInclusive<u16>) -> Vec<f32> {
    range.clone().map(|num| 1.0 / f32::from(num)).collect()
}

// === SUBEXERCISE 5: numbers from 1 to 100 divisible by 3 but not by 4 === \\
// The exercise using loop
#[allow(dead_code)]
fn divisible_nums_by_3_but_not_by_4_by_loop(range: &RangeInclusive<u32>) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    // loop in range argument
    for num in range.clone() {
        // if the num is divisible by 3 but not by 4
        if num % 3 == 0 && num % 4 != 0 {
            // if yes – add to the result vector
            res.push(num);
        }
    }

    // Return the result
    res
}

// The exercise using closure
#[allow(dead_code)]
fn divisible_nums_by_3_but_not_by_4_by_closure(range: &RangeInclusive<u32>) -> Vec<u32> {
    range
        .clone()
        .filter(|num| num % 3 == 0 && num % 4 != 0)
        .collect()
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

    #[test]
    fn test_ten_consecutive_powers_of_two() {
        let expected = vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

        // Test the funtion using `loop`
        assert_eq!(expected, ten_consecutive_powers_of_two_by_loop());

        // Test the funtion using `closure`
        assert_eq!(expected, ten_consecutive_powers_of_two_by_closure());
    }

    #[test]
    fn test_reciprocals_of_every_num() {
        let expected: Vec<f32> = vec![
            1.0,
            0.5,
            1.0 / 3.0,
            0.25,
            0.2,
            1.0 / 6.0,
            1.0 / 7.0,
            0.125,
            1.0 / 9.0,
            0.1,
            1.0 / 11.0,
            1.0 / 12.0,
            1.0 / 13.0,
            1.0 / 14.0,
            1.0 / 15.0,
            0.0625,
            1.0 / 17.0,
            1.0 / 18.0,
            1.0 / 19.0,
            0.05,
        ];

        let range = 1..=20;

        // Test the funtion using `loop`
        assert_eq!(expected, reciprocals_of_every_num_by_loop(&range));

        // Test the funtion using `loop`
        assert_eq!(expected, reciprocals_of_every_num_by_closure(&range));
    }

    #[test]
    fn test_divisible_nums_by_3_but_not_by_4() {
        let expected = vec![
            3, 6, 9, 15, 18, 21, 27, 30, 33, 39, 42, 45, 51, 54, 57, 63, 66, 69, 75, 78, 81, 87,
            90, 93, 99,
        ];
        let range = 1..=100;

        assert_eq!(expected, divisible_nums_by_3_but_not_by_4_by_loop(&range));

        assert_eq!(
            expected,
            divisible_nums_by_3_but_not_by_4_by_closure(&range)
        );
    }
}
