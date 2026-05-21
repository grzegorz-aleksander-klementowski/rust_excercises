// 1. Wyświetl tabelę widzialnych znaków ASCII wraz z kodami (od 33 do 126).
// 1. Display the table of visible ASCII characters and codes (from 33 to 126).

use std::io::Write;
use std::ops::RangeInclusive;

pub trait AskForData {
    fn ask_for_data(text: &str) -> Self;
}

impl AskForData for u8 {
    fn ask_for_data(text: &str) -> Self {
        print!("{text}");
        std::io::stdout().flush().expect("Can't release the buf.");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Can't read line.");

        buf.trim().parse().expect("Can't parse.")
    }
}

pub fn show_asci_table(range: RangeInclusive<u8>) {
    println!("+-----+----+");

    let mut is_three_num = false;
    for asci_n in range {
        if let 0..100 = asci_n {
            println!("| {asci_n}  | {}  |", asci_n as char);
        } else {
            println!("| {asci_n} | {}  |", asci_n as char);
            is_three_num = true;
        }
    }

    if is_three_num {
        println!("+----+----+");
    } else {
        println!("+-----+----+");
    }
}
