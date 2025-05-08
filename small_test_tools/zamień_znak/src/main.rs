use std::io;

struct Wieści {
        zawartość: String,
    }
    
enum Zwrot {
    Zwykły(Wieści),
    Uwaga(Wieści),
    Błąd(Wieści),
}

impl Zwrot {
    fn nowy_zwrot (zawartość: &str) -> Zwrot {
        Zwrot::Zwykły(Wieści { 
            zawartość: zawartość.to_string(),
        })
    }

    fn nowa_uwaga(zawartość: &str) -> Zwrot {
        Zwrot::Uwaga(Wieści {
            zawartość: zawartość.to_string(),
        })
    }

    fn nowy_błąd (zawartość: &str) -> Zwrot {
        Zwrot::Błąd(Wieści {
            zawartość: zawartość.to_string(),
        })
    }

    fn wypisz_zwrot(&self) {
        match self {
            Zwrot::Zwykły(wiadomość) => println!("{}", wiadomość.zawartość), 
            Zwrot::Uwaga(wiadomość) => println!("Uwaga: {}", wiadomość.zawartość),
            Zwrot::Błąd(wiadomość) => println!("Błąd! {}", wiadomość.zawartość),
        }
    }
}


fn main() {
    
       let przywitanie = Zwrot::nowy_zwrot("Wprowadz pismo, w jakim chcesz wprowadzić zmiany: ");
       przywitanie.wypisz_zwrot();

       let mut pismo = String::new();
       io::stdin()
           .read_line(&mut pismo)
           .expect("Błąd podczas przetwarzania pisma.");
        
       let uwaga = Zwrot::nowa_uwaga("Wprowadono pismo.");
       uwaga.wypisz_zwrot();
       let zwrot = Zwrot::nowy_zwrot("Wprowadź znak, jaki chcesz, aby ostał zamieniony.");
       zwrot.wypisz_zwrot();

       println!("{}", &pismo);


}

