mod args;
mod config;
mod profile;
mod script;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = args::Args::parse();
    let config = config::Config::load(&args.config_path)?;
    let profile = config.get_profile(&args.profile_name)?.merge(&args.profile)?;
    println!("{}", script::render(&profile)?);
    Ok(())
}