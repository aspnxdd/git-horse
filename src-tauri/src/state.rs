use crate::git::Repo;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MyState {
    pub repo: Arc<Mutex<Option<Repo>>>,
}

pub type AppArg<'a> = tauri::State<'a, MyState>;

#[derive(Serialize, Deserialize, Debug)]
pub enum MyBranchType {
    Local,
    Remote,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileStatus {
    pub file_name: String,
    pub status: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub deletions: usize,
    pub insertions: usize,
    pub files_changed: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiff {
    pub diff_content: String,
    pub new_line: Option<u32>,
    pub old_line: Option<u32>,
    pub origin: char,
}
