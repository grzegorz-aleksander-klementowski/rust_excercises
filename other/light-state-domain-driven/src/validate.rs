use std::error::Error;

use crate::Light;

trait Validate {
    type Ok;
    type Err;

    fn validate_repeatation(&self) -> Result<Self::Ok, Self::Err>;
}
