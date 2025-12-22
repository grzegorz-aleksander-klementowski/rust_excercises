fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut imię = String::new();
    let mut nazwisko = String::new();
    let mut l_dalnomównika: usize = 0;

    // let mut nr_linii = 1;
    for (nr_linii, linia) in std::fs::read_to_string("wizytówka.txt")?
        .lines()
        .enumerate()
    {
        match nr_linii {
            0 => imię = linia.to_string(),
            1 => nazwisko = linia.to_string(),
            2 => l_dalnomównika = linia.trim().parse()?,
            _ => {}
        }
    }

    println!("{imię}\n{nazwisko}\n{l_dalnomównika}");
    Ok(())
}
