/*
    This file contains the core functionality of the program. 
    I've added comments throughout to help explain what stuff is.
*/

//imports
use blocks::{blocks_to_string, string_to_blocks};
use error::ArgumentError;
use rand::Rng;
use std::{error::Error, ops::RangeInclusive};

mod error;
mod blocks;

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
                        ciphertext: blocks_to_string(&args[2]), // args[2] is the ciphertext when decryption mode is enabled
                        key: blocks_to_string(&args[3]), // args[3] is the key when decryption mode is enabled
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
    fn are_encryption_args_valid(args: &[String]) -> Result<bool, ArgumentError> {
        if args.len() == 3 { // check args length
            for i in args {
                if !i.is_ascii() { // make sure plaintext is ascii
                    return Err(ArgumentError::TextNotASCII);
                }
            }
            Ok(true)
        } else {
            return Err(ArgumentError::InvalidArgumentNumber);
        }
    }

    fn are_decryption_args_valid(args: &[String]) -> Result<bool, ArgumentError> {
        const DECRYPTION_ARGS_RANGE: std::ops::Range<usize> = 2..3; // indices of ciphertext and key

        if args.len() == 4 { // check args length
            for i in DECRYPTION_ARGS_RANGE { // iterate through ciphertext and key 
                if !args[i].chars().all(|x| x == '█' || x == ' ') { // if any characters aren't '█' or ' ', throw error
                    return Err(ArgumentError::UnsupportedDecryptionArguments);
                }
            }
            Ok(true)
        } else {
            return Err(ArgumentError::InvalidArgumentNumber);
        }
    }
}

// the Program struct contains a Mode and is used to run the actual program
pub struct Program {
    mode: Mode,
}
impl Program {
    pub fn build(args: &[String]) -> Result<Self, ArgumentError> {
        if args.len() < 2 { // check args length >= 2
            return Err(ArgumentError::InvalidArgumentNumber);
        }

        Ok(Self {
            mode: Mode::try_from(args)?, // try to parse args, propogating errors
        })
    }
}

// this function runs the program :D
pub fn run(program: Program) -> Result<(), Box<dyn Error>> {
    match program.mode { // match each mode with the execution function
        Mode::Help => Ok(print_help()),
        Mode::Version => Ok(print_version()),
        Mode::Encrypt { plaintext } => Ok(run_encryption(&plaintext)),
        Mode::Decrypt { ciphertext, key } => Ok(run_decryption(&ciphertext, &key)),
    }
}

// these next four functions are self-explanatory.
fn print_help() {
    println!(
        "\nUsage: otp [args] <plaintext | ciphertext key>
    \nWhere args include: 
        \n    -h, --help\tDisplay this message
        \n    -v, --version\tDisplay version information
        \n    -e, --encrypt [plaintext]\tEncrypt some ASCII plaintext
        \n    -d, --decrypt [ciphertext] [key]\tDecrypt some ASCII ciphertext with a key\n"
    );
}

