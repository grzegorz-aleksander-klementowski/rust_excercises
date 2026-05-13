// 7. Napisz program, który wyświetla cyfry danej liczby całkowitej (od końca).
// 7. Write a program that displays the digits of a given integer (from the end).

trait Reverse {
    fn reverse(self) -> Self;
}

impl Reverse for String {
    // === MANUAL „HEAVY” WAY
    fn reverse(self) -> Self {
        let mut rev_s = String::new();

        for char_pos in (0..self.len()).rev() {
            let char = self.chars().nth(char_pos);
            match char {
                Some(ch) => rev_s.push(ch),
                None => {
                    eprintln!("No character in position {char_pos} of string {self}");
                }
            }
        }

        rev_s

        // === AUTOMATIC–IDIOMATIC WAY ===
        // self.chars().rev().collect()
    }
}

impl Reverse for usize {
    fn reverse(self) -> Self {
        self.to_string()
            .reverse()
            .parse::<usize>()
            .expect("Can't parse the number!")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Cannot read linec!");
    let num: usize = buf.trim().parse().expect("Can't parse the number!");
    let rev_num = num.reverse();
    println!("{rev_num}");
}
