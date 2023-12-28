use clap::{Args, Subcommand};
use std::fmt;

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Generate Gaussian16 Job Script")]
    Gaussian16(CommandArg),

    #[command(about = "Generate ADF Job Script")]
    Adf(CommandArg),

    #[command(about = "Generate CRYSTAL Job Script")]
    Crystal(CommandArg),
    
    #[command(about = "Generate QuantumATK Job Script")]
    QuantumAtk(CommandArg),

    #[command(about = "Generate VASP4 Job Script")]
    Vasp4(CommandArg),

    #[command(about = "Generate VASP5 Job Script")]
    Vasp5(CommandArg),

    #[command(about = "Generate VASP6 Job Script")]
    Vasp6(CommandArg),

    #[command(about = "Generate WIEN2k Job Script")]
    Wien2k(CommandArg),

    #[command(about = "Generate CPMD Job Script")]
    Cpmd(CommandArg),

    #[command(about = "Generate ABINIT Job Script")]
    Abinit(CommandArg),

    #[command(about = "Generate LAMMPS Job Script")]
    Lammps(CommandArg),

    #[command(about = "Generate SIESTA Job Script")]
    Siesta(CommandArg),

    #[command(about = "Generate Quantum ESPRESSO Job Script")]
    QuantumEspresso(CommandArg),
}

#[derive(Debug, Args)]
pub struct CommandArg {
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
            Command::Adf(_) => {
                write!(f, "adf")
            },
            Command::Crystal(_) => {
                write!(f, "crystal")
            },
            Command::QuantumAtk(_) => {
                write!(f, "quantumatk")
            },
            Command::Vasp4(_) => {
                write!(f, "vasp4")
            },
            Command::Vasp5(_) => {
                write!(f, "vasp5")
            },
            Command::Vasp6(_) => {
                write!(f, "vasp6")
            },
            Command::Wien2k(_) => {
                write!(f, "wien2k")
            },
            Command::Cpmd(_) => {
                write!(f, "cpmd")
            },
            Command::Abinit(_) => {
                write!(f, "abinit")
            },
            Command::Lammps(_) => {
                write!(f, "lammps")
            },
            Command::Siesta(_) => {
                write!(f, "siesta")
            },
            Command::QuantumEspresso(_) => {
                write!(f, "quantumespresso")
            }
        }
    }
}

impl Command {
    pub fn version(&self) -> Option<String> {
        match self {
            Command::Gaussian16(arg) => {
                arg.version.clone()
            },
            Command::Adf(arg) => {
                arg.version.clone()
            },
            Command::Crystal(arg) => {
                arg.version.clone()
            },
            Command::QuantumAtk(arg) => {
                arg.version.clone()
            },
            Command::Vasp4(arg) => {
                arg.version.clone()
            },
            Command::Vasp5(arg) => {
                arg.version.clone()
            },
            Command::Vasp6(arg) => {
                arg.version.clone()
            },
            Command::Wien2k(arg) => {
                arg.version.clone()
            },
            Command::Cpmd(arg) => {
                arg.version.clone()
            },
            Command::Abinit(arg) => {
                arg.version.clone()
            },
            Command::Lammps(arg) => {
                arg.version.clone()
            },
            Command::Siesta(arg) => {
                arg.version.clone()
            },
            Command::QuantumEspresso(arg) => {
                arg.version.clone()
            },
        }
    }

    pub fn bin(&self) -> Option<String> {
        match self {
            Command::Gaussian16(arg) => {
                arg.bin.clone()
            },
            Command::Adf(arg) => {
                arg.bin.clone()
            },
            Command::Crystal(arg) => {
                arg.bin.clone()
            },
            Command::QuantumAtk(arg) => {
                arg.bin.clone()
            },
            Command::Vasp4(arg) => {
                arg.bin.clone()
            },
            Command::Vasp5(arg) => {
                arg.bin.clone()
            },
            Command::Vasp6(arg) => {
                arg.bin.clone()
            },
            Command::Wien2k(arg) => {
                arg.bin.clone()
            },
            Command::Cpmd(arg) => {
                arg.bin.clone()
            },
            Command::Abinit(arg) => {
                arg.bin.clone()
            },
            Command::Lammps(arg) => {
                arg.bin.clone()
            },
            Command::Siesta(arg) => {
                arg.bin.clone()
            },
            Command::QuantumEspresso(arg) => {
                arg.bin.clone()
            },
        }
    }
}