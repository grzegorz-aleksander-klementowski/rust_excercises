use std::array;

fn main() {
    // The variables prepeared according to the training way – to train arrays and match
    let mut temat = String::new();
    let mut nick = String::new();
    const LICZBA_PYTAŃ: usize = 5;

    let mut treść: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());
    let mut odp_a: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());
    let mut odp_b: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());
    let mut odp_c: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());
    let mut odp_d: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());
    let mut poprawna_odp: [String; LICZBA_PYTAŃ] = array::from_fn(|_| String::new());

    // Load the quiz file
    let quiz = std::fs::read_to_string("quiz.txt").expect("Nie udało się odczytać pliku!");

    let mut nr_pytania = 0;
    let mut nr_linii = 1;
    // Parse the loaded file into particular string array variables.
    for linia in quiz.lines() {
        println!("Nr lini: {nr_linii}");

        match nr_linii {
            1 => temat = linia.to_string(),
            2 => nick = linia.to_string(),

            3 => treść[nr_pytania] = linia.to_string(),
            4 => odp_a[nr_pytania] = linia.to_string(),
            5 => odp_b[nr_pytania] = linia.to_string(),
            6 => odp_c[nr_pytania] = linia.to_string(),
            7 => odp_d[nr_pytania] = linia.to_string(),
            8 => poprawna_odp[nr_pytania] = linia.to_string(),
            _ => eprintln!("Niepoprawna linia"),
        }
        if nr_linii >= 8 {
            nr_linii = 2;
            nr_pytania += 1;
            if nr_pytania >= 5 {
                break;
            }
            println!("Przypisuje nr pytania: {nr_pytania}");
        }
        nr_linii += 1;
    }

    println!("{nick:?}");
}
