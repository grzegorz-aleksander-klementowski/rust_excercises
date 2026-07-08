/* 5. Napisz funkcję o nagłówku

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
*/

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    println!("LICZBY: a: {a}, b: {b}");

    // Prepared Strings to push there potensial zeros
    let mut a_sized = String::new();
    let mut b_sized = String::new();
    // Calculating the lenth of the strings
    let a_len = a.len();
    let b_len = b.len();
    // Checking which one is longer and try to make them the same size
    // …if a is bigger then b
    if a_len > b_len {
        let mut a_b_diff = a_len - b_len;
        while a_b_diff != 0 {
            b_sized.push('0');
            a_b_diff -= 1;
        }
        // Push zeros (fillers) into front of b string
        b_sized.push_str(b);
    } else {
        b_sized = b.to_string();
    }
    // …if b is bigger then a
    if a_len < b_len {
        let mut b_a_diff = b_len - a_len;
        while b_a_diff != 0 {
            a_sized.push('0');
            b_a_diff -= 1;
        }
        // Push zeros (fillers) into front of a string.
        a_sized.push_str(a);
    } else {
        a_sized = a.to_string();
    }

    // Adding toghether „by hand” in loop, zipped pairs.
    let mut res = String::new();
    let mut carry: u8 = 0;
    for (a, b) in a_sized.chars().rev().zip(b_sized.chars().rev()) {
        // Transfer chars into `u8`.
        let a = a as u8 - b'0';
        let b = b as u8 - b'0';

        // Calculation „by hand” (with adding the carry). I had to understand here when ti add b`0` binarry to not mess the final result
        let mut addition = a + b + carry;
        // Reset carry to zero
        carry = 0;
        // Catch the case if the „by hand addition” is larger than 9
        if addition > 9 {
            //…if it's larger, then save tens into `carry`, and cut it (by div).
            carry = addition / 10;
            addition %= 10;
        }

        /* // Prevent to accumulate zeros in result. It make sure the result won't be full of only 0.
        if res == "0" && addition == 0 {
            continue;
        } */
        // Transfer the addition into char and puh it into the final result
        res.push((addition + b'0') as char);
    }
    if carry != 0 {
        res.push((carry + b'0') as char);
    }

    // Thus the push–ing, reverst the result to get the right number
    res = res.chars().rev().collect();
    // Check if the front of the number are zeros
    let mut res_char = res.chars();
    if res_char.next() == Some('0') {
        let mut none_zero_index: usize = 0;
        for (i, c) in res_char.enumerate() {
            // If there is non zero char – we found where the number starts OR if we went to the end – it means the full number is zero.
            if c != '0' || res.len() - 2 == i {
                none_zero_index = i + 1;
                break;
            }
        }
        // Split the zeros off the number
        res = res.split_off(none_zero_index);
    }

    res
}

fn main() {
    println!("WYNIK: {}", dodaj_pisemnie("000", "000"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dodaje_przyklady_z_polecenia() {
        assert_eq!(dodaj_pisemnie("1", "3"), "4");
        assert_eq!(dodaj_pisemnie("8", "3"), "11");
        assert_eq!(dodaj_pisemnie("10", "23"), "33");
        assert_eq!(dodaj_pisemnie("1", "0"), "1");
        assert_eq!(dodaj_pisemnie("11", "00"), "11");
        assert_eq!(dodaj_pisemnie("131", "9000"), "9131");
        assert_eq!(dodaj_pisemnie("998", "7"), "1005");
        assert_eq!(dodaj_pisemnie("24872947", "294729478"), "319602425");
        assert_eq!(
            dodaj_pisemnie("592472987429874982471852", "6782893629472904209740298",),
            "7375366616902779192212150"
        );
    }

    #[test]
    fn dodaje_zera() {
        assert_eq!(dodaj_pisemnie("0", "0"), "0");
        assert_eq!(dodaj_pisemnie("1", "0"), "1");
        assert_eq!(dodaj_pisemnie("11", "00"), "11");
        assert_eq!(dodaj_pisemnie("000", "000"), "0");
        assert_eq!(dodaj_pisemnie("0001", "0009"), "10");
    }

    #[test]
    fn dodaje_z_przeniesieniem() {
        assert_eq!(dodaj_pisemnie("8", "3"), "11");
        assert_eq!(dodaj_pisemnie("998", "7"), "1005");
        assert_eq!(dodaj_pisemnie("999", "1"), "1000");
        assert_eq!(dodaj_pisemnie("999999", "1"), "1000000");
        assert_eq!(dodaj_pisemnie("1099", "1"), "1100");
        assert_eq!(dodaj_pisemnie("567", "789"), "1356");
    }

    #[test]
    fn dodaje_bez_przeniesienia() {
        assert_eq!(dodaj_pisemnie("1", "3"), "4");
        assert_eq!(dodaj_pisemnie("10", "23"), "33");
        assert_eq!(dodaj_pisemnie("123", "456"), "579");
        assert_eq!(dodaj_pisemnie("131", "9000"), "9131");
    }

    #[test]
    fn dodaje_duze_liczby() {
        assert_eq!(dodaj_pisemnie("24872947", "294729478"), "319602425");

        assert_eq!(
            dodaj_pisemnie("592472987429874982471852", "6782893629472904209740298",),
            "7375366616902779192212150"
        );

        assert_eq!(
            dodaj_pisemnie("12345678901234567890", "0"),
            "12345678901234567890"
        );
    }
}
