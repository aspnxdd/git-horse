use git2::BranchType;
use git2::{AutotagOption, FetchOptions, PushOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;
use std::{fs, str};
use tauri::command;

use crate::db;
use crate::error::{GitError, SledError};
use crate::git;
use crate::pull::{do_fetch, do_merge};
use crate::state::AppArg;
use crate::utils::{get_absolute_path_from_relative, get_origin_and_current_name_from_line};

const INTERESTING: git2::Status = git2::Status::from_bits_truncate(
    git2::Status::WT_NEW.bits()
        | git2::Status::CONFLICTED.bits()
        | git2::Status::WT_MODIFIED.bits()
        | git2::Status::WT_DELETED.bits(),
);
const INTERESTING_STAGED: git2::Status = git2::Status::from_bits_truncate(
    git2::Status::INDEX_DELETED.bits()
        | git2::Status::INDEX_MODIFIED.bits()
        | git2::Status::INDEX_NEW.bits()
        | git2::Status::INDEX_RENAMED.bits()
        | git2::Status::INDEX_TYPECHANGE.bits(),
);
#[command]
pub fn open(state: AppArg, path: &str) -> Result<String, GitError> {
    match git::Repo::open(path) {
        Ok(repo) => {
            let mut handle = state.repo.lock().unwrap();
            *handle = Some(repo);
            return Ok(handle
                .as_ref()
                .unwrap()
                .repo
                .path()
                .to_str()
                .unwrap()
                .to_string());
        }
        Err(e) => {
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
    let branches = git2::Repository::branches(&repo.repo, filters);
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
        let branch = repo.get_current_branch_name()?;
        return Ok(branch);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_repo_name(state: AppArg) -> Result<String, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let name = repo.get_repo_name()?;
        return Ok(name);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn checkout_branch(state: AppArg, branch_name: String) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        repo.checkout_branch(&branch_name)?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_remotes(state: AppArg) -> Result<Vec<String>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let remotes = repo.repo.remotes()?;
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
        let mut cb = RemoteCallbacks::new();
        let mut remote = repo
            .repo
            .find_remote(remote)
            .or_else(|_| repo.repo.remote_anonymous(remote))?;
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
        let mut cb = RemoteCallbacks::new();
        let mut remote = repo
            .repo
            .find_remote(remote)
            .or_else(|_| repo.repo.remote_anonymous(remote))?;
        let git_config = git2::Config::open_default().unwrap();
        let mut ch = git2_credentials::CredentialHandler::new(git_config);
        cb.credentials(move |url, username, allowed| {
            ch.try_next_credential(url, username, allowed)
        });
        let head = repo.repo.head()?;
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

#[command]
pub fn pull_from_remote(state: AppArg, remote: Option<String>) -> Result<(), GitError> {
    let remote_name = &remote.unwrap_or("origin".to_string());
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let remote_branch = repo.get_current_branch_name()?;
        let mut remote = repo.repo.find_remote(remote_name)?;
        let fetch_commit = do_fetch(&repo.repo, &[&remote_branch], &mut remote)?;
        do_merge(&repo.repo, &remote_branch, fetch_commit)?;
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
        let statuses = repo.repo.statuses(Some(
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
                        let file_status = FileStatus {
                            file_name: path.to_owned(),
                            status: status.bits(),
                        };
                        return Some(file_status);
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
        let stats = match repo.repo.diff_index_to_workdir(None, None) {
            Ok(diff) => match diff.stats() {
                Ok(stats) => stats,
                Err(e) => {
                    return Err(GitError::GetStatsFailed);
                }
            },
            Err(e) => {
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
        let mut index = repo.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_staged_files(state: AppArg) -> Result<Vec<FileStatus>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let mut status_options = git2::StatusOptions::new();
        let files_statuses = repo
            .repo
            .statuses(Some(status_options.include_ignored(false)))?
            .iter()
            .filter_map(|entry| {
                let status = entry.status();
                if status.intersects(INTERESTING_STAGED) {
                    if let Some(path) = entry.path() {
                        let file_status = FileStatus {
                            file_name: path.to_owned(),
                            status: status.bits(),
                        };
                        return Some(file_status);
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
#[command]
pub fn add(state: AppArg, files: Vec<String>) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let statuses = repo.repo.statuses(None).unwrap();
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
        let mut index = repo.repo.index()?;
        index.add_all(files_to_add.iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn remove(state: AppArg, files: Vec<String>) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let statuses = repo.repo.statuses(None).unwrap();
        let mut files_to_discard = Vec::new();
        for entry in statuses.iter() {
            for file in &files {
                let status = entry.status();
                if status.intersects(INTERESTING) {
                    if let Some(path) = entry.path() {
                        if path == file {
                            files_to_discard.push(path.to_owned());
                        }
                    }
                }
            }
        }
        println!("{:?}", files_to_discard);
        let cb = &mut |path: &Path, _matched_spec: &[u8]| -> i32 { 0 as i32 };
        let mut index = repo.repo.index()?;
        index.remove_all(files_to_discard.iter(), Some(cb))?;
        index.write()?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn discard(state: AppArg, files: Vec<String>) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        for file in files {
            let file_abs_path = get_absolute_path_from_relative(repo, &file);
            let file_path = Path::new(&file_abs_path);
            let head = repo.repo.head()?;

            // Get the last commit (OID) for the file
            let target = head
                .target()
                .ok_or_else(|| git2::Error::from_str("invalid head"))?;

            // Get the object for the file in the last commit
            let obj = repo
                .repo
                .find_object(target, Some(git2::ObjectType::Commit))?;

            // Get the commit from the object
            let commit = obj
                .as_commit()
                .ok_or_else(|| git2::Error::from_str("invalid commit"))?;

            let tree = commit.tree()?;
            let entry = tree.get_path(Path::new(&file))?;
            let blob = repo.repo.find_blob(entry.id())?;

            // Write the contents of the blob to the file in the working directory
            let contents = blob.content();
            fs::write(file_path, contents).unwrap();

            // Add the file to the index
            let mut index = repo.repo.index()?;
            index.add_path(Path::new(&file))?;
            index.write()?;
        }

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
        let head = repo.repo.head()?;

        // Get the last commit (OID) for the file
        let target = head
            .target()
            .ok_or_else(|| git2::Error::from_str("invalid head"))?;

        // Get the object for the file in the last commit
        let obj = repo
            .repo
            .find_object(target, Some(git2::ObjectType::Commit))?;

        // Get the commit from the object
        let commit = obj
            .as_commit()
            .ok_or_else(|| git2::Error::from_str("invalid commit"))?;

        let tree = commit.tree()?;

        let mut status_options = git2::StatusOptions::new();
        let mut diff_opts = git2::DiffOptions::new();

        let mut files_paths = vec![];
        repo.repo
            .statuses(Some(
                status_options
                    .include_ignored(false)
                    .include_untracked(true),
            ))?
            .iter()
            .for_each(|entry| {
                let head = entry.head_to_index();
                let status = entry.status();
                let path = entry.path();
                if head.is_some() && status.intersects(INTERESTING_STAGED) {
                    let head = head.unwrap();
                    if head.new_file().size() != 0 {
                        diff_opts.pathspec(Path::new(&path.unwrap().to_owned()));
                    }
                } else {
                    if status.intersects(INTERESTING) {
                        diff_opts.pathspec(Path::new(&path.unwrap().to_owned()));
                        files_paths.push(path.unwrap().to_owned());
                    }
                }
            });
        let diff = repo
            .repo
            .diff_tree_to_workdir_with_index(Some(&tree), Some(&mut diff_opts))?;

        let mut lines: Vec<GitDiff> = Vec::new();

        diff.print(git2::DiffFormat::Patch, |_, _, line_diff| {
            let line = str::from_utf8(line_diff.content()).unwrap().to_owned();
            lines.push(GitDiff {
                diff_content: line,
                new_line: line_diff.new_lineno(),
                old_line: line_diff.old_lineno(),
                origin: line_diff.origin(),
            });
            if line_diff.origin() == 'F' {
                let line = str::from_utf8(line_diff.content()).unwrap().to_owned();
                let (origin_name, current_name) = get_origin_and_current_name_from_line(&line);
                files_paths = files_paths
                    .iter()
                    .filter(|x| x != &&origin_name && x != &&current_name)
                    .map(|x| x.to_owned())
                    .collect();
            }
            return true;
        })?;

        for file_path in files_paths {
            let file_abs_path = get_absolute_path_from_relative(repo, &file_path);
            let file = fs::read_to_string(Path::new(&file_abs_path)).unwrap();
            lines.push(GitDiff {
                diff_content: format!("diff --git a/{} b/{}", file_path, file_path),
                new_line: None,
                old_line: None,
                origin: 'F',
            });

            for (idx, line) in file.lines().enumerate() {
                lines.push(GitDiff {
                    diff_content: format!("{}\n", line.to_owned()),
                    new_line: Some(idx as u32 + 1),
                    old_line: Some(idx as u32 + 1),
                    origin: '+',
                });
            }
        }
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
        let mut index = repo.repo.index().unwrap();
        let oid = index.write_tree()?;
        let tree = repo.repo.find_tree(oid)?;
        let parent = repo.repo.head()?.peel_to_commit()?;
        let parent_id = parent.id();
        let parent_id = repo.repo.find_commit(parent_id)?;
        let commit = repo.repo.commit_create_buffer(
            &repo.repo.signature().unwrap(),
            &repo.repo.signature().unwrap(),
            &message,
            &tree,
            &[&parent_id],
        )?;
        let commit_signed = repo.repo.commit_signed(
            &str::from_utf8(&commit).unwrap().to_string(),
            &repo.repo.signature().unwrap().to_string(),
            None,
        )?;
        let commit_id = repo.repo.find_commit(commit_signed)?;
        let head = repo.repo.head()?;
        let head_id = head.peel_to_commit()?.id();
        if head_id == commit_id.id() {
            return Ok(());
        }
        let mut head_ref = repo.repo.head().unwrap();
        head_ref.set_target(commit_id.id(), "commit")?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_pending_commits_to_push(state: AppArg) -> Result<u32, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let branch_name = repo.get_current_branch_name()?;
        let local_branch_oid = repo.repo.revparse_single(&branch_name)?.id();
        let remote_branch_oid = repo
            .repo
            .revparse_single(format!("origin/{}", &branch_name).as_str())?
            .id();
        let local_branch = repo.repo.find_commit(local_branch_oid)?;
        let remote_branch = repo.repo.find_commit(remote_branch_oid)?;
        let mut revwalk = repo.repo.revwalk()?;

        // At the start the revwalk is empty, we add the top/tip commit and all its ancestors
        revwalk.push(local_branch.id())?;
        // With hide we remove the latest commit in remote branch and all its ancestors
        revwalk.hide(remote_branch.id())?;
        return Ok(revwalk.count() as u32);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn get_pending_commits_to_pull(state: AppArg) -> Result<u32, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let branch_name = repo.get_current_branch_name()?;
        let local_branch_oid = repo.repo.revparse_single(&branch_name)?.id();
        let remote_branch_oid = repo
            .repo
            .revparse_single(format!("origin/{}", &branch_name).as_str())?
            .id();
        let local_branch = repo.repo.find_commit(local_branch_oid)?;
        let remote_branch = repo.repo.find_commit(remote_branch_oid)?;
        let mut revwalk = repo.repo.revwalk()?;

        // At the start the revwalk is empty, we add the top/tip commit and all its ancestors
        revwalk.push(remote_branch.id())?;
        // With hide we remove the latest commit in remote branch and all its ancestors
        revwalk.hide(local_branch.id())?;
        return Ok(revwalk.count() as u32);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn db_insert(key: Option<String>, value: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
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
