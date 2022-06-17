use git2::BranchType;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use tauri::command;

use crate::error::PError;
use crate::git;
use crate::state::AppArg;

#[command]
pub fn open(state: AppArg, path: &str) -> Result<String, PError> {
    match git::Repo::open(path) {
        Ok(repo) => {
            let mut handle = state.repo.lock().unwrap();

            *handle = Some(repo.repo);
            return Ok(handle
                .as_ref()
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .to_string());
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(PError::RepoNotFound);
        }
    }
}
// #[serde(remote = "BranchType")]
// #[derive(Debug, Serialize, Deserialize)]
// enum LocalBranchType {
//     Local,
//     Remote,
// }
// #[serde_as("LocalBranchType")]
#[derive(Serialize, Deserialize, Debug)]
pub enum MyBranchType {
    Local,
    Remote,
}

#[command]
pub fn find_branches(state: AppArg, filter: Option<MyBranchType>) -> Result<Vec<String>, PError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref().unwrap();
    let mut filters: Option<BranchType> = None;
    if let Some(fu) = filter {
        match fu {
            MyBranchType::Local => filters = Some(BranchType::Local),
            MyBranchType::Remote => filters = Some(BranchType::Remote),
        }
    }
    let branches = git2::Repository::branches(repo, filters);
    match branches {
        Ok(branches) => {
            let mut result = Vec::new();
            for branch in branches {
                if let Ok((branch, _)) = branch {
                    result.push(branch.name().unwrap().unwrap().to_string());
                }
            }
            println!("res: {:#?}", result);
            Ok(result)
        }
        Err(_) => {
            return Err(PError::NoBranches);
        }
    }
}

#[command]
pub fn get_current_branch_name(state: AppArg) -> Result<String, PError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let head = repo.head()?;
        println!("head: {:#?}", head.name());
        let head = head.shorthand().unwrap();
        return Ok(head.to_string());
    }
    Err(PError::RepoNotFound)
}

#[command]
pub fn get_repo_name(state: AppArg) -> Result<String, PError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let path_name = repo.path().to_str().unwrap().to_string();
        let repo_name: Vec<&str> = path_name.split("/").collect();
        return Ok(repo_name[repo_name.len() - 3].to_owned());
    }
    Err(PError::RepoNotFound)
}

#[command]
pub fn checkout_branch(state: AppArg, branch_name: String) -> Result<(), PError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        if let Ok(branch) = repo.find_branch(&branch_name, BranchType::Local) {
            if branch.get().is_branch() {
                let branch_ref = branch.get().name().unwrap();
                println!("branch_ref: {}", branch_ref);
                repo.set_head(&branch_ref)?;
                return Ok(());
            }
        }

        // Should create local branch of the remote branch
        if let Ok(branch) = repo.find_branch(&branch_name, BranchType::Remote) {
            if branch.get().is_remote() {
                let branch_ref = branch.get().name().unwrap();
                println!("branch_ref: {}", branch_ref);
                // let new_branch = repo.branch(&branch_ref, BranchType::Local)?;
                repo.set_head(&branch_ref)?;
                return Ok(());
            }
        }
    }
    Err(PError::RepoNotFound)
}
