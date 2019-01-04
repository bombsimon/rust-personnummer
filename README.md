# Personnummer

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
