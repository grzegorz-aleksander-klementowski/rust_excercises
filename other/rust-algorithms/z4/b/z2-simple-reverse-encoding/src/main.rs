/* 2. Zdefiniuj funkcję o nagłówku

```
fn szyfruj(napis: ..., klucz: ...) -> ...
```

która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:

szyfruj("Aladyn", 2) == "lAdayn"
szyfruj("Aladyn", 3) == "alAdyn"
szyfruj("Aladyn", 4) == "dalAny"
szyfruj("Aladyn", 5) == "ydalAn"
szyfruj("koza", 3) == "zoka"
szyfruj("kaszanka", 3) == "saknazak"
szyfruj("kot Mruczek", 9) == "zcurM tokke"
szyfruj("kot Mruczek", 1) == "kot Mruczek"
szyfruj("kot Mruczek", 2) == "ok trMcuezk"
``` */

/*
* klucz 2
Al ad yn
lA da yn

klucz 3
Ala dyn
alA nyd

klucz 4
Al ad yn
da lA yn

klucz 5
Aladyn
yndal A
 */

fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut res = String::new();
    let mut napis = String::from(napis);

    while !napis.is_empty() {
        let taken_window: String = napis.chars().take(klucz).collect();
        let rev_window: String = taken_window.chars().rev().collect();
        res.push_str(&rev_window);
        napis = napis.chars().skip(klucz).collect();
    }

    res.push_str(&napis);
    res
}

fn main() {
    let próba = "kot Mruczek";
    let oczewiwany = "ok trMcuezk";
    println!("Próba: {próba}\t{oczewiwany}");
    let szyfrowanie = szyfruj(próba, 2);
    println!("{szyfrowanie}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_key_1_returns_same_string() {
        assert_eq!(szyfruj("kot Mruczek", 1), "kot Mruczek");
    }

    #[test]
    fn encrypt_key_2() {
        assert_eq!(szyfruj("Aladyn", 2), "lAdany");
        assert_eq!(szyfruj("kot Mruczek", 2), "ok trMcuezk");
    }

    #[test]
    fn encrypt_key_3() {
        assert_eq!(szyfruj("Aladyn", 3), "alAnyd");
        assert_eq!(szyfruj("koza", 3), "zoka");
        assert_eq!(szyfruj("kaszanka", 3), "saknazak");
    }

    #[test]
    fn encrypt_key_4() {
        assert_eq!(szyfruj("Aladyn", 4), "dalAny");
    }

    #[test]
    fn encrypt_key_5() {
        assert_eq!(szyfruj("Aladyn", 5), "ydalAn");
    }

    #[test]
    fn encrypt_key_9() {
        assert_eq!(szyfruj("kot Mruczek", 9), "zcurM tokke");
    }
}
