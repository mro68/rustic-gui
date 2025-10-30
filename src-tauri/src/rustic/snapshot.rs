use crate::types::{RetentionPolicy, SnapshotDto};
use crate::error::RusticGuiError;
use rustic_core::{Repository, RepositoryOptions, Id, repofile::SnapshotId};
use rustic_backend::BackendOptions;
use tracing::{info, error};

/// Wendet eine Retention-Policy an und löscht Snapshots gemäß Policy.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `policy` - Retention-Policy
///
/// # Returns
/// Liste der gelöschten Snapshot-IDs
pub async fn forget_snapshots(
    repository_path: &str,
    password: &str,
    policy: &RetentionPolicy,
) -> Result<Vec<String>, RusticGuiError> {
    info!(repo = repository_path, "Wende Retention-Policy an");
    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends()
        .map_err(|e| RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}")))?;
    let repo = Repository::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Fehler beim Initialisieren des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Fehler beim Öffnen des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?;

    // 1. Alle Snapshots laden
    let mut snaps = repo.get_all_snapshots()
        .map_err(|e| RusticGuiError::Internal(format!("Snapshots konnten nicht geladen werden: {e}")))?;

    // 2. Filter nach Policy (nur keep_last als Beispiel)
    snaps.sort_by(|a, b| b.time.cmp(&a.time));
    let to_delete = if let Some(keep) = policy.keep_last {
        if snaps.len() > keep as usize {
            snaps[keep as usize..].to_vec()
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    let ids: Vec<_> = to_delete.iter().map(|s| s.id.clone()).collect();
    if ids.is_empty() {
        return Ok(vec![]);
    }
    repo.delete_snapshots(&ids)
        .map_err(|e| RusticGuiError::Internal(format!("Snapshots konnten nicht gelöscht werden: {e}")))?;
    Ok(ids.iter().map(|id| id.to_string()).collect())
}

/// Löscht einen Snapshot anhand der ID.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `snapshot_id` - ID des Snapshots
///
/// # Returns
/// Ok(()) bei Erfolg, sonst Fehler
pub async fn delete_snapshot(
    repository_path: &str,
    password: &str,
    snapshot_id: &str,
) -> Result<(), RusticGuiError> {
    info!(repo = repository_path, id = snapshot_id, "Lösche Snapshot");
    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends()
        .map_err(|e| RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}")))?;
    let repo = Repository::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Fehler beim Initialisieren des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Fehler beim Öffnen des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?;
    let id: Id = snapshot_id.parse()
        .map_err(|e| RusticGuiError::Internal(format!("Ungültige Snapshot-ID: {e}")))?;
    let snap_id = SnapshotId::from(id);
    repo.delete_snapshots(&[snap_id])
        .map_err(|e| RusticGuiError::Internal(format!("Snapshot konnte nicht gelöscht werden: {e}")))?;
    Ok(())
}

/// Lädt einen einzelnen Snapshot anhand der ID.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `snapshot_id` - ID des Snapshots
///
/// # Returns
/// SnapshotDto oder Fehler
pub async fn get_snapshot(
    repository_path: &str,
    password: &str,
    snapshot_id: &str,
) -> Result<SnapshotDto, RusticGuiError> {
    info!(repo = repository_path, id = snapshot_id, "Lese Snapshot-Details");
    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends()
        .map_err(|e| RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}")))?;
    let repo = Repository::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Fehler beim Initialisieren des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Fehler beim Öffnen des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?;
    let snaps = repo.get_snapshots(&[snapshot_id])
        .map_err(|e| RusticGuiError::Internal(format!("Snapshot konnte nicht geladen werden: {e}")))?;
    let snap = snaps.into_iter().next().ok_or_else(|| RusticGuiError::SnapshotNotFound { id: snapshot_id.to_string() })?;
    let repo_id = format!("repo-{}", repository_path);
    let dto = SnapshotDto {
        id: snap.id.to_string(),
        time: snap.time.to_rfc3339(),
        hostname: snap.hostname.to_string(),
        tags: snap.tags.iter().map(|t| t.to_string()).collect(),
        paths: snap.paths.iter().map(|p| p.to_string()).collect(),
        file_count: snap.summary.as_ref().map(|s| s.total_files_processed).unwrap_or(0),
        total_size: snap.summary.as_ref().map(|s| s.total_bytes_processed).unwrap_or(0),
        repository_id: repo_id,
        username: Some(snap.username.clone()),
        summary: snap.summary.as_ref().map(|s| crate::types::SnapshotSummary {
            files_count: Some(s.total_files_processed),
            dirs_count: Some(s.total_dirs_processed),
            data_size: Some(s.total_bytes_processed),
        }),
    };
    Ok(dto)
}

/// Listet alle Snapshots eines Repositories auf.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
///
/// # Returns
/// Vektor von SnapshotDto
pub async fn list_snapshots(
    repository_path: &str,
    password: &str,
) -> Result<Vec<SnapshotDto>, RusticGuiError> {
    info!(repo = repository_path, "Lese Snapshots aus Repository");
    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends()
        .map_err(|e| RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}")))?;
    let repo = Repository::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Fehler beim Initialisieren des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Fehler beim Öffnen des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?;
    let snaps = repo.get_all_snapshots()
        .map_err(|e| RusticGuiError::Internal(format!("Snapshots konnten nicht geladen werden: {e}")))?;
    let repo_id = format!("repo-{}", repository_path);
    let mut snapshots: Vec<SnapshotDto> = snaps.into_iter().map(|snap| SnapshotDto {
        id: snap.id.to_string(),
        time: snap.time.to_rfc3339(),
        hostname: snap.hostname.to_string(),
        tags: snap.tags.iter().map(|t| t.to_string()).collect(),
        paths: snap.paths.iter().map(|p| p.to_string()).collect(),
        file_count: snap.summary.as_ref().map(|s| s.total_files_processed).unwrap_or(0),
        total_size: snap.summary.as_ref().map(|s| s.total_bytes_processed).unwrap_or(0),
        repository_id: repo_id.clone(),
        username: Some(snap.username.clone()),
        summary: snap.summary.as_ref().map(|s| crate::types::SnapshotSummary {
            files_count: Some(s.total_files_processed),
            dirs_count: Some(s.total_dirs_processed),
            data_size: Some(s.total_bytes_processed),
        }),
    }).collect();
    snapshots.sort_by(|a, b| b.time.cmp(&a.time));
    Ok(snapshots)
}
