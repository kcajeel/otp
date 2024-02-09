use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ArgumentError {
    NotEnoughArguments,
    InvalidArguments,
    TextNotASCII,
    InvalidArgumentLengths,
}
impl Error for ArgumentError {}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentError::NotEnoughArguments => write!(f, "Error: Not enough arguments."),
            ArgumentError::InvalidArguments => write!(f, "Error: Invalid arguments."),
            ArgumentError::TextNotASCII => write!(f, "Error: Non-ASCII text. This program currently only supports ASCII encoded text."),
            ArgumentError::InvalidArgumentLengths => write!(f, "Error: Key is shorter than the Ciphertext."),            
        }
    }
}
