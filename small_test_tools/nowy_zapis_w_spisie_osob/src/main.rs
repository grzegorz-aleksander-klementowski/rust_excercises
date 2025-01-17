use nowy_zapis_w_spisie_osob::ZapiskiOsobowe;
use nowy_zapis_w_spisie_osob::components::{ output, input, process };

fn main() {
    println!("{}", output::WiadomościDoUżytkownika::Przywitanie);
    let nowy_zapisek: ZapiskiOsobowe = ZapiskiOsobowe::new();
}
