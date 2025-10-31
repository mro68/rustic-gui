use crate::error::RusticGuiError;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Emitter;
use tracing::{error, info};

/// Fortschrittsdaten für Backup-Prozess (an Frontend gesendet)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupProgress {
    pub files_processed: u64,
    pub files_total: Option<u64>,
    pub bytes_uploaded: u64,
    pub bytes_total: Option<u64>,
    pub current_file: Option<String>,
    pub percent: Option<f32>,
}

/// Optionen für Backup-Start
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupOptions {
    /// Repository-Pfad
    pub repository: String,
    /// Quellpfade
    pub source_paths: Vec<String>,
    /// Snapshot-Tags
    pub tags: Option<Vec<String>>,
    /// Exclude-Patterns
    pub exclude: Option<Vec<String>>,
    /// Kompression (optional)
    pub compression: Option<String>,
    /// Job-ID für Progress-Events (optional, empfohlen)
    pub job_id: Option<String>,
}

/// Testbare Backup-Logik ohne Tauri-API (für Unit-Tests)
pub async fn run_backup_logic<F>(
    options: &BackupOptions,
    on_progress: F,
) -> Result<(), RusticGuiError>
where
    F: Fn(BackupProgress) + Send + Sync + 'static,
{
    // Validierung
    if options.repository.is_empty() {
        error!("Kein Repository angegeben");
        return Err(RusticGuiError::InvalidConfig { field: "repository".into() });
    }
    if options.source_paths.is_empty() {
        error!("Keine Quellpfade angegeben");
        return Err(RusticGuiError::InvalidConfig { field: "source_paths".into() });
    }

    let repo_path = options.repository.clone();
    info!(repo = %repo_path, "Backup gestartet");

    // Simulierter Progress-Loop (bis rustic_core integriert ist)
    let total_files = 10u64;
    let total_bytes = 10_000_000u64;
    for i in 0..=total_files {
        let progress = BackupProgress {
            files_processed: i,
            files_total: Some(total_files),
            bytes_uploaded: (i * total_bytes / total_files),
            bytes_total: Some(total_bytes),
            current_file: Some(format!("/pfad/datei_{}.txt", i)),
            percent: Some((i as f32) / (total_files as f32)),
        };
        on_progress(progress);
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }

    info!(repo = %repo_path, "Backup abgeschlossen");
    Ok(())
}

/// Produktive Backup-Logik mit Tauri-Event-Emission
pub async fn run_backup<F>(
    app: AppHandle,
    options: BackupOptions,
    on_progress: F,
) -> Result<(), RusticGuiError>
where
    F: Fn(BackupProgress) + Send + Sync + 'static,
{
    let job_id = options.job_id.clone().unwrap_or_else(|| "default".to_string());
    let event_name = format!("backup-progress-{}", job_id);
    let app = app.clone();
    run_backup_logic(&options, move |progress| {
        on_progress(progress.clone());
        let _ = app.emit(&event_name, &progress);
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_run_backup_happy_path_progress() {
        let options = BackupOptions {
            repository: "/tmp/testrepo".to_string(),
            source_paths: vec!["/tmp/source1".to_string(), "/tmp/source2".to_string()],
            tags: Some(vec!["daily".to_string()]),
            exclude: Some(vec!["*.tmp".to_string()]),
            compression: None,
            job_id: Some("testjob1".to_string()),
        };
        let progress_vec = Arc::new(Mutex::new(Vec::new()));
        let progress_clone = progress_vec.clone();
        let cb = move |progress: BackupProgress| {
            progress_clone.lock().unwrap().push(progress);
        };
        let result = run_backup_logic(&options, cb).await;
        assert!(result.is_ok());
        let progress = progress_vec.lock().unwrap();
        assert!(!progress.is_empty());
        assert_eq!(progress.first().unwrap().files_processed, 0);
        assert_eq!(progress.last().unwrap().files_processed, 10);
        assert_eq!(progress.last().unwrap().percent, Some(1.0));
    }

    #[tokio::test]
    async fn test_run_backup_error_empty_repository() {
        let options = BackupOptions {
            repository: "".to_string(),
            source_paths: vec!["/tmp/source1".to_string()],
            tags: None,
            exclude: None,
            compression: None,
            job_id: Some("errjob1".to_string()),
        };
        let cb = |_p: BackupProgress| {};
        let result = run_backup_logic(&options, cb).await;
        assert!(
            matches!(result, Err(RusticGuiError::InvalidConfig { field }) if field == "repository")
        );
    }

    #[tokio::test]
    async fn test_run_backup_error_empty_source_paths() {
        let options = BackupOptions {
            repository: "/tmp/testrepo".to_string(),
            source_paths: vec![],
            tags: None,
            exclude: None,
            compression: None,
            job_id: Some("errjob2".to_string()),
        };
        let cb = |_p: BackupProgress| {};
        let result = run_backup_logic(&options, cb).await;
        assert!(
            matches!(result, Err(RusticGuiError::InvalidConfig { field }) if field == "source_paths")
        );
    }

    #[tokio::test]
    async fn test_run_backup_progress_event_count() {
        let options = BackupOptions {
            repository: "/tmp/testrepo".to_string(),
            source_paths: vec!["/tmp/source1".to_string()],
            tags: None,
            exclude: None,
            compression: None,
            job_id: Some("testjob2".to_string()),
        };
        let progress_vec = Arc::new(Mutex::new(Vec::new()));
        let progress_clone = progress_vec.clone();
        let cb = move |progress: BackupProgress| {
            progress_clone.lock().unwrap().push(progress);
        };
        let result = run_backup_logic(&options, cb).await;
        assert!(result.is_ok());
        let progress = progress_vec.lock().unwrap();
        // Es sollten 11 Progress-Events (0..=10) gesendet werden
        assert_eq!(progress.len(), 11);
    }
}
