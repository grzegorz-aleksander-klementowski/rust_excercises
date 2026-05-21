use std::io::Write;

pub trait AskForData {
    fn ask_for_data(text: &str) -> Self;
}

impl<T> AskForData for T
where
    T: std::ops::Add + std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
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
