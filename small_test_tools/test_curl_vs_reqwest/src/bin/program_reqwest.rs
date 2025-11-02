use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..100 {
        let response = reqwest::blocking::get("http://example.com")?.text()?;
        println!("Iteration {}: {}", i + 1, response);
    }
    Ok(())
}
