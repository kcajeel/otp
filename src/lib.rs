use error::ArgumentError;
use rand::Rng;
use std::{error::Error, ops::RangeInclusive};

mod error;

pub enum Mode {
    Help,
    Version,
    Encrypt { plaintext: String },
    Decrypt { ciphertext: String, key: String },
}
impl TryFrom<&[String]> for Mode {
    type Error = ArgumentError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        match value[1].as_str() {
            "-h" | "--help" => Ok(Mode::Help),
            "-v" | "--version" => Ok(Mode::Version),
            "-e" | "--encrypt" => {
                if Self::args_are_valid(value, 3)? {
                    Ok(Mode::Encrypt {
                        plaintext: value[2].clone(),
                    })
                } else {
                    Err(ArgumentError::InvalidArguments)
                }
            },
            "-d" | "--decrypt" => {
                Err(ArgumentError::Unfinished)
                // if Self::args_are_valid(value, 4)? {
                //     if value[2].len() == value[3].len() {
                //         Ok(Mode::Decrypt {
                //             ciphertext: value[2].clone(),
                //             key: value[3].clone(),
                //         })
                //     } else {
                //         Err(ArgumentError::InvalidArgumentLengths)
                //     }
                // } else {
                //     Err(ArgumentError::InvalidArguments)
                // }
            },
            _ => Err(ArgumentError::InvalidArguments),
        }
    }
}
impl Mode {
    fn args_are_valid(args: &[String], cmd_length: usize) -> Result<bool, ArgumentError> {
        if args.len() > cmd_length - 1 {
            for i in args {
                if !i.is_ascii() {
                    return Err(ArgumentError::TextNotASCII);
                }
            }
            Ok(true)
        } else {
            return Err(ArgumentError::InvalidArgumentNumber);
        }
    }
}

pub struct Program {
    mode: Mode,
}
impl Program {
    pub fn build(args: &[String]) -> Result<Self, ArgumentError> {
        if args.len() < 2 {
            return Err(ArgumentError::InvalidArgumentNumber);
        }

        Ok(Self {
            mode: Mode::try_from(args)?,
        })
    }
}

fn print_help() {
    println!("Usage: otp [args] <plaintext | ciphertext key>
    \nwhere arguments include: 
        \n\t-h, --help\tDisplay this message
        \n\t-v, --version\tDisplay version information
        \n\tOptional <-e, --encrypt> [plaintext]\tEncrypt some ASCII plaintext
        \n\t-d, --decrypt [ciphertext] [key]\tDecrypt some ASCII ciphertext with a key\n");
}

fn print_version() {
    println!("otp v{}", env!("CARGO_PKG_VERSION"));
}

pub fn run(program: Program) -> Result<(), Box<dyn Error>> {
    match program.mode {
        Mode::Help => Ok(print_help()),
        Mode::Version => Ok(print_version()),
        Mode::Encrypt { plaintext } => Ok(run_encryption(&plaintext)),
        Mode::Decrypt { ciphertext, key } => Ok(run_decryption(&ciphertext, &key)),
    }
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

    let recovered_plaintext = encrypt(&ciphertext, &key);
    println!(
        "Testing cipher, decrypting with key. Plaintext recovered: \n{:?}",
        recovered_plaintext
    );
}

fn run_decryption(ciphertext: &String, key: &String) {
    println!("Decrypting \"{}\" with key \"{}\"", ciphertext, key);

    debug_assert_eq!(ciphertext.len(), key.len(), "cipher len: {}, key len: {}", ciphertext.len(), key.len());

    let plaintext = encrypt(&blocks_to_string(ciphertext), &blocks_to_string(key));
    debug_assert_eq!(plaintext.len(), ciphertext.len());

    println!("Plaintext: \"{}\"", plaintext);
}

fn generate_key(length: usize) -> String {
    const ASCII_RANGE: RangeInclusive<u8> = 32..=126; //range of printable ASCII characters

    println!("Generating key with length {}", length);
    let key: Vec<u8> = (0..length)
        .map(|_| rand::thread_rng().gen_range(ASCII_RANGE))
        .collect();
    debug_assert_eq!(key.len(), length);

    vec_to_string(&key)
}

fn encrypt(plaintext: &String, key: &String) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    debug_assert_eq!(plaintext_bytes.len(), key.len());
    let key_bytes = key.as_bytes();
    debug_assert_eq!(plaintext_bytes.len(), key_bytes.len());

    let ciphertext: Vec<u8> = plaintext_bytes
        .iter()
        .zip(key_bytes)
        .map(|(x, y)| (x ^ y))
        .collect();

    debug_assert_eq!(ciphertext.len(), plaintext_bytes.len());
    vec_to_string(&ciphertext)
}

fn vec_to_string(vec: &Vec<u8>) -> String {
    let mut string = String::new();
    for i in 0..vec.len() {
        string.push(vec[i] as char);
    }
    assert_eq!(string.len(), vec.len());
    string
}

fn string_to_blocks(string: &String) -> String {
    let bytes = string.as_bytes();
    let mut blocks = String::new();
    
    for byte in bytes {
        for i in 0..8 {
            if byte & 1 << i != 0 {
                blocks.push('█');
            } else {
                blocks.push(' ');
            }
        }
    }
    blocks
}

fn blocks_to_string(blocks: &String) -> String {
    let bits = blocks_to_bits(blocks.to_string());
    let mut bytes: Vec<u8> = vec![];

    for byte in bits.chunks(8) {
        let mut count = 0;
        if byte.len() % 8 == 0 {
            for i in 0..8 {
                if byte[i] == 1 {
                    count += 2_u8.pow(i.try_into().unwrap());
                }
            }
            bytes.push(count);
        }
    }
    
    let string = String::from_utf8_lossy(&bytes);
    string.into_owned()
}

fn blocks_to_bits(mut blocks: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    
    for _i in 0..blocks.len() {
        if let Some(block) = blocks.pop() {
            // println!("\"{}\"", block);
            if block == '█' {
                bytes.push(1);
            } else {
                bytes.push(0);
            }
        }
    }
    bytes.reverse();
    bytes
}


#[cfg(test)]
mod test {
    use crate::{encrypt, generate_key};

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
}
