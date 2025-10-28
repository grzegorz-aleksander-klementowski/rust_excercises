// TO–DO: domain-specific handling (e.g., logging, emitting an event, avoiding an expensive write);

use std::fmt::Display;

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
}

impl Default for Light {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status: &str = match self.state {
            State::Off => "Off",
            State::On => "On",
        };
        write!(f, "status: {status}")
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
    fn test_error_handling_if_light_allready_off() -> Result<(), LightError> {
        let mut light = Light::new(); // The default state of a new light is off.
        let result_light = light.turn_off();
        assert!(matches!(result_light, Err(LightError::AlredyOff)));
        Ok(())
    }

    #[test]
    fn test_error_handling_if_light_allready_on() -> Result<(), LightError> {
        let mut light = Light::new(); // The default state of a new light is off.
        light.turn_on()?;
        let result_light = light.turn_on();

        assert!(matches!(result_light, Err(LightError::AlredyOn)));
        Ok(())
    }

    #[test]
    fn test_show_status() -> Result<(), LightError> {
        let light_turned_off = Light::new();
        let mut light_turned_on = Light::new();
        light_turned_on.turn_on()?;

        assert_eq!(format!("{light_turned_off}"), "status: Off");
        assert_eq!(format!("{light_turned_on}"), "status: On");
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
