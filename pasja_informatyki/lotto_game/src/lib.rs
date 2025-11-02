use rand::Rng;
use std::process::Command;
use std::{thread, time};

/// Generates a beep sound based on the operating system.
fn beep() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/c", "echo", "\x07"])
            .status()
            .expect("failed to execute process.");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("sh")
            .arg("-c")
            .arg("printf '\\007'")
            .status()
            .expect("failed to execute process.");
    }
}

/// Pauses the execution of the program for a specified number of seconds.
pub fn stop_for_seconds(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}

/// Represents a set of Lotto numbers.
pub struct LottoSet {
    set: [u8; 6],
}

/// Represents various messages that can be displayed to the user.
pub enum Message {
    Welcome,
    LottoResults(LottoSet),
}

impl Message {
    /// Creates a new LottoResults message with the given set of numbers.
    pub fn new_lotto_set(content_of_set: [u8; 6]) -> Message {
        Message::LottoResults(LottoSet {
            set: content_of_set,
        })
    }

    /// Prints the message to the console.
    pub fn print_message(&self) {
        match self {
            Message::Welcome => println!(
                "Hello!\n In 3 second the blocade will be realesed and the drawing will start!"
            ),
            Message::LottoResults(lotto_result_set) => {
                println!("LottoResults: ");
                for number in &lotto_result_set.set {
                    println!("( {} )", number);
                    beep();
                    stop_for_seconds(1);
                }
            }
        }
    }
}

/// Generates a random Lotto number that is not already in the set.
fn generate_lotto_num_for_set(set: &[Option<u8>; 6]) -> Result<u8, &'static str> {
    loop {
        let lotto_number: u8 = rand::thread_rng().gen_range(1..=49);
        if !set.contains(&Some(lotto_number)) {
            return Ok(lotto_number);
        }
        let mut attemps: u8 = 0;
        attemps += 1;
        if attemps > 100 {
            return Err("Too many attemps to generate lotto set. ");
        }
    }
}

/// Generates a set of 6 unique Lotto numbers.
pub fn generate_lotto_set() -> Result<[Option<u8>; 6], &'static str> {
    let mut lotto_numer_arr: [Option<u8>; 6] = [None; 6];
    for i in 0..6 {
        let lotto_numer: Result<u8, &'static str> = generate_lotto_num_for_set(&lotto_numer_arr);
        match lotto_numer {
            Ok(value) => {
                lotto_numer_arr[i] = Some(value);
                match lotto_numer_arr[i] {
                    Some(value) => {
                        if !(1..49).contains(&value) {
                            return Err(
                                "Lotto number generation failed. The number is out of range. ",
                            );
                        }
                    }
                    None => {
                        return Err(
                            "Generation of lotto set failed. Number in set is unexpected empty. ",
                        )
                    }
                }
            }
            Err(e) => {
                let error_message = format!(
                    "Generation of lotto set failed due to an error. Additionally: {}",
                    e
                );
                return Err(Box::leak(error_message.into_boxed_str()));
            }
        }
    }
    return Ok(lotto_numer_arr);
}

/// Converts the result of the Lotto set generation to an array of u8.
pub fn generate_lotto_set_output(
    lotto_set_result: Result<[Option<u8>; 6], &'static str>,
) -> [u8; 6] {
    let mut lotto_set: [u8; 6] = [0u8; 6];
    match lotto_set_result {
        Ok(set_of_options) => {
            for (i, option_number) in set_of_options.iter().enumerate() {
                match option_number {
                    Some(value) => lotto_set[i] = *value,
                    None => {
                        lotto_set[i] = 0;
                        eprintln!("Error durring lotto set generation.");
                    }
                }
            }
            return lotto_set;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return [0u8; 6];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lotto_num_repeat_in_set() {
        let set = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        let lotto_number = 2;
        assert!(set.contains(&Some(lotto_number)));
    }
    #[test]
    fn test_generate_lotto_num_not_repeat_in_set() {
        let set = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        let lotto_number = 7;
        assert!(!set.contains(&Some(lotto_number)));
    }

    #[test]
    fn test_generate_lotto_set() {
        let set = generate_lotto_set();
        assert!(set.is_ok(), "Funciton should return ok, generate lotto set failed. Function find null in set, where shound not.");

        let number = generate_lotto_num_for_set(&set.unwrap_or([Some(0u8); 6]));
        assert!(number.is_ok(), "Function should return ok, gennerate num failed. Too many attemps to generate number, which is not the same numers from existed the lotto set numbers.");
    }

    #[test]
    fn test_generate_lotto_set_output() {
        let set_result: Result<[Option<u8>; 6], &'static str> =
            Ok([Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let result = generate_lotto_set_output(set_result);
        assert_eq!(result.len(), 6);
        assert_eq!(result, [1, 2, 3, 4, 5, 6]);
    }
}
