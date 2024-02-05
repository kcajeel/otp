use std::error::Error;

use rand::Rng;


pub struct Config {
    pub plaintext: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments. Use \"-h\" or \"--help\" for usage information.");
        }

        let plaintext = args[1].clone();
        Ok(Self { plaintext, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //what could go wrong? 
    //Not much. I just encrypt user input. Pretty simple.
    //And i generate the key. Pretty simple.
    //This kind of thinking definitely leads to errors. Neither of those are Actually simple..
    
    let key = generate_key(config.plaintext.len());

    println!("Encrypting \"{}\" with key: \n{:#0X?}", config.plaintext, key);
    assert_eq!(key.len(), config.plaintext.len());

    let ciphertext = encrypt(&config.plaintext, &key);
    println!("Ciphertext: \n{:0X?}", ciphertext);
    assert_eq!(ciphertext.len(), config.plaintext.len());
    
    let ciphertext_str = vec_to_string(ciphertext);

    let recovered_plaintext = encrypt(&ciphertext_str, &key);
    println!("Testing cipher, decrypting with key. Plaintext recovered: \n{:?}", recovered_plaintext);
    
    Ok(())
}

fn generate_key(length: usize) -> Vec<u8> {
    println!("generateing key with length {}", length);
    let mut key: Vec<u8> = vec![];
    for _i in 0..length {
        key.push(rand::thread_rng().gen());
    }
    key
}

fn encrypt(plaintext: &str, key: &Vec<u8>) -> Vec<u8> {
    let plaintext_bytes = plaintext.as_bytes();
    assert_eq!(plaintext.len(), plaintext_bytes.len());
    assert_eq!(plaintext.len(), key.len());
    let mut ciphertext: Vec<u8> = vec![];

    if key.len() != plaintext_bytes.len() {
        panic!("Key length must match plaintext length: key_len={}, plaintext_len={}", key.len(), plaintext_bytes.len());
    }

    for i in 0..plaintext_bytes.len() {
        if i >= key.len() {
            panic!("Index out of bounds: i={}, key_len={}", i, key.len());
        }
        ciphertext.push(plaintext_bytes[i] ^ key[i]);
    }

    ciphertext
}

fn vec_to_string(vec: Vec<u8>) -> String {
    let mut string = String::with_capacity(vec.len());

    for i in 0..vec.len() {
        string.push(vec[i] as char);
    }
    assert_eq!(vec.len(), string.len());
    string
}

    //otp this thing
    //encrypting with key:
    //key
    //ciphertext: 
    //ciphertext
    //cat foo.txt < otp
    //cat foo.txt | otp > bar.txt
    //not in my control, i think these will just work.