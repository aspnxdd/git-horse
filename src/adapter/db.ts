import type { Repos } from "src/shared/types";

import { invoke } from "@tauri-apps/api";

export function getLastOpenedRepoFromDb(): Promise<string> {
  return invoke<string>("read_last_opened_repo");
}

export function setLastOpenedRepoToDb(repo: string): Promise<void> {
  return invoke<void>("write_last_opened_repo", { repo });
}

export function getThemeFromDb(): Promise<string> {
  return invoke<string>("read_theme");
}

export function setThemeToDb(theme: string): Promise<void> {
  return invoke<void>("write_theme", { theme });
}

export function addNewRepoToDb(
  repoName: string,
  repoPath: string
): Promise<void> {
  return invoke("add_new_repo", {
    repoName,
    repoPath,
  });
}

export function getAllReposFromDb(): Promise<Repos[]> {
  return invoke("get_all_repos");
}
