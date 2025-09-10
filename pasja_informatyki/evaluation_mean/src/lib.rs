use std::fmt;
use std::io::{self, Write};

// struct to store all grades
pub struct Gradesbook {
    pub grades: Vec<(f32, f32)>,
}

// implementation of Gradesbook for grades with constructor, getter and validations
impl Gradesbook {
    pub fn new() -> Self {
        // constructor
        Gradesbook { grades: Vec::new() }
    }

    // gettet for grades
    fn show_grades(&self) -> &Vec<(f32, f32)> {
        &self.grades
    }

    // add grade to Gradebook
    fn add(&mut self, grade_with_wage: (f32, f32)) {
        self.grades.push(grade_with_wage);
    }

    // use input trait to add grades with weight to Gradesbook
    pub fn add_with_input_many_times(&mut self, numer_of_needed_grades_to_add: usize) {
        let mut numer_of_added_grades: usize = 1;
        while numer_of_added_grades <= numer_of_needed_grades_to_add {
            println!("{numer_of_added_grades}.:");

            print!("{}", Messages::InformToInsertGrade);
            io::stdout().flush().unwrap();
            let grade: f32 = self.get_valid_input_with_attempts();

            print!("{}", Messages::InformToInsertWeight);
            io::stdout().flush().unwrap();
            let weight: f32 = self.get_valid_input_with_attempts();

            println!();
            self.add((grade, weight));

            numer_of_added_grades += 1;
        }
    }

    // calculate mean of grades
    pub fn weighted_average(&self) -> f32 {
        let sum_weighted: f32 = self
            .show_grades()
            .iter()
            .map(|(grade, weight)| grade * weight)
            .sum();
        let total_weights: f32 = self.show_grades().iter().map(|(_, weight)| weight).sum();
        let weighted_average: f32 = sum_weighted / total_weights;

        (weighted_average * 100.0).round() / 100.0
    }
}

