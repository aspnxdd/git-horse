#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::shell;
use tauri::Manager;

mod cmd;
mod error;
mod git;
mod menu;
mod state;
#[tauri::command]
fn backend_add(number: i32) -> i32 {
    println!("Backend was called with an argument: {}", number);
    return number + 2;
}

fn main() {
    tauri::Builder::default()
        .manage(state::MyState::default())
        .invoke_handler(tauri::generate_handler![
            backend_add,
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
            cmd::add
        ])
        .menu(menu::Menu::new())
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/Uninen/tauri-vue-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
