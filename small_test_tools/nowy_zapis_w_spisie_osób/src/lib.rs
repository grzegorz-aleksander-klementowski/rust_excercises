// książnica

mod output;
mod input;
mod config;
mod process;

pub struct ZapiskiOsobowe {
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_email: String,
    pub nagłówek_dalnomównik: String,
}

impl ZapiskiOsobowe {
    pub fn new() -> Self {
        Self {
            nagłówek_fn: String::new(),
            nagłówek_n: String::new(),
            nagłówek_email: String::new(),
            nagłówek_dalnomównik: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
