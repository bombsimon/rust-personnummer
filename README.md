# Personnummer

[![Build
Status](https://travis-ci.com/bombsimon/rust-personnummer.svg?branch=master)](https://travis-ci.com/bombsimon/rust-personnummer)

Validate Swedish social security numbers with
[Rust](https://www.rust-lang.org/).

## Example

```rust
extern crate personnummer;

fn main() {
    personnummer.valid("19130401+2931")  // => true
    personnummer.valid("19900101-0017")  // => true
}
```
