use oblicz_koszty_roweru::{self, KosztCzęściRowerowych};
use std::{fmt, ops::Sub, process::Output};

fn main() {
    let poprzedni_zakup = KosztCzęściRowerowych::new(1190, 1198, 129, 125, 119);
    let nowy_zakup = KosztCzęściRowerowych::new(1190, 1390, 199, 0, 219);
    let różnica = KosztCzęściRowerowych::sub(nowy_zakup, poprzedni_zakup);
    let wartość_dopłaty = KosztCzęściRowerowych::suma(&różnica);

    println!("Różnica w częściach: {}", różnica);
    println!("Więc nalezy dopłacić: {}", wartość_dopłaty);
}
