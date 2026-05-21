// 1. Napisz program, który wyświetla informację o przestępności danego roku.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter a year to check if it is an affordable year:");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    let year = buf.trim().parse::<u32>()?;

    if (year % 100 != 0 && year % 4 == 0) || year % 400 == 0 {
        println!("This year is affordable.");
    } else {
        println!("This year is not affordable.");
    }

    Ok(())
}
