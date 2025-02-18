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
FN:imię
N:nazwisko
ORG:zrzeszenie
EMAIL;TYPE=INTERNET:poczta
TEL;TYPE=CELL:dalnomównik
END:VCARD"#);
    }
}
