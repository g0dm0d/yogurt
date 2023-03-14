use std::{fs, path::PathBuf};

use crate::tools::path::get_path;

/// Create a copy of the instance
#[tauri::command(async)]
pub async fn make_copy_instance(name: String) {
    copy_recursively(
        get_path(&format!("instances/{name}")),
        get_path(&format!("instances/{name}(copy)")),
    );
    fs::copy(
        get_path(&format!("configs/{name}.toml")),
        get_path(&format!("configs/{name}(copy).toml")),
    )
    .unwrap();
}

pub fn copy_recursively(source: PathBuf, destination: PathBuf) {
    fs::create_dir_all(&destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let filetype = entry.file_type().unwrap();
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
        }
    }
}

#[tauri::command(async)]
pub async fn delete_instance(name: String) {
    fs::remove_file(get_path(&format!("configs/{name}.toml"))).unwrap();
    fs::remove_dir_all(get_path(&format!("instances/{name}"))).unwrap();
}
