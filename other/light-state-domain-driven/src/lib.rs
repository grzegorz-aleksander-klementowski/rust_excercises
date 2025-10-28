// TO–DO: domain-specific handling (e.g., logging, emitting an event, avoiding an expensive write);

use validate::{LightError, Validate};

pub mod validate;

pub struct Light {
    state: State,
}

#[derive(PartialEq)]
pub enum State {
    On,
    Off,
}

// Defining the light behavior
impl Light {
    pub fn new() -> Self {
        Light { state: State::Off }
    }

    pub fn turn_on(&mut self) -> Result<&mut Self, LightError> {
        let res_validation: &mut Light = self.validate_repeatation(State::On)?;
        res_validation.state = State::On;
        Ok(res_validation)
    }

    pub fn turn_off(&mut self) -> &mut Self {
        self.state = State::Off;
        self
    }

    pub fn is_on(&self) -> bool {
        self.state == State::On
    }

    pub fn is_off(&self) -> bool {
        self.state == State::Off
    }

    pub fn show_status(&self) {
        let status: &str = match self.state {
            State::On => "On",
            State::Off => "Off",
        };
        println!("status: {status}");
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::new()
    }
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

    #[test]
    fn test_check_status_is_off_and_is_on() {
        let mut light = Light::new();
        assert!(light.is_off());
        assert!(!light.is_on());

        light.turn_on();
        assert!(light.is_on());
        assert!(!light.is_off());
    }

    #[test]
    fn correct_transition() {
        // let bedroom_light = Light::new();
        // let bedroom_light = bedroom_light.turn_on().turn_off().turn_on();
    }
}
