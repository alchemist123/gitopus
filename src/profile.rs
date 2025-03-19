use crate::error::*;
use std::process::Command;
use std::path::Path;

pub struct ProfileManager;

impl ProfileManager {
    pub fn activate_profile(profile: &crate::config::Profile) -> Result<()> {
        // Add SSH key to agent
        Self::ensure_ssh_key_added(&profile.ssh_key_path)?;

        // Set Git config
        Ok(())
    }

    fn ensure_ssh_key_added(key_path: &Path) -> Result<()> {
        let pub_key_path = key_path.with_extension("pub");
        
        // Get fingerprint
        let output = Command::new("ssh-keygen")
            .args(["-lf", pub_key_path.to_str().unwrap()])
            .output()?;
        
        if !output.status.success() {
            return Err(GitOpusError::SshError(
                "Failed to get key fingerprint".into()
            ));
        }

        let fingerprint = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .nth(1)
            .unwrap_or_default()
            .to_string();

        // Check if already added
        let keys = Command::new("ssh-add").arg("-l").output()?;
        if !keys.status.success() {
            return Err(GitOpusError::SshError(
                "Failed to list SSH keys".into()
            ));
        }

        if !String::from_utf8_lossy(&keys.stdout).contains(&fingerprint) {
            let status = Command::new("ssh-add")
                .arg(key_path.to_str().unwrap())
                .status()?;
            
            if !status.success() {
                return Err(GitOpusError::SshError(
                    "Failed to add SSH key to agent".into()
                ));
            }
        }

        Ok(())
    }
}