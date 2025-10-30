// TODO.md: Phase 1 - Restore Commands
// Status: ✅ Teilweise implementiert in lib.rs (restore_files_v1, get_file_tree_command)
// Implementierung: ⏳ Stub-Implementation, benötigt rustic_core Integration
// Referenz: TODO.md Zeile 195-198
//
// Verwendung in Frontend: src/lib/api/restore.ts, RestoreDialog.svelte
// Note: restore_files_v1 in lib.rs:314-371 simuliert Restore mit Events
//       get_file_tree_command in lib.rs:312 nutzt rustic/restore.rs:181

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
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 195-196 (restore_files_v1 in lib.rs ist aktuell aktiv)
    // Frontend: RestoreDialog.svelte:243 wartet auf vollständige Implementation
    Err("restore_files_command: Noch nicht implementiert".into())
}

/// Listet Dateien eines Snapshots (File-Browser)
#[tauri::command]
pub async fn list_snapshot_files(
    repository_id: String,
    snapshot_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<FileTreeNode, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 198 (get_file_tree_command in lib.rs ist aktuell aktiv)
    // Frontend: RestoreDialog.svelte:242 wartet auf vollständige Implementation
    Err("list_snapshot_files: Noch nicht implementiert".into())
}