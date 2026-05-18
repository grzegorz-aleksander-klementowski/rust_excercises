// 1. Wyświetl tabelę widzialnych znaków ASCII wraz z kodami (od 33 do 126).

fn main() {
    for c in 'a'..='z' {
        let c = c.to_ascii_uppercase();
        println!("{c}\t");
    }
}
