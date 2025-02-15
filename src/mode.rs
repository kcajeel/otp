use crate::error::ArgumentError;

// Mode depends on the args passed and will change the output of the program accordingly
pub enum Mode {
    Help,
    Version,
    Encrypt { plaintext: String },
    Decrypt { ciphertext: String, key: String },
}
// this impl is used to parse cli args into a Mode
impl TryFrom<&[String]> for Mode {
    type Error = ArgumentError;

    fn try_from(args: &[String]) -> Result<Self, Self::Error> {
        match args[1].as_str() {
            "-h" | "--help" => Ok(Mode::Help),
            "-v" | "--version" => Ok(Mode::Version),
            "-e" | "--encrypt" => {
                if Self::are_encryption_args_valid(args)? {
                    Ok(Mode::Encrypt {
                        plaintext: args[2].clone(), // args[2] is the plaintext when encryption mode is enabled
                    })
                } else {
                    Err(ArgumentError::InvalidArguments)
                }
            }
            "-d" | "--decrypt" => {
                if Self::are_decryption_args_valid(args)? {
                    Ok(Mode::Decrypt {
                        ciphertext: args[2].clone(), // args[2] is the ciphertext when decryption mode is enabled
                        key: args[3].clone(), // args[3] is the key when decryption mode is enabled
                    })
                } else {
                    Err(ArgumentError::InvalidArguments)
                }
            }
            _ => Err(ArgumentError::InvalidArguments),
        }
    }
}

// this impl contains functions that determine if the cli args are valid for the mode specified
impl Mode {

    pub fn are_encryption_args_valid(args: &[String]) -> Result<bool, ArgumentError> {
        if args.len() == 3 { // check args length
            Ok(true)
        } else {
            return Err(ArgumentError::InvalidArgumentNumber);
        }
    }

    pub fn are_decryption_args_valid(args: &[String]) -> Result<bool, ArgumentError> {
        if args.len() == 4 { // check args length
            Ok(true)
        } else {
            return Err(ArgumentError::InvalidArgumentNumber);
        }
    }
}