use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use crate::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSpec {
    name: String,
    queues: Vec<String>,
    versions: Vec<VersionSpec>,
    #[serde(default)]
    template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionSpec {
    name: String,
    bins: Vec<BinSpec>,
    #[serde(default)]
    dir: Option<String>,
    #[serde(default)]
    is_default: bool,
    #[serde(default)]
    config: Option<String>,
    #[serde(default)]
    template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinSpec {
    name: String,
    path: String,
    #[serde(default)]
    is_default: bool,
    #[serde(default)]
    template: Option<String>,
}

impl AppSpec {
    pub fn from_json(json_path: &str) -> Self {
        let spec_json = fs::read_to_string(&json_path).expect("should have been able to read the file");
        serde_json::from_str(&spec_json).expect("JSON format error")
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_default_queue(&self) -> String {
        self.queues[0].clone()
    }

    pub fn is_available_on_queue(&self, queue: &str) -> bool {
        self.queues.contains(&queue.to_owned())
    }

    pub fn get_version_by_name(&self, name: &str) -> Option<&VersionSpec> {
        self.versions.iter().filter(|&version| version.name == name).next()
    }

    pub fn get_default_version(&self) -> &VersionSpec {
        if let Some(version) = self.versions.iter().filter(|&version| version.is_default).next() {
            version
        } else {
            &self.versions[0]    // if is_default is not specified, the first element is treated as the default version.
        }
    }

    pub fn get_version(&self, cmd: &command::Command) -> &VersionSpec {
        if let Some(version) = &cmd.version() {
            self.get_version_by_name(&version).expect("no such version")
        } else {
            self.get_default_version()
        }
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }

}

impl VersionSpec {
    pub fn get_bin_by_name(&self, name: &str) -> Option<&BinSpec> {
        self.bins.iter().filter(|&bin| bin.name == name).next()
    }

    pub fn get_default_bin(&self) -> &BinSpec {
        if let Some(bin) = self.bins.iter().filter(|&bin| bin.is_default).next() {
            bin
        } else {
            &self.bins[0]
        }
    }

    pub fn get_bin(&self, cmd: &command::Command) -> &BinSpec {
        if let Some(bin) = &cmd.bin() {
            self.get_bin_by_name(&bin).expect("no such binary")
        } else {
            self.get_default_bin()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_dir(&self) -> Option<String> {
        self.dir.clone()
    }

    pub fn get_config(&self) -> Option<String> {
        self.config.clone()
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }
}

impl BinSpec {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_template(&self) -> Option<String> {
        self.template.clone()
    }
}