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
use crate::types::SnapshotDto;

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
/// TODO Phase 1: Refactoring benötigt - rustic_core 0.8.0 API hat sich geändert
/// - Kein `node_from_snapshot()` mehr verfügbar
/// - Muss mit `get_snapshots()` und Tree-Streamer neu implementiert werden
/// - Siehe backup-restore-snapshots.instructions.md für korrekte Implementierung
/*
#[tauri::command]
pub async fn compare_snapshots(
    snapshot_id_a: String,
    snapshot_id_b: String,
    state: tauri::State<'_, AppState>,
) -> Result<DiffResultDto, String> {
    tracing::debug!(
        "compare_snapshots called: id_a={}, id_b={}",
        snapshot_id_a,
        snapshot_id_b
    );

    // Get current repository
    let repo = state
        .get_current_repository_id()
        .ok_or("Kein Repository ausgewählt")?;
    
    let repo = state
        .get_repository(&repo)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Load both snapshots
    let snapshot_a = repo
        .get_snapshot_from_str(&snapshot_id_a)
        .map_err(|e| format!("Snapshot A nicht gefunden: {}", e))?;

    let snapshot_b = repo
        .get_snapshot_from_str(&snapshot_id_b)
        .map_err(|e| format!("Snapshot B nicht gefunden: {}", e))?;

    // Get trees for both snapshots
    let tree_a = repo
        .node_from_snapshot(&snapshot_a)
        .map_err(|e| format!("Tree A laden fehlgeschlagen: {}", e))?;

    let tree_b = repo
        .node_from_snapshot(&snapshot_b)
        .map_err(|e| format!("Tree B laden fehlgeschlagen: {}", e))?;

    // Compare trees to find differences
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();

    // Build path maps for efficient lookup
    use std::collections::HashMap;
    let mut paths_a: HashMap<String, _> = HashMap::new();
    let mut paths_b: HashMap<String, _> = HashMap::new();

    // Collect all paths from snapshot A
    for entry in tree_a.recurse_entries(&repo, None).flatten() {
        if let Ok(path_str) = entry.path().to_str() {
            paths_a.insert(
                path_str.to_string(),
                (entry.meta().size, entry.meta().mtime),
            );
        }
    }

    // Collect all paths from snapshot B
    for entry in tree_b.recurse_entries(&repo, None).flatten() {
        if let Ok(path_str) = entry.path().to_str() {
            let path = path_str.to_string();
            let size_b = entry.meta().size;
            let mtime_b = entry.meta().mtime;

            if let Some((size_a, mtime_a)) = paths_a.remove(&path) {
                // Path exists in both - check if modified
                if size_a != size_b || mtime_a != mtime_b {
                    modified.push(path.clone());
                }
            } else {
                // Path only in B - it was added
                added.push(path.clone());
            }

            paths_b.insert(path, (size_b, mtime_b));
        }
    }

    // Remaining paths in A were removed in B
    removed.extend(paths_a.into_keys());

    // Calculate statistics
    let added_count = added.len() as u64;
    let removed_count = removed.len() as u64;
    let modified_count = modified.len() as u64;

    // Calculate size changes
    let size_a = snapshot_a.summary.as_ref().map(|s| s.total_bytes_processed).unwrap_or(0);
    let size_b = snapshot_b.summary.as_ref().map(|s| s.total_bytes_processed).unwrap_or(0);
    let total_size_change = (size_b as i64) - (size_a as i64);

    tracing::info!(
        "Snapshot-Vergleich abgeschlossen: +{} -{} ~{}",
        added_count,
        removed_count,
        modified_count
    );

    Ok(DiffResultDto {
        added,
        removed,
        modified,
        stats: DiffStats {
            added_count,
            removed_count,
            modified_count,
            total_size_change,
        },
    })
}
*/

/// Löscht einzelnen Snapshot
#[tauri::command]
pub async fn delete_snapshot(id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: Phase 1 Zeile 185 (delete_snapshot implementiert, aber in lib.rs:73)
    Err("delete_snapshot: Noch nicht implementiert".into())
}

/// Löscht Snapshots gemäß Policy (Batch-Operation)
/// M4.5: Batch-Operations
#[tauri::command]
pub async fn forget_snapshots(
    snapshot_ids: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<usize, String> {
    use rustic_core::repofile::SnapshotId;
    
    tracing::info!("Lösche {} Snapshots (Batch)", snapshot_ids.len());

    let repo_id = state
        .get_current_repository_id()
        .ok_or("Kein Repository ausgewählt")?;
    
    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Parse Snapshot IDs
    let mut ids: Vec<SnapshotId> = Vec::new();
    for snapshot_id in snapshot_ids.iter() {
        let id: rustic_core::Id = snapshot_id
            .parse()
            .map_err(|_| format!("Ungültige Snapshot-ID: {}", snapshot_id))?;
        ids.push(SnapshotId::from(id));
    }

    // Lösche alle Snapshots in einem Batch
    repo.delete_snapshots(&ids)
        .map_err(|e| format!("Snapshots löschen fehlgeschlagen: {}", e))?;

    let deleted = ids.len();
    tracing::info!("{} Snapshots erfolgreich gelöscht", deleted);

    Ok(deleted)
}

/*
/// Fügt Tags zu einem Snapshot hinzu
/// TODO Phase 1: Refactoring benötigt - save_snapshot() API geändert
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
    let repo_id = state
        .get_current_repository_id()
        .ok_or("Kein Repository ausgewählt")?;
    
    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Load snapshot
    let mut snapshot = repo
        .get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Add tags (StringList::from takes &str, not from_str)
    let tag_lists: Vec<rustic_core::StringList> = tags
        .into_iter()
        .map(|t| rustic_core::StringList::from(&t))
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
*/

/*
/// Entfernt Tags von einem Snapshot
/// TODO Phase 1: Refactoring benötigt - save_snapshot() API geändert
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
    let repo_id = state
        .get_current_repository_id()
        .ok_or("Kein Repository ausgewählt")?;
    
    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Load snapshot
    let mut snapshot = repo
        .get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Remove tags (StringList::from takes &str, not from_str)
    let tag_lists: Vec<rustic_core::StringList> = tags
        .into_iter()
        .map(|t| rustic_core::StringList::from(&t))
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
*/
