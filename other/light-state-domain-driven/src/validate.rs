use crate::{Light, State};
// Trait to validate functions in the program especially Light structure functions
pub trait Validate {
    type Ok<'a_type>
    where
        Self: 'a_type;
    type Err;

    // Using GAT (Generic Associated Type)
    fn validate_repeatation<'a_fn>(
        &'a_fn mut self,
        action: State,
    ) -> Result<Self::Ok<'a_fn>, Self::Err>; // in Self::Ok<'a_fn lifetimes 'a_fn will subsitute 'a_type (subsitution of lifetimes parameters)
}

// To turn on the light, the light need to be turned off and the intention is to
// …turn the light.
// To turn off the light, the light need to be turned on and the intention of the
// …action is turn the light off.
impl Validate for Light {
    type Ok<'a_type> = &'a_type mut Self;
    type Err = LightError;

    fn validate_repeatation<'a_fn>(
        &'a_fn mut self,
        action: State,
    ) -> Result<Self::Ok<'a_fn>, Self::Err> {
        match (self.is_on(), action) {
            (false, State::On) => Ok(self),
            (true, State::Off) => Ok(self),
            (false, State::Off) => Err(LightError::AlredyOff),
            (true, State::On) => Err(LightError::AlredyOn),
        }
    }
}

// enumeration of possible LightErrors
pub enum LightError {
    AlredyOn,
    AlredyOff,
}
