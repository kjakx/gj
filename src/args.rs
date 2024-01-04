//use clap::{Args, Parser, Subcommand};
use clap::{Parser};
use crate::command::Command;
use crate::job::Job;
use std::path::PathBuf;
use anyhow::Result;

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
    
    // queue name
    #[arg(long, value_name = "TEXT", global = true)]
    pub queue: Option<String>,
    
    // job name
    #[arg(long, value_name = "TEXT", global = true)]
    pub name: Option<String>,

    // the number of nodes
    #[arg(long, value_name = "INT", global = true)]
    pub nodes: Option<u8>,
    
    // the number of cpus
    #[clap(long, value_name = "INT", global = true)]
    pub ncpus: Option<u8>,
    
    // the number of gpus
    #[clap(long, value_name = "INT", global = true)]
    pub ngpus: Option<u8>,
    
    // walltime
    #[arg(long, value_name = "HH:MM:SS", global = true)]
    pub walltime: Option<String>,
    
    // mail address
    #[arg(long, value_name = "TEXT", global = true)]
    pub mail_address: Option<String>,
    
    // mail options
    #[arg(long, value_name = "TEXT", global = true)]
    pub mail_flags: Option<String>,
    
    // the total number of processes
    #[arg(long, value_name = "INT", global = true)]
    pub nprocs: Option<u16>,
    
    // the number of processes per node
    #[arg(long, value_name = "INT", global = true)]
    pub ppn: Option<u8>,
    
    // the total number of threads
    #[arg(long, value_name = "INT", global = true)]
    pub threads: Option<u8>,

    // the number of threads per core
    #[arg(long, value_name = "INT", global = true)]
    pub tpc: Option<u8>,

    // input file
    #[arg(long, value_name = "TEXT", global = true)]
    pub input: Option<String>,

    // stdout file
    #[arg(long, value_name = "TEXT", global = true)]
    pub stdout: Option<String>,

    // stderr file
    #[arg(long, value_name = "TEXT", global = true)]
    pub stderr: Option<String>,

    // move to current working directory
    #[arg(long, global = true)]
    pub cwd: Option<bool>,
}

impl Args {
    pub fn to_job(&self) -> Result<Job> {
        Job::from_args(self)
    }
}
