use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // gathering data:
    // ask about name
    print!("Podaj imię: ");
    io::stdout().flush()?;
    let mut imię = String::new();
    io::stdin().read_line(&mut imię);

    // ask about lastname
    print!("Podaj nazwisko: ");
    io::stdout().flush()?;
    let mut nazwisko = String::new();
    io::stdin().read_line(&mut nazwisko);

    // ask about phone number
    print!("Podaj liczby dalnomówika: ");
    io::stdout().flush()?;
    let mut dalnomówink = String::new();
    io::stdin().read_line(&mut dalnomówink);

    let plik = File::create("wizytówka.txt")?;
    plik.write_all(b[imię]);

    Ok(())
}
