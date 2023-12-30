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
    Quantumatk(CommandArg),

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

    #[command(about = "Generate OpenMX Job Script")]
    Openmx(CommandArg),

    #[command(about = "Generate SMASH Job Script")]
    Smash(CommandArg),

    #[command(about = "Generate TOMBO Job Script")]
    Tombo(CommandArg),

    #[command(about = "Generate RSDFT Job Script")]
    Rsdft(CommandArg),

    #[command(about = "Generate HPhi Job Script")]
    Hphi(CommandArg),

    #[command(about = "Generate mVMC Job Script")]
    Mvmc(CommandArg),

    #[command(about = "Generate CP2K Job Script")]
    Cp2k(CommandArg),

    #[command(about = "Generate Elk Job Script")]
    Elk(CommandArg),

    #[command(about = "Generate ALAMODE Job Script")]
    Alamode(CommandArg),

    #[command(about = "Generate SALMON Job Script")]
    Salmon(CommandArg),

    #[command(about = "Generate OCTOPUS Job Script")]
    Octopus(CommandArg),

    #[command(about = "Generate Wannier90 Job Script")]
    Wannier90(CommandArg),
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
            Command::Quantumatk(_) => {
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
                write!(f, "quantum-espresso")
            },
            Command::Openmx(_) => {
                write!(f, "openmx")
            },
            Command::Smash(_) => {
                write!(f, "smash")
            },
            Command::Tombo(_) => {
                write!(f, "tombo")
            },
            Command::Rsdft(_) => {
                write!(f, "rsdft")
            },
            Command::Hphi(_) => {
                write!(f, "hphi")
            },
            Command::Mvmc(_) => {
                write!(f, "mvmc")
            },
            Command::Cp2k(_) => {
                write!(f, "cp2k")
            },
            Command::Elk(_) => {
                write!(f, "elk")
            },
            Command::Alamode(_) => {
                write!(f, "alamode")
            },
            Command::Salmon(_) => {
                write!(f, "salmon")
            },
            Command::Octopus(_) => {
                write!(f, "octopus")
            },
            Command::Wannier90(_) => {
                write!(f, "wannier90")
            },
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
            Command::Quantumatk(arg) => {
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
            Command::Openmx(arg) => {
                arg.version.clone()
            },
            Command::Smash(arg) => {
                arg.version.clone()
            },
            Command::Tombo(arg) => {
                arg.version.clone()
            },
            Command::Rsdft(arg) => {
                arg.version.clone()
            },
            Command::Hphi(arg) => {
                arg.version.clone()
            },
            Command::Mvmc(arg) => {
                arg.version.clone()
            },
            Command::Cp2k(arg) => {
                arg.version.clone()
            },
            Command::Elk(arg) => {
                arg.version.clone()
            },
            Command::Alamode(arg) => {
                arg.version.clone()
            },
            Command::Salmon(arg) => {
                arg.version.clone()
            },
            Command::Octopus(arg) => {
                arg.version.clone()
            },
            Command::Wannier90(arg) => {
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
            Command::Quantumatk(arg) => {
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
            Command::Openmx(arg) => {
                arg.bin.clone()
            },
            Command::Smash(arg) => {
                arg.bin.clone()
            },
            Command::Tombo(arg) => {
                arg.bin.clone()
            },
            Command::Rsdft(arg) => {
                arg.bin.clone()
            },
            Command::Hphi(arg) => {
                arg.bin.clone()
            },
            Command::Mvmc(arg) => {
                arg.bin.clone()
            },
            Command::Cp2k(arg) => {
                arg.bin.clone()
            },
            Command::Elk(arg) => {
                arg.bin.clone()
            },
            Command::Alamode(arg) => {
                arg.bin.clone()
            },
            Command::Salmon(arg) => {
                arg.bin.clone()
            },
            Command::Octopus(arg) => {
                arg.bin.clone()
            },
            Command::Wannier90(arg) => {
                arg.bin.clone()
            },
        }
    }
}