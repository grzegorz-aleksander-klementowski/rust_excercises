use crate::KosztCzęściRowerowych;
use std::fmt;

impl fmt::Display for KosztCzęściRowerowych {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