fn print_version() {
    println!("\notp v{}\n
    Written by Jack Lee\n
    Source: https://github.com/kcajeel/otp\n", env!("CARGO_PKG_VERSION"));
}

fn run_encryption(plaintext: &String) {
    let key = generate_key(plaintext.len());

    println!(
        "Encrypting \"{}\" with key: \n\"{}\"",
        plaintext,
        string_to_blocks(&key)
    );
    debug_assert_eq!(key.len(), plaintext.len());

    let ciphertext = encrypt(plaintext, &key);
    debug_assert_eq!(ciphertext.len(), plaintext.len());
    println!("Ciphertext: \n\"{}\"", string_to_blocks(&ciphertext));
}

fn run_decryption(ciphertext: &String, key: &String) {
    println!("Decrypting \"{}\" with key \"{}\"", ciphertext, key);

    debug_assert_eq!(
        ciphertext.len(),
        key.len(),
        "cipher len: {}, key len: {}",
        ciphertext.len(),
        key.len()
    );

    let plaintext = encrypt(&ciphertext, &key);
    debug_assert_eq!(plaintext.len(), ciphertext.len());

    println!("Plaintext: \"{}\"", plaintext);
}

// generate a pseudorandom key of a specified length
fn generate_key(length: usize) -> String {
    const ASCII_RANGE: RangeInclusive<u8> = 32..=126; //range of printable ASCII characters

    println!("Generating key with length {}", length);
    let key: Vec<u8> = (0..length)
        .map(|_| rand::thread_rng().gen_range(ASCII_RANGE))
        .collect(); // add random numbers in ASCII_RANGE to the key Vec
    debug_assert_eq!(key.len(), length);

    vec_to_string(&key)
}

fn encrypt(plaintext: &String, key: &String) -> String {
    // convert key and plaintext to bytes to xor them
    let plaintext_bytes = plaintext.as_bytes();
    debug_assert_eq!(plaintext_bytes.len(), key.len());
    let key_bytes = key.as_bytes();
    debug_assert_eq!(plaintext_bytes.len(), key_bytes.len());

    // create an iterator from ciphertext, zip it with the key's bytes to form an iterator of tuples from each Vec
    // then xor each element and collect them into one Vec. This functional stuff is pretty handy.
    let ciphertext: Vec<u8> = plaintext_bytes
        .iter()
        .zip(key_bytes)
        .map(|(x, y)| (x ^ y))
        .collect();

    debug_assert_eq!(ciphertext.len(), plaintext_bytes.len());
    vec_to_string(&ciphertext)
}

// convert a vec to a string. I think String::from_utf8() would work too and I might get rid of this
fn vec_to_string(vec: &Vec<u8>) -> String {
    let mut string = String::new();
    for i in 0..vec.len() {
        string.push(vec[i] as char);
    }
    debug_assert_eq!(string.len(), vec.len());
    string
}

// Tests. These are explanatory by their names
#[cfg(test)]
mod test {
    use crate::{encrypt, generate_key, Mode};

    #[test]
    fn test_encryption() {
        let plaintext = "testing".to_string();
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(&plaintext, &key);

        let recovered_plaintext = encrypt(&ciphertext, &key);
        assert_eq!(recovered_plaintext, plaintext);
    }

    #[test]
    fn test_decryption() {
        let plaintext = "testing".to_string();
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(&plaintext, &key);
        assert_eq!(encrypt(&ciphertext, &key), plaintext);
    }

    #[test]
    fn test_encryption_valid_args() {
        let args = [
            "command".to_owned(),
            "-e".to_owned(),
            "testing args".to_owned(),
        ];
        assert!(Mode::are_encryption_args_valid(&args).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_encryption_invalid_arg_number() {
        let args = [
            "command".to_owned(),
            "-e".to_owned(),
            "testing".to_owned(),
            "args".to_owned(),
        ];
        let _test = Mode::are_decryption_args_valid(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_encryption_non_ascii_args() {
        let args = ["command".to_owned(), "-e".to_owned(), "你好".to_owned()];
        let _test = Mode::are_decryption_args_valid(&args).unwrap();
    }

    #[test]
    fn test_decryption_valid_args() {
        let args = [
            "command".to_owned(),
            "-d".to_owned(),
            "█           █    █ █ █   █  █   ".to_owned(),
            "█ █ ███ █ █ ███ █  ██ █  ██  ██ ".to_owned(),
        ];
        assert!(Mode::are_decryption_args_valid(&args).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_decryption_invalid_arg_number() {
        let args = [
            "command".to_owned(),
            "-d".to_owned(),
            "█           █    █ █ █   █  █   ".to_owned(),
            "█ █ ███ █ █ ███ █  ██ █  ██  ██ ".to_owned(),
            "█ ███   ██████    █ █".to_owned(),
        ];
        let _test = Mode::are_decryption_args_valid(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decryption_unsupported_args() {
        let args = [
            "command".to_owned(),
            "-d".to_owned(),
            "█           █ a  █ █ █   █  █   ".to_owned(),
            "this is invalid".to_owned(),
        ];

        let _test = Mode::are_decryption_args_valid(&args).unwrap();
    }
}
