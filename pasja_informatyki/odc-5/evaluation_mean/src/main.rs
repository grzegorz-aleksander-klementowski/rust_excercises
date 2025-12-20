use evaluation_mean::{Gradesbook, Input, Messages, System, Terminal};

fn main() {
    let mut dzinnik_ocen_gradesbook: Gradesbook = Gradesbook::new();
    println!("{}", Messages::Welcome);
    let number_of_inserted_grades: usize = dzinnik_ocen_gradesbook.get_valid_input_with_attempts();
    println!(
        "{}",
        Messages::InformToStartWriteGrades(number_of_inserted_grades)
    );
    dzinnik_ocen_gradesbook.add_with_input_many_times(number_of_inserted_grades);
    println!("{}", Messages::PrintSetOfGrades(&dzinnik_ocen_gradesbook));
    let grades_mean: f32 = dzinnik_ocen_gradesbook.weighted_average();
    println!("{}", Messages::PrintEvaluationMean(grades_mean));

    System.end_terminal_key();
}
