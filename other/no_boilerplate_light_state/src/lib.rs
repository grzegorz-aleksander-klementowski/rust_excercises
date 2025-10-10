struct Light<State> {
    state: State,
}

enum State {
    On,
    Off,
}

impl Light<State> {
    fn new() -> Self {
        Light { state: State::Off }
    }
    fn turn_on(self) -> Light<State> {
        Light { state: State::Off }
    }
    fn turn_off(self) -> Light<State> {
        Light { state: State::Off }
    }
}

pub fn correct_transition() {
    let bedroom_light = Light::new();
    let bedroom_light = bedroom_light.turn_on().turn_off().turn_on();
}
