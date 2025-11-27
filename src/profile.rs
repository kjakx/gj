use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use merge_struct::merge;
use serde::{Serialize, Deserialize};

#[derive(Parser, Serialize, Deserialize)]
pub struct Profile {
    /// job scheduler (currently has no effect)
    #[arg(long, value_name = "TEXT")]
    pub scheduler: Option<String>,
    /// shell name
    #[arg(long, value_name = "TEXT")]
    pub shell: Option<PathBuf>,
    /// job name
    #[arg(short = 'N', long, value_name = "TEXT")]
    pub name: Option<String>,
    /// queue name
    #[arg(short = 'q', long, value_name = "TEXT")]
    pub queue: Option<String>,
    /// resources
    #[arg(short = 'l', long, value_name = "\"key=value\"")]
    pub resources: Option<Vec<String>>,
    /// mail address
    #[arg(short = 'M', long, value_name = "TEXT")]
    pub email_address: Option<String>,
    /// mail options
    #[arg(short = 'm', long, value_name = "TEXT")]
    pub email_flags: Option<String>,
    /// move to current working directory
    #[arg(long, value_name = "BOOL")]
    pub cwd: Option<bool>,
    /// commands to run
    #[arg(long, value_name = "\"command\"")]
    pub commands: Option<Vec<String>>,
    /// template file path
    #[arg(long, value_name = "PATH")]
    pub template_path: Option<PathBuf>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            scheduler: None,
            shell: None,
            name: None,
            queue: None,
            resources: Some(vec![]),
            email_address: None,
            email_flags: None,
            cwd: Some(true),
            commands: Some(vec![]),
            template_path: None,
        }
    }
}

impl Profile {
    pub fn merge(&self, overrides: &Self) -> Result<Self> {
        let mut merged = Self::default();
        merged = merge(&merged, self)?;
        merged = merge(&merged, overrides)?;
        Ok(merged) 
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn default_profile_has_expected_values() {
        let d = Profile::default();
        assert_eq!(d.shell.unwrap(), "/bin/sh");
        assert_eq!(d.cwd.unwrap(), true);
        assert!(d.resources.unwrap().is_empty());
        assert!(d.commands.unwrap().is_empty());
    }

    #[test]
    fn merge_overrides_take_precedence_and_defaults_apply() {
        let root = Profile {
            scheduler: Some("pbs".into()),
            shell: None,
            ..Profile::default()
        };

        let overrides = Profile {
            scheduler: Some("slurm".into()),
            queue: Some("workq".into()),
            ..Profile::default()
        };

        let merged = root.merge(&overrides).unwrap();
        // overrides.scheduler should win
        assert_eq!(merged.scheduler.unwrap(), "slurm");
        // overrides.queue should be present
        assert_eq!(merged.queue.unwrap(), "workq");
        // shell should be filled from default (since root.shell was None)
        assert_eq!(merged.shell.unwrap(), "/bin/sh");
    }

    #[test]
    fn config_get_profile_merges_root_and_profile() {
        let mut cfg = Config::default();
        cfg.root = Profile {
            scheduler: Some("pbs".into()),
            shell: Some("/bin/bash".into()),
            ..Profile::default()
        };

        cfg.profiles.insert(
            "special".into(),
            Profile {
                shell: Some("/bin/zsh".into()),
                queue: Some("high".into()),
                ..Profile::default()
            },
        );

        let got = cfg.get_profile(&Some("special".into())).unwrap();
        // profile's shell overrides root
        assert_eq!(got.shell.unwrap(), "/bin/zsh");
        // root's scheduler remains
        assert_eq!(got.scheduler.unwrap(), "pbs");

        // missing profile: root.merge(Profile::default())
        // The merge_struct lib merges bottom-up: default -> root -> profile(default)
        // so profile::default() wins, giving shell="/bin/sh" from Profile::default()
        let got2 = cfg.get_profile(&Some("missing".into())).unwrap();
        assert_eq!(got2.shell.unwrap(), "/bin/sh");
        assert_eq!(got2.scheduler.unwrap(), "pbs");
    }
}