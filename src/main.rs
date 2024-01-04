mod spec;
mod job;
mod args;
mod command;

use clap::Parser;
use anyhow::Result;

use crate::args::Args;

fn main() -> Result<()> {
    let script = Args::try_parse()?.to_job()?.to_script()?;
    println!("{}", script);
    Ok(())
}
