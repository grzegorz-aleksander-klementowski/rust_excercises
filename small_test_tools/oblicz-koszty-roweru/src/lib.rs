mod output;
pub use output::*;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq)]
pub struct KosztCzęściRowerowych {
    koło_z_silnikiem: i32,
    bateria: i32,
    ładowarka: i32,
    podstawa_do_baterii: i32,
    adapter: i32, // jako część trzymająca baterię (adapter/bagażnik)
}

impl KosztCzęściRowerowych {
    pub fn new(
        koło_z_silnikiem: i32,
        bateria: i32,
        ładowarka: i32,
        podstawa_do_baterii: i32,
        adapter: i32,
    ) -> Self {
        Self {
            koło_z_silnikiem,
            bateria,
            ładowarka,
            podstawa_do_baterii,
            adapter,
        }
    }

    pub fn suma(&self) -> i32 {
        self.koło_z_silnikiem
            + self.bateria
            + self.ładowarka
            + self.podstawa_do_baterii
            + self.adapter
    }
}
impl Sub for KosztCzęściRowerowych {
    type Output = Self;
    fn sub(self, inny: Self) -> Self {
        Self {
            koło_z_silnikiem: self.koło_z_silnikiem - inny.koło_z_silnikiem,
            bateria: self.bateria - inny.bateria,
            ładowarka: self.ładowarka - inny.ładowarka,
            podstawa_do_baterii: self.podstawa_do_baterii - inny.podstawa_do_baterii,
            adapter: self.adapter - inny.adapter,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obliczania_różnicy() {
        let testowe_koszty = KosztCzęściRowerowych::new(20, 20, 20, 20, 20);
        let testowe_koszty_różnicy = KosztCzęściRowerowych::new(5, 5, 5, 5, 5);

        let prawidłowy_wynik = KosztCzęściRowerowych::new(15, 15, 15, 15, 15);
        let wynik = KosztCzęściRowerowych::sub(testowe_koszty, testowe_koszty_różnicy);

        assert_eq!(wynik, prawidłowy_wynik);
    }

    #[test]
    fn test_sumowanie_kosztów() {
        let testowe_koszty = KosztCzęściRowerowych::new(1, 1, 1, 1, 1);
        let wynik: i32 = KosztCzęściRowerowych::suma(&testowe_koszty);

        assert_eq!(wynik, 5);
    }
}
