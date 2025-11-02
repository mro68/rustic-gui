use crate::error::RusticGuiError;
use crate::types::{FileTreeNode, RestoreOptionsDto, RestoreProgress};
use rustic_backend::BackendOptions;
use rustic_core::{
    LocalDestination, LsOptions, NoProgressBars, Repository, RepositoryOptions, RestoreOptions,
    repofile::SnapshotFile,
};
use std::collections::HashMap;
use tauri::{AppHandle, Emitter};
use tracing::{error, info};

/// Lädt die Dateistruktur eines Snapshots.
///
/// Erstellt eine hierarchische Baumstruktur aller Dateien und Verzeichnisse
/// in einem Snapshot für die Anzeige im File-Browser.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `snapshot_id` - ID des Snapshots
/// * `path` - Optionaler Pfad innerhalb des Snapshots (für Lazy-Loading)
///
/// # Returns
/// Wurzelknoten des Dateibaums
pub async fn get_file_tree(
    repository_path: &str,
    password: &str,
    snapshot_id: &str,
    path: Option<&str>,
) -> Result<FileTreeNode, RusticGuiError> {
    info!(repo = repository_path, snapshot = snapshot_id, path = ?path, "Lade Dateibaum");

    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends().map_err(|e| {
        RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}"))
    })?;
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

    // Snapshot laden
    let snaps = repo.get_snapshots(&[snapshot_id]).map_err(|e| {
        RusticGuiError::Internal(format!("Snapshot konnte nicht geladen werden: {e}"))
    })?;
    let snapshot = snaps
        .into_iter()
        .next()
        .ok_or_else(|| RusticGuiError::SnapshotNotFound { id: snapshot_id.to_string() })?;

    // Erstelle Baumstruktur aus den Pfaden des Snapshots
    let mut _root = FileTreeNode {
        name: "/".to_string(),
        path: "/".to_string(),
        is_directory: true,
        size: Some(0),
        modified: None,
        children: Some(Vec::new()),
    };

    // Gruppiere Pfade nach Verzeichnissen
    let mut path_map: HashMap<String, Vec<String>> = HashMap::new();

    for snap_path in &snapshot.paths {
        let path_str = snap_path.to_string();
        let parts: Vec<&str> = path_str.split('/').collect();

        for i in 0..parts.len() {
            let parent_path =
                if i == 0 { "/".to_string() } else { format!("/{}", parts[0..i].join("/")) };
            let item_name = parts[i].to_string();

            path_map.entry(parent_path).or_insert_with(Vec::new).push(item_name);
        }
    }

    // Baue rekursiv den Baum auf
    fn build_tree(
        path: &str,
        path_map: &HashMap<String, Vec<String>>,
        snapshot: &SnapshotFile,
    ) -> FileTreeNode {
        let children = path_map.get(path).cloned().unwrap_or_default();
        let mut child_nodes = Vec::new();

        for child_name in children {
            let child_path = if path == "/" {
                format!("/{}", child_name)
            } else {
                format!("{}/{}", path, child_name)
            };

            // Prüfe ob es ein Verzeichnis ist (hat Kinder)
            let is_dir = path_map.contains_key(&child_path);

            let node = if is_dir {
                FileTreeNode {
                    name: child_name,
                    path: child_path.clone(),
                    is_directory: true,
                    size: Some(0),
                    modified: None,
                    children: Some(
                        build_tree(&child_path, path_map, snapshot).children.unwrap_or_default(),
                    ),
                }
            } else {
                // Für Dateien: Versuche Größe und Modified aus Summary zu bekommen
                // Dies ist eine Vereinfachung - in der Realität bräuchte man Index-Daten
                FileTreeNode {
                    name: child_name,
                    path: child_path,
                    is_directory: false,
                    size: Some(1024), // Placeholder
                    modified: Some(snapshot.time.to_rfc3339()),
                    children: None,
                }
            };

            child_nodes.push(node);
        }

        FileTreeNode {
            name: path.split('/').last().unwrap_or("/").to_string(),
            path: path.to_string(),
            is_directory: true,
            size: Some(0),
            modified: Some(snapshot.time.to_rfc3339()),
            children: Some(child_nodes),
        }
    }

    let root = build_tree("/", &path_map, &snapshot);

    Ok(root)
}

