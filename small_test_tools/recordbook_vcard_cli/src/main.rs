use recordbook_vcard_cli::components::output;
use recordbook_vcard_cli::components::output::Wyjście;
use recordbook_vcard_cli::ZapiskiOsobowe;

fn main() {
    println!("{}", output::WiadomościDoUżytkownika::Przywitanie);
    let nowy_zapisek: ZapiskiOsobowe = ZapiskiOsobowe::new();
    let sformatowany_zapisek: String = format!("{}", &nowy_zapisek);
    nowy_zapisek.wyjście_do_pliku_cvf(sformatowany_zapisek);
}
