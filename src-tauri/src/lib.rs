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
// ✅ Snapshot-Management (TODO.md Zeile 182-187): VOLLSTÄNDIG IMPLEMENTIERT
//    - list_snapshots, get_snapshot, delete_snapshot, forget_snapshots: IMPLEMENTIERT
//    - compare_snapshots: ✅ IMPLEMENTIERT (Tree-basierter Diff mit rustic CLI Pattern)
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

use tauri::Emitter;

pub mod commands;
pub mod config;
pub mod error;
pub mod keychain;
pub mod rustic;
pub mod scheduler;
pub mod state;
pub mod storage;
pub mod types;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Stellt geplante Jobs beim App-Start wieder her
///
/// # Arguments
/// * `state` - AppState-Referenz
/// * `app_handle` - App-Handle für Events
///
/// # Returns
/// Result mit Anzahl wiederhergestellter Jobs
async fn restore_scheduled_jobs(
    state: &state::AppState,
    app_handle: &tauri::AppHandle,
) -> anyhow::Result<usize> {
    let jobs_to_schedule: Vec<_> = {
        let config = state.config.lock();
        config
            .backup_jobs
            .iter()
            .filter(|j| j.enabled && j.schedule.is_some())
            .map(|j| (j.id.clone(), j.schedule.clone().unwrap()))
            .collect()
    };

    let mut restored_count = 0;

    for (job_id, cron_expression) in jobs_to_schedule {
        tracing::info!("Stelle geplanten Job wieder her: {} ({})", job_id, cron_expression);

        let job_id_clone = job_id.clone();
        let app_handle_clone = app_handle.clone();
        let state_clone = state.clone();

        let mut scheduler_lock = state.scheduler.lock().await;
        if let Some(scheduler) = scheduler_lock.as_mut() {
            match scheduler
                .schedule_job(job_id.clone(), &cron_expression, move || {
                    let job_id = job_id_clone.clone();
                    let app_handle = app_handle_clone.clone();
                    let _state = state_clone.clone();

                    Box::pin(async move {
                        tracing::info!("Scheduled backup gestartet: {}", job_id);

                        // Event: Backup gestartet
                        let _ = app_handle.emit(
                            "scheduled-backup-started",
                            serde_json::json!({
                                "job_id": job_id,
                                "time": chrono::Utc::now().to_rfc3339(),
                            }),
                        );

                        // TODO M3.3: Integriere run_backup hier
                        // Vorläufig: Simuliere Backup-Ausführung
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                        // Event: Backup abgeschlossen
                        let _ = app_handle.emit(
                            "scheduled-backup-completed",
                            serde_json::json!({
                                "job_id": job_id,
                                "time": chrono::Utc::now().to_rfc3339(),
                            }),
                        );

                        tracing::info!("Scheduled backup abgeschlossen: {}", job_id);
                    })
                })
                .await
            {
                Ok(_) => {
                    restored_count += 1;
                    tracing::info!("Job '{}' erfolgreich wiederhergestellt", job_id);
                }
                Err(e) => {
                    tracing::error!("Fehler beim Wiederherstellen von Job '{}': {}", job_id, e);
                }
            }
        }
    }

    tracing::info!("{} geplante Jobs wiederhergestellt", restored_count);
    Ok(restored_count)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // App-State erstellen
    // TODO.md: Phase 1 - Grund-Setup ✅ KOMPLETT
    // - AppState mit thread-sicheren Locks (Parking_lot::Mutex)
    // - CancellationToken für Backup-Abbruch
    let app_state = state::AppState::new().expect("AppState initialisieren fehlgeschlagen");
    let app_state_clone = app_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Scheduler async initialisieren
            tauri::async_runtime::block_on(async {
                app_state_clone
                    .init_scheduler()
                    .await
                    .expect("Scheduler-Initialisierung fehlgeschlagen");

                // M3 Task 3.2.2: Lade gespeicherte scheduled Jobs aus Config
                restore_scheduled_jobs(&app_state_clone, &app_handle)
                    .await
                    .expect("Wiederherstellung geplanter Jobs fehlgeschlagen");
            });

            // Status des portablen Speichers an das Frontend senden
            app_state_clone.emit_portable_status_event(&app_handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // --- System/Utility ---
            greet,
            commands::system::prepare_shutdown,
            commands::system::get_portable_status,
            // --- Repository Management ---
            commands::repository::init_repository,
            commands::repository::open_repository,
            commands::repository::get_repository_info,
            commands::repository::switch_repository,
            commands::repository::store_repository_password,
            commands::repository::get_repository_password,
            commands::repository::delete_repository_password,
            commands::repository::list_repositories,
            commands::repository::delete_repository,
            commands::repository::check_repository,
            commands::repository::prune_repository_v1,
            commands::repository::change_password_v1,
            commands::repository::prune_repository,
            commands::repository::change_password,
            commands::repository::test_repository_connection,
            commands::repository::save_favorite_location,
            commands::repository::list_favorite_locations,
            commands::repository::update_favorite_last_used,
            commands::repository::delete_favorite_location,
            commands::repository::get_repository_stats,
            // --- Backup-Jobs ---
            commands::backup::run_backup,
            commands::backup::run_backup_command,
            commands::backup::cancel_backup,
            commands::backup::create_backup_job,
            commands::backup::update_backup_job,
            commands::backup::delete_backup_job,
            commands::backup::get_backup_job,
            commands::backup::list_backup_jobs,
            commands::backup::schedule_backup,
            commands::backup::unschedule_backup,
            commands::backup::list_scheduled_backups,
            commands::backup::list_job_history,
            // --- Snapshot Management ---
            commands::snapshot::list_snapshots_command,
            commands::snapshot::list_snapshots_filtered_command,
            commands::snapshot::get_snapshot_command,
            commands::snapshot::delete_snapshot_command,
            commands::snapshot::forget_snapshots,
            commands::snapshot::compare_snapshots,
            commands::snapshot::add_snapshot_tags,
            commands::snapshot::remove_snapshot_tags,
            // --- Retention Policy ---
            commands::retention::preview_retention_policy,
            commands::retention::apply_retention_policy,
            // --- Restore ---
            commands::restore::get_file_tree_command,
            commands::restore::restore_files_v1,
            // --- Settings ---
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::reset_settings,
            commands::settings::update_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
