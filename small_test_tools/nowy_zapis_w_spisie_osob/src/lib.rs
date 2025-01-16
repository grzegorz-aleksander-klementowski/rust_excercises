// książnica
pub mod components;
use components::{config, input, output, process};


pub struct ZapiskiOsobowe {
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_zrzeszenie: String,
    pub nagłówek_poczta: String,
    pub nagłówek_dalnomównik: String,
}
