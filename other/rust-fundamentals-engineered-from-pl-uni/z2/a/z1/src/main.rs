// 1. Wyświetl tabelę widzialnych znaków ASCII wraz z kodami (od 33 do 126).
// 1. Display the table of visible ASCII characters and codes (from 33 to 126).

use std::u8;

use z1::AskForData;
use z1::show_asci_table;

fn main() {
    let range_from = u8::ask_for_data("Typ ASCII num from where to start: ");
    let range_to = u8::ask_for_data("Typ ASCII num where to end: ");
    show_asci_table(range_from..=range_to);
}
