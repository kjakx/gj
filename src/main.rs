mod spec;
mod job;
mod args;
mod command;

//use std::error::Error;
use clap::Parser;
use anyhow::Result;

use crate::job::Job;
use crate::args::Args;

fn main() -> Result<()> {
    let script = Args::try_parse()?.to_job()?.to_script()?;
    println!("{}", script);
    Ok(())
}
