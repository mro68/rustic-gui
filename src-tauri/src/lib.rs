// TODO.md: Phase 1 - Rust-Backend (Tauri 2-Befehle & Events) ✅ GRÖSSTENTEILS KOMPLETT
// Referenz: TODO.md Zeile 154-203
//
// Status-Übersicht:
// ✅ Grund-Setup (TODO.md Zeile 158-163): KOMPLETT
//    - AppState mit thread-sicheren Locks (state.rs)
//    - CancellationToken für Backup-Abbruch
//    - 24 Commands registriert in invoke_handler (Zeile 383-426)
//
// ✅ Repository-Management (TODO.md Zeile 164-174): REGISTRIERT (teilweise Stubs)
//    - list_repositories, delete_repository: IMPLEMENTIERT
//    - init_repository, open_repository: SIMULIERT
//    - check_repository, prune_repository, change_password: STUBS
//
// ✅ Backup-Job-Management (TODO.md Zeile 175-181): VOLLSTÄNDIG IMPLEMENTIERT
//    - Alle 5 Commands (list, create, update, delete, get) in commands/backup.rs
//
// ✅ Snapshot-Management (TODO.md Zeile 182-187): REGISTRIERT (teilweise Stubs)
//    - list_snapshots, get_snapshot, delete_snapshot, forget_snapshots: IMPLEMENTIERT in lib.rs
//    - compare_snapshots: STUB (auskommentiert in Zeile 422)
//
// ✅ Prozess-Steuerung (TODO.md Zeile 189-198): SIMULIERT mit Events
//    - run_backup (Zeile 121): Simuliert mit BackupEvent
//    - cancel_backup (Zeile 37): Implementiert mit CancellationToken
//    - restore_files_v1 (Zeile 324): Simuliert mit RestoreEvent
//    - get_file_tree_command (Zeile 312): Stub
//
// ⏳ Weitere Integration benötigt:
//    - Echte rustic_core Integration statt Simulationen
//    - Job-Scheduler für zeitgesteuerte Backups
//    - Vollständige Error-DTOs (types.rs:45-51)

pub mod commands;
pub mod config;
pub mod error;
pub mod keychain;
pub mod rustic;
pub mod state;
pub mod types;

use serde::Serialize;
use std::time::Duration;
use tauri::Emitter;
use tokio::time::sleep;
use types::{
    FileTreeNode, RepositoryDto, RestoreOptionsDto, RestoreProgress, RetentionPolicy, SnapshotDto,
};

/// Event-Format für Restore-Progress
#[derive(Serialize)]
struct RestoreEvent {
    #[serde(rename = "type")]
    event_type: String, // "progress" | "completed" | "error"
    progress: Option<RestoreProgress>,
    message: Option<String>,
    snapshotId: String,
    targetPath: String,
}

/// Event-Format für Backup-Abbruch
#[derive(Serialize)]
struct BackupCancelEvent {
    #[serde(rename = "type")]
    event_type: String, // "cancelled"
    jobId: String,
    message: Option<String>,
}

