// TODO.md: Phase 1 - Snapshot-Management Commands
// Status: ✅ Registriert in lib.rs (auskommentiert, da noch nicht fertig)
// Implementierung: ⏳ Alle Commands sind Stubs, benötigen rustic_core Integration
// Referenz: TODO.md Zeile 182-187
//
// Verwendung in Frontend: src/lib/api/snapshots.ts
// Note: Die meisten Snapshot-Operationen werden aktuell direkt in lib.rs implementiert
//       (list_snapshots_command, get_snapshot_command, delete_snapshot_command, forget_snapshots_command)
//       Diese Commands hier sind für erweiterte Funktionen gedacht.

use crate::state::AppState;
use crate::types::{DiffResultDto, SnapshotDto};

/// Listet alle Snapshots eines Repositories
#[tauri::command]
pub async fn list_snapshots(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SnapshotDto>, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 183 (list_snapshots implementiert, aber in lib.rs:96)
    Err("list_snapshots: Noch nicht implementiert".into())
}

/// Holt Details zu einem Snapshot
#[tauri::command]
pub async fn get_snapshot_info(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<SnapshotDto, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 184 (get_snapshot implementiert, aber in lib.rs:84)
    Err("get_snapshot_info: Noch nicht implementiert".into())
}

/// Vergleicht zwei Snapshots
#[tauri::command]
pub async fn compare_snapshots(
    id_a: String,
    id_b: String,
    state: tauri::State<'_, AppState>,
) -> Result<DiffResultDto, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 186 (auskommentiert in lib.rs:422)
    // Frontend: CompareSnapshotsDialog.svelte wartet auf diese Implementation
    Err("compare_snapshots: Noch nicht implementiert".into())
}

/// Löscht einen Snapshot
#[tauri::command]
pub async fn delete_snapshot(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 185 (delete_snapshot implementiert, aber in lib.rs:73)
    Err("delete_snapshot: Noch nicht implementiert".into())
}

/// Löscht Snapshots gemäß Policy
#[tauri::command]
pub async fn forget_snapshots(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 187 (forget_snapshots implementiert, aber in lib.rs:62)
    Err("forget_snapshots: Noch nicht implementiert".into())
}

/// Fügt Tags zu einem Snapshot hinzu
#[tauri::command]
pub async fn add_snapshot_tags(
    snapshot_id: String,
    tags: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    tracing::debug!(
        "add_snapshot_tags called: snapshot_id={}, tags={:?}",
        snapshot_id,
        tags
    );

    // Get current repository
    let repo_guard = state.current_repository.lock().map_err(|e| e.to_string())?;
    let repo = repo_guard
        .as_ref()
        .ok_or("Kein Repository geöffnet")?;

    // Load snapshot
    let mut snapshot = repo
        .get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Add tags
    let tag_lists: Vec<rustic_core::StringList> = tags
        .into_iter()
        .map(|t| rustic_core::StringList::from_str(&t))
        .collect();

    if snapshot.add_tags(tag_lists) {
        // Save updated snapshot
        repo.save_snapshot(&snapshot)
            .map_err(|e| format!("Snapshot speichern fehlgeschlagen: {}", e))?;
        
        tracing::info!("Tags erfolgreich zu Snapshot {} hinzugefügt", snapshot_id);
        Ok(())
    } else {
        Err("Keine neuen Tags hinzugefügt".into())
    }
}

/// Entfernt Tags von einem Snapshot
#[tauri::command]
pub async fn remove_snapshot_tags(
    snapshot_id: String,
    tags: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    tracing::debug!(
        "remove_snapshot_tags called: snapshot_id={}, tags={:?}",
        snapshot_id,
        tags
    );

    // Get current repository
    let repo_guard = state.current_repository.lock().map_err(|e| e.to_string())?;
    let repo = repo_guard
        .as_ref()
        .ok_or("Kein Repository geöffnet")?;

    // Load snapshot
    let mut snapshot = repo
        .get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Remove tags
    let tag_lists: Vec<rustic_core::StringList> = tags
        .into_iter()
        .map(|t| rustic_core::StringList::from_str(&t))
        .collect();

    if snapshot.remove_tags(&tag_lists) {
        // Save updated snapshot
        repo.save_snapshot(&snapshot)
            .map_err(|e| format!("Snapshot speichern fehlgeschlagen: {}", e))?;
        
        tracing::info!("Tags erfolgreich von Snapshot {} entfernt", snapshot_id);
        Ok(())
    } else {
        Err("Keine Tags entfernt".into())
    }
}
