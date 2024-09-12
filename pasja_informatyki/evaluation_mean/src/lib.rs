pub enum Messeges {
    Welcome,
    PrintEvaluations,
}

impl Messeges {
    pub fn print_messge (&self) {
        match self {
            Messeges::Welcome           => "Please enter value:".to_string(),
            Messeges::PrintEvaluations  => println!("placeholder"),
        }
    }
}

pub struct EvaluationMean {
    evaluations: Vec<u8>, 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_print_evaluation() {
       let evaluationMessage = Messeges::PrintEvaluations(vec![3, 4, 5 , 6, 7, 8]);
       assert_eq!();
    }
}
