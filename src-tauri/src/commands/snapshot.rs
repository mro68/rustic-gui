// TODO.md: Phase 1 - Snapshot-Management Commands ✅ IMPLEMENTIERT
// Status: Verschoben von lib.rs zu commands/snapshot.rs
// Referenz: TODO.md Zeile 182-187

use crate::state::AppState;
use crate::types::{DiffResultDto, DiffStats, SnapshotDto};

/// Listet alle Snapshots eines Repositories
#[tauri::command]
pub async fn list_snapshots_command(
    repository_path: String,
    password: String,
) -> std::result::Result<Vec<SnapshotDto>, String> {
    crate::rustic::snapshot::list_snapshots(&repository_path, &password)
        .await
        .map_err(|e| e.to_string())
}

/// Listet Snapshots mit Filter
#[tauri::command]
pub async fn list_snapshots_filtered_command(
    repository_path: String,
    password: String,
    filter: Option<crate::rustic::snapshot::SnapshotFilter>,
) -> std::result::Result<Vec<SnapshotDto>, String> {
    crate::rustic::snapshot::list_snapshots_filtered(&repository_path, &password, filter)
        .await
        .map_err(|e| e.to_string())
}

/// Holt Details zu einem Snapshot
#[tauri::command]
pub async fn get_snapshot_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
) -> std::result::Result<SnapshotDto, String> {
    crate::rustic::snapshot::get_snapshot(&repository_path, &password, &snapshot_id)
        .await
        .map_err(|e| e.to_string())
}

/// Löscht einzelnen Snapshot
#[tauri::command]
pub async fn delete_snapshot_command(
    repository_path: String,
    password: String,
    snapshot_id: String,
) -> std::result::Result<(), String> {
    crate::rustic::snapshot::delete_snapshot(&repository_path, &password, &snapshot_id)
        .await
        .map_err(|e| e.to_string())
}

/// Vergleicht zwei Snapshots mittels Tree-basiertem Diff
///
/// Basierend auf rustic CLI: https://github.com/rustic-rs/rustic/blob/main/src/commands/diff.rs
/// Verwendet Repository::ls() für vollständigen File-Tree-Vergleich
#[tauri::command]
pub async fn compare_snapshots(
    snapshot_id_a: String,
    snapshot_id_b: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<DiffResultDto, String> {
    use rustic_backend::BackendOptions;
    use rustic_core::{LsOptions, NoProgressBars, Repository, RepositoryOptions};
    use std::collections::HashMap;
    use tracing::{error, info};

    info!("compare_snapshots: {} vs {}", snapshot_id_a, snapshot_id_b);

    // Repository-Pfad holen (parking_lot::Mutex hat kein map_err)
    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;
    let repository_path = {
        let config = state.config.lock();
        config
            .repositories
            .iter()
            .find(|r| r.id == repo_id)
            .ok_or_else(|| format!("Repository nicht gefunden: {}", repo_id))?
            .path
            .clone()
    };

    // Repository NEU öffnen und indexieren (wie restore.rs)
    // Grund: Arc<Repository> kann nicht mit to_indexed() konsumiert werden
    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());

    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.clone());

    let backends = backend_opts.to_backends().map_err(|e| {
        error!(?e, "Backend erstellen fehlgeschlagen");
        format!("Backend erstellen fehlgeschlagen: {}", e)
    })?;

    let repo = Repository::<NoProgressBars, _>::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Repository initialisieren fehlgeschlagen");
            format!("Repository initialisieren fehlgeschlagen: {}", e)
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Repository öffnen fehlgeschlagen");
            format!("Repository öffnen fehlgeschlagen: {}", e)
        })?
        .to_indexed()
        .map_err(|e| {
            error!(?e, "Repository indexieren fehlgeschlagen");
            format!("Repository indexieren fehlgeschlagen: {}", e)
        })?;

    // Beide Snapshots laden
    let snaps =
        repo.get_snapshots(&[snapshot_id_a.as_str(), snapshot_id_b.as_str()]).map_err(|e| {
            error!(?e, "Snapshots laden fehlgeschlagen");
            format!("Snapshots laden fehlgeschlagen: {}", e)
        })?;

    if snaps.len() != 2 {
        return Err("Nicht alle Snapshots gefunden".into());
    }

    let snapshot_a = &snaps[0];
    let snapshot_b = &snaps[1];

    // Root-Nodes in Snapshots laden (siehe rustic CLI diff.rs#L110-112)
    let node_a = repo
        .node_from_snapshot_and_path(snapshot_a, "")
        .map_err(|e| format!("Node A laden fehlgeschlagen: {}", e))?;

    let node_b = repo
        .node_from_snapshot_and_path(snapshot_b, "")
        .map_err(|e| format!("Node B laden fehlgeschlagen: {}", e))?;

    // File-Trees mit ls() laden (siehe rustic CLI diff.rs#L113-115)
    let ls_opts = LsOptions::default();
    let tree_a =
        repo.ls(&node_a, &ls_opts).map_err(|e| format!("Tree A listen fehlgeschlagen: {}", e))?;

    let tree_b =
        repo.ls(&node_b, &ls_opts).map_err(|e| format!("Tree B listen fehlgeschlagen: {}", e))?;

    // Dateien aus beiden Trees in HashMaps sammeln
    let mut files_a: HashMap<String, rustic_core::repofile::Node> = HashMap::new();
    let mut files_b: HashMap<String, rustic_core::repofile::Node> = HashMap::new();

    for item in tree_a {
        let (path, node) = item.map_err(|e| format!("Tree A Item fehlgeschlagen: {}", e))?;
        files_a.insert(path.to_string_lossy().to_string(), node);
    }

    for item in tree_b {
        let (path, node) = item.map_err(|e| format!("Tree B Item fehlgeschlagen: {}", e))?;
        files_b.insert(path.to_string_lossy().to_string(), node);
    }

    // Unterschiede berechnen (siehe rustic CLI diff.rs#L477-505)
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    let mut total_size_change: i64 = 0;

    // In B, aber nicht in A → hinzugefügt
    for (path, node_b) in &files_b {
        if !files_a.contains_key(path) {
            added.push(path.clone());
            total_size_change += node_b.meta.size as i64;
        }
    }

    // In A, aber nicht in B → entfernt
    for (path, node_a) in &files_a {
        if !files_b.contains_key(path) {
            removed.push(path.clone());
            total_size_change -= node_a.meta.size as i64;
        }
    }

    // In beiden, aber unterschiedlich → geändert (siehe rustic CLI diff.rs#L294-321)
    for (path, node_a) in &files_a {
        if let Some(node_b) = files_b.get(path) {
            let content_changed = node_a.content != node_b.content;
            let size_changed = node_a.meta.size != node_b.meta.size;

            if content_changed || size_changed {
                modified.push(path.clone());
                total_size_change += (node_b.meta.size as i64) - (node_a.meta.size as i64);
            }
        }
    }

    let added_count = added.len() as u64;
    let removed_count = removed.len() as u64;
    let modified_count = modified.len() as u64;

    tracing::info!(
        "Diff: +{} -{} ~{} (size_change: {})",
        added_count,
        removed_count,
        modified_count,
        total_size_change
    );

    Ok(DiffResultDto {
        added,
        removed,
        modified,
        stats: DiffStats { added_count, removed_count, modified_count, total_size_change },
    })
}

