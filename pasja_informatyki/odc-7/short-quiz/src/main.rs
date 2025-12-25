// This is excercies of C++ style (build with this intention)

use std::array;
use std::io;
use std::io::Write;

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
        //println!("Nr lini: {nr_linii}");

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
            //println!("Przypisuje nr pytania: {nr_pytania}");
        }
        nr_linii += 1;
    }

    // Interact with the user
    let mut punkty: usize = 0;

    for nr_pytania in 0..LICZBA_PYTAŃ {
        // Print the question for the user
        println!("Pytanie {}: {}", nr_pytania + 1, treść[nr_pytania]);
        println!("A. {}", odp_a[nr_pytania]);
        println!("B. {}", odp_b[nr_pytania]);
        println!("C. {}", odp_c[nr_pytania]);
        println!("D. {}", odp_d[nr_pytania]);

        // Ask about the user answer
        print!("Twoja odpowiedź: ");
        io::stdout().flush().expect("Nie mogę wyczyścić linii");
        let mut odpowiedz = String::new();
        io::stdin()
            .read_line(&mut odpowiedz)
            .expect("Nie mogę przeczytać odpowiedzi.");
        let odpowiedz = odpowiedz.trim();

        // Check if the answer is correct
        if poprawna_odp[nr_pytania] == odpowiedz {
            println!(
                "To jest poprawna_odp! \n poprawna: {}\nużytkownika: {}\n",
                poprawna_odp[nr_pytania], odpowiedz
            );
            punkty += 1;
        } else {
            println!(
                "Błędna odpowiedź! \n poprawna: {}\nużytkownika: {}\n",
                poprawna_odp[nr_pytania], odpowiedz
            );
        }
    }

    println!("{nick} zdobył {punkty} punktów");
}
