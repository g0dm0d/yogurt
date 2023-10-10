use std::process::Command;

use crate::tools::path::get_path;

#[cfg(any(target_os = "macos", target_os = "linux"))]
const PATH: &str = "instances/";

#[cfg(target_os = "windows")]
const PATH: &str = "instances\\";

#[cfg(target_os = "macos")]
const OPEN_COMMAND: &str = "open";

#[cfg(target_os = "windows")]
const OPEN_COMMAND: &str = "explorer";

#[cfg(target_os = "linux")]
const OPEN_COMMAND: &str = "xdg-open";

/// Open the folder with the game files of this instance
#[tauri::command(async)]
pub async fn open_instance_folder(name: String) -> Result<(), String> {
    Command::new(OPEN_COMMAND)
        .arg(get_path(&format!("{PATH}{name}")))
        .spawn()
        .map_err(|err| err.to_string())?;
    Ok(())
}
