use std::{fs, env};

use crate::CliArgs;
use anyhow::Result;

pub fn main(args: CliArgs) -> Result<()> {
    let local = env::current_dir()?;

    let dir = match args.path {
        None => local,
        Some(sub_dir) => local.join(sub_dir),
    };

    Ok(())
}
