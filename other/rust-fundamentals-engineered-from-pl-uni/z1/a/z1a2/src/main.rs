// 2. Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.

use std::io::Write;

enum LiczbaDniMiesiąca {
    Trzydzieści,        //30
    TrzydzieściJeden,   // 31
    DwadzieściaOsiem,   //28
    DwadziściaDziewięć, // 29
}

impl std::fmt::Display for LiczbaDniMiesiąca {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trzydzieści => write!(f, "30"),
            Self::TrzydzieściJeden => write!(f, "31"),
            Self::DwadzieściaOsiem => write!(f, "28"),
            Self::DwadziściaDziewięć => write!(f, "29"),
        }
    }
}

// Priting that allow write after the print
fn print_in_line(s: &str) -> Result<(), std::io::Error> {
    print!("{s}");
    std::io::stdout().flush()?;
    Ok(())
}

fn stdin_num() -> Result<u32, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse::<u32>()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Writing out question to a user:
    print_in_line("Wpisz numer miesiąca: ")?;

    // Get the number of mouther from the user
    let numer_miesiąca = stdin_num()?;

    // Writing out the question to a user
    print_in_line("Wpisz rok: ")?;

    // Get the year
    let rok = stdin_num()?;
    let mut rok_przystępny = false;

    if (rok % 4 == 0 && rok % 100 != 0) || (rok % 400 == 0) {
        rok_przystępny = true;
    }

    let ile_dni_miesiąca = match numer_miesiąca {
        m if ((1..=7).contains(&m) && m % 2 != 0) || ((8..=12).contains(&m) && m % 2 == 0) => {
            LiczbaDniMiesiąca::TrzydzieściJeden
        }
        m if m == 2 && !rok_przystępny => LiczbaDniMiesiąca::DwadzieściaOsiem,
        m if m == 2 && rok_przystępny => LiczbaDniMiesiąca::DwadziściaDziewięć,
        13.. => panic!("Nieprawidłowy numer miesiąca"), // VALIDATOIN → invalid mouth number→
        _ => LiczbaDniMiesiąca::Trzydzieści,
    };

    println!("Ten miesiąc ma {ile_dni_miesiąca} dni.");

    Ok(())
}
