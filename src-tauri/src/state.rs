use std::sync::{Arc, Mutex};
use crate::git::Repo;

use git2;
#[derive(Default)]
pub struct MyState {
    pub repo: Arc<Mutex<Option<Repo>>>,
}
pub type AppArg<'a> = tauri::State<'a, MyState>;

