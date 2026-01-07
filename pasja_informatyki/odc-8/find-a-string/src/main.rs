fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find a string in text
    println!("Program do wyszukiwanai frazy w tekście");
    let text = String::from("Ala ma kota");
    println!("Wyszukuję w napisie „{text}”, a wyszukuję „kota”.");
    let wyszukane = text.find("kota").unwrap();

    if text.contains("kota") {
        println!("Słowo znajduje się w tekście.");
    } else {
        println!("Słowo nie znajduje się w tekście.")
    }
    println!("znalezione w pozycji: {wyszukane:?}");

    let mut text = text.replace("kota", "");
    println!("{text}");

    text.insert_str(wyszukane, "psa");
    println!("{text}");

    Ok(())
}
