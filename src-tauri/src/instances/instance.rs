use std::{fs, path::PathBuf};

use crate::tools::path::get_path;

/// Create a copy of the instance
#[tauri::command(async)]
pub async fn copy_instance(name: String) -> Result<(), String> {
    copy_recursively(
        get_path(&format!("instances/{name}")),
        get_path(&format!("instances/{name}(copy)")),
    )?;
    fs::copy(
        get_path(&format!("configs/{name}.toml")),
        get_path(&format!("configs/{name}(copy).toml")),
    )
    .map_err(|err| err.to_string())?;
    Ok(())
}

pub fn copy_recursively(source: PathBuf, destination: PathBuf) -> Result<(), String> {
    fs::create_dir_all(&destination).map_err(|err| err.to_string())?;
    for entry in fs::read_dir(source).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let filetype = entry.file_type().map_err(|err| err.to_string())?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name()))
                .map_err(|err| err.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command(async)]
pub async fn delete_instance(name: String) -> Result<(), String> {
    fs::remove_file(get_path(&format!("configs/{name}.toml"))).map_err(|err| err.to_string())?;
    fs::remove_dir_all(get_path(&format!("instances/{name}"))).map_err(|err| err.to_string())?;
    Ok(())
}
