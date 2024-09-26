use std::fmt;


fn add_evaluations(set_of_grates: &mut Vec<f32>, grate: f32) {
    set_of_grates.push(grate);
}


pub enum Messeges {
    Welcome,
    PrintSetOfGrates(Vec<f32>),
}

impl fmt::Display for Messeges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messeges::Welcome           => "Please enter value:",
            Messeges::PrintSetOfGrates(grates)  => "placeholder",
        };
        write!(f, "{}", message)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(Messeges::Welcome, "Please enter value:");
    }

    #[test] 
    fn test_print_evaluation() {
       let grates_message: Vec<f32>= vec![3.0, 4.5, 5.0 , 3.5, 4.0, 2.5];
       let message = format!("{}", Messeges::PrintSetOfGrates(grates_message));
       assert_eq!(message, "test");
    }
}
