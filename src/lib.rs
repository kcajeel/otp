/*
    This file contains the core functionality of the program.
    I've added comments throughout to help explain what stuff is.
*/

use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
//imports
use error::ArgumentError;
use mode::Mode;
use rand::Rng;
use std::{error::Error, ops::RangeInclusive};

mod error;
mod mode;

// the Program struct contains a Mode and is used to run the actual program
pub struct Program {
    mode: Mode,
}
impl Program {
    pub fn build(args: &[String]) -> Result<Self, ArgumentError> {
        if args.len() < 2 {
            // check args length >= 2
            return Err(ArgumentError::InvalidArgumentNumber);
        }

        Ok(Self {
            mode: Mode::try_from(args)?, // try to parse args, propogating errors
        })
    }
}

// this function runs the program :D
pub fn run(program: Program) -> Result<(), Box<dyn Error>> {
    match program.mode {
        // match each mode with the execution function
        Mode::Help => Ok(print_help()),
        Mode::Version => Ok(print_version()),
        Mode::Encrypt { plaintext } => Ok(run_encryption(&plaintext)),
        Mode::Decrypt { ciphertext, key } => Ok(run_decryption(&ciphertext, &key)?),
    }
}

// these next four functions are self-explanatory.
fn print_help() {
    println!(
        "Usage: otp [args] <plaintext | ciphertext key>
    Where args include: 
    -h, --help\t\t\t\tDisplay this message
    -v, --version\t\t\tDisplay version information
    -e, --encrypt [plaintext]\t\tEncrypt plaintext
    -d, --decrypt [ciphertext] [key]\tDecrypt ciphertext with a key"
    );
}

fn print_version() {
    println!(
        "otp v{}
    Written by Jack Lee
    Source: https://github.com/kcajeel/otp",
        env!("CARGO_PKG_VERSION")
    );
}

fn run_encryption(plaintext: &String) {
    let base64_plaintext = BASE64_STANDARD_NO_PAD.encode(plaintext);

    let mut key = generate_key(base64_plaintext.len());

    println!("Encrypting \"{}\" ...", plaintext);

    let mut ciphertext = encrypt(&base64_plaintext, &key);
    debug_assert_eq!(ciphertext.len(), base64_plaintext.len());

    ciphertext = BASE64_STANDARD_NO_PAD.encode(ciphertext);
    key = BASE64_STANDARD_NO_PAD.encode(key);
    
    println!("Ciphertext: \n\"{}\"", ciphertext);
    println!("Key: \"{}\"", key);
    println!("To decrypt: \notp -d \"{}\" \"{}\"", ciphertext, key);
}

fn run_decryption(b64_ciphertext: &String, b64_key: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Decrypting \"{}\" with key \"{}\"", b64_ciphertext, b64_key);

    let ciphertext = handle_b64_decoding(b64_ciphertext)?;
    let key = handle_b64_decoding(b64_key)?;
    // using unicode width because some utf-8 chars are multiple bytes
    debug_assert_eq!(
        ciphertext.len(),
        key.len(),
        "cipher len: {}, key len: {}\nciphertext: {}\nkey: {}",
        ciphertext.len(),
        key.len(),
        ciphertext,
        key
    );

    let b64_plaintext = encrypt(&ciphertext, &key);
    debug_assert_eq!(b64_plaintext.len(), ciphertext.len());

    let plaintext = handle_b64_decoding(&b64_plaintext)?;
    println!("Plaintext: \"{}\"", plaintext);

    Ok(())
}

// generate a pseudorandom key of a specified length
fn generate_key(length: usize) -> String {
    const ASCII_RANGE: RangeInclusive<u8> = 32..=126; //range of printable ASCII characters

    println!("Generating key with length {}", length);
    let key: Vec<u8> = (0..length)
        .map(|_| rand::thread_rng().gen_range(ASCII_RANGE))
        .collect(); // add random numbers in ASCII_RANGE to the key Vec
    debug_assert_eq!(key.len(), length);

    String::from_utf8(key).expect("error parsing utf-8 (in genkey)")
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
    String::from_utf8(ciphertext).expect("error parsing utf-8 (encrypt)")
}

