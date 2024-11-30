use std::fmt;
use std::error::Error;

// struct to store all grades
struct Gradesbook {
     grades: Vec<f32>,
}

// implementation of Gradesbook for grades with constructor, getter and validations
impl Gradesbook {
    fn new() -> Self { // constructor
        Gradesbook { 
            grades: Vec::new(),
        }
    }

    // validate whether the grade is  in range (1-6), and if it can be only full integer or
    // integer+half. Which 0.5 represent „+” (ei. '4.5' represent '4+')
    fn add(&mut self, grade: f32) -> Result<(), ErrMessages> {
        if (grade >= 1.0 && grade <= 6.0) && ((grade.fract() == 0.0) || (grade.fract() == 0.5)) { self.grades.push(grade); Ok(()) }
        else { Err(ErrMessages::GradeOutOfRange) } 
    }

    // gettet for grades
    fn show_grades(&self) -> &Vec<f32> { 
        &self.grades
    }
}

// trait definition for validation
trait Validator<T> {
    fn validate(&mut self, value: T) -> Result<(), ErrMessages>;
}

// trait that validate whether the grade is  in range (1-6), and if it can be only full integer or
// integer+half. Which 0.5 represent „+” (ei. '4.5' represent '4+')
impl Validator<f32> for Gradesbook {
    fn validate(&mut self, grade: f32) -> Result<(), ErrMessages> {
        if (grade >= 1.0 && grade <= 6.0) && ((grade.fract() == 0.0) || (grade.fract() == 0.5)) { self.grades.push(grade); Ok(()) }
        else { Err(ErrMessages::GradeOutOfRange) } 
    }
}

// Enum to define message to for user interaction
enum Messages<'a> {
    Welcome,
    PrintSetOfGrades(&'a Gradesbook),
    GradeAdded,
}

// Implementation of Display trail to format messages
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
                        .join("\n");
                    let formatted_message = format!("Grades:\n{}", grades_str);
                    return write!(f, "{}", formatted_message);
            }
        };
        write!(f, "{}", message)
    }
}

// Enum for ErrMessages
#[derive(Debug)]
pub enum ErrMessages {
    GradeOutOfRange,
}

// Implementation of Display trail to format ErrMessage
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
       let list_of_grades: Vec<f32>= vec![3.0, 4.5, 5.0 , 3.5, 4.0, 2.5];
       let test_gradesbook = Gradesbook { grades: list_of_grades };
       let message = format!("{}", Messages::PrintSetOfGrades(&test_gradesbook));
       assert_eq!(message, "Grades:\n3\n4.5\n5\n3.5\n4\n2.5");
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
        let correct_grades: [f32; 3] = [5.0, 5.5, 1.0];
        let uncorrect_grades: [f32; 3] = [5.3, 7.0, 0.5];

        for &grade in correct_grades.iter() {
            assert!(test_gradesbook.validate(grade).is_ok(), "Expected „ok” for grade: {}", &grade);
        }
        for &grade in uncorrect_grades.iter() {
            assert!(test_gradesbook.validate(grade).is_err(), "Expected „err” for grade: {}", &grade);
        }
    }

}
