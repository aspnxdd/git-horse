use git2::BranchType;
use git2::{AutotagOption, FetchOptions, RemoteCallbacks, Repository};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::io::{self, Write};
use std::str;
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

#[command]
pub fn get_remotes(state: AppArg) -> Result<Vec<String>, PError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let remotes = repo.remotes()?;
        let mut result: Vec<String> = Vec::new();
        remotes.iter().for_each(|remote| {
            result.push(remote.unwrap().to_owned());
        });
        return Ok(result);
    }
    Err(PError::RepoNotFound)
}

#[command]
pub fn fetch_remote(state: AppArg, remote: Option<String>) -> Result<(), PError> {
    let remote = &remote.unwrap_or("origin".to_string());

    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        // Figure out whether it's a named remote or a URL
        println!(
            "Fetching {} for repo {}",
            remote,
            repo.path().to_str().unwrap()
        );
        let mut cb = RemoteCallbacks::new();
        let mut remote = repo
            .find_remote(remote)
            .or_else(|_| repo.remote_anonymous(remote))?;
        println!("remote: {:#?}", remote.name().unwrap());
        cb.sideband_progress(|data| {
            print!("remote: {}", str::from_utf8(data).unwrap());
            io::stdout().flush().unwrap();
            true
        });
        // This callback gets called for each remote-tracking branch that gets
        // updated. The message we output depends on whether it's a new one or an
        // update.
        cb.update_tips(|refname, a, b| {
            if a.is_zero() {
                println!("[new]     {:20} {}", b, refname);
            } else {
                println!("[updated] {:10}..{:10} {}", a, b, refname);
            }
            true
        });
        // Here we show processed and total objects in the pack and the amount of
        // received data. Most frontends will probably want to show a percentage and
        // the download rate.
        cb.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                print!(
                    "Resolving deltas {}/{}\r",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                );
            } else if stats.total_objects() > 0 {
                print!(
                    "Received {}/{} objects ({}) in {} bytes\r",
                    stats.received_objects(),
                    stats.total_objects(),
                    stats.indexed_objects(),
                    stats.received_bytes()
                );
            }
            io::stdout().flush().unwrap();
            true
        });
        // Download the packfile and index it. This function updates the amount of
        // received data and the indexer stats which lets you inform the user about
        // progress.
        // cb.credentials(|_url, username_from_url, _allowed_types| {
        //     git2::Cred::ssh_key(
        //       username_from_url.unwrap(),
        //       None,
        //       std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
        //       None,
        //     )
        //   });

        // let git_config = git2::Config::open_default().unwrap();
        // for entry in &git_config.entries(None).unwrap() {
        //     let entry = entry.unwrap();
        //     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
        // }
        // cb.credentials(move|_url, username_from_url, _allowed_types| {
        //     println!("_url {}", _url);
        //     println!("username_from_url {:#?}", username_from_url);

        //     git2::Cred::ssh_key(
        //         username_from_url.unwrap(),
        //         None,
        //         std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
        //         None,
        //     )
        // });
        let git_config = git2::Config::open_default().unwrap();
        let mut ch = git2_credentials::CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        remote.download(&[] as &[String], Some(&mut fo))?;
        {
            // If there are local objects (we got a thin pack), then tell the user
            // how many objects we saved from having to cross the network.
            let stats = remote.stats();
            if stats.local_objects() > 0 {
                println!(
                    "\rReceived {}/{} objects in {} bytes (used {} local \
                     objects)",
                    stats.indexed_objects(),
                    stats.total_objects(),
                    stats.received_bytes(),
                    stats.local_objects()
                );
            } else {
                println!(
                    "\rReceived {}/{} objects in {} bytes",
                    stats.indexed_objects(),
                    stats.total_objects(),
                    stats.received_bytes()
                );
            }
        }
        // Disconnect the underlying connection to prevent from idling.
        remote.disconnect()?;
        // Update the references in the remote's namespace to point to the right
        // commits. This may be needed even if there was no packfile to download,
        // which can happen e.g. when the branches have been changed but all the
        // needed objects are available locally.
        remote.update_tips(None, true, AutotagOption::Unspecified, None)?;
        return Ok(());
    }
    Err(PError::RepoNotFound)
}
