// 2. Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.

use std::io::Write;

// Priting that allow write after the print
fn print_in_line(s: &str) -> Result<(), std::io::Error> {
    print!("{s}");
    std::io::stdout().flush()?;
    Ok(())
}

fn stdin_num() -> Result<usize, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse::<usize>()?)
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

    Ok(())
}
