use std::fmt;


fn add_gradtes(set_of_grades: &mut Vec<f32>, grade: f32) {
    set_of_grades.push(grade);
}


pub enum Messages {
    Welcome,
    PrintSetOfGrades(Vec<f32>),
}

impl fmt::Display for Messages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messages::Welcome                   => "Please enter value: ",
            Messages::PrintSetOfGrades(grades)  => "placeholder",
        };
        write!(f, "{}", message)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        let message = format!("{}", Messages::Welcome);
        assert_eq!(message, "Please enter value: ");
    }

    #[test] 
    fn test_print_evaluation() {
       let grades_message: Vec<f32>= vec![3.0, 4.5, 5.0 , 3.5, 4.0, 2.5];
       let message = format!("{}", Messages::PrintSetOfGrades(grades_message));
       assert_eq!(message, "Grades:\n3.0, \n4.5, \n5.0, \n3.5, \n4.0, \n2.5");
    }
}
