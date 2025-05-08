use std::io;

fn main() {
    println!("Witaj! Jestem obliczanikowym Swinksenm. \nZadaj mi pytanie a prawdę Ci powiem…");


    loop {
        let mut pytanie = String::new();

        io::stdin()
            .read_line(&mut pytanie)
            .expect("Nie udało się przeczytać napisu…");
        let wymów: &str = &pytanie.trim();
        match wymów {
            "Kogo kocham?" => println!("Kogo kochasz? Zosię kochasz."),
            "A z kim chcę się wiązać?" => println!("Myślę, że chcesz tylko Zosię."),
            "Dziękuję!" => {
                println!("Do widzenia!");
                break;
            }
            _ => println!("Mówiłem, że tylko prawdę mówię!"),
        }
    }
}
