# Personnummer

[![Build
Status](https://travis-ci.com/bombsimon/rust-personnummer.svg?branch=master)](https://travis-ci.com/bombsimon/rust-personnummer)

Validate Swedish social security numbers with
[Rust](https://www.rust-lang.org/).

## Usage

```rust
use personnummer::Personnummer;

fn main() {
    let pnr = Personnummer::new("19900101-0017");

    if !pnr.valid() {
        println!("invalid social security number provided");
        return;
    }

    let gender = if pnr.is_female() { "female" } else { "male" };

    println!(
        "The person with social security number {} is a {} of age {}",
        pnr.format().long(),
        gender,
        pnr.get_age()
    );
}
```
