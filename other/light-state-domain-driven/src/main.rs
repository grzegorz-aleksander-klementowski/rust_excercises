/*TO-DO:
1. Walidacja przejść – np. blokada niepoprawnych kombinacji (turn_on na już
2. Wydzielenie logiki w State – implementują State::transition(action) lub e
3. Obsługa zdarzeń – logowanie / emitowanie eventów w turn_on/turn_off.
4. Testy scenariuszy – np. test pokazujący ciąg przejść w correct_transition
5. Więcej stanów – dodanie Broken, Dimmed itp., żeby ćwiczyć maszynę stanów. */

use no_boilerplate_light_state::*;

fn main() {
    let mut sample_light_1 = Light::new();
    let mut sample_light_2 = Light::new();

    // Action 1
    sample_light_1.show_status(); // should be off
    sample_light_2.show_status(); // should be off

    // Action 2
    sample_light_1.turn_on();
    sample_light_2.turn_off().turn_on().turn_off().turn_on();
    sample_light_1.show_status(); // should be on
    sample_light_2.show_status(); // should be on

    // Action 3 (turning off)
    sample_light_1.turn_off();
    sample_light_2.turn_off();
    sample_light_1.show_status(); // should be off
    sample_light_2.show_status(); // should be off
}