/// Tauri-Command: Bricht ein laufendes Backup ab und sendet ein Cancel-Event.
#[tauri::command]
async fn cancel_backup(
    app: tauri::AppHandle,
    job_id: String,
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<(), crate::types::ErrorDto> {
    let mut tokens = state.cancellation_tokens.lock();
    if let Some(token) = tokens.remove(&job_id) {
        token.cancel();
        let event = BackupCancelEvent {
            event_type: "cancelled".to_string(),
            jobId: job_id.clone(),
            message: Some("Backup wurde abgebrochen".to_string()),
        };
        let _ = app.emit("backup-cancelled", &event);
        tracing::info!(job = %job_id, "Backup-Abbruch ausgelöst");
        Ok(())
    } else {
        tracing::warn!(job = %job_id, "Kein laufender Backup-Job zum Abbrechen gefunden");
        Err(crate::types::ErrorDto {
            code: "BackupJobNotFound".to_string(),
            message: format!("Kein laufender Backup-Job mit ID '{}' gefunden", job_id),
            details: None,
        })
    }
}
#[tauri::command]
async fn forget_snapshots_command(
    repository_path: String,
    password: String,
    policy: RetentionPolicy,
) -> std::result::Result<Vec<String>, String> {
    rustic::snapshot::forget_snapshots(&repository_path, &password, &policy)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_snapshot_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
) -> std::result::Result<(), String> {
    rustic::snapshot::delete_snapshot(&repository_path, &password, &snapshot_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_snapshot_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
) -> std::result::Result<SnapshotDto, String> {
    rustic::snapshot::get_snapshot(&repository_path, &password, &snapshot_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_snapshots_command(
    repository_path: String,
    password: String,
) -> std::result::Result<Vec<SnapshotDto>, String> {
    rustic::snapshot::list_snapshots(&repository_path, &password).await.map_err(|e| e.to_string())
}

use rustic::backup::{BackupOptions, BackupProgress, run_backup};

/// Event-Format für Backup-Progress
#[derive(Serialize)]
struct BackupEvent {
    #[serde(rename = "type")]
    event_type: String, // "progress" | "completed" | "error"
    progress: Option<BackupProgress>,
    message: Option<String>,
    jobId: String,
}

/// Tauri-Command: Startet ein Backup und sendet Progress-, Completed- und Error-Events im einheitlichen Format an das Frontend.
#[tauri::command]
async fn run_backup_command(
    app: tauri::AppHandle,
    mut options: BackupOptions,
) -> std::result::Result<(), crate::types::ErrorDto> {
    tracing::info!("run_backup_command aufgerufen");
    let job_id = options.job_id.clone().unwrap_or_else(|| "default".to_string());
    options.job_id = Some(job_id.clone());

    // Closure für Progress-Events
    let app_progress = app.clone();
    let job_id_progress = job_id.clone();
    let progress_callback = move |progress: BackupProgress| {
        let event = BackupEvent {
            event_type: "progress".to_string(),
            progress: Some(progress.clone()),
            message: None,
            jobId: job_id_progress.clone(),
        };
        let _ = app_progress.emit("backup-progress", &event);
        tracing::debug!(
            files = progress.files_processed,
            bytes = progress.bytes_uploaded,
            percent = ?progress.percent,
            "Backup-Progress"
        );
    };

    // Backup ausführen und Events senden
    match run_backup(app.clone(), options, progress_callback).await {
        Ok(_) => {
            let event = BackupEvent {
                event_type: "completed".to_string(),
                progress: None,
                message: Some("Backup erfolgreich abgeschlossen".to_string()),
                jobId: job_id.clone(),
            };
            let _ = app.emit("backup-completed", &event);
            Ok(())
        }
        Err(e) => {
            let event = BackupEvent {
                event_type: "error".to_string(),
                progress: None,
                message: Some(format!("Backup fehlgeschlagen: {}", e)),
                jobId: job_id.clone(),
            };
            let _ = app.emit("backup-failed", &event);
            Err(crate::types::ErrorDto::from(&e))
        }
    }
}

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
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // 1. Repository initialisieren mit rustic_core
    let dto = rustic::repository::init_repository(&path, &password, &backend_type, backend_options)
        .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // 2. Repository-ID generieren
    let repo_id = dto.id.clone();

    // 3. Passwort in Keychain speichern
    let password_stored = match keychain::store_password(&repo_id, &password) {
        Ok(_) => {
            tracing::info!("Passwort für Repository '{}' in Keychain gespeichert", repo_id);
            true
        }
        Err(e) => {
            tracing::warn!(
                "Passwort konnte nicht in Keychain gespeichert werden: {}. Repository kann trotzdem verwendet werden.",
                e
            );
            false
        }
    };

    // 4. Repository in Config speichern
    {
        let mut config = state.config.lock();
        let repo_config = crate::config::RepositoryConfig {
            id: repo_id.clone(),
            name: dto.name.clone(),
            path: dto.path.clone(),
            backend_type: match backend_type.as_str() {
                "local" => crate::config::BackendType::Local,
                "sftp" => crate::config::BackendType::Sftp,
                "s3" => crate::config::BackendType::S3,
                "rest" => crate::config::BackendType::Rest,
                "rclone" => crate::config::BackendType::Rclone,
                _ => crate::config::BackendType::Local,
            },
            backend_options: None,
            password_stored,
        };
        config.add_repository(repo_config);
    }

    // 5. Config speichern
    state.save_config().map_err(|e| crate::types::ErrorDto {
        code: "ConfigError".to_string(),
        message: format!("Config-Speicherung fehlgeschlagen: {}", e),
        details: None,
    })?;

    Ok(dto)
}

#[tauri::command]
fn open_repository(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // Öffne das Repository intern
    let _repo = rustic::repository::open_repository(&path, &password).map_err(|e| crate::types::ErrorDto::from(&e))?;

    // Gib ein DTO zurück (ohne das Repository zu speichern)
    // Das eigentliche Speichern geschieht via switch_repository
    Ok(RepositoryDto {
        id: format!("repo-{}", std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("unknown")),
        name: std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("Unknown Repository").to_string(),
        path: path.clone(),
        repository_type: crate::types::RepositoryType::Local,
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count: 0, // TODO: Get from repo
        total_size: 0,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

#[tauri::command]
fn check_repository_v1(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    rustic::repository::check_repository(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

#[tauri::command]
fn get_repository_info(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    rustic::repository::get_repository_info(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

#[tauri::command]
fn prune_repository_v1(
    path: String,
    password: String,
    dry_run: bool,
) -> std::result::Result<(u32, u64), crate::types::ErrorDto> {
    rustic::repository::prune_repository(&path, &password, dry_run)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

#[tauri::command]
fn change_password_v1(
    path: String,
    old_password: String,
    new_password: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    rustic::repository::change_password(&path, &old_password, &new_password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

#[tauri::command]
fn switch_repository(
    repository_id: String,
    password: String,
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<types::RepositoryDto, crate::types::ErrorDto> {
    // 1. Altes Repository schließen (TODO M2: Wenn Caching implementiert ist)
    // Für M1: Wir öffnen Repositories bei Bedarf, kein Caching

    // 2. Repo-Config laden
    let (path, repo_name, backend_type) = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&repository_id)
            .ok_or_else(|| crate::error::RusticGuiError::RepositoryNotFound {
                path: format!("Repository-ID: {}", repository_id),
            })
            .map_err(|e| crate::types::ErrorDto::from(&e))?;

        (repo.path.clone(), repo.name.clone(), repo.backend_type.clone())
    };

    // 3. Repository öffnen und validieren (aber nicht im State speichern für M1)
    let opened = rustic::repository::open_repository(&path, &password).map_err(|e| crate::types::ErrorDto::from(&e))?;

    // 4. Repository-Info für Frontend erstellen
    let info = types::RepositoryDto {
        id: repository_id.clone(),
        name: repo_name,
        path: path.clone(),
        repository_type: match backend_type {
            crate::config::BackendType::Local => types::RepositoryType::Local,
            crate::config::BackendType::Sftp => types::RepositoryType::Sftp,
            crate::config::BackendType::S3 => types::RepositoryType::S3,
            crate::config::BackendType::Rest => types::RepositoryType::Rest,
            crate::config::BackendType::Rclone => types::RepositoryType::Rclone,
        },
        status: types::RepositoryStatus::Healthy,
        snapshot_count: opened.snapshot_count,
        total_size: opened.total_size,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // 6. TODO M2: Repository in State speichern für Performance
    // Für jetzt lassen wir es weg wegen Type-Komplexität

    // 7. Passwort aktualisieren in Keychain
    if let Err(e) = keychain::store_password(&repository_id, &password) {
        tracing::warn!("Passwort konnte nicht in Keychain aktualisiert werden: {}", e);
    }

    tracing::info!("Repository gewechselt: {} ({})", repository_id, path);

    Ok(info)
}

#[tauri::command]
fn prepare_shutdown(
    state: tauri::State<'_, state::AppState>,
) -> std::result::Result<bool, crate::types::ErrorDto> {
    // Prüfe ob laufende Backups existieren
    let running_backups = state.cancellation_tokens.lock().len();

    if running_backups > 0 {
        tracing::warn!("Shutdown verhindert: {} laufende Backups", running_backups);
        return Err(crate::types::ErrorDto {
            code: "ShutdownBlocked".to_string(),
            message: format!("Shutdown verhindert: {} laufende Backups", running_backups),
            details: None,
        });
    }

    // TODO: Weitere Cleanup-Logik (Scheduler stoppen, etc.)

    tracing::info!("Shutdown vorbereitet - keine laufenden Operationen");
    Ok(true) // Shutdown erlaubt
}

#[tauri::command]
fn store_repository_password(
    repo_id: String,
    password: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    keychain::store_password(&repo_id, &password).map_err(|e| crate::types::ErrorDto {
        code: "KeychainStoreFailed".to_string(),
        message: format!("Passwort speichern fehlgeschlagen: {}", e),
        details: None,
    })
}

#[tauri::command]
fn get_repository_password(repo_id: String) -> std::result::Result<String, crate::types::ErrorDto> {
    keychain::load_password(&repo_id).map_err(|e| crate::types::ErrorDto {
        code: "KeychainLoadFailed".to_string(),
        message: format!("Passwort laden fehlgeschlagen: {}", e),
        details: None,
    })
}

#[tauri::command]
fn delete_repository_password(repo_id: String) -> std::result::Result<(), crate::types::ErrorDto> {
    keychain::delete_password(&repo_id).map_err(|e| crate::types::ErrorDto {
        code: "KeychainDeleteFailed".to_string(),
        message: format!("Passwort löschen fehlgeschlagen: {}", e),
        details: None,
    })
}

#[tauri::command]
async fn get_file_tree_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
    path: Option<String>,
) -> std::result::Result<FileTreeNode, String> {
    rustic::restore::get_file_tree(&repository_path, &password, &snapshot_id, path.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn restore_files_v1(
    app: tauri::AppHandle,
    repository_path: String,
    password: String,
    snapshot_id: String,
    files: Vec<String>,
    target_path: String,
    options: RestoreOptionsDto,
) -> std::result::Result<(), String> {
    tracing::info!("restore_files_command aufgerufen");
    
    // Sende initialen Progress-Event
    let total = files.len().max(1) as u64;
    let initial_progress = RestoreProgress {
        base: types::ProgressInfo {
            current: 0,
            total,
            message: Some("Starte Restore...".to_string()),
            percentage: Some(0.0),
        },
        files_restored: 0,
        bytes_restored: 0,
        current_file: None,
    };
    let event = RestoreEvent {
        event_type: "progress".to_string(),
        progress: Some(initial_progress),
        message: None,
        snapshotId: snapshot_id.clone(),
        targetPath: target_path.clone(),
    };
    let _ = app.emit("restore-progress", &event);
    
    // Führe echten Restore aus
    match rustic::restore::restore_files(
        &repository_path,
        &password,
        &snapshot_id,
        files.clone(),
        &target_path,
        &options,
    ).await {
        Ok(_) => {
            // Sende finalen Progress-Event
            let final_progress = RestoreProgress {
                base: types::ProgressInfo {
                    current: total,
                    total,
                    message: None,
                    percentage: Some(100.0),
                },
                files_restored: total,
                bytes_restored: total * 1024, // Placeholder
                current_file: None,
            };
            let event = RestoreEvent {
                event_type: "progress".to_string(),
                progress: Some(final_progress),
                message: None,
                snapshotId: snapshot_id.clone(),
                targetPath: target_path.clone(),
            };
            let _ = app.emit("restore-progress", &event);
            
            // Sende Completed-Event
            let event = RestoreEvent {
                event_type: "completed".to_string(),
                progress: None,
                message: Some("Restore erfolgreich abgeschlossen".to_string()),
                snapshotId: snapshot_id,
                targetPath: target_path,
            };
            let _ = app.emit("restore-completed", &event);
            Ok(())
        }
        Err(e) => {
            let event = RestoreEvent {
                event_type: "error".to_string(),
                progress: None,
                message: Some(format!("Restore fehlgeschlagen: {}", e)),
                snapshotId: snapshot_id,
                targetPath: target_path,
            };
            let _ = app.emit("restore-failed", &event);
            Err(format!("Restore fehlgeschlagen: {}", e))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // App-State erstellen
    // TODO.md: Phase 1 - Grund-Setup ✅ KOMPLETT
    // - AppState mit thread-sicheren Locks (Parking_lot::Mutex)
    // - CancellationToken für Backup-Abbruch
    let app_state = state::AppState::new().expect("AppState initialisieren fehlgeschlagen");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // TODO.md: Phase 1 - Alle Backend-Commands registriert ✅
            // 24 Commands registriert, viele als Stubs (siehe TODO-Kommentare in jeweiligen Modulen)

            // --- System/Utility ---
            greet,
            prepare_shutdown,
            // --- Repository Management ---
            init_repository,
            open_repository,
            get_repository_info,
            check_repository_v1,
            prune_repository_v1,
            change_password_v1,
            switch_repository,
            store_repository_password,
            get_repository_password,
            delete_repository_password,
            // --- Backup-Jobs ---
            run_backup_command,
            cancel_backup,
            commands::backup::create_backup_job,
            commands::backup::update_backup_job,
            commands::backup::delete_backup_job,
            commands::backup::get_backup_job,
            commands::backup::list_backup_jobs,
            // --- Snapshot Management ---
            list_snapshots_command,
            get_snapshot_command,
            delete_snapshot_command,
            forget_snapshots_command,
            // --- Restore ---
            get_file_tree_command,
            restore_files_v1,
            // --- Additional Repository Commands ---
            commands::repository::list_repositories,
            commands::repository::delete_repository,
            commands::repository::check_repository,
            commands::repository::prune_repository,
            commands::repository::change_password,
            commands::repository::test_repository_connection,
            commands::snapshot::compare_snapshots,
            // --- Platzhalter für weitere geplante Commands (TODO) ---
            // commands::restore::restore_files_command, // TODO
            // commands::system::check_repository_health, // TODO
            // commands::system::force_unlock_repository, // TODO
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
