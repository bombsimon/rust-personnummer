use personnummer::{Personnummer, PersonnummerError};
use std::env;

fn main() -> Result<(), PersonnummerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example personnummer <personnummer>");
        return Err(PersonnummerError::InvalidInput);
    }

    let pnr = Personnummer::new(&args[1])?;

    if pnr.valid() {
        let gender = if pnr.is_female() { "female" } else { "male" };

        println!(
            "The person with personal identity number {} is a {} of age {}",
            pnr.format().long(),
            gender,
            pnr.get_age()
        );
    } else {
        println!("invalid personal identity number provided");
    }

    Ok(())
}
