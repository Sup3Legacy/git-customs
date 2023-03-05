pub mod storage;
pub mod engine;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CliArgs {
    path: Option<String>,

    #[arg(short, long)]
    /// Override identity detection
    identity: Option<String>,

    #[arg(short, long)]
    /// Do not confirm identity match
    force: bool,
}
