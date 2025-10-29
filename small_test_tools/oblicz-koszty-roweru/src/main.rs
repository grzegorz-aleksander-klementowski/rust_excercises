pub mod cli;
use clap::Parser;

use cli::Cli;
use oblicz_koszty_roweru::{self, KosztCzęściRowerowych};
use std::ops::Sub;

fn main() {
    // let poprzedni_zakup = KosztCzęściRowerowych::new(1190, 1198, 129, 125, 119);
    // let nowy_zakup = KosztCzęściRowerowych::new(1190, 1390, 199, 0, 219);

    let cli = Cli::parse();

    let poprzedni_zakup = KosztCzęściRowerowych::new(
        cli.koło_z_silnikiem_poprzedni,
        cli.bateria_poprzedni,
        cli.ładowarka_poprzedni,
        cli.podstawa_do_baterii_poprzedni,
        cli.adapter_poprzedni,
    );

    let nowy_zakup = KosztCzęściRowerowych::new(
        cli.koło_z_silnikiem_nowy,
        cli.bateria_nowa,
        cli.ładowarka_nowa,
        cli.podstawa_do_baterii_nowa,
        cli.adapter_nowa,
    );

    println!("Zakupiono pierwszy raz za: {}", &poprzedni_zakup);
    println!("Nowe części kosztują: {}", &nowy_zakup);

    let różnica = KosztCzęściRowerowych::sub(nowy_zakup, poprzedni_zakup);
    let wartość_dopłaty = KosztCzęściRowerowych::suma(&różnica);

    println!("Różnica w częściach: {różnica}");
    println!("Więc nalezy dopłacić: {wartość_dopłaty}");
}
