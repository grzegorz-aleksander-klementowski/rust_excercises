/*TO-DO:
1. Walidacja przejść – np. blokada niepoprawnych kombinacji (turn_on na już
2. Wydzielenie logiki w State – implementują State::transition(action) lub e
3. Obsługa zdarzeń – logowanie / emitowanie eventów w turn_on/turn_off.
4. Testy scenariuszy – np. test pokazujący ciąg przejść w correct_transition
5. Więcej stanów – dodanie Broken, Dimmed itp., żeby ćwiczyć maszynę stanów. */

use light_state_domain_driven::validate::LightError;

fn main() -> Result<(), LightError> {
    run()?;
    Ok(())
}

fn run() -> Result<(), LightError> {
    // A training program. Usage by `cargo test`.
    println!("A training program. Usage by `cargo test`.");
    Ok(())
}
