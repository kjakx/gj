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
        let fallback = &confy::get_configuration_file_path("gj", "config")?;
        let config_path = config_path.as_deref().unwrap_or(fallback);
        let config = confy::load_path(config_path)?;
        Ok(config)
    }

    /// Return profile with a given name in the config file.
    /// If not given profile name, use "default" instead. 
    /// Note: The specified profile overrides the root profile.
    pub fn get_profile(&self, profile_name: &Option<String>) -> Result<Profile> {
        let fallback = &Profile::default();
        let profile_name = profile_name.as_deref().unwrap_or("default");
        let profile = self.profiles.get(profile_name).unwrap_or(fallback);
        self.root.merge(profile)
    }
}
