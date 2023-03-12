use std::process::Command;

use crate::tools::path::get_path;

#[cfg(target_os = "macos")]
const OPEN_COMMAND: &str = "open";

#[cfg(target_os = "windows")]
const OPEN_COMMAND: &str = "explorer";

#[cfg(target_os = "linux")]
const OPEN_COMMAND: &str = "xdg-open";

/// Open the data folder from the game
#[tauri::command(async)]
pub async fn open_instance_folder(name: String) {
    Command::new(OPEN_COMMAND)
        .arg(get_path(&format!("instances/{}", name)))
        .spawn()
        .unwrap();
}

