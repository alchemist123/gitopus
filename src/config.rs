use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::error::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub username: String,
    pub ssh_key_path: PathBuf,
    pub domain: String,
    #[serde(default)]
    pub default: bool,
}

pub struct ConfigManager {
    profiles: Vec<Profile>,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "gitopus")
            .ok_or_else(|| GitOpusError::ConfigError("Could not get project directories".into()))?;
        
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }

        let profiles = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str(&content).map_err(|e| GitOpusError::ConfigError(e.to_string()))?
        } else {
            Vec::new()
        };

        Ok(Self {
            profiles,
            config_path,
        })
    }

    pub fn add_profile(&mut self, profile: Profile) -> Result<()> {
        self.profiles.push(profile);
        self.save()
    }

    pub fn get_profiles(&self) -> &[Profile] {
        &self.profiles
    }

    pub fn set_default_profile(&mut self, name: &str) -> Result<()> {
        for p in &mut self.profiles {
            p.default = p.name == name;
        }
        self.save()
    }

    fn save(&self) -> Result<()> {
        let content = toml::to_string(&self.profiles)
            .map_err(|e| GitOpusError::ConfigError(e.to_string()))?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
}