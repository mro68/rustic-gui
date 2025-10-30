use crate::config::BackupJobConfig;
use crate::types::{BackupJobDto, RetentionPolicy};
use crate::state::AppState;
use std::path::PathBuf;
use uuid::Uuid;

/// Erstellt einen neuen Backup-Job.
///
/// # Parameter
/// - `name`: Anzeigename des Jobs
/// - `repository_id`: ID des zugehörigen Repositories
/// - `source_paths`: Liste der zu sichernden Pfade
/// - `exclude_patterns`: Ausschlussmuster (optional)
/// - `tags`: Tags für Snapshots (optional)
/// - `schedule`: Cron-Expression für Scheduling (optional)
/// - `retention`: Retention-Policy (optional)
///
/// # Rückgabe
/// Die ID des erstellten Jobs
#[tauri::command]
pub async fn create_backup_job(
    name: String,
    repository_id: String,
    source_paths: Vec<String>,
    exclude_patterns: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    schedule: Option<String>,
    retention: Option<crate::types::RetentionPolicy>,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Validierung
    if name.trim().is_empty() {
        return Err("Job-Name darf nicht leer sein".to_string());
    }

    if repository_id.trim().is_empty() {
        return Err("Repository-ID darf nicht leer sein".to_string());
    }

    if source_paths.is_empty() {
        return Err("Mindestens ein Quellpfad muss angegeben werden".to_string());
    }

    // Prüfe ob Repository existiert
    {
        let config = state.config.lock();
        if config.get_repository(&repository_id).is_none() {
            return Err(format!("Repository '{}' nicht gefunden", repository_id));
        }
    }

    // Validiere Pfade
    let source_paths: Vec<PathBuf> = source_paths
        .into_iter()
        .map(PathBuf::from)
        .collect();

    for path in &source_paths {
        if !path.exists() {
            return Err(format!("Quellpfad existiert nicht: {}", path.display()));
        }
    }

    // Erstelle Job-Konfiguration
    let job_id = Uuid::new_v4().to_string();
    let job_config = BackupJobConfig {
        id: job_id.clone(),
        name: name.trim().to_string(),
        repository_id,
        source_paths,
        exclude_patterns: exclude_patterns.unwrap_or_default(),
        tags: tags.unwrap_or_default(),
        schedule,
        retention: retention.map(|r| RetentionPolicy {
            keep_last: r.keep_last,
            keep_daily: r.keep_daily,
            keep_weekly: r.keep_weekly,
            keep_monthly: r.keep_monthly,
            keep_yearly: r.keep_yearly,
        }).unwrap_or_default(),
        enabled: true,
    };

    // Speichere in Config
    {
        let mut config = state.config.lock();
        config.add_backup_job(job_config);
    }

    // Speichere Config auf Disk
    state.save_config().map_err(|e| {
        format!("Job erstellt aber Config-Speicherung fehlgeschlagen: {}", e)
    })?;

    tracing::info!("Backup-Job '{}' erstellt (ID: {})", name, job_id);

    Ok(job_id)
}

/// Aktualisiert einen bestehenden Backup-Job.
///
/// # Parameter
/// - `job_id`: ID des zu aktualisierenden Jobs
/// - `name`: Neuer Anzeigename (optional)
/// - `source_paths`: Neue Quellpfade (optional)
/// - `exclude_patterns`: Neue Ausschlussmuster (optional)
/// - `tags`: Neue Tags (optional)
/// - `schedule`: Neue Cron-Expression (optional)
/// - `retention`: Neue Retention-Policy (optional)
/// - `enabled`: Ob Job aktiviert ist (optional)
#[tauri::command]
pub async fn update_backup_job(
    job_id: String,
    name: Option<String>,
    source_paths: Option<Vec<String>>,
    exclude_patterns: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    schedule: Option<String>,
    retention: Option<crate::types::RetentionPolicy>,
    enabled: Option<bool>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Hole existierenden Job
    let mut job_config = {
        let config = state.config.lock();
        config.get_backup_job(&job_id)
            .ok_or_else(|| format!("Backup-Job '{}' nicht gefunden", job_id))?
            .clone()
    };

    // Validierung
    if let Some(ref name) = name {
        if name.trim().is_empty() {
            return Err("Job-Name darf nicht leer sein".to_string());
        }
        job_config.name = name.trim().to_string();
    }

    if let Some(paths) = source_paths {
        if paths.is_empty() {
            return Err("Mindestens ein Quellpfad muss angegeben werden".to_string());
        }

        let source_paths: Vec<PathBuf> = paths
            .into_iter()
            .map(PathBuf::from)
            .collect();

        // Validiere Pfade
        for path in &source_paths {
            if !path.exists() {
                return Err(format!("Quellpfad existiert nicht: {}", path.display()));
            }
        }

        job_config.source_paths = source_paths;
    }

    // Aktualisiere optionale Felder
    if let Some(patterns) = exclude_patterns {
        job_config.exclude_patterns = patterns;
    }

    if let Some(tags) = tags {
        job_config.tags = tags;
    }

    if let Some(schedule) = schedule {
        job_config.schedule = Some(schedule);
    }

    if let Some(retention) = retention {
        job_config.retention = RetentionPolicy {
            keep_last: retention.keep_last,
            keep_daily: retention.keep_daily,
            keep_weekly: retention.keep_weekly,
            keep_monthly: retention.keep_monthly,
            keep_yearly: retention.keep_yearly,
        };
    }

    if let Some(enabled) = enabled {
        job_config.enabled = enabled;
    }

    // Speichere aktualisierten Job
    {
        let mut config = state.config.lock();
        config.add_backup_job(job_config);
    }

    // Speichere Config auf Disk
    state.save_config().map_err(|e| {
        format!("Job aktualisiert aber Config-Speicherung fehlgeschlagen: {}", e)
    })?;

    tracing::info!("Backup-Job '{}' aktualisiert", job_id);

    Ok(())
}

