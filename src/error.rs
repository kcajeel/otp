use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ArgumentError {
    InvalidArgumentNumber,
    InvalidArguments,
    InvalidArgumentLengths,
}
impl Error for ArgumentError {}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentError::InvalidArgumentNumber => write!(f, "Error: Invalid number of arguments. Use \"-h\" or \"--help\" for usage information."),
            ArgumentError::InvalidArguments => write!(f, "Error: Invalid arguments. Use \"-h\" or \"--help\" for usage information."),
            ArgumentError::InvalidArgumentLengths => write!(f, "Error: Key is shorter than the Ciphertext."),
        }
    }
}
