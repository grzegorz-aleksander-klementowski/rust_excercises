// TO–DO: domain-specific handling (e.g., logging, emitting an event, avoiding an expensive write);

pub struct Light<T> {
    state: T,
}

pub enum State {
    On,
    Off,
}

impl Light<State> {
    pub fn new() -> Self {
        Light { state: State::Off }
    }

    pub fn turn_on(&mut self) -> &mut Self {
        self.state = State::On;
        self
    }

    pub fn turn_off(&mut self) -> &mut Self {
        self.state = State::Off;
        self
    }

    pub fn show_status<'a>(&self) {
        let status: &str = match self.state {
            State::On => "On",
            State::Off => "Off",
        };
        println!("status: {status}");
    }
}

pub fn correct_transition() {
    // let bedroom_light = Light::new();
    // let bedroom_light = bedroom_light.turn_on().turn_off().turn_on();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_light_starts() {
        let light = Light::new();
        assert!(matches!(light.state, State::Off));
    }

    #[test]
    fn turn_off_sets_state_to_on() {
        let mut light = Light::new();
        light.turn_on();
        assert!(matches!(light.state, State::On));
    }

    #[test]
    fn turn_on_sets_state_to_off() {
        let mut light = Light::new();
        light.turn_off();
        assert!(matches!(light.state, State::Off));
    }
}
