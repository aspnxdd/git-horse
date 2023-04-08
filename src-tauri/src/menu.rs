use tauri::{CustomMenuItem, Menu as TauriMenu, Submenu};

pub struct Menu {}
impl Menu {
    pub fn new() -> TauriMenu {
        TauriMenu::os_default("Tauri Vue Template").add_submenu(Submenu::new(
            "Help",
            TauriMenu::with_items([CustomMenuItem::new(
                "Online Documentation",
                "Online Documentation",
            )
            .into()]),
        ))
    }
}
