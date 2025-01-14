// książnica
pub mod components;
use components::{config, input, output, process};


pub struct ZapiskiOsoboweKrotka(String, String, String, String, String);

pub struct ZapiskiOsobowe {
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_zrzeszenie: String,
    pub nagłówek_poczta: String,
    pub nagłówek_dalnomównik: String,
}


/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprawdzian_układania_zapisków() {
        let zapiski_czuwamira = ZapiskiOsobowe { 
            nagłówek_fn: String::from("Czuwamir"),
            nagłówek_n: String::from("Rychły"), 
            nagłówek_email: String::from("czuwamir.rychły@poczta.pl"), nagłówek_dalnomównik: String::from("+48123456789") 
        };
        let wynik = zapiski_czuwamira;

        assert_eq!(wynik, "BEGIN:VCARD\nVERSION:2.0\nFN:Czuwamir\nN:Rychły\nORG:Rychłowie\nEMAIL;TYPE=INTERNET:czuwamir.rychły@poczta.pl\nTEL;TYPE=CELL:+48123456789\nEND:VCARD");

    }
}
*/
