use std::fmt;
use serde::Serialize;
use tera::{Tera, Context};
use crate::arg::Arg;
use crate::spec;
use std::error::Error;

#[derive(Serialize)]
pub struct Job {
    pub app: String,
    pub queue: String,
    pub name: String,
    pub nodes: u8,
    pub ppn: u8,
    pub walltime: Option<String>,
    pub mail_address: Option<String>,
    pub mail_flags: Option<String>,
    pub exec: String,
    pub config: Option<String>,
    pub use_workdir: bool,
    pub template: String,
}

impl Job {
    pub fn from_arg(arg: Arg) -> Self {
        let app = spec::App::from_json(&format!("./{}.json", &arg.command.to_string()));
        let exec = app.get_exec_path(&arg.command);
        let config = app.get_config_path(&arg.command);
        let template = app.get_template(&arg.command);

        let name = if let Some(name) = arg.name {
            name
        } else {
            app.get_name()
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
            app: app.get_name(),
            queue: arg.command.queue(),
            name: name,
            nodes: nodes,
            ppn: ppn,
            walltime: arg.walltime,
            mail_address: arg.mail_address,
            mail_flags: arg.mail_flags,
            exec: exec,
            config: config,
            use_workdir: use_workdir,
            template: template,
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
    
        let rendered = tera.render(&self.template, &Context::from_serialize(self)?)?;
        Ok(rendered)
    }
}