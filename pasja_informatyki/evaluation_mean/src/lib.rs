pub enum Messeges {
    Welcome,
    PrintEvaluations,
}

impl Messeges {
    pub fn print_messge (&self) {
        mach self {
            Messeges::Welcome => println!("Please enter value:"),
            PrintEvaluations    => println!("placeholder"),
        }
    }
}

pub struct Evaluation {
    
}


#[cfg(tests)]
mod tests {
    use super::*;

    #[test] 
    fn test_print_evaluation() {
       let evaluationMessage = Messeges::PrintEvaluations(vec![3, 4, 5 , 6, 7, 8]);
       assert_eq!
    }
}