fn handle_b64_decoding(b64_string: &String) -> Result<String, Box<dyn Error>> {
    match BASE64_STANDARD_NO_PAD.decode(&b64_string) {
        Ok(decoded_vec) => match String::from_utf8(decoded_vec) {
            Ok(decoded_string) => Ok(decoded_string),
            Err(e) => {
                eprintln!(
                    "Error encoding \"{}\" into a UTF-8 String: {}",
                    b64_string, e
                );
                Err("UTF-8 conversion failed.".into())
            }
        },

        Err(e) => {
            eprintln!("Error decoding \"{}\" from base64: {}", b64_string, e);
            Err("Base64 decoding failed. ".into())
        }
    }
}

// Tests. These are explanatory by their names
#[cfg(test)]
mod test {
    use crate::{run_decryption, run_encryption, Mode};
    use base64::prelude::{Engine, BASE64_STANDARD};

    use crate::{encrypt, generate_key};

    // this test quickly generates new ciphertext and keys
    #[test]
    fn test_run_encryption() {
        run_encryption(&"👋testing👋".to_string());
    }

    #[test]
    fn test_run_decryption() {
        let result = run_decryption(
            &String::from("MiADCCQxFl9XEiRWWjAeTzpMQ0w"),
            &String::from("a05JYEV+RWY4XXM6LGNaOFR5Kjg"),
        ).unwrap();
        assert_eq!(result, ())
    }

    // This test has helped me debug the decryption step
    // It seems to work with b64-encoded ciphertext instead of utf-8 ciphertext
    // I'll implement this change in the run_encryption function
    #[test]
    fn test_decryption_with_b64_ciphertext() {
        let ciphertext = String::from_utf8(
            BASE64_STANDARD
                .decode("XRYaASsSay08QTozHgsSMBpWOQA=")
                .unwrap(),
        )
        .unwrap();
        let b64_plaintext = encrypt(&ciphertext, &String::from(r"e\1SB!9A_rhC|ftGtc~L"));
        println!("b64_plaintext: \"{}\"", b64_plaintext);
        println!(
            "decoded plaintext: \n\"{}\"",
            String::from_utf8(BASE64_STANDARD.decode(b64_plaintext).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_encryption() {
        let plaintext = "testing".to_string();
        let key = generate_key(plaintext.len());
        let ciphertext = encrypt(&plaintext, &key);

        let recovered_plaintext = encrypt(&ciphertext, &key);
        assert_eq!(recovered_plaintext, plaintext);
    }

    #[test]
    fn test_b64_encryption_ascii() {
        let plaintext = "testing in ascii".to_string();
        let b64_plaintext = BASE64_STANDARD.encode(&plaintext);
        let key = generate_key(b64_plaintext.len());
        let ciphertext = encrypt(&b64_plaintext, &key);
        println!(
            "Plaintext: {}\nB64 plaintext: {}\nKey: {}\nCiphertext: {}",
            plaintext, b64_plaintext, key, ciphertext
        );

        let b64_recovered_plaintext = encrypt(&ciphertext, &key);
        assert_eq!(b64_recovered_plaintext.len(), ciphertext.len());

        let recovered_plaintext =
            String::from_utf8(BASE64_STANDARD.decode(&b64_recovered_plaintext).unwrap()).unwrap();

        println!(
            "b64 recovered plaintext: {}\nrecovered plaintext: {}",
            b64_recovered_plaintext, recovered_plaintext
        );
        assert_eq!(recovered_plaintext, plaintext);
    }

    #[test]
    fn test_b64_encryption_utf8() {
        let plaintext = "你好👋!".to_string();
        let b64_plaintext = BASE64_STANDARD.encode(&plaintext);
        let key = generate_key(b64_plaintext.len());
        let ciphertext = encrypt(&b64_plaintext, &key);
        println!(
            "Plaintext: \"{}\"\nB64 plaintext: \"{}\"\nKey: \"{}\"\nCiphertext: \"{}\"",
            plaintext, b64_plaintext, key, ciphertext
        );

        let b64_recovered_plaintext = encrypt(&ciphertext, &key);
        assert_eq!(b64_recovered_plaintext.len(), ciphertext.len());

        let recovered_plaintext =
            String::from_utf8(BASE64_STANDARD.decode(&b64_recovered_plaintext).unwrap()).unwrap();

        println!(
            "b64 recovered plaintext: \"{}\"\nrecovered plaintext: \"{}\"",
            b64_recovered_plaintext, recovered_plaintext
        );
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
        let _test = Mode::are_encryption_args_valid(&args).unwrap();
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
}
