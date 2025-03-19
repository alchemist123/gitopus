use crate::error::*;
use std::process::{Command, Stdio};

pub struct GitManager;

impl GitManager {
    pub fn execute_git_command(profile: &crate::config::Profile, args: &[String]) -> Result<()> {
        let status = Command::new("git")
            .env("GIT_COMMITTER_NAME", &profile.name)
            .env("GIT_COMMITTER_EMAIL", &profile.email)
            .env("GIT_AUTHOR_NAME", &profile.name)
            .env("GIT_AUTHOR_EMAIL", &profile.email)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(GitOpusError::GitError(
                format!("Git command failed with exit code: {:?}", status.code())
            ))
        }
    }

    pub fn get_remote_domains() -> Result<Vec<String>> {
        let output = Command::new("git")
            .args(["remote", "-v"])
            .output()?;

        if !output.status.success() {
            return Err(GitOpusError::GitError(
                "Failed to get git remotes".into()
            ));
        }

        let remotes = String::from_utf8_lossy(&output.stdout);
        Ok(Self::parse_remote_domains(&remotes))
    }

    fn parse_remote_domains(remotes: &str) -> Vec<String> {
        remotes.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                parts.get(1).and_then(|url| Self::extract_domain(url))
            })
            .collect()
    }

    fn extract_domain(url: &str) -> Option<String> {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.split('/').nth(2).map(|s| s.to_string())
        } else if let Some(at_pos) = url.find('@') {
            url[at_pos+1..]
                .split(|c| c == ':' || c == '/')
                .next()
                .map(|s| s.to_string())
        } else {
            None
        }
    }
}