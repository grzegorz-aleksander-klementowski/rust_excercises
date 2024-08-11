use std::io;

fn main() {
    //start - ask about values
    println!("Please enter value:");
    
    //define Vectors
    let mut evaluations: Vec<u8> = Vec::new();

    //define reader of the values
    let mut evaluation: u8;
    while evaluations.len() < 7 {
        io::stdin()
            .read_line(&mut evaluation)
            .expect("Failed to read line");
        //push read values
        evaluations.push(evaluation);
    }
    
    //show values to test
    for e in evaluations {
        println!("You entered evaluations: ");
    }
        

}
