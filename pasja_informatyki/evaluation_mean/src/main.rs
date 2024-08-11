use std::io;

fn main() {
    println!("Please enter value:");

    let mut evaluations: Vec<u8> = Vec::new();

    let mut evaluation: u8;
    while evaluations.len() < 7 {
        io::stdin()
            .read_line(&mut evaluation)
            .expect("Failed to read line");
        
        evaluations.push(evaluation);
    }
    

}
