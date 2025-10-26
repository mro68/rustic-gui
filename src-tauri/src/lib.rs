// Hauptmodul für rustic-gui
pub mod error;
pub mod types;
pub mod rustic;

// Re-export wichtige Typen
pub use error::{RusticGuiError, Result};
pub use types::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_repository(
    path: String,
    password: String,
    backend_type: String,
    backend_options: Option<serde_json::Value>,
) -> Result<RepositoryDto> {
    rustic::repository::init_repository(&path, &password, &backend_type, backend_options)
}

#[tauri::command]
fn open_repository(path: String, password: String) -> Result<RepositoryDto> {
    rustic::repository::open_repository(&path, &password)
}

#[tauri::command]
fn get_repository_info(path: String, password: String) -> Result<RepositoryDto> {
    rustic::repository::get_repository_info(&path, &password)
}

#[tauri::command]
fn check_repository(path: String, password: String) -> Result<RepositoryDto> {
    rustic::repository::check_repository(&path, &password)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            init_repository,
            open_repository,
            get_repository_info,
            check_repository
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
