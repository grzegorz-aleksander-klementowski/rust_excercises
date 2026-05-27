use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const ILOŚĆ_OCEN: usize = 5;
    let mut oceny = [0.0; ILOŚĆ_OCEN];

    for (wskaźnik, ocena) in oceny.iter_mut().enumerate() {
        let l_oceny = wskaźnik + 1;
        print!("Wpisz {l_oceny} ocenę: ");
        std::io::stdout().flush()?; // uwalnia bufor

        let mut wejście = String::new();
        std::io::stdin().read_line(&mut wejście)?;

        *ocena = wejście.trim().parse()?;
    }

    let suma = oceny.iter().sum::<f32>().round();
    #[allow(clippy::cast_precision_loss)]
    let ilość_ocen = oceny.iter().len() as f32;
    let średnia_arytmetyczna = suma / ilość_ocen;

    println!("oceny: {oceny:?}");
    println!("Średnia arytmetyczna: {średnia_arytmetyczna:.2}");
    Ok(())
}
