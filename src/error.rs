use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitOpusError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("SSH error: {0}")]
    SshError(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, GitOpusError>;