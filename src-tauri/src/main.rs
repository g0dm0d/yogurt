#![cfg_attr(
    all(not(debug_assertions), target_os = "linux"),
    windows_subsystem = "linux"
)]

mod minecraft;
mod tools;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            minecraft::get_minecraft::get_minecraft,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
