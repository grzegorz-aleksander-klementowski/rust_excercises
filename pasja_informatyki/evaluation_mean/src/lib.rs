use std::fmt;

// struct to store all grades
pub struct Gradesbook {
     grades: Vec<f32>,
}

// implementation of Gradesbook for grades with constructor, getter and validations
impl Gradesbook {
    fn new() -> Self { // constructor
        Gradesbook { 
            grades: Vec::new(),
        }
    }

    // add grade to Gradebook
    fn add(&mut self, grade: f32) {
            self.grades.push(grade); 
        }

    // gettet for grades
    fn show_grades(&self) -> &Vec<f32> { 
        &self.grades
    }

    // calculate mean of grades
    fn evaluation_mean(&self) -> f32 {
        let number_of_grades: &usize = &self.show_grades().len();
        let number_of_grades_float: f32 = *number_of_grades as f32;
        let grades_sum: f32 = self.show_grades().iter().sum::<f32>() as f32;
        grades_sum / number_of_grades_float
    }
}

// ------------- trait definition for validation ------------- \\
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



// trait validete input usize numers to check if the user input correct number of needed grade \\
//to insert
impl Validator<usize> for Gradesbook {
    fn validate(&mut self, num_of_needed_grades: usize) -> Result<(), ErrMessages> {
        if num_of_needed_grades > 100000 || num_of_needed_grades < 1 {
            Err(ErrMessages::InvalidNumberOfNeededGrades)
        } else {
            Ok(())
        }
    }
}
/*----------------------------------------------------------------------*/

// -------------- traits interaface for Input functions -------------- \\
trait Input<T> {
    //fn read_input(&mut self) -> Result<f32, ErrMessages>;
    fn read_input(&mut self) -> Result<T, ErrMessages>;
    fn get_valid_input_with_attempts(&mut self) -> T; // use `read_input` function result to get grade and unwrap it
    fn input_many_times(&mut self, numer_of_many_times_input: usize);
}

// Implementation Input to Gredesbook that read line, sent the line to validation,
// is is ok, then return possitive result with grade 
impl Input<f32> for Gradesbook {

