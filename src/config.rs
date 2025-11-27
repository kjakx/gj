use crate::profile::Profile;
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub root: Profile,
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

impl Config {
    /// Load configuration from config file.
    /// If a config path is provided as argument, use that.
    /// If not, use $XDG_CONFIG_HOME/gj/config.toml
    pub fn load(config_path: &Option<PathBuf>) -> Result<Self> {
        let config_path = if let Some(config_path) = config_path {
            config_path
        } else {
            &confy::get_configuration_file_path("gj", "config")?
        };
        let config = confy::load_or_else(config_path, || Config::default())?;
        Ok(config)
    }

    /// Return profile with a given name in the config file.
    /// If not given profile name, use "default" instead. 
    /// Note: The specified profile overrides the root profile.
    pub fn get_profile(&self, name: &Option<String>) -> Result<Profile> {
        let profile_name = if let Some(profile_name) = name {
            profile_name
        } else {
            "default"
        };
        let profile = if let Some(profile) = self.profiles.get(profile_name) {
            profile
        } else {
            &Profile::default()
        };
        self.root.merge(profile)
    }
}