use std::sync::{Arc, Mutex};

use git2;
#[derive(Default)]
pub struct MyState {
    pub repo: Arc<Mutex<Option<git2::Repository>>>,
}
pub type AppArg<'a> = tauri::State<'a, MyState>;

