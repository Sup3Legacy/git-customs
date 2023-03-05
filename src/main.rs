use clap::*;
use gitcustoms::*;

fn main() -> Result<(), ()> {
    let args = CliArgs::parse();
    println!("Hello, world!");
    let config = storage::File::read();
    println!("{:#?}", config);
    Ok(())
}
