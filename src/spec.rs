use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use crate::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    name: String,
    queues: Vec<Queue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Queue {
    name: String,
    versions: Vec<Version>,
    #[serde(default)]
    template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    name: String,
    #[serde(default)]
    is_default: bool,
    bins: Vec<Bin>,
    #[serde(default)]
    config_path: Option<String>,
    #[serde(default)]
    template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    name: String,
    #[serde(default)]
    is_default: bool,
    path: String,
    #[serde(default)]
    template: Option<String>,
}

impl App {
    pub fn from_json(json_path: &str) -> Self {
        //let mut jsonref = JsonRef::new();
        let spec_json = fs::read_to_string(&json_path).expect("should have been able to read the file");
        //let mut spec_json = jsonref.deref_file(&json_path).expect("should have been able to read the file");
        //jsonref.deref_value(&mut spec_json).unwrap();
        //println!("{}", spec_json);
        serde_json::from_str(&spec_json).expect("JSON format error")
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_queue_by_name(&self, name: &str) -> Option<&Queue> {
        self.queues.iter().filter(|&queue| queue.name == name).next()
    }

    pub fn get_config_path(&self, cmd: &command::Command) -> Option<String> {
        let queue = self.get_queue_by_name(&cmd.queue()).unwrap();
        let version = if let Some(version) = &cmd.version() {
            queue.get_version_by_name(&version).unwrap()
        } else {
            queue.get_default_version().unwrap()
        };

        version.config_path.clone()
    }

    pub fn get_exec_path(&self, cmd: &command::Command) -> String {
        let queue = self.get_queue_by_name(&cmd.queue()).unwrap();
        let version = if let Some(version) = &cmd.version() {
            queue.get_version_by_name(&version).unwrap()
        } else {
            queue.get_default_version().unwrap()
        };
        let bin = if let Some(bin) = &cmd.bin() {
            version.get_bin_by_name(&bin).unwrap()
        } else {
            version.get_default_bin().unwrap()
        };

        bin.get_path()
    }

    pub fn get_template(&self, cmd: &command::Command) -> String {
        let queue = self.get_queue_by_name(&cmd.queue()).unwrap();
        let version = if let Some(version) = &cmd.version() {
            queue.get_version_by_name(&version).unwrap()
        } else {
            queue.get_default_version().unwrap()
        };
        let bin = if let Some(bin) = &cmd.bin() {
            version.get_bin_by_name(&bin).unwrap()
        } else {
            version.get_default_bin().unwrap()
        };

        if let Some(template) = bin.get_template() {
            template
        } else if let Some(template) = version.get_template() {
            template
        } else if let Some(template) = queue.get_template() {
            template
        } else {
            panic!("template file is not specified");
        }
    }
}

impl Queue {
    pub fn get_version_by_name(&self, name: &str) -> Option<&Version> {
        self.versions.iter().filter(|&version| version.name == name).next()
    }

    pub fn get_default_version(&self) -> Option<&Version> {
        self.versions.iter().filter(|&version| version.is_default).next()
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }
}

impl Version {
    pub fn get_bin_by_name(&self, name: &str) -> Option<&Bin> {
        self.bins.iter().filter(|&bin| bin.name == name).next()
    }

    pub fn get_default_bin(&self) -> Option<&Bin> {
        self.bins.iter().filter(|&bin| bin.is_default).next()
    }

    pub fn get_config(&self) -> Option<String> {
        self.config_path.clone()
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }
}

impl Bin {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }
}