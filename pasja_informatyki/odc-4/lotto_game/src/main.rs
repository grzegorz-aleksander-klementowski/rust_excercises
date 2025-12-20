use lotto_game::{generate_lotto_set, generate_lotto_set_output, stop_for_seconds, Message};
use std::io::{self, Write};

//Strict documentation is located in `lib.rs`
fn main() {
    Message::Welcome.print_message();
    io::stdout().flush().unwrap();
    stop_for_seconds(3);
    let result_of_lotto_set_generate: Result<[Option<u8>; 6], &'static str> = generate_lotto_set();
    let lotto_set: [u8; 6] = generate_lotto_set_output(result_of_lotto_set_generate);
    Message::new_lotto_set(lotto_set).print_message();
}
