use super::*;

#[test]
fn simple_text_encoding_test() {
    let input: String = String::from("Hello World!");
    let enc: String = encode_data(input.as_bytes());

    let output: String = String::from("SGVsbG8gV29ybGQh");
    assert_eq!(enc, output);
}

#[test]
fn simple_text_decoding_test() {
    let input: String = String::from("SGVsbG8gV29ybGQh");
    let dec: Vec<u8> = decode_data(input);
    let string: String = String::from_utf8(dec).expect("The computed bytes are not UTF-8!");
    
    let output = String::from("Hello World!");
    assert_eq!(string, output);
}

#[test]
fn binary_encoding_decoding() {
    let input: Vec<u8> = vec![12, 14, 26, 48, 53, 22, 2, 12];
    let enc: String = encode_data(input.as_slice());
    let dec: Vec<u8> = decode_data(enc);

    assert_eq!(dec, input);
}