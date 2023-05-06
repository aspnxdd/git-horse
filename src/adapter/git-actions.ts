import type { FileStatus, GitDiff, RepoDiffStats } from "src/shared/types";

import { invoke } from "@tauri-apps/api";

export function addAllFiles(): Promise<void> {
  return invoke("add_all");
}

export function addFiles(files: string[]): Promise<void> {
  return invoke("add", { files });
}

export function discardFiles(files: string[]): Promise<void> {
  return invoke("discard", { files });
}

export function commit(message: string): Promise<void> {
  return invoke("commit", { message });
}

export function getGitDiff(): Promise<GitDiff[]> {
  return invoke("git_diff");
}

export function getModifiedFiles(): Promise<FileStatus[]> {
  return invoke("get_modified_files");
}

export function getStagedFiles(): Promise<FileStatus[]> {
  return invoke("get_staged_files");
}

export function getRepoDiff(): Promise<RepoDiffStats> {
  return invoke("get_repo_diff");
}

export function openRepo(path: string): Promise<void> {
  return invoke("open", { path });
}

export function getRepoName(): Promise<[string, string]> {
  return invoke("get_repo_name");
}

export function findBranches(filter: "Local" | "Remote"): Promise<string[]> {
  return invoke("find_branches", { filter });
}

export function getCurrentBranchName(): Promise<string> {
  return invoke("get_current_branch_name");
}

export function getPendingCommitsToPush(): Promise<number> {
  return invoke("get_pending_commits_to_push");
}

export function getPendingCommitsToPull(): Promise<number> {
  return invoke("get_pending_commits_to_pull");
}

export function checkoutBranch(branchName: string): Promise<void> {
  return invoke("checkout_branch", { branchName });
}

export function checkoutRemoteBranch(branchName: string): Promise<void> {
  return invoke("checkout_remote_branch", { branchName });
}

export function getRemotes(): Promise<string[]> {
  return invoke("get_remotes");
}

export function fetchRemote(): Promise<void> {
  return invoke("fetch_remote");
}

export function pullRemote(): Promise<void> {
  return invoke("pull_from_remote");
}

export function pushRemote(): Promise<void> {
  return invoke("push_remote");
}
