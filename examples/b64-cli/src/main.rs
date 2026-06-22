//! A small CLI wrapper around the `b64` crate for encoding/decoding base64
//! from the command line. See `examples/b64-cli/README.md` for usage.

use b64::{decode_data, encode_data};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to encode, or base64 string to decode.
    #[arg(short, long, default_value_t = String::from("Hello World!"))]
    input: String,
    /// Decode `input` instead of encoding it.
    #[arg(short, long, default_value_t = false)]
    decode: bool,
    /// When decoding, print raw bytes instead of a UTF-8 string.
    #[arg(short = 'b', long = "bytes", action = ArgAction::SetFalse, default_value_t = true)]
    string: bool
}

/// Encodes or decodes `args.input` based on the parsed CLI flags, then
/// prints the result to stdout.
fn main() {
    let args = Args::parse();

    if !args.decode {
        let enc: String = encode_data(args.input.as_bytes());
        println!("Encoded Data: {}", enc);
    } else {
        let dec: Vec<u8> = decode_data(&args.input).expect("Invalid base64 string!");

        if args.string {
            let string: String = String::from_utf8(dec).expect("The computed bytes are not UTF-8!");
            println!("Decoded Data: {}", string);
            return;
        }
        println!("Decoded Data: {:?}", dec);
    }
}
