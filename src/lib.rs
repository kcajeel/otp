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

    println!(
        "Encrypting \"{:?}\" with key: \n{:#0X?}",
        config.plaintext, key
    );
    assert_eq!(key.len(), config.plaintext.len());

    let ciphertext = encrypt(&config.plaintext, &key);
    println!("Ciphertext: \n{:0X?}", ciphertext);
    assert_eq!(ciphertext.len(), config.plaintext.len());

    let recovered_plaintext = encrypt(&ciphertext, &key);
    println!(
        "Testing cipher, decrypting with key. Plaintext recovered: \n{:?}",
        recovered_plaintext
    );

    Ok(())
}

fn generate_key(length: usize) -> String {
    //make this a String
    println!("generateing key with length {}", length);
    let mut key: Vec<u8> = Vec::new();
    for _i in 0..length {
        key.push(rand::thread_rng().gen());
    }
    assert_eq!(key.len(), length);
    vec_to_string(key)
}

fn encrypt(plaintext: &String, key: &String) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();

    assert_eq!(plaintext_bytes.len(), key_bytes.len());

    let mut ciphertext: Vec<u8> = vec![];
    for i in 0..plaintext_bytes.len() {
        ciphertext.push(plaintext_bytes[i] ^ key_bytes[i]);
    }

    vec_to_string(ciphertext)
}

fn vec_to_string(vec: Vec<u8>) -> String { //what am i doing wrong :(
    let mut string = String::new();
    for i in 0..vec.len() {
        string.push(vec[i] as char);
    }
    assert_eq!(string.len(), vec.len());
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
