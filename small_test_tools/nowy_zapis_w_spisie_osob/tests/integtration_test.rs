// intergration tests
use nowy_zapis_w_spisie_osob::components;

#[test]
fn sprawdzian_współpracy_z_wyjściem() {
    let wynik: String = format!(
        "{}",
        components::output::WiadomościoBłędach::WiadomośćSprawdzająca
    );
    assert_eq!(wynik, "Wiadomość sprawdzająca");
}

#[test]
fn sprawdzan_współpracy_z_zapiskami_ustawiającymi() {
    let wynik: String = format!("{}", components::config::ZAWARTOŚĆ_WIADOMOŚCI_O_BŁĘDACH[2]);
    assert_eq!(wynik, "Wiadomość sprawdzająca");
}
