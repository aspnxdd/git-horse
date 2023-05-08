use git2::BranchType;
use git2::{AutotagOption, FetchOptions, PushOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;
use std::{fs, str, vec};
use tauri::command;

use crate::db;
use crate::error::{GitError, SledError};
use crate::git;
use crate::pull::{do_fetch, do_merge};
use crate::state::{AppArg, FileStatus, GitDiff, MyBranchType, MyState, Stats};
use crate::utils::{
    get_absolute_path_from_relative, get_origin_and_current_name_from_line, path_is_file,
};

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

#[command]
pub fn find_branches(state: AppArg, filter: Option<MyBranchType>) -> Result<Vec<String>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref().unwrap();
    let mut filters: Option<BranchType> = None;
    if let Some(branch_type) = filter {
        match branch_type {
            MyBranchType::Local => filters = Some(BranchType::Local),
            MyBranchType::Remote => filters = Some(BranchType::Remote),
        }
    }
    let branches = git2::Repository::branches(&repo.repo, filters);
    match branches {
        Ok(branches) => {
            let result: Vec<String> = branches
                .filter_map(|branch| {
                    if let Ok((branch, _)) = branch {
                        return Some(branch.name().unwrap().unwrap().to_string());
                    }
                    return None;
                })
                .collect::<Vec<String>>();

            return Ok(result);
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
pub fn get_repo_name(state: AppArg) -> Result<(String, String), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let (name, path) = repo.get_repo_name()?;
        return Ok((name, path));
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
pub fn checkout_remote_branch(state: AppArg, branch_name: String) -> Result<(), GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let mut remote = repo
            .repo
            .find_remote(git::DEFAULT_REMOTE)
            .or_else(|_| repo.repo.remote_anonymous(&git::DEFAULT_REMOTE))?;
        let remote_name = format!("{}/", remote.name().unwrap());
        let branch_name_no_origin = branch_name.replace(&remote_name, "");
        let cb = git::get_remote_callbacks();
        let connection = remote.connect_auth(git2::Direction::Fetch, Some(cb), None)?;
        let remote_ref_name = format!("refs/heads/{}", branch_name_no_origin);
        let remote_head = connection
            .list()?
            .iter()
            .find(|x| x.name() == &remote_ref_name)
            .ok_or(GitError::RemoteHeadNotFound)?;

        let remote_branch_oid = remote_head.oid();

        let mut reference = repo.repo.reference(
            &remote_ref_name,
            remote_branch_oid,
            true,
            &format!("Setting {} to {}", branch_name_no_origin, remote_branch_oid),
        )?;

        reference.set_target(remote_branch_oid, "checkout")?;

        let mut branch = git2::Branch::wrap(reference);

        branch.set_upstream(Some(&branch_name)).unwrap();

        repo.repo.set_head(&remote_ref_name)?;

        repo.repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

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
        let result: Vec<String> = remotes
            .iter()
            .map(|remote| remote.unwrap().to_owned())
            .collect::<Vec<String>>();

        return Ok(result);
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn fetch_remote(state: AppArg, remote: Option<String>) -> Result<(), GitError> {
    let remote = &remote.unwrap_or(git::DEFAULT_REMOTE.to_string());

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
    let remote = &remote.unwrap_or(git::DEFAULT_REMOTE.to_string());

    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let cb = git::get_remote_callbacks();
        let mut remote = repo
            .repo
            .find_remote(remote)
            .or_else(|_| repo.repo.remote_anonymous(remote))?;

        let head = repo.repo.head()?;
        let head = head.shorthand().unwrap();
        let mut conn = remote.connect_auth(git2::Direction::Push, Some(cb), None)?;
        let mut push_options = PushOptions::new();
        let refspecs = format!("refs/heads/{}", head.to_string());
        conn.remote().push(&[refspecs], Some(&mut push_options))?;
        conn.remote().disconnect()?;
        conn.remote()
            .update_tips(None, true, AutotagOption::Unspecified, None)?;
        return Ok(());
    }
    Err(GitError::RepoNotFound)
}

#[command]
pub fn pull_from_remote(state: AppArg, remote: Option<String>) -> Result<(), GitError> {
    let remote_name = &remote.unwrap_or(git::DEFAULT_REMOTE.to_string());
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
                if status.intersects(git::INTERESTING) {
                    if let Some(path) = entry.path() {
                        if !path_is_file(&path) {
                            return None;
                        }
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
        return Ok(Stats {
            deletions: stats.deletions(),
            insertions: stats.insertions(),
            files_changed: stats.files_changed(),
        });
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
                if status.intersects(git::INTERESTING_STAGED) {
                    if let Some(path) = entry.path() {
                        if !path_is_file(&path) {
                            return None;
                        }
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
        let mut files_to_add = vec![];
        for entry in statuses.iter() {
            for file in &files {
                let status = entry.status();
                if status.intersects(git::INTERESTING) {
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
            let target = head.target().ok_or_else(|| GitError::InvalidHead)?;

            // Get the object for the file in the last commit
            let obj = repo
                .repo
                .find_object(target, Some(git2::ObjectType::Commit))?;

            // Get the commit from the object
            let commit = obj.as_commit().ok_or_else(|| GitError::InvalidCommit)?;

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

#[command]
pub fn git_diff(state: AppArg) -> Result<Vec<GitDiff>, GitError> {
    let repo = state.repo.clone();
    let repo = repo.lock().unwrap();
    let repo = repo.as_ref();
    if let Some(repo) = repo {
        let head = repo.repo.head()?;

        // Get the last commit (OID) for the file
        let target = head.target().ok_or_else(|| GitError::InvalidHead)?;

        // Get the object for the file in the last commit
        let obj = repo
            .repo
            .find_object(target, Some(git2::ObjectType::Commit))?;

        // Get the commit from the object
        let commit = obj.as_commit().ok_or_else(|| GitError::InvalidCommit)?;

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
                let path = entry.path().unwrap();
                if !path_is_file(&path) {
                    return;
                }
                if head.is_some() && status.intersects(git::INTERESTING_STAGED) {
                    let head = head.unwrap();
                    if head.new_file().size() != 0 {
                        diff_opts.pathspec(Path::new(&path.to_owned()));
                    }
                } else {
                    if status.intersects(git::INTERESTING) {
                        diff_opts.pathspec(Path::new(&path.to_owned()));
                        files_paths.push(path.to_owned());
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
            .revparse_single(format!("{}/{}", git::DEFAULT_REMOTE, &branch_name).as_str())?
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
            .revparse_single(format!("{}/{}", git::DEFAULT_REMOTE, &branch_name).as_str())?
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
pub fn add_new_repo(repo_name: Option<String>, repo_path: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
    db.insert(repo_name.unwrap().as_str(), repo_path.unwrap().as_str())?;
    return Ok(());
}

#[command]
pub fn get_all_repos() -> Result<Vec<db::Repo>, SledError> {
    let db = db::Db::new()?;
    let res = db.get_all()?;
    return Ok(res);
}

#[command]
pub fn write_last_opened_repo(repo: Option<String>) -> Result<(), SledError> {
    let db = db::Db::new()?;
    let res = db.write_last_opened_repo(repo.unwrap().as_str())?;
    return Ok(res);
}

#[command]
pub fn read_last_opened_repo() -> Result<String, SledError> {
    let db = db::Db::new()?;
    let res = db.read_last_opened_repo()?;
    return Ok(res);
}

#[command]
pub fn read_theme() -> Result<String, SledError> {
    let db = db::Db::new()?;
    let res = db.read_theme()?;
    return Ok(res);
}

#[command]
pub fn write_theme(theme: String) -> Result<(), SledError> {
    let db = db::Db::new()?;
    let res = db.write_theme(theme.as_str())?;
    return Ok(res);
}
