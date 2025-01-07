use evaluation_mean::Messages;

fn main() {
    let dzinnik_ocen_gradesbook: Gradesbook = Gradesbook::new();
    println!("{}", Messages::Welcome);    
    let mut number_of_inserted_grades: usize = dzinnik_ocen_gradesbook.get_valid_input_with_attempts();
    dzinnik_ocen_gradesbook.input_many_times(number_of_inserted_grades);
}
