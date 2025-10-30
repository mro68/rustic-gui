use crate::types::{SnapshotDto, DiffResultDto};
use crate::state::AppState;

/// Listet alle Snapshots eines Repositories
#[tauri::command]
pub async fn list_snapshots(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SnapshotDto>, String> {
    // TODO: Implementieren
    Err("list_snapshots: Noch nicht implementiert".into())
}

/// Holt Details zu einem Snapshot
#[tauri::command]
pub async fn get_snapshot_info(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<SnapshotDto, String> {
    // TODO: Implementieren
    Err("get_snapshot_info: Noch nicht implementiert".into())
}

/// Vergleicht zwei Snapshots
#[tauri::command]
pub async fn compare_snapshots(
    id_a: String,
    id_b: String,
    state: tauri::State<'_, AppState>,
) -> Result<DiffResultDto, String> {
    // TODO: Implementieren
    Err("compare_snapshots: Noch nicht implementiert".into())
}

/// Löscht einen Snapshot
#[tauri::command]
pub async fn delete_snapshot(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren
    Err("delete_snapshot: Noch nicht implementiert".into())
}

/// Vergisst Snapshots gemäß Policy
#[tauri::command]
pub async fn forget_snapshots(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    // TODO: Implementieren
    Err("forget_snapshots: Noch nicht implementiert".into())
}