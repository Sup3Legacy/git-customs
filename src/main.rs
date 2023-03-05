use clap::*;
use gitcustoms::*;

fn main() -> Result<(), ()> {
    let args = CliArgs::parse();
    let config = storage::File::read();
    println!("{:#?}", config);
    Ok(())
}
