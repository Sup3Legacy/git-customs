use clap::*;
use gitcustoms::*;

fn main() -> Result<(), ()> {
    let args = CliArgs::parse();
    println!("Hello, world!");
    Ok(())
}
