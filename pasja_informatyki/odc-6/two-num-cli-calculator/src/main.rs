use crossterm::{event, terminal};
use std::io::{Read, Write, stdin, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
        // Add number 1 to calculate. In one line.
        print!("Podaj liczbę 1: ");
        stdout().flush()?;
        let mut wejście = String::new();
        stdin().read_line(&mut wejście)?;
        let liczba_1 = wejście.trim().parse::<f32>()?;

        // Add number 2 to calculate. In one line.
        print!("Podaj liczbę 2: ");
        stdout().flush()?;
        let mut wejście = String::new();
        stdin().read_line(&mut wejście)?;
        let liczba_2 = wejście.trim().parse::<f32>()?;

        // Printing the menu
        println!("MENU GŁÓWNE");
        println!("-----------------");
        println!("1. Dodawanie");
        println!("2. Odejmowanie");
        println!("3. Mnożenie");
        println!("4. Dzielenie");
        println!("0. WYJŚCIE");

        terminal::enable_raw_mode()?;
        if let event::Event::Key(event) = event::read()? {
            match event.code {
                event::KeyCode::Char('0') => {
                    println!("Wyjście!");
                    break;
                }
                event::KeyCode::Char('1') => println!("suma: {}", liczba_1 + liczba_2),
                event::KeyCode::Char('2') => println!("różnica: {}", liczba_1 - liczba_2),
                event::KeyCode::Char('3') => println!("iloczyn: {}", liczba_1 * liczba_2),
                event::KeyCode::Char('4') => {
                    if liczba_2 == 0.0 {
                        eprintln!("Nie można dzielić przez zero! ");
                    }
                    println!("iloraz: {:.2}", liczba_1 / liczba_2);
                }
                _ => {}
            }
        }
        terminal::disable_raw_mode()?;

        // Clearn after pressing [ENTER]
        println!("Wciśnij [ENTER] by liczyć dalej…");
        let _ = stdin().read(&mut [0u8])?;
    }

    Ok(())
}
