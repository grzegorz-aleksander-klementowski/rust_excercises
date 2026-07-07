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
    println!("a_sized: {a_sized}");
    println!("b_sized: {b_sized}");

    // Adding toghether „by hand” in loop, zipped pairs.
    let mut res = String::new();
    for (a, b) in a_sized.chars().rev().zip(b_sized.chars().rev()) {
        // Transfer chars into `u8`.
        let a = a as u8 - b'0';
        let b = b as u8 - b'0';

        println!("{a} + {b} = {}", (a + b + b'0') as char);
        res.push((a + b + b'0') as char);
    }

    res.chars().rev().collect()
}

fn main() {
    println!("WYNIK: {}", dodaj_pisemnie("1133", "2"));
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
