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

use std::u128;

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    // THE WRONG SIMPLE APPROACH;
    let a = a.to_string().trim().parse::<u128>().unwrap_or_default();
    let b = b.to_string().trim().parse::<u128>().unwrap_or_default();

    (a + b).to_string()
}

fn main() {
    println!("Hello, world!");
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
