//! # b64
//! A base64 string encoder and decoder library written in Rust with zero dependencies.
//! 
//! ## Getting Started
//!
//! ### Encoding data
//! 
//! ```no_run
//! use b64::encode_data;
//! 
//! fn main() {
//!   let input: String = String::from("Hello World!");
//! 
//!   let enc: String = encode_data(input.as_bytes());
//!   println!("Encoded Data: {}", enc);
//! }
//! ```
//! 
//! ### Decoding data
//! 
//! ```no_run
//! use b64::decode_data;
//! 
//! fn main() {
//!   let input: String = String::from("SGVsbG8gV29ybGQh");
//!   let dec: Vec<u8> = decode_data(&input).expect("Invalid base64 string!");
//!   let string: String = String::from_utf8(dec).expect("The computed bytes are not UTF-8!");
//! 
//!   println!("Decoded Data: {}", string);
//! }
//! ```
//! 
//! ### Check if a string is valid base64 string
//! 
//! ```no_run
//! use b64::is_valid_b64;
//! 
//! fn main() {
//!     let input1: String = String::from("ABCDEFGH");
//!     let input2: String = String::from("ABSJF$#A");
//! 
//!     println!("Is '{}' a valid base64 string: {}", input1, is_valid_b64(&input1));
//!     println!("Is '{}' a valid base64 string: {}", input2, is_valid_b64(&input2));
//! }
//! ```

use core::fmt;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, B64Error>;

#[derive(Debug, Clone)]
pub enum B64Error {
    InvalidCharacters,
    InvalidLength
}

impl fmt::Display for B64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            B64Error::InvalidCharacters => {
                write!(f, "The base64 string contains invalid characters!")
            },
            B64Error::InvalidLength => {
                write!(f, "The length of the base64 string is invalid!")
            }
        }
    }
}

/// Encodes the given byte slice(`&[u8]`) into a base64 `String`.
pub fn encode_data(data: &[u8]) -> String {
    let mut res: String = String::new();

    for chunk in data.chunks(3) {
        let st: String = encode_triplet(chunk);
        res.push_str(st.as_str());
    }

    res
}

fn encode_triplet(chars: &[u8]) -> String {
    const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let mut idxs: Vec<usize> = Vec::new();

    match chars.len() {
        1 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push(((chars[0] & 0b0000_0011) << 4) as usize);
            idxs.push(64);
            idxs.push(64);
        },
        2 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push((((chars[0] & 0b0000_0011) << 4) + ((chars[1] & 0b1111_0000) >> 4)) as usize);
            idxs.push(((chars[1] & 0b0000_1111) << 2) as usize);
            idxs.push(64);
        },
        3 => {
            idxs.push(((chars[0] & 0b1111_1111) >> 2) as usize);
            idxs.push((((chars[0] & 0b0000_0011) << 4) + ((chars[1] & 0b1111_0000) >> 4)) as usize);
            idxs.push((((chars[1] & 0b0000_1111) << 2) + ((chars[2] & 0b1100_0000) >> 6)) as usize);
            idxs.push((chars[2] & 0b0011_1111) as usize);
        },
        _ => ()
    }

    let mut res: String = String::new();
    for idx in idxs {
        res.push(CHARACTERS[idx] as char);
    }

    res
}

/// Maps an ASCII byte to its base64 sextet value, or `255` if it isn't a
/// base64 alphabet character. Built once at compile time to avoid
/// reconstructing a lookup table on every call.
const DECODE_TABLE: [u8; 256] = {
    const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut table = [255u8; 256];
    let mut i = 0;
    while i < CHARACTERS.len() {
        table[CHARACTERS[i] as usize] = i as u8;
        i += 1;
    }
    table
};

fn decode_quadruplet(data: &str) -> Vec<u8> {
    let mut mdata: Vec<u8> = Vec::new();

    for &b in data.as_bytes() {
        let val = DECODE_TABLE[b as usize];
        if val == 255 {
            break;
        }
        mdata.push(val);
    }

    let mut v: Vec<u8> = Vec::new();
    match mdata.len() {
        2 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
        },
        3 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
            v.push(((mdata[1] & 0b0000_1111) << 4) + ((mdata[2] & 0b0011_1100) >> 2));
        },
        4 => {
            v.push(((mdata[0] & 0b0011_1111) << 2) + ((mdata[1] & 0b0011_0000) >> 4));
            v.push(((mdata[1] & 0b0000_1111) << 4) + ((mdata[2] & 0b0011_1100) >> 2));
            v.push(((mdata[2] & 0b0000_0011) << 6) + mdata[3]);
        },
        _ => ()
    }

    v
}

/// Decodes the input string into a byte array `Vec<u8>`.
pub fn decode_data(data: &str) -> Result<Vec<u8>> {
    if data.len() % 4 != 0 {
        return Err(B64Error::InvalidLength);
    }

    if !is_valid_b64(data) {
        return Err(B64Error::InvalidCharacters);
    }

    // '=' is only valid as a trailing padding character (at most 2 of them).
    let trimmed = data.trim_end_matches('=');
    let padding_len = data.len() - trimmed.len();
    if padding_len > 2 || trimmed.contains('=') {
        return Err(B64Error::InvalidCharacters);
    }

    let mut decoded_data: Vec<u8> = Vec::new();

    for chunk in data.as_bytes().chunks(4) {
        let quad = std::str::from_utf8(chunk).unwrap();
        let mut res: Vec<u8> = decode_quadruplet(quad);
        decoded_data.append(&mut res);
    }

    Ok(decoded_data)
}

/// Returns `true` if the given string is valid base64 string
pub fn is_valid_b64(inp: &str) -> bool {
    if inp.len() % 4 != 0 {
        return false;
    }
    
    for ch in inp.bytes() {
        // Uppercase Alphabets: 65-90
        // Lowercase Alphabets: 97-122
        // Numbers: 48-57
        // +: 43
        // /: 47
        // =: 61
        if !((ch >= 65 && ch <= 90) || (ch >= 97 && ch <= 122) || (ch >= 48 && ch <= 57) || ch == 43 || ch == 47 || ch == 61) {
            return false;
        }
    }

    true
}
