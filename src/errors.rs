use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct RustyError(pub String);

impl Display for RustyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program responded with the following error: {}", self.0)
    }
}

impl Error for RustyError {}