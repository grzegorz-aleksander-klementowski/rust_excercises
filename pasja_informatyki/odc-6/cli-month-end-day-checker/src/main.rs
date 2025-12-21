use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // asking the user for the month number
    print!("Podaj numer miesiąca: ");
    loop {
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<usize>() {
            Ok(n) => {
                match n {
                    // months that have 31 days
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => println!("Ten miesiąc ma 31 dni. "),
                    // months that have 30 days
                    4 | 6 | 9 | 11 => eprintln!("Ten miesiąc ma 30 dni. "),
                    // february case – depends on the year…
                    2 => {
                        // asking about the year
                        print!("Podaj rok: ");
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let rok = input
                            .trim()
                            .parse::<usize>()
                            .expect("Nie podałeś poprawnego roku!");

                        // checking if the year is leap year
                        if ((rok % 4 == 0) && (rok % 100 != 0)) || (rok % 400 == 0) {
                            println!("Ten miesiąc ma 29 dni. ");
                        } else {
                            println!("Ten miesiąc ma 28 dni. ");
                        }
                    }
                    _ => println!("Niepoprawny numer miesiąca."),
                }

                break;
            }
            Err(_) => {
                eprint!("Wprowadzono niepoprawne dane. Wpisz ponownie: ");
            }
        }
    }

    Ok(())
}
