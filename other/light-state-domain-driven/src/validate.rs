use crate::{Light, State};

trait Validate {
    type Ok<'a_type>
    where
        Self: 'a_type;
    type Err;

    fn validate_repeatation<'a_fn>(
        &'a_fn self,
        action: State,
    ) -> Result<Self::Ok<'a_fn>, Self::Err>; // in Self::Ok<'a_fn lifetimes 'a_fn will subsitute 'a_type (subsitution of lifetimes parameters)
}

impl Validate for Light {
    type Ok<'a_type> = Self;
    type Err = LightError;

    fn validate_repeatation<'a_fn>(
        &'a_fn self,
        action: State,
    ) -> Result<Self::Ok<'a_fn>, Self::Err> {
        if self.is_off() && action == State::On {
            todo!()
        } else {
            todo!()
        }
    }
}

pub enum LightError {
    AlredyOn,
    AlredyOff,
}
