use std::error::Error;

use crate::{Light, State};

trait Validate {
    type Ok<'a>;
    type Err;

    fn validate_repeatation(&self, action: State) -> Result<Self::Ok, Self::Err>;
}

impl Validate for Light {
    type Ok<'a> = &'a Self;
    type Err = LightError;

    fn validate_repeatation(&self, action: State) -> Result<Self::Ok, Self::Err> {
        if self.is_on() && action == State::Off {
            todo!()
        }
    }
}

enum LightError {
    AlredyOn,
    AlredyOff,
}
