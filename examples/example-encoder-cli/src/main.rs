use b64::encode_data;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Hello World!"))]
    input: String
}

fn main() {
    let args = Args::parse();

    let enc: String = encode_data(args.input.as_bytes());
    println!("Encoded Data: {}", enc);
}
