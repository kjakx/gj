mod spec;
mod job;
mod args;
mod command;

use clap::Parser;
use anyhow::Result;

use crate::args::Args;

fn main() -> Result<()> {
    let script = Args::parse().to_job()?.to_script()?;
    println!("{}", script);
    Ok(())
}