    fn read_input(&mut self) -> Result<f32, ErrMessages> {
        use std::io::{ self };
        let mut grade = String::new();
        // println!("{}", Messages::Welcome); // First welcome to ask about numer [CHEK IT]
        match io::stdin().read_line(&mut grade) {
            Ok(_)   => {
                match grade.trim().parse::<f32>() {
                    Ok(grade)   => {
                        match self.validate(grade) { // number goes to validation
                            Ok(()) => return Ok(grade),
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e)  => Err(ErrMessages::InvalidInput(Box::new(e))),
                }
            }
            Err(e)  => Err(ErrMessages::InvalidInput(Box::new(e))),
        }
    }


    // try again up 3 times in a case of failed read line durring input and unwrap the result
    fn get_valid_input_with_attempts(&mut self) -> f32 {

       let mut attempts: usize = 0;
       let max_attempts: usize = 3;

        loop {
            match self.read_input() {
                Ok(grade) => return grade,
                Err(e) => {
                    attempts += 1;
                    if attempts < max_attempts {
                        eprintln!("{}", e);
                    } else { // Extreme error - cannot read line
                        eprintln!("Too many attemps to read line. Exiting.");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

   fn input_many_times(&mut self, numer_of_needed_grades_to_add: usize) {
        let mut numer_of_added_grades: usize = 0;
        while numer_of_added_grades <= numer_of_needed_grades_to_add {
            let grade: f32 = self.get_valid_input_with_attempts();
            self.add(grade);
            numer_of_added_grades += 1;
        }
    }

}

impl Input<usize> for Gradesbook {

    fn read_input(&mut self) -> Result<usize, ErrMessages> {
        use std::io::{ self };
        let mut num_of_needed_grades = String::new();
        // println!("{}", Messages::Welcome); // First welcome to ask about numer [CHEK IT]
        match io::stdin().read_line(&mut num_of_needed_grades) {
            Ok(_)   => {
                match num_of_needed_grades.trim().parse::<usize>() {
                    Ok(num_of_needed_grades)   => {
                        match self.validate(num_of_needed_grades) { // number goes to validation
                            Ok(()) => return Ok(num_of_needed_grades),
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e)  => Err(ErrMessages::InvalidInput(Box::new(e))),
                }
            }
            Err(e)  => Err(ErrMessages::InvalidInput(Box::new(e))),
        }
    }

    // try again up 3 times in a case of failed read line durring input and unwrap the result
    fn get_valid_input_with_attempts(&mut self) -> usize {

       let mut attempts: usize = 0;
       let max_attempts: usize = 3;

        loop {
            match self.read_input() {
                Ok(grade) => return grade,
                Err(e) => {
                    attempts += 1;
                    if attempts < max_attempts {
                        eprintln!("{}", e);
                    } else { // Extreme error - cannot read line
                        eprintln!("Too many attemps to read line. Exiting.");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    fn input_many_times(&mut self, numer_of_needed_grades_to_add: usize) {
        let mut numer_of_added_grades: usize = 0;
        while numer_of_added_grades <= numer_of_needed_grades_to_add {
            let grade: f32 = self.get_valid_input_with_attempts();
            self.add(grade);
            numer_of_added_grades += 1;
        }
    }

}

/*-----------------------------------------------------------------------*/

// ------------ Enum to define message to for user interaction ------------ \\
pub enum Messages<'a> {
    Welcome,
    InformToStartWriteGrades,
    PrintSetOfGrades(&'a Gradesbook),
    GradeAdded,
}

// Implementation of Display trail to format messages
impl<'a> fmt::Display for Messages<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messages::Welcome                   => "Please enter a numer of grades you want to calculate: ",
            Messages::InformToStartWriteGrades  => return write!(f, "Write {} grades and cofirm by [enter] key. ", 27),
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
    InvalidInput(Box<dyn std::error::Error>),
    InvalidNumberOfNeededGrades,
}

// Implementation of Display trail to format ErrMessage
impl fmt::Display for ErrMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
        ErrMessages::GradeOutOfRange => "Gradus feriunt. Grade is out of range. Grades must be in a range from 1.0 to 6.0 and be full (5.0) or half (5.5) number. ".to_string(),
        ErrMessages::InvalidInput(error) => format!("Aliquam numerus. Failed to read input: {}", error),
        ErrMessages::InvalidNumberOfNeededGrades => "Gradus feriunt. Numer of needed grades is too big or equals zero! Try use lower numer, up to 100000. ".to_string(),
        
        };
        write!(f, "Error: {}", err_message)
    }
}
/*-----------------------------------------------------------------*/



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
            test_gradesbook.add(grade);
        }
        assert_eq!(test_gradesbook.show_grades(), &vec!(5.5, 2.5, 4.0));
    }
    
    #[test]
    fn test_add_error_handling_range() {
        let result: Result<(), ErrMessages> = Gradesbook::new().validate(7.0);
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

    #[test]
    fn test_validation_of_insert_needed_num_of_grades() {
        let mut test_gradesbook = Gradesbook::new();
        let mock_positive_insered_num:      usize = 3;
        let mock_neg_insered_num_too_big:   usize = 150000;
        let mock_neg_insered_num_zero:      usize = 0;

        let result_pos          = test_gradesbook.validate(mock_positive_insered_num);
        let result_neg_too_big  = test_gradesbook.validate(mock_neg_insered_num_too_big);
        let result_neg_zero     = test_gradesbook.validate(mock_neg_insered_num_zero);

        assert!(result_pos.is_ok(), "Expected „Ok()” for numer of needed grade, but is error. Tested numer: {}", mock_positive_insered_num);
        assert!(result_neg_too_big.is_err(), "Expected „Err()” for numer of needed grade, but is possitive, when the numer is too big. Test numer: {}", mock_neg_insered_num_too_big);
        assert!(result_neg_zero.is_err(), "Expected „Err()” for numer of needed grade, but is possitive, when the numer is zero. Test numer: {}", mock_neg_insered_num_zero);
    }

    #[test]
    fn test_evaluation_mean() {
        let mut test_gradesbook = Gradesbook::new();
        test_gradesbook.grades = vec![5.0, 2.5, 3.0, 4.0, 4.5];
        assert_eq!(3.8, test_gradesbook.evaluation_mean());
    }

/*    #[test]
    fn test_get_valid_grade_with_attempts() {
        let mut test_gradesbook = Gradesbook::new();
        fn test_read_input_grade(_: &mut Gradesbook) -> Result<f32, ErrMessages> {
            return Ok(5.0);
        }
        let grade: f32 = test_gradesbook.get_valid_grade_with_attempts();
        assert_eq!(5.0, grade);
    }
*/
}
