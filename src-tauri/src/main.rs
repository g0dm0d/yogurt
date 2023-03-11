#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "linux"
)]

use std::fs;

mod accounts;
mod minecraft;
mod tools;

fn main() {
    let result = fs::create_dir_all(tools::path::get_path(""));
    if result.is_err() {
        panic!("Failed to create directory: {:?}", result.err());
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            minecraft::get_minecraft::get_minecraft,
            accounts::add_account::add_account,
            accounts::account::get_all_users,
            minecraft::config::get_all_instances,
            minecraft::run_minecraft::run_minecraft,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
