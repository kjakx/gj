//use clap::{Args, Parser, Subcommand};
use clap::{Parser};
use crate::command::Command;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    // job queue
    #[arg(long, value_name = "Job Queue", global = true)]
    pub queue: Option<String>,
    // job name
    #[arg(long, value_name = "Job Name", global = true)]
    pub name: Option<String>,
    // the number of nodes
    #[arg(long, value_name = "The Number of Nodes", global = true)]
    pub nodes: Option<u8>,
    // the number of cpus
    #[clap(long, value_name = "The Number of CPUs", global = true)]
    pub ncpus: Option<u8>,
    // the number of gpus
    #[clap(long, value_name = "The Number of GPUs", global = true)]    
    pub ngpus: Option<u8>,
    // the number of processes per node
    #[arg(long, value_name = "The Number of Processes per Node", global = true)]
    pub ppn: Option<u8>,
    // walltime
    #[arg(long, value_name = "CPU Wall Time", global = true)]
    pub walltime: Option<String>,
    // mail address
    #[arg(long, value_name = "Mail Address", global = true)]
    pub mail_address: Option<String>,
    // mail options
    #[arg(long, value_name = "Mail Options", global = true)]
    pub mail_flags: Option<String>,
    // check if use work directory
    #[arg(long, value_name = "Check if you use /work directory", global = true)]
    pub use_workdir: Option<bool>,
}
