use crate::types::FileTreeNode;
use crate::state::AppState;

/// Restore-Operation für ausgewählte Dateien
#[tauri::command]
pub async fn restore_files_command(
    repository_id: String,
    snapshot_id: String,
    files: Vec<String>,
    target_path: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren
    Err("restore_files_command: Noch nicht implementiert".into())
}

/// Listet Dateien eines Snapshots (File-Browser)
#[tauri::command]
pub async fn list_snapshot_files(
    repository_id: String,
    snapshot_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<FileTreeNode, String> {
    // TODO: Implementieren
    Err("list_snapshot_files: Noch nicht implementiert".into())
}