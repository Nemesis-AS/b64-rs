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
  let dec: Vec<u8> = decode_data(&input).expect("Invalid base64 string!");
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

The library provides three public functions, for encoding, decoding, and validation. Their signatures are:

```rs
fn encode_data(data: &[u8]) -> String
```

```rs
fn decode_data(data: &str) -> Result<Vec<u8>, B64Error>
```

```rs
fn is_valid_b64(data: &str) -> bool
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

## CLI

A command-line wrapper around this library is provided in [`examples/b64-cli`](examples/b64-cli/README.md). See its README for build and usage instructions.
