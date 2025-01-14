// process

use crate::{ ZapiskiOsobowe,output, input::Wejście };


impl ZapiskiOsobowe {
    pub fn new() -> Self {
        let mut zapiski = Self {
            nagłówek_fn: String::new(),
            nagłówek_n: String::new(),
            nagłówek_zrzeszenie: String::new(),
            nagłówek_poczta: String::new(),
            nagłówek_dalnomównik: String::new(),
        };

        zapiski.nagłówek_fn = zapiski.ułóż(1);
        zapiski.nagłówek_n = zapiski.ułóż(2);
        zapiski.nagłówek_zrzeszenie = zapiski.ułóż(3);
        zapiski.nagłówek_poczta = zapiski.ułóż(4);
        zapiski.nagłówek_dalnomównik = zapiski.ułóż(5);

        zapiski
    }

    fn ułóż(&self, lp: usize) -> String {
        let zapisek = self.pobierz_zapisek_z_próbami();
        format!("{}{}", lp, zapisek)
    }
}
