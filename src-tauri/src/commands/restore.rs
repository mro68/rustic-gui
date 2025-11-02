// TODO.md: Phase 1 - Restore Commands ✅ IMPLEMENTIERT
// Status: Verschoben von lib.rs zu commands/restore.rs
// Referenz: TODO.md Zeile 195-198
//
// Verwendung in Frontend: src/lib/api/restore.ts, RestoreDialog.svelte

use crate::types::{FileTreeNode, RestoreOptionsDto, RestoreProgress};
use serde::Serialize;
use tauri::Emitter;

/// Event-Format für Restore-Progress
#[derive(Serialize)]
struct RestoreEvent {
    #[serde(rename = "type")]
    event_type: String, // "progress" | "completed" | "error"
    progress: Option<RestoreProgress>,
    message: Option<String>,
    #[serde(rename = "snapshotId")]
    snapshot_id: String,
    #[serde(rename = "targetPath")]
    target_path: String,
}

/// Holt den Dateibaum eines Snapshots für File-Browser
#[tauri::command]
pub async fn get_file_tree_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
    path: Option<String>,
) -> std::result::Result<FileTreeNode, String> {
    crate::rustic::restore::get_file_tree(
        &repository_path,
        &password,
        &snapshot_id,
        path.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())
}

/// Restore-Operation mit Progress-Events
#[tauri::command]
pub async fn restore_files_v1(
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
        base: crate::types::ProgressInfo {
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
        snapshot_id: snapshot_id.clone(),
        target_path: target_path.clone(),
    };
    let _ = app.emit("restore-progress", &event);

    // Führe echten Restore aus
    match crate::rustic::restore::restore_files(
        &repository_path,
        &password,
        &snapshot_id,
        files.clone(),
        &target_path,
        &options,
        Some(app.clone()),
    )
    .await
    {
        Ok(_) => {
            // Sende finalen Progress-Event
            let final_progress = RestoreProgress {
                base: crate::types::ProgressInfo {
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
                snapshot_id: snapshot_id.clone(),
                target_path: target_path.clone(),
            };
            let _ = app.emit("restore-progress", &event);

            // Sende Completed-Event
            let event = RestoreEvent {
                event_type: "completed".to_string(),
                progress: None,
                message: Some("Restore erfolgreich abgeschlossen".to_string()),
                snapshot_id: snapshot_id,
                target_path: target_path,
            };
            let _ = app.emit("restore-completed", &event);
            Ok(())
        }
        Err(e) => {
            let event = RestoreEvent {
                event_type: "error".to_string(),
                progress: None,
                message: Some(format!("Restore fehlgeschlagen: {}", e)),
                snapshot_id: snapshot_id,
                target_path: target_path,
            };
            let _ = app.emit("restore-failed", &event);
            Err(format!("Restore fehlgeschlagen: {}", e))
        }
    }
}
