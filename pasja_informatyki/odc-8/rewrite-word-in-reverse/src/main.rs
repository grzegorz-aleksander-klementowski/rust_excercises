use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Revers a word
    print!("Podaj słowo do odwrócenia: ");
    io::stdout().flush()?;
    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    word = word.trim().to_string();
    let mut reversed_word = String::new();
    for c in word.chars().rev() {
        reversed_word.push(c);
    }
    println!("odwócone: {reversed_word}");

    Ok(())
}
