# b64-rs

A base64 string encoder and decoder library written in Rust with zero dependencies.

## Getting Started

### Encoding data

```rs
use b64::encode_data;

fn main() {
  let input: String = String::from("Hello World!");

  let enc: String = encode_data(input.as_bytes());
  println!("Encoded Data: {}", enc);
}
```

### Decoding data

```rs
use b64::decode_data;

fn main() {
  let input: String = String::from("SGVsbG8gV29ybGQh");
  let dec: Vec<u8> = decode_data(input);
  let string: String = String::from_utf8(dec).expect("The computed bytes are not UTF-8!");

  println!("Decoded Data: {}", string);
}
```

### Check if a string is valid base64 string

```rs
use b64::is_valid_b64;
 
fn main() {
  let input1: String = String::from("ABCDEFGH");
  let input2: String = String::from("ABSJF$#A");
 
  println!("Is '{}' a valid base64 string: {}", input1, is_valid_b64(&input1));
  println!("Is '{}' a valid base64 string: {}", input2, is_valid_b64(&input2));
}
```

## API

The library provides two public functions, one each for encoding and decoding. Their signatures are:

```rs
fn encode_data(data: &[u8]) -> String
```

```rs
fn decode_quadruplet(data: &str) -> Vec<u8>
```

```rs
fn is_valid_b64(data: &String) -> bool
```

## Builing from Source

1. Clone this repo
2. Build using
```shell
cargo build
```

## Testing

Some basic tests are provided in the repo, those can be run using
```shell
cargo test
```

## Docs

The library has documentation comments. Build the docs using
```shell
cargo doc
```
