use b64::{decode_data, encode_data};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Hello World!"))]
    input: String,
    #[arg(short, long, default_value_t = false)]
    decode: bool,
    #[arg(short, long, default_value_t = true)]
    string: bool
}

fn main() {
    let args = Args::parse();

    if !args.decode {
        let enc: String = encode_data(args.input.as_bytes());
        println!("Encoded Data: {}", enc);
    } else {
        let dec: Vec<u8> = decode_data(args.input);

        if args.string {
            let string: String = String::from_utf8(dec).expect("The computed bytes are not UTF-8!");
            println!("Decoded Data: {}", string);
            return;
        }
        println!("Decoded Data: {:?}", dec);
    }
}