/// Löscht einen Backup-Job.
///
/// # Parameter
/// - `job_id`: ID des zu löschenden Jobs
#[tauri::command]
pub async fn delete_backup_job(
    job_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Prüfe ob Job existiert
    {
        let config = state.config.lock();
        if config.get_backup_job(&job_id).is_none() {
            return Err(format!("Backup-Job '{}' nicht gefunden", job_id));
        }
    }

    // Entferne Job aus Config
    {
        let mut config = state.config.lock();
        if !config.remove_backup_job(&job_id) {
            return Err(format!("Backup-Job '{}' konnte nicht entfernt werden", job_id));
        }
    }

    // Speichere Config auf Disk
    state.save_config().map_err(|e| {
        format!("Job gelöscht aber Config-Speicherung fehlgeschlagen: {}", e)
    })?;

    tracing::info!("Backup-Job '{}' gelöscht", job_id);

    Ok(())
}

/// Holt Details eines Backup-Jobs.
///
/// # Parameter
/// - `job_id`: ID des Jobs
///
/// # Rückgabe
/// BackupJobDto mit allen Job-Details
#[tauri::command]
pub async fn get_backup_job(
    job_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<BackupJobDto, String> {
    let config = state.config.lock();
    let job = config.get_backup_job(&job_id)
        .ok_or_else(|| format!("Backup-Job '{}' nicht gefunden", job_id))?;

    let dto = BackupJobDto {
        id: job.id.clone(),
        name: job.name.clone(),
        repository_id: job.repository_id.clone(),
        source_paths: job.source_paths.iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
        tags: job.tags.clone(),
        schedule: job.schedule.clone(),
        enabled: job.enabled,
        last_run: None, // TODO: Implementieren wenn Job-History verfügbar
        next_run: None, // TODO: Implementieren wenn Scheduler verfügbar
        retention: Some(job.retention.clone().into()),
    };

    Ok(dto)
}

/// Listet alle Backup-Jobs auf.
///
/// # Parameter
/// - `repository_id`: Optional - filtere nach Repository
///
/// # Rückgabe
/// Liste aller Backup-Jobs als BackupJobDto
#[tauri::command]
pub async fn list_backup_jobs(
    repository_id: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<BackupJobDto>, String> {
    let config = state.config.lock();

    let jobs = if let Some(repo_id) = repository_id {
        config.get_backup_jobs_for_repository(&repo_id)
            .into_iter()
            .cloned()
            .collect()
    } else {
        config.backup_jobs.clone()
    };

    let dtos = jobs.into_iter()
        .map(|job| BackupJobDto {
            id: job.id,
            name: job.name,
            repository_id: job.repository_id,
            source_paths: job.source_paths.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            tags: job.tags,
            schedule: job.schedule,
            enabled: job.enabled,
            last_run: None, // TODO: Implementieren wenn Job-History verfügbar
            next_run: None, // TODO: Implementieren wenn Scheduler verfügbar
            retention: Some(job.retention.clone().into()),
        })
        .collect();

    Ok(dtos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RetentionPolicy;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_create_backup_job_validation() {
        let state = AppState::new().unwrap();

        // Leerer Name sollte fehlschlagen
        let result = create_backup_job(
            "".to_string(),
            "repo1".to_string(),
            vec!["/tmp".to_string()],
            None,
            None,
            None,
            None,
            tauri::State::new(state.clone()),
        ).await;
        assert!(result.is_err());

        // Leere Repository-ID sollte fehlschlagen
        let result = create_backup_job(
            "Test Job".to_string(),
            "".to_string(),
            vec!["/tmp".to_string()],
            None,
            None,
            None,
            None,
            tauri::State::new(state.clone()),
        ).await;
        assert!(result.is_err());

        // Leere Source-Paths sollten fehlschlagen
        let result = create_backup_job(
            "Test Job".to_string(),
            "repo1".to_string(),
            vec![],
            None,
            None,
            None,
            None,
            tauri::State::new(state),
        ).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_backup_job_not_found() {
        let state = AppState::new().unwrap();

        let result = get_backup_job(
            "nonexistent".to_string(),
            tauri::State::new(state),
        ).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nicht gefunden"));
    }
}