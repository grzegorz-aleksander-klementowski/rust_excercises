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
