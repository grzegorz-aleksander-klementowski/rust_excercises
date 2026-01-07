use std::io::{self, Write};

#[derive(Debug)]
enum Płeć {
    Mężczyzna,
    Kobieta,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Działalnik do sprawdzania płci na podstawie imienia.");
    print!("Podaj imię: ");
    io::stdout().flush()?;
    let mut imię = String::new();
    io::stdin().read_line(&mut imię)?;
    let imię = imię.trim();

    if imię.ends_with('a') {
        println!("{:?}", Płeć::Kobieta);
    } else {
        println!("{:?}", Płeć::Mężczyzna);
    }

    Ok(())
}
