mod config;
mod profile;
mod git;
mod error;

use clap::{Parser, Subcommand};
use crate::error::Result;
use crate::config::{ConfigManager, Profile};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "GitOpus - Git multi-account management tool\nVersion: 0.2.0",
    name = "gitopus"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage profiles
    Profile {
        #[command(subcommand)]
        action: ProfileCommands,
    },
    /// Execute Git commands
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ProfileCommands {
    /// Create new profile
    Create {
        name: String,
        email: String,
        username: String,
        ssh_key_path: String,
        domain: String,
    },
    /// List all profiles
    List,
    /// Set default profile
    SetDefault {
        name: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Profile { action } => handle_profile_command(action),
        Commands::Git { args } => handle_git_command(&args),
    }
}

fn handle_profile_command(action: ProfileCommands) -> Result<()> {
    let mut config = ConfigManager::new()?;

    match action {
        ProfileCommands::Create { name, email, username, ssh_key_path, domain } => {
            let profile = Profile {
                name,
                email,
                username,
                ssh_key_path: ssh_key_path.into(),
                domain,
                default: false,
            };
            config.add_profile(profile)?;
            println!("Profile created successfully!");
            Ok(())
        }
        ProfileCommands::List => {
            for profile in config.get_profiles() {
                println!(
                    "{} ({}): {} - {}",
                    profile.name,
                    if profile.default { "default" } else { "" },
                    profile.email,
                    profile.domain
                );
            }
            Ok(())
        }
        ProfileCommands::SetDefault { name } => {
            config.set_default_profile(&name)?;
            println!("Default profile set to {}", name);
            Ok(())
        }
    }
}

fn handle_git_command(args: &[String]) -> Result<()> {
    let config = ConfigManager::new()?;
    let domains = git::GitManager::get_remote_domains()?;
    
    // Find matching profile
    let profile = config.get_profiles().iter()
        .find(|p| domains.iter().any(|d| d == &p.domain))
        .or_else(|| config.get_profiles().iter().find(|p| p.default))
        .ok_or_else(|| error::GitOpusError::Other("No matching profile found".into()))?;

    profile::ProfileManager::activate_profile(profile)?;
    git::GitManager::execute_git_command(profile, args)
}