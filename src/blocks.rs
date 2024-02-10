/*
    This file contains code for encoding and decoding Strings into the block format.
    Most of the functions are self explanatory, but I've provided comments to help their legibility.
*/

pub fn string_to_blocks(string: &String) -> String {
    let bytes = string.as_bytes();
    let mut blocks = String::new();

    //iterate through the string's bytes, bit-by-bit
    for byte in bytes {
        for i in 0..8 {
            // add '█' to blocks if byte[i] is 1, add ' ' is 0
            if byte & 1 << i != 0 {
                blocks.push('█');
            } else {
                blocks.push(' ');
            }
        }
    }
    blocks
}

pub fn blocks_to_string(blocks: &String) -> String {
    let bits = blocks_to_bits(blocks.to_string());
    let mut bytes: Vec<u8> = vec![];

    // split bits into 8-bit chunks (bytes)
    for byte in bits.chunks(8) {
        let mut count = 0;
        if byte.len() % 8 == 0 {
            for i in 0..8 {
                // if there's a 1, add its binary value into the count
                if byte[i] == 1 {
                    // this .unwrap() is safe because the max value, 2^7 = 128, which will not overflow a u8
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
        // iterate through the blocks string and get the binary by decoding the character sequence
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
