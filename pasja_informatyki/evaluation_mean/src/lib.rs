use std::fmt;

pub enum Messeges {
    Welcome,
    PrintEvaluations(Evaluation),
}

impl fmt::Display for Messeges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messeges::Welcome           => "Please enter value:",
            Messeges::PrintEvaluations(evaluation)  => "placeholder",
        };
        write!(f, "{}", message)
    }
}

pub struct Evaluation {
    evaluations: Vec<f32>, 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(Messases::Welcome, "Please enter value:");
    }

    #[test] 
    fn test_print_evaluation(evaluation) {
       let evaluationMessage = Messeges::PrintEvaluations(vec![3, 4, 5 , 6, 7, 8]);
       assert_eq!();
    }
}
