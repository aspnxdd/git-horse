use git2::{AutotagOption, FetchOptions, PushOptions, RemoteCallbacks};
use git2::{BranchType, DiffFormat, DiffLine};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::str;
use tauri::command;

use crate::db;
use crate::error::{GitError, SledError};
use crate::git;
use crate::state::AppArg;

const INTERESTING: git2::Status = git2::Status::from_bits_truncate(
    git2::Status::WT_NEW.bits()
        | git2::Status::CONFLICTED.bits()
        | git2::Status::WT_MODIFIED.bits()
        | git2::Status::WT_DELETED.bits(),
);
const INTERESTING_STAGED: git2::Status = git2::Status::from_bits_truncate(
    git2::Status::INDEX_DELETED.bits()
        | git2::Status::INDEX_MODIFIED.bits()
        | git2::Status::INDEX_MODIFIED.bits()
        | git2::Status::INDEX_RENAMED.bits()
        | git2::Status::INDEX_TYPECHANGE.bits(),
);
#[command]
pub fn open(state: AppArg, path: &str) -> Result<String, GitError> {
    println!("path = {}", path);
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
            return Err(GitError::RepoNotFound);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MyBranchType {
    Local,
    Remote,
}

#[command]
pub fn find_branches(state: AppArg, filter: Option<MyBranchType>) -> Result<Vec<String>, GitError> {
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
            return Err(GitError::NoBranches);
        }
    }
}

