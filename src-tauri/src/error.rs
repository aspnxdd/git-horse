use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GitError {
    RepoNotFound,
    NoBranches,
    GetDiffFailed,
    GetStatsFailed,
    GitError(String),
    SledError(String),
    SledCantInsert,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum SledError {
    SledError(String),
    SledCantInsert,
}

impl std::error::Error for GitError {}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<git2::Error> for GitError {
    fn from(err: git2::Error) -> Self {
        GitError::GitError(format!("{:#?}", err.message()))
    }
}

impl From<sled::Error> for SledError {
    fn from(err: sled::Error) -> Self {
        SledError::SledError(format!("{:#?}", err.to_string()))
    }
}

impl From<String> for SledError {
    fn from(err: String) -> Self {
        SledError::SledError(format!("{:#?}", err.to_string()))
    }
}
