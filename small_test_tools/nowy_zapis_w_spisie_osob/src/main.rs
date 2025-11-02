use nowy_zapis_w_spisie_osob::components::output;
use nowy_zapis_w_spisie_osob::components::output::Wyjście;
use nowy_zapis_w_spisie_osob::ZapiskiOsobowe;

fn main() {
    println!("{}", output::WiadomościDoUżytkownika::Przywitanie);
    let nowy_zapisek: ZapiskiOsobowe = ZapiskiOsobowe::new();
    let sformatowany_zapisek: String = format!("{}", &nowy_zapisek);
    nowy_zapisek.wyjście_do_pliku_cvf(sformatowany_zapisek);
}
