// książnica
pub mod components;
pub mod tests;
use components::{config, input, output, process};


pub struct ZapiskiOsoboweKrotka(String, String, String, String, String);

pub struct ZapiskiOsobowe {
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_zrzeszenie: String,
    pub nagłówek_poczta: String,
    pub nagłówek_dalnomównik: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprawdzian_układania_zapisków() {
    }

}
