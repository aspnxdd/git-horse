use crate::git::Repo;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MyState {
    pub repo: Arc<Mutex<Option<Repo>>>,
}
pub type AppArg<'a> = tauri::State<'a, MyState>;
