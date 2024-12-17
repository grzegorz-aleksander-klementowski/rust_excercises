// książnica

mod output;
mod input;
mod config;

pub struct ZapiskiOsobowe<'a> {
    pub nagłówek_fn: String,
    pub nagłówek_n: String,
    pub nagłówek_email: String,
    pub nagłówek_dalnomównik: String,
}


#[cfg(test)]
mod tests {
    use super::*;

}
