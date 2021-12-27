use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ScanError {}

impl ScanError {
    pub fn raise() -> Self {
        Self {}
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScanError.",)
    }
}

impl Error for ScanError {}
