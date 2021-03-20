# Personnummer

[![Build
Status](https://travis-ci.com/bombsimon/rust-personnummer.svg?branch=master)](https://travis-ci.com/bombsimon/rust-personnummer)

Validate Swedish [personal identity
numbers](https://en.wikipedia.org/wiki/Personal_identity_number_(Sweden)) with
[Rust](https://www.rust-lang.org/).

## Usage

```rust
use personnummer::Personnummer;

fn main() {
    match Personnummer::new("199001011-0017") {
        Ok(pnr) => println!("{}: {}", pnr.format().long(), pnr.valid()),
        Err(e) => panic!("Error: {}", e),
    }
}
```

Fore more details, see [examples](examples) and/or run

```sh
$ cargo run --example personnummer <personnummer>
```
