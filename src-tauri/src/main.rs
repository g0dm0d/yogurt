#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

mod accounts;
mod instances;
mod minecraft;
mod mods;
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
            instances::config::get_all_instances,
            minecraft::run_minecraft::run_minecraft,
            instances::explorer::open_instance_folder,
            mods::fabric::install_fabric,
            instances::config::make_copy_instance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
