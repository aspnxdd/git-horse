use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PError {
    RepoNotFound,
    NoBranches,
    GitError(String),
}

impl std::error::Error for PError {}

impl std::fmt::Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<git2::Error> for PError {
    fn from(err: git2::Error) -> Self {
        PError::GitError(format!("{:#?}", err.message()))
    }
}