#[command]
pub fn get_current_branch_name(state: AppArg) -> Result<String, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let head = repo.head()?;
        println!("head: {:#?}", head.name());
        let head = head.shorthand().unwrap();
        return Ok(head.to_string());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_repo_name(state: AppArg) -> Result<String, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let path_name = repo.path().to_str().unwrap().to_string();
        let repo_name: Vec<&str> = path_name.split("/").collect();
        return Ok(repo_name[repo_name.len() - 3].to_owned());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn checkout_branch(state: AppArg, branch_name: String) -> Result<(), GitError> {
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
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_remotes(state: AppArg) -> Result<Vec<String>, GitError> {
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
    Err(GitError::RepoNotFound)
}

#[command]
pub fn fetch_remote(state: AppArg, remote: Option<String>) -> Result<(), GitError> {
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
    Err(GitError::RepoNotFound)
}

#[command]
pub fn push_remote(state: AppArg, remote: Option<String>) -> Result<(), GitError> {
    let remote = &remote.unwrap_or("origin".to_string());

    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        // Figure out whether it's a named remote or a URL
        println!(
            "pushing {} for repo {}",
            remote,
            repo.path().to_str().unwrap()
        );
        let mut cb = RemoteCallbacks::new();
        let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;
        let git_config = git2::Config::open_default().unwrap();
        let mut ch = git2_credentials::CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });
        let head = repo.head()?;
        let head = head.shorthand().unwrap();
        let mut conn = remote.connect_auth(git2::Direction::Push, Some(cb), None)?;
        let mut po = PushOptions::new();
        let refspecs = format!("refs/heads/{}", head.to_string());
        conn.remote().push(&[refspecs], Some(&mut po))?;

        conn.remote().disconnect()?;

     
        conn.remote()
            .update_tips(None, true, AutotagOption::Unspecified, None)?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileStatus {
    file_name: String,
    status: u32,
}
#[command]
pub fn get_modified_files(state: AppArg) -> Result<Vec<FileStatus>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    let mut status_options = git2::StatusOptions::new();

    if let Some(repo) = repo {
        let statuses = repo.statuses(Some(
            status_options
                .include_ignored(false)
                .include_untracked(true),
        ))?;

        let files_statuses = statuses
            .iter()
            .filter_map(|entry| {
                let status = entry.status();
                if status.intersects(INTERESTING) {
                    if let Some(path) = entry.path() {
                        let f = FileStatus {
                            file_name: path.to_owned(),
                            status: status.bits(),
                        };
                        return Some(f);
                    } else {
                        return None;
                    };
                } else {
                    return None;
                }
            })
            .collect();
        return Ok(files_statuses);
    }
    Err(GitError::RepoNotFound)
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    deletions: usize,
    insertions: usize,
    files_changed: usize,
}
#[command]
pub fn get_repo_diff(state: AppArg) -> Result<Stats, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let stats = match repo.diff_index_to_workdir(None, None) {
            Ok(diff) => match diff.stats() {
                Ok(stats) => stats,
                Err(e) => {
                    println!("get stats failed : {:?}", e);
                    return Err(GitError::GetStatsFailed);
                }
            },
            Err(e) => {
                println!("get diff failed : {:?}", e);
                return Err(GitError::GetDiffFailed);
            }
        };
        let stats = Stats {
            deletions: stats.deletions(),
            insertions: stats.insertions(),
            files_changed: stats.files_changed(),
        };
        return Ok(stats);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn add_all(state: AppArg) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        for s in repo.statuses(None).unwrap().iter() {
            if s.status() != git2::Status::IGNORED {
                println!("{:?}", s.path());
            }
        }
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}
#[command]
pub fn get_staged_files(state: AppArg) -> Result<Vec<String>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let mut status_options = git2::StatusOptions::new();
        let files = repo
            .statuses(Some(status_options.include_ignored(false)))?
            .iter()
            .filter_map(|s| {
                if s.status().intersects(INTERESTING_STAGED) {
                    return Some(s.path().unwrap().to_owned());
                }
                None
            })
            .collect();
        return Ok(files);
    }
    Err(GitError::RepoNotFound)
}
#[command]
pub fn add(state: AppArg, files: Vec<String>) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let statuses = repo.statuses(None).unwrap();
        let mut files_to_add = Vec::new();
        for entry in statuses.iter() {
            for file in &files {
                let status = entry.status();
                if status.intersects(INTERESTING) {
                    if let Some(path) = entry.path() {
                        if path == file {
                            files_to_add.push(path.to_owned());
                        }
                    }
                }
            }
        }
        let mut index = repo.index()?;
        index.add_all(files_to_add.iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiff {
    diff_content: String,
    new_line: Option<u32>,
    old_line: Option<u32>,
    origin: char,
}

#[command]
pub fn git_diff(state: AppArg) -> Result<Vec<GitDiff>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let index = repo.index()?;
        let mut lines: Vec<GitDiff> = Vec::new();
        let diff = repo.diff_index_to_workdir(Some(&index), None)?;
        diff.print(git2::DiffFormat::Patch, |_, _, l| {
            let line = str::from_utf8(l.content()).unwrap().to_owned();
            lines.push(GitDiff {
                diff_content: line,
                new_line: l.new_lineno(),
                old_line: l.old_lineno(),
                origin: l.origin(),
            });
            return true;
        })?;
        // println!("lines: {:?}", lines);
        return Ok(lines);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn commit(state: AppArg, message: String) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let mut index = repo.index().unwrap();
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let parent = repo.head()?.peel_to_commit()?;
        let parent_id = parent.id();
        let parent_id = repo.find_commit(parent_id)?;
        let commit = repo.commit_create_buffer(
            &repo.signature().unwrap(),
            &repo.signature().unwrap(),
            &message,
            &tree,
            &[&parent_id],
        )?;
        let commit_signed = repo.commit_signed(
            &str::from_utf8(&commit).unwrap().to_string(),
            &repo.signature().unwrap().to_string(),
            None,
        )?;
        let commit_id = repo.find_commit(commit_signed)?;
        let head = repo.head()?;
        let head_id = head.peel_to_commit()?.id();
        if head_id == commit_id.id() {
            return Ok(());
        }
        let mut head_ref = repo.head().unwrap();
        head_ref.set_target(commit_id.id(), "commit")?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn db_insert(key: Option<String>, value: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
    println!("key {:#?}", key);
    println!("value {:#?}", value);
    db.insert(key.unwrap().as_str(), value.unwrap().as_str())?;
    return Ok(());
}

#[command]
pub fn db_get(key: Option<String>) -> Result<String, SledError> {
    let db = db::Db::new()?;
    let res = db.get(key.unwrap().as_str())?;
    Ok(res)
}

#[command]
pub fn db_remove(key: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
    db.remove(key.unwrap().as_str())?;
    return Ok(());
}

#[command]
pub fn db_get_all() -> Result<Vec<db::Repo>, SledError> {
    let db = db::Db::new()?;
    let res = db.get_all()?;
    return Ok(res);
}

#[command]
pub fn write_last_opened_repo(key: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
    let res = db.write_last_opened_repo(key.unwrap().as_str())?;
    return Ok(res);
}

#[command]
pub fn read_last_opened_repo() -> Result<String, SledError> {
    let db = db::Db::new()?;
    let res = db.read_last_opened_repo()?;
    return Ok(res);
}
