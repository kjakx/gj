use serde::Serialize;
use tera::{Tera, Context};
use crate::arg::Arg;
use crate::spec;
use std::error::Error;

#[derive(Serialize)]
pub struct App {
    name: String,
    version: String,
    dir: Option<String>,
    bin: String,
    config: Option<String>,
    template: String,
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
    pub fn from_arg(arg: Arg) -> Self {
        let app_spec = spec::AppSpec::from_json(&format!("./spec/{}.json", &arg.command.to_string()));
        let version_spec = app_spec.get_version(&arg.command);
        let bin_spec = version_spec.get_bin(&arg.command);

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

        let queue = if let Some(queue) = arg.queue {
            if app_spec.is_available_on_queue(&queue) {
                queue
            } else {
                panic!("the queue is not supported");
            }
        } else {
            app_spec.get_default_queue()
        };

        let job_name = if let Some(name) = arg.name {
            name
        } else {
            app_spec.get_name()
        };

        let nodes = if let Some(nodes) = arg.nodes {
            nodes
        } else {
            1
        };

        let ppn = if let Some(ppn) = arg.ppn {
            ppn
        } else {
            36
        };
        
        let use_workdir = if let Some(use_workdir) = arg.use_workdir {
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
            walltime: arg.walltime,
            mail_address: arg.mail_address,
            mail_flags: arg.mail_flags,
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
    
        let rendered = tera.render(&self.app.template, &Context::from_serialize(self)?)?;
        Ok(rendered)
    }
}