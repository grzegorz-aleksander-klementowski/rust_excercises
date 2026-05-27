use std::io::{self, Write};

// metry na cale
fn metres_to_inch(m: f32) -> f32 {
    m * 39.37
}

// metry na jardy
fn metres_to_yards(m: f32) -> f32 {
    m * 1.0936
}

// metry na mile
fn metres_to_miles(m: f32) -> f32 {
    m * 0.000_621_371
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Provide metres to convert
    println!("Enter metres: ");
    io::stdout().flush()?;
    let mut metres = String::new();
    io::stdin().read_line(&mut metres)?;
    let metres = metres.trim().parse::<f32>()?;

    // Ouput
    println!("Inches: {:.2}", metres_to_inch(metres));
    println!("Yards: {:.2}", metres_to_yards(metres));
    println!("Miles: {:.2}", metres_to_miles(metres));

    Ok(())
}
