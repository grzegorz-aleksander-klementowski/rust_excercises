use hex_color_display::print_color;

fn main() {
    let colors = vec![
        "#51202A", "#FF0000", "#52462A", "#FFCC00", "#2A5246", "#00FFFF", "#2A4652", "#00FF00",
    ];
    for color in colors {
        print_color(color);
    }
}
