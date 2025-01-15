// process

use crate::{ ZapiskiOsobowe,output::WiadomościDoUżytkownika, input::Wejście };


impl ZapiskiOsobowe {
    pub fn new() -> Self {
        let mut zapiski = Self {
            nagłówek_fn: String::new(),
            nagłówek_n: String::new(),
            nagłówek_zrzeszenie: String::new(),
            nagłówek_poczta: String::new(),
            nagłówek_dalnomównik: String::new(),
        };

        zapiski.nagłówek_fn = zapiski.ułóż(WiadomościDoUżytkownika::ZapytanieOImię);
        zapiski.nagłówek_n = zapiski.ułóż(WiadomościDoUżytkownika::ZapytanieONazwisko);
        zapiski.nagłówek_zrzeszenie = zapiski.ułóż(WiadomościDoUżytkownika::ZapytanieOZrzeszenie);
        zapiski.nagłówek_poczta = zapiski.ułóż(WiadomościDoUżytkownika::ZapytanieOPocztę);
        zapiski.nagłówek_dalnomównik = zapiski.ułóż(WiadomościDoUżytkownika::ZapytanieONumerDalnomównika);

        zapiski
    }

    fn ułóż(&self, wiadomość: WiadomościDoUżytkownika) -> String {
        let zapisek = self.pobierz_zapisek_z_próbami();
        format!("{}{}", wiadomość, zapisek)
    }
}
