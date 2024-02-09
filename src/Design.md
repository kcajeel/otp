# Design Document
I'm writing this to map out some issues and think through this problem. As of yet, it isn't a formal technical document and the language will be very casual.

```rs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // generate key with length >= length of plaintext
    // call encryption function (xor plaintext and key)
    // print ciphertext and key
}
```
- I want to decrypt the ciphertext with the same key to test the encryption function. This could probably be done from a test module and I plan to make one soon. It doesn't really matter where it is right now but when I get everything working I won't want the tests included in the binary.

- Rust does not support xor-ing Strings or characters. It does support xor-ing u8's. 
- This is fine as long as the characters I want to encrypt are of fixed length (e.g. ASCII). In fact, the program works right not for ASCII characters.
- Rust uses UTF-8. UTF-8 characters are variable-length, ranging from 1-4 bytes. 
- I can convert a String into a byte sequence with the `.as_bytes()` method.
- BUT, the resulting byte sequence may be longer in length than the String because it may contain multiple-byte UTF-8 characters.
- OTP relies on the key and ciphertext to be length-preserving. I need a way to reduce a UTF-8 string into its bytes, XOR that with a random key, and then convert that back into a string of the same length that will be decryptable with the key.
    - I could encrypt the string one character at a time.
    - This way, I can ensure that each ciphertext character has the same length as each plaintext character. 

    - I could "bruteforce" it by encrypting a character or string and repeating that until the ciphertext is the same length as the plaintext.
    - this probably defeats the purpose of OTP because the key will not be random this way.