/// Löscht einzelnen Snapshot
/// Task 4.1: Snapshot-Deletion (einzeln)
#[tauri::command]
pub async fn delete_snapshot(
    snapshot_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    use rustic_core::repofile::SnapshotId;

    tracing::info!("Lösche Snapshot: {}", snapshot_id);

    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Parse Snapshot ID
    let id: rustic_core::Id =
        snapshot_id.parse().map_err(|_| format!("Ungültige Snapshot-ID: {}", snapshot_id))?;
    let snap_id = SnapshotId::from(id);

    // Lösche Snapshot (rustic_core nutzt delete_snapshots auch für single Snapshot)
    repo.delete_snapshots(&[snap_id])
        .map_err(|e| format!("Snapshot löschen fehlgeschlagen: {}", e))?;

    tracing::info!("Snapshot {} erfolgreich gelöscht", snapshot_id);

    Ok(())
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

    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Parse Snapshot IDs
    let mut ids: Vec<SnapshotId> = Vec::new();
    for snapshot_id in snapshot_ids.iter() {
        let id: rustic_core::Id =
            snapshot_id.parse().map_err(|_| format!("Ungültige Snapshot-ID: {}", snapshot_id))?;
        ids.push(SnapshotId::from(id));
    }

    // Lösche alle Snapshots in einem Batch
    repo.delete_snapshots(&ids).map_err(|e| format!("Snapshots löschen fehlgeschlagen: {}", e))?;

    let deleted = ids.len();
    tracing::info!("{} Snapshots erfolgreich gelöscht", deleted);

    Ok(deleted)
}

/// Fügt Tags zu einem Snapshot hinzu
#[tauri::command]
pub async fn add_snapshot_tags(
    snapshot_id: String,
    tags: Vec<String>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    use rustic_core::repofile::StringList;

    tracing::debug!("add_snapshot_tags called: snapshot_id={}, tags={:?}", snapshot_id, tags);

    // Get current repository
    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Load snapshot
    let snaps = repo
        .get_snapshots(&[&snapshot_id])
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    let mut snapshot = snaps.into_iter().next().ok_or("Snapshot ist leer")?;

    // Convert tags to StringList using FromStr trait
    let tag_lists: Vec<StringList> = tags.into_iter().filter_map(|t| t.parse().ok()).collect();

    if snapshot.add_tags(tag_lists) {
        // Save updated snapshot back to repository
        repo.save_snapshots(vec![snapshot])
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
    use rustic_core::repofile::StringList;

    tracing::debug!("remove_snapshot_tags called: snapshot_id={}, tags={:?}", snapshot_id, tags);

    // Get current repository
    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Load snapshot
    let snaps = repo
        .get_snapshots(&[&snapshot_id])
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    let mut snapshot = snaps.into_iter().next().ok_or("Snapshot ist leer")?;

    // Convert tags to StringList using FromStr trait
    let tag_lists: Vec<StringList> = tags.into_iter().filter_map(|t| t.parse().ok()).collect();

    if snapshot.remove_tags(&tag_lists) {
        // Save updated snapshot back to repository
        repo.save_snapshots(vec![snapshot])
            .map_err(|e| format!("Snapshot speichern fehlgeschlagen: {}", e))?;

        tracing::info!("Tags erfolgreich von Snapshot {} entfernt", snapshot_id);
        Ok(())
    } else {
        Err("Keine Tags entfernt".into())
    }
}
