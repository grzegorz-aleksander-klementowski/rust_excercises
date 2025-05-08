fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

pub fn print_color(hex: &str) {
    if let Some((r, g, b)) = hex_to_rgb(hex) {
        println!("\x1b[48;2;{};{};{}m   {}   \x1b[0m", r, g, b, hex);
    } else {
        eprintln!("Invalid hex color: {}", hex);
    }
}
