use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ArgumentError {
    InvalidArgumentNumber,
    InvalidArguments,
    TextNotASCII,
    InvalidArgumentLengths,
    UnsupportedDecryptionArguments,
}
impl Error for ArgumentError {}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentError::InvalidArgumentNumber => write!(f, "Error: Invalid number of arguments. Use \"-h\" or \"--help\" for usage information."),
            ArgumentError::InvalidArguments => write!(f, "Error: Invalid arguments. Use \"-h\" or \"--help\" for usage information."),
            ArgumentError::TextNotASCII => write!(f, "Error: Non-ASCII text. This program currently only supports ASCII encoded text."),
            ArgumentError::InvalidArgumentLengths => write!(f, "Error: Key is shorter than the Ciphertext."),
            ArgumentError::UnsupportedDecryptionArguments => write!(f, "Error: Please decrypt with the \"█/ \" format"),
        }
    }
}
