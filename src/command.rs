use clap::{Args, Subcommand};
use std::fmt;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Generate Gaussian16 Job Script")]
    Gaussian16(Gaussian16Arg),

    #[command(about = "Generate VASP4 Job Script")]
    Vasp4(Vasp4Arg),

    #[command(about = "Generate VASP5 Job Script")]
    Vasp5(Vasp5Arg),
}

#[derive(Debug, Args)]
pub struct Gaussian16Arg {
    #[arg(short, long, ignore_case = true)]
    queue: String,

    #[arg(short, long, ignore_case = true)]
    version: Option<String>,
}

#[derive(Debug, Args)]
pub struct Vasp4Arg {
    #[arg(short, long, ignore_case = true)]
    queue: String,

    #[arg(short, long, ignore_case = true)]
    version: Option<String>,
}

#[derive(Debug, Args)]
pub struct Vasp5Arg {
    #[arg(short, long, ignore_case = true)]
    queue: String,

    #[arg(short, long, ignore_case = true)]
    version: Option<String>,

    #[arg(short, long, ignore_case = true)]
    bin: Option<String>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Gaussian16(_) => {
                write!(f, "gaussian16")
            },
            Command::Vasp4(_) => {
                write!(f, "vasp4")
            },
            Command::Vasp5(_) => {
                write!(f, "vasp5")
            },
            _ => { unimplemented!(); }
        }
    }
}

impl Command {
    pub fn queue(&self) -> String {
        match self {
            Command::Gaussian16(arg) => {
                arg.queue.clone()
            },
            Command::Vasp4(arg) => {
                arg.queue.clone()
            },
            Command::Vasp5(arg) => {
                arg.queue.clone()
            },
        }
    }

    pub fn version(&self) -> Option<String> {
        match self {
            Command::Gaussian16(arg) => {
                arg.version.clone()
            },
            Command::Vasp4(arg) => {
                arg.version.clone()
            },
            Command::Vasp5(arg) => {
                arg.version.clone()
            },
            _ => None
        }
    }

    pub fn bin(&self) -> Option<String> {
        match self {
            Command::Vasp5(arg) => {
                arg.bin.clone()
            },
            _ => None
        }
    }
}