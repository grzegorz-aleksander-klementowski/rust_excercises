use curl::easy::Easy;
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    easy.url("http://example.com")?; // Ustaw URL

    for i in 0..100 {
        let mut response_data = Vec::new(); // Bufor na dane odpowiedzi
        {
            let mut transfer = easy.transfer(); // Utwórz transfer
            transfer.write_function(|data| {
                response_data.extend_from_slice(data); // Zapisuj dane do bufora
                Ok(data.len())
            })?;
            transfer.perform()?; // Wykonaj żądanie
        }
        println!(
            "Iteration {}: {}",
            i + 1,
            str::from_utf8(&response_data)? // Wyświetl zawartość odpowiedzi jako tekst
        );
    }

    Ok(())
}
