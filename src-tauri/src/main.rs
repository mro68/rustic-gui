// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// --- Rustic GUI: Tauri Main Entry ---
// Best-Practice: AppState, Command-Registrierung, DTO-Sync, Events

fn main() {
    // Starte die Tauri-App über das Library-Modul
    rustic_gui_lib::run()
}

// Hinweis: Die eigentliche App-Initialisierung und Command-Registrierung erfolgt in src/lib.rs (siehe run())
// - AppState wird dort erstellt und via .manage() geteilt
// - invoke_handler! registriert alle Commands (siehe TODO.md)
// - Weitere Commands und State-Objekte bitte in src/lib.rs und src/commands/* ergänzen
