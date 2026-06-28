use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ShipflowError>;

#[derive(Debug, Error)]
pub enum ShipflowError {
    #[error("task not found: {0}")]
    TaskNotFound(String),

    #[error("ambiguous match — multiple tasks match '{query}':\n{matches}")]
    AmbiguousMatch { query: String, matches: String },

    #[error("storage error at {path}: {message}")]
    Storage { path: PathBuf, message: String },

    #[error("git error: {0}")]
    Git(String),

    #[error("invalid commit: {0}")]
    InvalidCommit(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl ShipflowError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::TaskNotFound(_) | Self::AmbiguousMatch { .. } | Self::InvalidCommit(_) => 1,
            Self::Storage { .. } | Self::Io(_) | Self::Serde(_) => 3,
            Self::Git(_) => 1,
        }
    }
}
