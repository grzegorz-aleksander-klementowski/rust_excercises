pub enum Messeges {
    Welcome,
    PrintEvaluations,
}

impl Messeges {
    pub fn print_messge (&self) -> String {
        match self {
            Messeges::Welcome           => "Please enter value:".to_string(),
            Messeges::PrintEvaluations  => "placeholder".to_string(),
        }
    }
}

pub struct Evaluation {
    evaluations: Vec<f32>, 
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
