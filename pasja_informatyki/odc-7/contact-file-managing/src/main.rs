use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // gathering data:
    // ask about name
    print!("Podaj imię: ");
    io::stdout().flush()?;
    let mut imię = String::new();
    io::stdin().read_line(&mut imię)?;

    // ask about lastname
    print!("Podaj nazwisko: ");
    io::stdout().flush()?;
    let mut nazwisko = String::new();
    io::stdin().read_line(&mut nazwisko)?;

    // ask about phone number
    print!("Podaj liczby dalnomówika: ");
    io::stdout().flush()?;
    let mut dalnomówink = String::new();
    io::stdin().read_line(&mut dalnomówink)?;

    let zawartość_pliku = format!("{imię}{nazwisko}{dalnomówink}");

    let mut plik = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("wizytówka.txt")?;

    plik.write_all(zawartość_pliku.as_bytes())?;

    Ok(())
}
