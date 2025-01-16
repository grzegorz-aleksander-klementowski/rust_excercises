// process

use crate::{ 
    output::{ NagłówkiVCF, WiadomościDoUżytkownika },
    input::Wejście,
    ZapiskiOsobowe,
};

 // ---------------- &&*&& ---------------- \\
// here is create a new instance of VCF Card \\
impl ZapiskiOsobowe {
    pub fn new() -> Self {
        let mut zapiski = Self {
            zaczynajka_karty_vcf: NagłówkiVCF::BeginVcard.to_string(),
            nagłówek_fn: String::new(),
            nagłówek_n: String::new(),
            nagłówek_zrzeszenie: String::new(),
            nagłówek_poczta: String::new(),
            nagłówek_dalnomównik: String::new(),
            kończajka_karty_vcf: NagłówkiVCF::EndVcard.to_string(),
        };

        
        zapiski.nagłówek_fn         = zapiski.ułóż(NagłówkiVCF::FN,     WiadomościDoUżytkownika::ZapytanieOImię);
        zapiski.nagłówek_n          = zapiski.ułóż(NagłówkiVCF::N,      WiadomościDoUżytkownika::ZapytanieONazwisko);
        zapiski.nagłówek_zrzeszenie = zapiski.ułóż(NagłówkiVCF::ORG,    WiadomościDoUżytkownika::ZapytanieOZrzeszenie);
        zapiski.nagłówek_poczta     = zapiski.ułóż(NagłówkiVCF::TEL,    WiadomościDoUżytkownika::ZapytanieOPocztę);
        zapiski.nagłówek_dalnomównik= zapiski.ułóż(NagłówkiVCF::EMAIL,  WiadomościDoUżytkownika::ZapytanieONumerDalnomównika);
        

        zapiski
    }

    // formtat text to create an instance of VCF card
    fn ułóż(&self, nagłówek_vcf: NagłówkiVCF,  wiadomość: WiadomościDoUżytkownika) -> String {
        let zapisek = self.zapytanie_o_zapisek(wiadomość);
        format!("{}{}", nagłówek_vcf, zapisek)
    }

    // add a question for user to create an instance of VCF card
    fn zapytanie_o_zapisek(&self, zapytanie: WiadomościDoUżytkownika) -> String {
        print!("{}", zapytanie);
        let zapisek: String = self.pobierz_zapisek_z_próbami();
        zapisek
    }
}
