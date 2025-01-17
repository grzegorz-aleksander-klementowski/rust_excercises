// książnica
pub mod components;
use components::{ config, input, output };


pub struct ZapiskiOsobowe {
    pub zaczynajka_karty_vcf: String,
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_zrzeszenie: String,
    pub nagłówek_poczta: String,
    pub nagłówek_dalnomównik: String,
    pub kończajka_karty_vcf: String,
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sprawdzian_zapisków() {
        let zapisek_sprawdzający: ZapiskiOsobowe = ZapiskiOsobowe { 
            zaczynajka_karty_vcf: output::NagłówkiVCF::BeginVcard.to_string(), 
            nagłówek_fn:            String::from("FN:imię"), 
            nagłówek_n:             String::from("N:nazwisko"), 
            nagłówek_zrzeszenie:    String::from("ORG:zrzeszenie"), 
            nagłówek_poczta:        String::from("EMAIL;TYPE=INTERNET:poczta"), 
            nagłówek_dalnomównik:   String::from("TEL;TYPE=CELL:dalnomównik"), 
            kończajka_karty_vcf: output::NagłówkiVCF::EndVcard.to_string(), 
        };

        let wynik = format!("{}", zapisek_sprawdzający);
        assert_eq!(wynik,
r#"BEGIN:VCARD
VERSION:2.0
FN:imięN:nazwiskoORG:zrzeszenieEMAIL;TYPE=INTERNET:pocztaTEL;TYPE=CELL:dalnomównikEND:VCARD"#);
    }

    #[test]
    fn sprawdzian_zapisywania_nazw_z_cvf() {
        let udawacz_fn: String = String::from("fn:Strzeżymir");
        let udaawacz_n: String = String::from("n:Myśliciel");

        let nagłówek_fn_do_nazwy_pliku: String = udawacz_fn.trim().chars().skip(3).collect();
        let nagłówek_n_do_nazwy_pliku: String  = udaawacz_n.trim().chars().skip(2).collect();

        let wynik: String = format!("styczność_{}_{}.cvf", nagłówek_fn_do_nazwy_pliku, nagłówek_n_do_nazwy_pliku);

        assert_eq!(wynik, "styczność_Strzeżymir_Myśliciel.cvf")

    }
}
