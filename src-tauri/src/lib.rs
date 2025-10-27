use rustic::backup::{run_backup, BackupOptions, BackupProgress};

/// Tauri-Command: Startet ein Backup und sendet Progress-Events an das Frontend.
/// Erwartet BackupOptions mit job_id (für Event-Name) und sendet Fortschritt via emit.
#[tauri::command]
async fn run_backup_command(
    app: tauri::AppHandle,
    mut options: BackupOptions,
) -> std::result::Result<(), String> {
    tracing::info!("run_backup_command aufgerufen");
    if options.job_id.is_none() {
        options.job_id = Some("default".to_string());
    }
    let progress_callback = |progress: BackupProgress| {
        tracing::debug!(
            files = progress.files_processed,
            bytes = progress.bytes_uploaded,
            percent = ?progress.percent,
            "Backup-Progress"
        );
    };
    run_backup(app, options, progress_callback)
        .await
        .map_err(|e| e.to_string())
}
// Hauptmodul für rustic-gui
pub mod config;
pub mod error;
pub mod keychain;
pub mod state;
pub mod types;
pub mod rustic;

// Re-export wichtige Typen
pub use config::*;
pub use error::{RusticGuiError, Result};
pub use keychain::*;
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
) -> std::result::Result<RepositoryDto, String> {
    rustic::repository::init_repository(&path, &password, &backend_type, backend_options)
        .map_err(|e| e.into())
}

#[tauri::command]
fn open_repository(path: String, password: String) -> std::result::Result<RepositoryDto, String> {
    rustic::repository::open_repository(&path, &password)
        .map_err(|e| e.into())
}

#[tauri::command]
fn check_repository(path: String, password: String) -> std::result::Result<RepositoryDto, String> {
    rustic::repository::check_repository(&path, &password)
        .map_err(|e| e.into())
}

#[tauri::command]
fn get_repository_info(path: String, password: String) -> std::result::Result<RepositoryDto, String> {
    rustic::repository::get_repository_info(&path, &password)
        .map_err(|e| e.into())
}

#[tauri::command]
fn switch_repository(
    repository_id: String,
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<types::RepositoryDto, String> {
    // TODO: Implementiere vollständiges Repository-Switching
    // Für jetzt nur ein Platzhalter mit vereinfachter Logik
    
    // 1. Altes Repository schließen
    {
        let mut current = state.current_repo.lock();
        if let Some(_old_repo) = current.take() {
            // drop(old_repo) - automatisch via take()
            tracing::debug!("Altes Repository geschlossen");
        }
    }
    
    // 2. TODO: Repo-Config laden (Milestone 1.4)
    // Für jetzt simulieren wir eine Config
    
    // 3. TODO: Neues Repository öffnen (richtige rustic_core Integration)
    // Für jetzt nur Platzhalter
    
    // 4. Repository-Info für Frontend erstellen
    let info = types::RepositoryDto {
        id: repository_id.clone(),
        name: format!("Repository {}", repository_id),
        path: format!("/tmp/repo-{}", repository_id), // Platzhalter
        repository_type: types::RepositoryType::Local,
        status: types::RepositoryStatus::Healthy,
        snapshot_count: 0, // TODO: Aus Repository lesen
        total_size: 0, // TODO: Berechnen
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    // 5. TODO: Repository in State speichern (wenn rustic_core fertig)
    // Für jetzt lassen wir current_repo None
    
    tracing::info!("Repository gewechselt zu: {}", repository_id);
    
    Ok(info)
}

#[tauri::command]
fn prepare_shutdown(
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<bool, String> {
    // Prüfe ob laufende Backups existieren
    let running_backups = state.cancellation_tokens.lock().len();
    
    if running_backups > 0 {
        tracing::warn!("Shutdown verhindert: {} laufende Backups", running_backups);
        return Ok(false); // Shutdown nicht erlaubt
    }
    
    // TODO: Weitere Cleanup-Logik (Scheduler stoppen, etc.)
    
    tracing::info!("Shutdown vorbereitet - keine laufenden Operationen");
    Ok(true) // Shutdown erlaubt
}

#[tauri::command]
fn store_repository_password(
    repo_id: String,
    password: String,
) -> std::result::Result<(), String> {
    keychain::store_password(&repo_id, &password)
        .map_err(|e| format!("Passwort speichern fehlgeschlagen: {}", e))
}

#[tauri::command]
fn get_repository_password(
    repo_id: String,
) -> std::result::Result<String, String> {
    keychain::load_password(&repo_id)
        .map_err(|e| format!("Passwort laden fehlgeschlagen: {}", e))
}

#[tauri::command]
fn delete_repository_password(
    repo_id: String,
) -> std::result::Result<(), String> {
    keychain::delete_password(&repo_id)
        .map_err(|e| format!("Passwort löschen fehlgeschlagen: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // App-State erstellen
    let app_state = state::AppState::new().expect("AppState initialisieren fehlgeschlagen");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            init_repository,
            open_repository,
            get_repository_info,
            check_repository,
            switch_repository,
            prepare_shutdown,
            store_repository_password,
            get_repository_password,
            delete_repository_password,
            run_backup_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
