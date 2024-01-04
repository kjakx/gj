use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSpec {
    name: String,
    queues: Vec<String>,
    versions: Vec<VersionSpec>,
    #[serde(default)]
    template: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionSpec {
    name: String,
    bins: Vec<BinSpec>,
    #[serde(default)]
    is_default: bool,
    #[serde(default)]
    dir: Option<PathBuf>,
    #[serde(default)]
    config: Option<PathBuf>,
    #[serde(default)]
    template: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinSpec {
    name: String,
    path: PathBuf,
    #[serde(default)]
    is_default: bool,
    #[serde(default)]
    template: Option<PathBuf>,
}

impl AppSpec {
    pub fn get_app_spec(app: &str) -> Result<Self> {
        Self::from_json(&Path::new(format!("./spec/{}.json", app).as_str()))
    }

    fn from_json(json_path: &Path) -> Result<Self> {
        let spec_str = fs::read_to_string(&json_path)?;
        let spec_json = serde_json::from_str(&spec_str)?;
        Ok(spec_json)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_available_on_queue(&self, queue: &str) -> bool {
        self.queues.contains(&queue.to_owned())
    }

    pub fn get_queue_by_name(&self, queue: &str) -> Option<String> {
        if self.is_available_on_queue(queue) {
            Some(queue.to_owned())
        } else {
            None
        }
    }

    pub fn get_queue(&self, queue: &Option<String>) -> Result<String> {
        match queue {
            Some(queue) => {
                match self.get_queue_by_name(&queue) {
                    Some(queue) => Ok(queue.to_string()),
                    None        => Err(anyhow!("no such queue: {}", &queue))
                }
            },
            None => {
                match self.get_default_queue() {
                    Some(queue) => Ok(queue.to_string()),
                    None        => Err(anyhow!("no queue is available"))
                }
            }
        }
    }

    pub fn get_default_queue(&self) -> Option<String> {
        self.queues.iter().next().cloned()
    }

    pub fn get_version_by_name(&self, name: &str) -> Option<&VersionSpec> {
        self.versions.iter().filter(|&version| version.get_name() == name).next()
    }

    pub fn get_default_version(&self) -> Option<&VersionSpec> {
        if let Some(version) = self.versions.iter().filter(|&version| version.is_default).next() {
            Some(version)
        } else {
            self.versions.first() // if is_default is not specified, the first element is treated as the default version.
        }
    }

    pub fn get_version_spec(&self, version: &Option<String>) -> Result<&VersionSpec> {
        match version {
            Some(version) => {
                match self.get_version_by_name(&version) {
                    Some(version_spec) => Ok(version_spec),
                    None               => Err(anyhow!("no such version: {}", &version))
                }
            },
            None => {
                match self.get_default_version() {
                    Some(version_spec) => Ok(version_spec),
                    None               => Err(anyhow!("no default version is available"))
                }
            }
        }
    }

    pub fn get_template(&self) -> &Option<PathBuf> {
        &self.template
    }

}

impl VersionSpec {
    pub fn get_bin_by_name(&self, name: &str) -> Option<&BinSpec> {
        self.bins.iter().filter(|&bin| bin.get_name() == name).next()
    }

    pub fn get_default_bin(&self) -> Option<&BinSpec> {
        if let Some(bin) = self.bins.iter().filter(|&bin| bin.is_default).next() {
            Some(bin)
        } else {
            self.bins.first()
        }
    }

    pub fn get_bin_spec(&self, bin: &Option<String>) -> Result<&BinSpec> {
        match bin {
            Some(bin) => match self.get_bin_by_name(&bin) {
                Some(bin_spec) => Ok(bin_spec),
                None           => Err(anyhow!("no such binary: {}", &bin))
            },
            None => match self.get_default_bin() {
                Some(bin_spec) => Ok(bin_spec),
                None           => Err(anyhow!("no default bin is available"))
            }
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_dir(&self) -> &Option<PathBuf> {
        &self.dir
    }

    pub fn get_config(&self) -> &Option<PathBuf> {
        &self.config
    }

    pub fn get_template(&self) -> &Option<PathBuf> {
        &self.template
    }
}

impl BinSpec {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_template(&self) -> &Option<PathBuf> {
        &self.template
    }
}