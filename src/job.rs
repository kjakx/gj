use serde::Serialize;
use tera::{Tera, Context};
use crate::args::Args;
use crate::spec;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Serialize)]
pub struct AppInfo {
    name: String,
    version: String,
    bin: PathBuf,
    dir: Option<PathBuf>,
    config: Option<PathBuf>,
}

#[derive(Serialize)]
pub struct PbsOpts {
    queue: String,
    jobname: String,
    nodes: u8,
    ncpus: Option<u8>,
    ngpus: Option<u8>,
    walltime: Option<String>,
    mail_address: Option<String>,
    mail_flags: Option<String>,
    cwd: bool,
}

#[derive(Serialize)]
pub struct RunOpts {
    nprocs: u16,
    ppn: u8,
    threads: u8,
    tpc: u8,
    input: String,
    stdout: String,
    stderr: String,
}

#[derive(Serialize)]
pub struct Job {
    pub app: AppInfo,
    pub pbs: PbsOpts,
    pub run: RunOpts,
    pub template: PathBuf,
}

impl Job {
    pub fn from_args(args: &Args) -> Result<Self> {
        let app_spec = spec::AppSpec::get_app_spec(&args.command.name())?;
        let version_spec = app_spec.get_version_spec(&args.command.version())?;
        let bin_spec = version_spec.get_bin_spec(&args.command.bin())?;

        let app_name = app_spec.get_name();
        let version = version_spec.get_name();
        let dir = version_spec.get_dir();
        let bin = bin_spec.get_path();
        let config = version_spec.get_config();

        let app = AppInfo {
            name: app_name,
            version,
            dir,
            bin,
            config,
        };

        let queue = app_spec.get_queue(&args.queue)?;
        let jobname = args.name.clone().unwrap_or(app_spec.get_name());
        let nodes = args.nodes.unwrap_or(1);

        let pbs = PbsOpts {
            queue,
            jobname,
            nodes,
            ncpus: args.ncpus,
            ngpus: args.ngpus,
            walltime: args.walltime.clone(),
            mail_address: args.mail_address.clone(),
            mail_flags: args.mail_flags.clone(),
            cwd: args.cwd,
        };

        let ppn = args.ppn.unwrap_or(36);
        let nprocs = args.nprocs.unwrap_or((nodes * ppn) as u16);
        let threads = args.threads.unwrap_or(36);
        let tpc = args.tpc.unwrap_or(1);
        let input = args.input.clone().unwrap_or(format!("{}.in", &pbs.jobname));
        let stdout = args.stdout.clone().unwrap_or(format!("{}.out", &pbs.jobname));
        let stderr = args.stderr.clone().unwrap_or(format!("{}.err", &pbs.jobname));

        let run = RunOpts {
            nprocs,
            ppn,
            threads,
            tpc,
            input,
            stdout,
            stderr,
        };

        let template = if let Some(template) = bin_spec.get_template() {
            template
        } else if let Some(template) = version_spec.get_template() {
            template
        } else if let Some(template) = app_spec.get_template() {
            template
        } else {
            panic!("template file is not specified");
        };

        let job = Job {
            app,
            pbs,
            run,
            template,
        };

        Ok(job)
    }

    pub fn to_script(&self) -> Result<String> {
        let tera = Tera::new("templates/**/*.sh")?;
        let rendered = tera.render(self.template.to_str().unwrap(), &Context::from_serialize(self)?)?;
        Ok(rendered)
    }
}