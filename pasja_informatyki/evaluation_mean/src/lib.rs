use std::fmt;
use std::error::Error;

struct Gradesbook {
     grades: Vec<f32>,
}

impl Gradesbook {
    fn new() -> Self {
        Gradesbook { 
            grades: Vec::new(),
        }
    }

    fn add(&mut self, grade: f32) -> Result<(), ErrMessages> {
        if (grade >= 1.0 && grade <= 6.0) && ((grade.fract() == 0.0) || (grade.fract() == 0.5)) { self.grades.push(grade); Ok(()) }
        else { Err(ErrMessages::GradeOutOfRange) } 
    }
    
    fn show_grades(&self) -> &Vec<f32> {
        &self.grades
    }
}

enum Messages<'a> {
    Welcome,
    PrintSetOfGrades(&'a Gradesbook),
    GradeAdded,
}

impl<'a> fmt::Display for Messages<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messages::Welcome                   => "Please enter a grade: ",
            Messages::GradeAdded                => "Grades added. ",
            Messages::PrintSetOfGrades(gradesbook)  => {
                    let grades_str = gradesbook
                        .show_grades()
                        .iter()
                        .map(|grade| grade.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                   format!("Grades: {}", grades_str)
            }
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum ErrMessages {
    GradeOutOfRange,
}

impl fmt::Display for ErrMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
        ErrMessages::GradeOutOfRange => "Gradus feriunt. Grade is out of range. Grades must be in a range from 1.0 to 6.0 and be full (5.0) or half (5.5) number. ",
        };
        write!(f, "Error: {}", err_message)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        let message = format!("{}", Messages::Welcome);
        assert_eq!(message, "Please enter a grade: ");
    }

    #[test] 
    fn test_print_evaluation() {
       let grades_message: Vec<f32>= vec![3.0, 4.5, 5.0 , 3.5, 4.0, 2.5];
       let message = format!("{}", Messages::PrintSetOfGrades(grades_message));
       assert_eq!(message, "Grades:\n3.0, \n4.5, \n5.0, \n3.5, \n4.0, \n2.5");
    }

    #[test]
    fn test_add() {
        let mut test_gradesbook = Gradesbook::new();
        let grade_array: [f32; 3] = [5.5, 2.5, 4.0];

        for &grade in grade_array.iter() { 
            test_gradesbook.add(grade).expect("Failed to add grade");
        }
        assert_eq!(test_gradesbook.show_grades(), &vec!(5.5, 2.5, 4.0));
    }
    
    #[test]
    fn test_add_error_handling_range() {
        let result: Result<(), ErrMessages> = Gradesbook::new().add(7.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Error: Gradus feriunt. Grade is out of range. Grades must be in a range from 1.0 to 6.0 and be full (5.0) or half (5.5) number. ");
        }

    #[test]
    fn test_add_error_handling_number_correctness() {
        let mut test_gradesbook = Gradesbook::new();

        assert!(test_gradesbook.add(5.0).is_ok());
        assert!(test_gradesbook.add(5.5).is_ok());
        assert!(test_gradesbook.add(1.0).is_ok());
        assert!(test_gradesbook.add(5.3).is_err());
        assert!(test_gradesbook.add(7.0).is_err());
        assert!(test_gradesbook.add(0.5).is_err());
    }

}
