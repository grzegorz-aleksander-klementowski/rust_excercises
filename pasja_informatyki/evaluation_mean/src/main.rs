use evaluation_mean::{Gradesbook, Messages, Input};

fn main() {
    let mut dzinnik_ocen_gradesbook: Gradesbook = Gradesbook::new();
    println!("{}", Messages::Welcome);    
    let number_of_inserted_grades: usize = dzinnik_ocen_gradesbook.get_valid_input_with_attempts();
    println!("{}", Messages::InformToStartWriteGrades(number_of_inserted_grades));
    <Gradesbook as Input<usize>>::input_many_times(&mut dzinnik_ocen_gradesbook, number_of_inserted_grades);
    println!("{}", Messages::PrintSetOfGrades(&dzinnik_ocen_gradesbook));
    let grades_mean: f32 = dzinnik_ocen_gradesbook.evaluation_mean();
    println!("{}", Messages::PrintEvaluationMean(grades_mean));
}
