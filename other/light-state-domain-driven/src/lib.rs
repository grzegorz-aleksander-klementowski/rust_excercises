// TO–DO: domain-specific handling (e.g., logging, emitting an event, avoiding an expensive write);

use validate::{LightError, Validate};

pub mod validate;

pub struct Light {
    state: State,
}

#[derive(Clone, Copy, PartialEq)]
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
        let action = State::On; // for dynamic transistion it easy to use `action` as fn argument
        self.validate_repeatation(action)?;
        self.state = action;
        Ok(self)
    }

    pub fn turn_off(&mut self) -> Result<&mut Self, LightError> {
        let action = State::Off; // for dynamic transistion it easy to use `action` as fn argument
        self.validate_repeatation(action)?;
        self.state = action;
        Ok(self)
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
    fn test_new_light_starts() {
        let light = Light::new();
        assert!(light.is_off());
    }

    #[test]
    fn test_turn_off_sets_state_to_on() -> Result<(), LightError> {
        let mut light = Light::new();
        light.turn_on()?;
        assert!(light.is_on());
        Ok(())
    }

    #[test]
    fn test_turn_on_sets_state_to_off() -> Result<(), LightError> {
        let mut light = Light::new();
        light.state = State::On;
        light.turn_off()?;
        assert!(light.is_off());
        Ok(())
    }

    #[test]
    fn test_check_status_is_off_and_is_on() -> Result<(), LightError> {
        let mut light = Light::new();
        assert!(light.is_off());
        assert!(!light.is_on());

        light.turn_on()?;
        assert!(light.is_on());
        assert!(!light.is_off());
        Ok(())
    }

    #[test]
    fn test_correct_transition() -> Result<(), LightError> {
        let mut light = Light::new();
        assert!(light.is_off());

        light.turn_on()?;
        assert!(light.is_on());

        light.turn_off()?;
        assert!(light.is_off());

        light.turn_on()?;
        assert!(light.is_on());

        light.turn_off()?.turn_on()?;
        assert!(light.is_on());

        light.turn_off()?.turn_on()?.turn_off()?;
        assert!(light.is_off());

        Ok(())
    }
}
