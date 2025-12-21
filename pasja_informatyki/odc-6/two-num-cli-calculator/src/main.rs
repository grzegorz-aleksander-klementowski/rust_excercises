use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Type 1 number: ");
    std::io::stdout().flush()?;
    let mut num_1 = String::new();
    std::io::stdin().read_line(&mut num_1)?;

    print!("Type 2 number: ");
    std::io::stdout().flush()?;
    let mut num_2 = String::new();
    std::io::stdin().read_line(&mut num_2)?;

    Ok(())
}
