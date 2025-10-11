// TO–DO: domain-specific handling (e.g., logging, emitting an event, avoiding an expensive write);

pub struct Light<State> {
    state: State,
}

pub enum State {
    On,
    Off,
}

impl Light<State> {
    pub fn new() -> Self {
        Light { state: State::Off }
    }
    fn turn_on(&self) -> Light<State> {
        Light { state: State::Off }
    }
    fn turn_off(&self) -> Light<State> {
        Light { state: State::Off }
    }

    fn show_status<'a>(&self) {
        let status: &str = match self.state {
            State::On => "On",
            State::Off => "Off",
        };
        println!("status: {status}");
    }
}

pub fn correct_transition() {
    let bedroom_light = Light::new();
    let bedroom_light = bedroom_light.turn_on().turn_off().turn_on();
}
