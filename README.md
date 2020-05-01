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
    let pnr = Personnummer::new("19900101-0017");

    if !pnr.valid() {
        println!("invalid personal identity number provided");
        return;
    }

    let gender = if pnr.is_female() { "female" } else { "male" };

    println!(
        "The person with personal identity number {} is a {} of age {}",
        pnr.format().long(),
        gender,
        pnr.get_age()
    );
}
```
