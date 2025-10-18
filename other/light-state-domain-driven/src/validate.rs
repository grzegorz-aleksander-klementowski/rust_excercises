use std::error::Error;

trait Validate {
    type Ok;
    type Err;

    fn validate_repeatation() -> Result<Self::Ok, Self::Err>;
}
