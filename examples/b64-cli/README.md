# b64-cli

A small command-line wrapper around the [`b64`](../../README.md) crate for encoding and decoding base64 from the terminal.

`b64-cli` is its own standalone crate (it has its own `Cargo.toml` and depends on `b64` via a path dependency) rather than a Cargo `examples/*.rs` target, so it can't be run with `cargo run --example b64-cli` from the repo root.

## Building

```shell
cd examples/b64-cli
cargo build --release
```

The binary is produced at `examples/b64-cli/target/release/b64-cli`.

## Running

From within `examples/b64-cli`:

```shell
cargo run -- [OPTIONS]
```

Or from the repo root, without changing directory:

```shell
cargo run --manifest-path examples/b64-cli/Cargo.toml -- [OPTIONS]
```

## Usage

```
Usage: b64-cli [OPTIONS]

Options:
  -i, --input <INPUT>  Text to encode, or base64 string to decode  [default: "Hello World!"]
  -d, --decode         Decode `input` instead of encoding it
  -b, --bytes          When decoding, print raw bytes instead of a UTF-8 string
  -h, --help           Print help
  -V, --version        Print version
```

## Examples

Encode text (the default mode):

```shell
cargo run -- -i "Hello World!"
# Encoded Data: SGVsbG8gV29ybGQh
```

Decode base64 to a UTF-8 string (the default decode output):

```shell
cargo run -- -d -i "SGVsbG8gV29ybGQh"
# Decoded Data: Hello World!
```

Decode base64 to raw bytes instead:

```shell
cargo run -- -d -i "SGVsbG8gV29ybGQh" -b
# Decoded Data: [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
```

## Notes

- If `--input` isn't valid base64 while decoding, or the decoded bytes aren't valid UTF-8 when printing as a string (i.e. without `-b`), the CLI will panic with an `expect()` message rather than print a friendly error. This is example code, not a hardened tool.
