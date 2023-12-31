use serde::Serialize;
use tera::{Tera, Context};
use crate::args::Args;
use crate::spec;
use std::path::PathBuf;
use std::error::Error;

#[derive(Serialize)]
pub struct App {
    name: String,
    version: String,
    dir: Option<PathBuf>,
    bin: PathBuf,
    config: Option<PathBuf>,
    template: PathBuf,
}

#[derive(Serialize)]
pub struct Job {
    pub app: App,
    pub queue: String,
    pub job_name: String,
    pub nodes: u8,
    pub ppn: u8,
    pub walltime: Option<String>,
    pub mail_address: Option<String>,
    pub mail_flags: Option<String>,
    pub use_workdir: bool,
}

impl Job {
    pub fn from_args(args: Args) -> Self {
        let app_spec = spec::AppSpec::get_app_spec(&args.command.name());
        let version_spec = app_spec.get_version_spec(&args.command.version());
        let bin_spec = version_spec.get_bin_spec(&args.command.bin());

        let app_name = app_spec.get_name();
        let version = version_spec.get_name();
        let dir = version_spec.get_dir();
        let bin = bin_spec.get_path();
        let config = version_spec.get_config();
        let template = if let Some(template) = bin_spec.get_template() {
            template
        } else if let Some(template) = version_spec.get_template() {
            template
        } else if let Some(template) = app_spec.get_template() {
            template
        } else {
            panic!("template file is not specified");
        };

        let app = App {
            name: app_name,
            version,
            dir,
            bin,
            config,
            template
        };

        let queue = if let Some(queue) = args.queue {
            if app_spec.is_available_on_queue(&queue) {
                queue
            } else {
                panic!("the queue is not supported");
            }
        } else {
            app_spec.get_default_queue()
        };

        let job_name = if let Some(name) = args.name {
            name
        } else {
            app_spec.get_name()
        };

        let nodes = if let Some(nodes) = args.nodes {
            nodes
        } else {
            1
        };

        let ppn = if let Some(ppn) = args.ppn {
            ppn
        } else {
            36
        };
        
        let use_workdir = if let Some(use_workdir) = args.use_workdir {
            use_workdir
        } else {
            true
        };

        Job {
            app: app,
            queue: queue,
            job_name: job_name,
            nodes: nodes,
            ppn: ppn,
            walltime: args.walltime,
            mail_address: args.mail_address,
            mail_flags: args.mail_flags,
            use_workdir: use_workdir,
        }
    }

    pub fn generate_script(&self) -> Result<String, Box<dyn Error>> {
        let tera = match Tera::new("templates/**/*.sh") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
    
        let rendered = tera.render(self.app.template.to_str().unwrap(), &Context::from_serialize(self)?)?;
        Ok(rendered)
    }
}