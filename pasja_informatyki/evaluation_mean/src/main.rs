use evaluation_mean::{Gradesbook, Messages, Input};

fn main() {
    let mut dzinnik_ocen_gradesbook: Gradesbook = Gradesbook::new();
    println!("{}", Messages::Welcome);    
    let number_of_inserted_grades: usize = dzinnik_ocen_gradesbook.get_valid_input_with_attempts();
    <Gradesbook as Input<f32>>::input_many_times(&mut dzinnik_ocen_gradesbook, number_of_inserted_grades);
}