/// Stellt Dateien aus einem Snapshot wieder her.
///
/// # Arguments
/// * `repository_path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `snapshot_id` - ID des Snapshots
/// * `files` - Liste der wiederherzustellenden Dateien/Pfade (leer = alle)
/// * `target_path` - Zielverzeichnis für die Wiederherstellung
/// * `options` - Restore-Optionen (Overwrite, Permissions, etc.)
/// * `app_handle` - Optional: Tauri AppHandle für Progress-Events
///
/// # Returns
/// Erfolg bei vollständiger Wiederherstellung
pub async fn restore_files(
    repository_path: &str,
    password: &str,
    snapshot_id: &str,
    files: Vec<String>,
    target_path: &str,
    options: &RestoreOptionsDto,
    app_handle: Option<AppHandle>,
) -> Result<(), RusticGuiError> {
    info!(
        repo = repository_path,
        snapshot = snapshot_id,
        files = files.len(),
        target = target_path,
        "Starte Restore"
    );

    let mut repo_opts = RepositoryOptions::default();
    repo_opts.password = Some(password.to_string());
    let mut backend_opts = BackendOptions::default();
    backend_opts.repository = Some(repository_path.to_string());
    let backends = backend_opts.to_backends().map_err(|e| {
        RusticGuiError::Internal(format!("Backend konnte nicht initialisiert werden: {e}"))
    })?;
    let repo = Repository::<NoProgressBars, _>::new(&repo_opts, &backends)
        .map_err(|e| {
            error!(?e, "Fehler beim Initialisieren des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .open()
        .map_err(|e| {
            error!(?e, "Fehler beim Öffnen des Repositories");
            RusticGuiError::RepositoryNotFound { path: repository_path.to_string() }
        })?
        .to_indexed()
        .map_err(|e| {
            error!(?e, "Repository-Indexierung fehlgeschlagen");
            RusticGuiError::Internal(format!("Repository-Indexierung fehlgeschlagen: {e}"))
        })?;

    // Snapshot laden
    let snaps = repo.get_snapshots(&[snapshot_id]).map_err(|e| {
        error!(?e, "Snapshot laden fehlgeschlagen");
        RusticGuiError::Internal(format!("Snapshot konnte nicht geladen werden: {e}"))
    })?;
    let snapshot = snaps
        .into_iter()
        .next()
        .ok_or_else(|| RusticGuiError::SnapshotNotFound { id: snapshot_id.to_string() })?;

    // Zielverzeichnis erstellen falls nicht vorhanden
    let target = std::path::PathBuf::from(target_path);
    if !target.exists() {
        std::fs::create_dir_all(&target).map_err(|e| {
            error!(?e, "Zielverzeichnis erstellen fehlgeschlagen");
            RusticGuiError::Internal(format!("Zielverzeichnis erstellen fehlgeschlagen: {e}"))
        })?;
    }

    // Tree des Snapshots laden (früh, um is_dir() zu prüfen)
    let tree = repo
        .node_from_snapshot_path("", |sn: &SnapshotFile| sn.id == snapshot.id)
        .map_err(|e| RusticGuiError::RestoreFailed { reason: e.to_string() })?;

    // Destination erstellen. create=true, flatten=!node.is_dir() (wie in rustic CLI)
    let dest =
        LocalDestination::new(target.to_str().unwrap(), true, !tree.is_dir()).map_err(|e| {
            error!(?e, "LocalDestination erstellen fehlgeschlagen");
            RusticGuiError::Internal(format!("Destination ungültig: {e}"))
        })?;

    // Restore-Optionen konfigurieren
    let restore_opts = RestoreOptions::default()
        .delete(options.overwrite)
        .no_ownership(!options.restore_permissions);

    if let Some(app) = app_handle.clone() {
        let job_id = format!("restore-{}", snapshot_id);
        let event_name = format!("restore-progress-{}", job_id);
        let _ = app.emit(
            &event_name,
            &RestoreProgress {
                base: crate::types::ProgressInfo {
                    current: 0,
                    total: 1,
                    message: Some("Restore wird vorbereitet...".to_string()),
                    percentage: Some(0.0),
                },
                files_restored: 0,
                bytes_restored: 0,
                current_file: None,
            },
        );
    }

    // Iterator für node_streamer erstellen
    // Wichtig: recursive muss true sein für vollständigen Restore
    let mut ls_opts = LsOptions::default();
    ls_opts.recursive = true;
    let node_streamer = repo
        .ls(&tree, &ls_opts)
        .map_err(|e| RusticGuiError::RestoreFailed { reason: e.to_string() })?;

    // Restore-Plan erstellen
    info!("Erstelle Restore-Plan...");
    let plan = repo
        .prepare_restore(&restore_opts, node_streamer, &dest, false)
        .map_err(|e| RusticGuiError::RestoreFailed { reason: e.to_string() })?;

    if let Some(app) = app_handle.clone() {
        let job_id = format!("restore-{}", snapshot_id);
        let event_name = format!("restore-progress-{}", job_id);
        let _ = app.emit(
            &event_name,
            &RestoreProgress {
                base: crate::types::ProgressInfo {
                    current: 0,
                    total: 100, // Placeholder
                    message: Some("Restore wird ausgeführt...".to_string()),
                    percentage: Some(50.0),
                },
                files_restored: 0,
                bytes_restored: 0,
                current_file: None,
            },
        );
    }

    // Node-Iterator für restore erstellen (erneuter ls-Aufruf)
    let node_streamer_restore = repo
        .ls(&tree, &ls_opts)
        .map_err(|e| RusticGuiError::RestoreFailed { reason: e.to_string() })?;

    // Restore ausführen
    info!("Führe Restore aus...");
    repo.restore(plan, &restore_opts, node_streamer_restore, &dest)
        .map_err(|e| RusticGuiError::RestoreFailed { reason: e.to_string() })?;

    info!("Restore erfolgreich abgeschlossen");
    Ok(())
}
