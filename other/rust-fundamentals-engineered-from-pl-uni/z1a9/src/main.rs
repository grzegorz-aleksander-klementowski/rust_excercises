// Napisz program, który znajduje wszystkie trójki pitagorejskie o wartościach nie większych niż dana.
//
//    Zakładamy, że 0 < a < b < c. Zadanie zrób na dwa sposoby — z użyciem pętli while/loop oraz z użyciem pętli for.
//    9. Write a program that finds all Pythagorean triples with values not greater than the given value.

//   We assume that 0 < a < b < c. Do the task in two ways - using a while/loop loop and using a for loop.

use std::io::Write;

use pythagorians_triangle::GeometricCondition;

mod pythagorians_triangle;

trait AskForData {
    fn ask_for_data(text: &str) -> Self;
}

impl AskForData for usize {
    fn ask_for_data(text: &str) -> Self {
        print!("{text}");
        std::io::stdout().flush().expect("Can't release the buf.");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Can't read line.");

        buf.trim().parse().expect("Can't parse.")
    }
}

fn main() {
    let range_from = usize::ask_for_data("Check the pythagorians triangles from: ");
    let range_to = usize::ask_for_data("To: ");

    let triangles_in_range =
        pythagorians_triangle::PythagoreanTriangle::find_conditions_in_range(range_from..=range_to);

    for (n, i) in triangles_in_range.iter().enumerate() {
        println!("{}. {i:?}", n + 1);
    }
}
