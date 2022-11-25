use std::{error::Error, fmt};

/// A custom result type for this crate
pub type EvoChessResult<T> = Result<T, EvoChessError>;

/// An error that can occur when using the library
#[derive(Debug)]
pub struct EvoChessError {
    message: String,
}

impl EvoChessError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for EvoChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for EvoChessError {}

// generic error conversion
impl From<Box<dyn Error>> for EvoChessError {
    fn from(error: Box<dyn Error>) -> Self {
        Self::new(error.to_string())
    }
}
/// from chess error
impl From<chess::Error> for EvoChessError {
    fn from(error: chess::Error) -> Self {
        Self::new(error.to_string())
    }
}