// ------------- Defaul implementation for Gradesbook ------------- \\
impl Default for Gradesbook {
    fn default() -> Self {
        Self::new()
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
        if (1.0..=6.0).contains(&grade) && ((grade.fract() == 0.0) || (grade.fract() == 0.5)) {
            Ok(())
        } else {
            Err(ErrMessages::GradeOutOfRange)
        }
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
pub trait Input<T> {
    //fn read_input(&mut self) -> Result<f32, ErrMessages>;
    fn read_input(&mut self) -> Result<T, ErrMessages>;
    fn get_valid_input_with_attempts(&mut self) -> T; // use `read_input` function result to get grade and unwrap it
}

// Implementation Input to Gredesbook that read line, sent the line to validation,
// is is ok, then return possitive result with grade
impl Input<f32> for Gradesbook {
    fn read_input(&mut self) -> Result<f32, ErrMessages> {
        let mut grade = String::new();
        match io::stdin().read_line(&mut grade) {
            Ok(_) => {
                match grade.trim().parse::<f32>() {
                    Ok(grade) => {
                        match self.validate(grade) {
                            // number goes to validation
                            Ok(()) => return Ok(grade),
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => Err(ErrMessages::InvalidInput(Box::new(e))),
                }
            }
            Err(e) => Err(ErrMessages::InvalidInput(Box::new(e))),
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
                    } else {
                        // Extreme error - cannot read line
                        eprintln!("Za duuuużo prób Zosieńko :c Wychodzę…");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

impl Input<usize> for Gradesbook {
    fn read_input(&mut self) -> Result<usize, ErrMessages> {
        let mut num_of_needed_grades = String::new();
        match io::stdin().read_line(&mut num_of_needed_grades) {
            Ok(_) => {
                match num_of_needed_grades.trim().parse::<usize>() {
                    Ok(num_of_needed_grades) => {
                        match self.validate(num_of_needed_grades) {
                            // number goes to validation
                            Ok(()) => return Ok(num_of_needed_grades),
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => Err(ErrMessages::InvalidInput(Box::new(e))),
                }
            }
            Err(e) => Err(ErrMessages::InvalidInput(Box::new(e))),
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
                    } else {
                        // Extreme error - cannot read line
                        eprintln!("Za duuuużo prób Zosieńko :c Wychodzę…");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

/*-----------------------------------------------------------------------*/

// ------------ System configurations trait ------------ \\

pub struct System;

pub trait Terminal {
    fn end_terminal_key(&self);
}

impl Terminal for System {
    fn end_terminal_key(&self) {
        println!("{}", Messages::WindowsExiting);
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
    }
}

// ------------ Enum to define message to for user interaction ------------ \\
pub enum Messages<'a> {
    Welcome,
    InformToInsertGrade,
    InformToInsertWeight,
    InformToStartWriteGrades(usize),
    PrintSetOfGrades(&'a Gradesbook),
    PrintEvaluationMean(f32),
    WindowsExiting,
}

// Implementation of Display trail to format messages
impl<'a, 'b> fmt::Display for Messages<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Messages::Welcome                   => "Witaj Kochana Zosieńko :** To jest program specjalnie stworzony dla Ciebie, abyś mogła w łatwy i szybki sposób obliczyć swoj oceny na swoim obliczniku. Wpisz ilość ocen, z jakich chcesz obliczyć średnią: ",
            Messages::InformToStartWriteGrades(num_of_grades_to_write)  => return write!(f, "Wpisz {} i potwierdź przez przycisk wejścia [enter]. ", num_of_grades_to_write),
            Messages::InformToInsertGrade           => "ocena: ",
            Messages::InformToInsertWeight          => "waga: ",
            Messages::PrintEvaluationMean(mean)     => return write!(f, "\nOto Twoja średnia: {}. Powodzenia w następnym semestrze :3", mean),
            Messages::PrintSetOfGrades(gradesbook)  => {
                    let grades_str = gradesbook
                        .show_grades()
                        .iter()
                        .map(|(grade, weight)| format!("ocena: {grade} | waga: {weight}"))
                        .collect::<Vec<String>>()
                        .join("\t");
                    let formatted_message = format!("Oceny:\n{}", grades_str);
                    return write!(f, "{}", formatted_message);
            }
            Messages::WindowsExiting            => "Program zakończył się kochanie :* Życzę Ci duuuużo wspaniałych ocen c:",
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
        ErrMessages::InvalidInput(error) => format!("Aliquam numerus. Nie udało się odczytać wiersza: {}", error),
        ErrMessages::InvalidNumberOfNeededGrades => "Gradus feriunt. Zooosiu, numer, który wpisujesz jest albo za duży, albo wpisałaś zero! Spróbuj wpisać mniejszy numer (tak do 100000) albo liczbę większą nić 0. ".to_string(),
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
        assert_eq!(message, "Witaj Kochana Zosieńko :** To jest program specjalnie stworzony dla Ciebie, abyś mogła w łatwy i szybki sposób obliczyć swoj oceny na swoim obliczniku. Wpisz ilość ocen, z jakich chcesz obliczyć średnią: ");
    }

    #[test]
    fn test_a() {
        let five_grades_needed: usize = 5;
        let message = format!("{}", Messages::InformToStartWriteGrades(five_grades_needed));

        assert_eq!(
            message,
            "Wpisz 5 i potwierdź przez przycisk wejścia [enter]. "
        );
    }

    #[test]
    fn test_print_evaluation() {
        let list_of_grades: Vec<(f32, f32)> = vec![
            (3.0, 1.0),
            (4.5, 2.0),
            (5.0, 2.0),
            (3.5, 3.0),
            (4.0, 1.0),
            (2.5, 3.0),
        ];
        let test_gradesbook = Gradesbook {
            grades: list_of_grades,
        };
        let message = format!("{}", Messages::PrintSetOfGrades(&test_gradesbook));
        assert_eq!(message, "Oceny:\nocena: 3 | waga: 1\tocena: 4.5 | waga: 2\tocena: 5 | waga: 2\tocena: 3.5 | waga: 3\tocena: 4 | waga: 1\tocena: 2.5 | waga: 3");
    }

    #[test]
    fn test_add() {
        let mut test_gradesbook = Gradesbook::new();
        let grade_array: [(f32, f32); 3] = [(5.5, 1.0), (2.5, 2.0), (4.0, 3.0)];

        for &grade in grade_array.iter() {
            test_gradesbook.add(grade);
        }
        assert_eq!(
            test_gradesbook.show_grades(),
            &vec!((5.5, 1.0), (2.5, 2.0), (4.0, 3.0))
        );
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
            assert!(
                test_gradesbook.validate(grade).is_ok(),
                "Expected „ok” for grade: {}",
                &grade
            );
        }
        for &grade in uncorrect_grades.iter() {
            assert!(
                test_gradesbook.validate(grade).is_err(),
                "Expected „err” for grade: {}",
                &grade
            );
        }
    }

    #[test]
    fn test_validation_of_insert_needed_num_of_grades() {
        let mut test_gradesbook = Gradesbook::new();
        let mock_positive_insered_num: usize = 3;
        let mock_neg_insered_num_too_big: usize = 150000;
        let mock_neg_insered_num_zero: usize = 0;

        let result_pos = test_gradesbook.validate(mock_positive_insered_num);
        let result_neg_too_big = test_gradesbook.validate(mock_neg_insered_num_too_big);
        let result_neg_zero = test_gradesbook.validate(mock_neg_insered_num_zero);

        assert!(
            result_pos.is_ok(),
            "Expected „Ok()” for numer of needed grade, but is error. Tested numer: {}",
            mock_positive_insered_num
        );
        assert!(result_neg_too_big.is_err(), "Expected „Err()” for numer of needed grade, but is possitive, when the numer is too big. Test numer: {}", mock_neg_insered_num_too_big);
        assert!(result_neg_zero.is_err(), "Expected „Err()” for numer of needed grade, but is possitive, when the numer is zero. Test numer: {}", mock_neg_insered_num_zero);
    }

    #[test]
    fn test_agarage_weight() {
        let mut test_gradesbook = Gradesbook::new();
        test_gradesbook.grades = vec![(5.5, 1.0), (2.5, 2.0), (4.0, 3.0), (4.5, 2.0)];
        assert_eq!(3.94, test_gradesbook.weighted_average());
    }
}
