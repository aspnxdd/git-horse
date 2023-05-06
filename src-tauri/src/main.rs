#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::Manager;

mod cmd;
mod db;
mod error;
mod git;
mod menu;
mod pull;
mod state;
mod utils;

fn main() {
    tauri::Builder::default()
        .manage(state::MyState::default())
        .invoke_handler(tauri::generate_handler![
            cmd::open,
            cmd::find_branches,
            cmd::get_current_branch_name,
            cmd::get_repo_name,
            cmd::checkout_branch,
            cmd::get_remotes,
            cmd::fetch_remote,
            cmd::get_modified_files,
            cmd::get_repo_diff,
            cmd::commit,
            cmd::add_all,
            cmd::add,
            cmd::remove,
            cmd::discard,
            cmd::get_staged_files,
            cmd::get_all_repos,
            cmd::add_new_repo,
            cmd::write_last_opened_repo,
            cmd::read_last_opened_repo,
            cmd::git_diff,
            cmd::push_remote,
            cmd::get_pending_commits_to_push,
            cmd::get_pending_commits_to_pull,
            cmd::pull_from_remote,
            cmd::read_theme,
            cmd::write_theme,
            cmd::checkout_remote_branch
        ])
        .menu(menu::Menu::new())
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/aspnxdd/git-horse".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
