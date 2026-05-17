// 5. Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59]).

use z1a5::{Hour, Minute, Second, Time};

fn main() {
    println!("The program substract times");
    let time1 = Time(Hour(2), Minute(25), Second(15));
    let time2 = Time(Hour(1), Minute(5), Second(45));
    print!("{time1:?} - {time2:?} = ");
    let time_result = time1 - time2;

    println!("{time_result:?}");
    println!("Type `cargo test` to see more tests.");
}
