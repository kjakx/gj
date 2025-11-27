use crate::profile::Profile;
use std::path::PathBuf;
use clap::Parser;

#[derive(Default, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct Args {
    #[command(flatten)]
    pub profile: Profile,
    /// profile name
    #[arg(value_name = "TEXT")]
    pub profile_name: Option<String>,
    /// config file path
    #[arg(long, value_name = "PATH")]
    pub config_path: Option<PathBuf>,
}