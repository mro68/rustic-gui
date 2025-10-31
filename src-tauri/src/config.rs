use crate::types::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Hauptkonfiguration der Anwendung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Liste aller konfigurierten Repositories
    pub repositories: Vec<RepositoryConfig>,
    /// Liste aller Backup-Jobs
    pub backup_jobs: Vec<BackupJobConfig>,
    /// Anwendungseinstellungen
    pub settings: AppSettings,
    /// Favorisierte Locations (M2 Task 2.3.2)
    #[serde(default)]
    pub favorite_locations: Vec<FavoriteLocation>,
    /// Job-Execution-History (M3 Task 3.2.1)
    #[serde(default)]
    pub job_executions: Vec<JobExecution>,
}

/// Konfiguration eines einzelnen Repositories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    /// Eindeutige ID
    pub id: String,
    /// Anzeigename
    pub name: String,
    /// Pfad oder URL zum Repository
    pub path: String,
    /// Backend-Typ
    pub backend_type: BackendType,
    /// Ob Passwort im Keychain gespeichert ist
    pub password_stored: bool,
    /// Zusätzliche Backend-Optionen (JSON)
    pub backend_options: Option<serde_json::Value>,
}

/// Konfiguration eines Backup-Jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobConfig {
    /// Eindeutige ID
    pub id: String,
    /// Anzeigename
    pub name: String,
    /// Repository ID
    pub repository_id: String,
    /// Quellpfade für Backup
    pub source_paths: Vec<PathBuf>,
    /// Ausschlussmuster
    pub exclude_patterns: Vec<String>,
    /// Tags für Snapshots
    pub tags: Vec<String>,
    /// Cron-Schedule (optional)
    pub schedule: Option<String>,
    /// Retention-Policy
    pub retention: RetentionPolicy,
    /// Ob Job aktiviert ist
    pub enabled: bool,
}

/// Anwendungseinstellungen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Theme (system, light, dark)
    pub theme: String,
    /// Log-Level (error, warn, info, debug, trace)
    pub log_level: String,
    /// Ob automatisch nach Updates gesucht werden soll
    pub check_updates: bool,
    /// Maximale Anzahl gleichzeitiger Backups
    pub max_concurrent_backups: usize,
}

/// Backend-Typen für Repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackendType {
    Local,
    Sftp,
    S3,
    Rest,
    Rclone,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            repositories: Vec::new(),
            backup_jobs: Vec::new(),
            settings: AppSettings {
                theme: "system".to_string(),
                log_level: "info".to_string(),
                check_updates: true,
                max_concurrent_backups: 1,
            },
            favorite_locations: Vec::new(),
            job_executions: Vec::new(),
        }
    }
}

impl AppConfig {
    /// Lädt Konfiguration von der Festplatte
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            tracing::info!("Keine Konfigurationsdatei gefunden, verwende Standardkonfiguration");
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .context("Konfigurationsdatei konnte nicht gelesen werden")?;

        let config: Self =
            toml::from_str(&content).context("Konfiguration konnte nicht geparst werden")?;

        tracing::debug!(
            "Konfiguration geladen: {} Repositories, {} Backup-Jobs",
            config.repositories.len(),
            config.backup_jobs.len()
        );

        Ok(config)
    }

    /// Speichert Konfiguration auf die Festplatte
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        // Übergeordnetes Verzeichnis erstellen falls nötig
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("Konfigurationsverzeichnis konnte nicht erstellt werden")?;
        }

        let toml = toml::to_string_pretty(self)
            .context("Konfiguration konnte nicht serialisiert werden")?;

        std::fs::write(&path, toml).context("Konfiguration konnte nicht geschrieben werden")?;

        tracing::debug!("Konfiguration gespeichert nach: {}", path.display());

        Ok(())
    }

    /// Gibt den plattformspezifischen Konfigurationspfad zurück
    ///
    /// - Linux: `~/.config/rustic-gui/config.toml`
    /// - Windows: `%APPDATA%\rustic-gui\config.toml`
    /// - macOS: `~/Library/Application Support/rustic-gui/config.toml`
    pub fn config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().context("Konfigurationsverzeichnis konnte nicht bestimmt werden")?;

        Ok(config_dir.join("rustic-gui").join("config.toml"))
    }

    /// Fügt ein neues Repository zur Konfiguration hinzu
    pub fn add_repository(&mut self, config: RepositoryConfig) {
        // Entferne existierendes Repository mit gleicher ID
        self.repositories.retain(|r| r.id != config.id);
        self.repositories.push(config);
    }

    /// Entfernt ein Repository aus der Konfiguration
    pub fn remove_repository(&mut self, id: &str) -> bool {
        let initial_len = self.repositories.len();
        self.repositories.retain(|r| r.id != id);
        initial_len != self.repositories.len()
    }

    /// Findet ein Repository anhand der ID
    pub fn get_repository(&self, id: &str) -> Option<&RepositoryConfig> {
        self.repositories.iter().find(|r| r.id == id)
    }

    /// Fügt einen neuen Backup-Job hinzu
    pub fn add_backup_job(&mut self, config: BackupJobConfig) {
        // Entferne existierenden Job mit gleicher ID
        self.backup_jobs.retain(|j| j.id != config.id);
        self.backup_jobs.push(config);
    }

    /// Entfernt einen Backup-Job
    pub fn remove_backup_job(&mut self, id: &str) -> bool {
        let initial_len = self.backup_jobs.len();
        self.backup_jobs.retain(|j| j.id != id);
        initial_len != self.backup_jobs.len()
    }

    /// Findet einen Backup-Job anhand der ID
    pub fn get_backup_job(&self, id: &str) -> Option<&BackupJobConfig> {
        self.backup_jobs.iter().find(|j| j.id == id)
    }

    /// Gibt alle Backup-Jobs für ein bestimmtes Repository zurück
    pub fn get_backup_jobs_for_repository(&self, repository_id: &str) -> Vec<&BackupJobConfig> {
        self.backup_jobs.iter().filter(|j| j.repository_id == repository_id).collect()
    }

    // ===== M3: Job-Execution-History Management =====

    /// Fügt einen Job-Execution-Eintrag hinzu
    pub fn add_job_execution(&mut self, execution: JobExecution) {
        self.job_executions.push(execution);

        // Halte History auf maximal 1000 Einträge
        if self.job_executions.len() > 1000 {
            self.job_executions.drain(0..(self.job_executions.len() - 1000));
        }
    }

    /// Gibt die letzten N Job-Executions für einen Job zurück
    pub fn get_job_executions(&self, job_id: &str, limit: usize) -> Vec<&JobExecution> {
        self.job_executions
            .iter()
            .filter(|e| e.job_id == job_id)
            .rev()
            .take(limit)
            .collect()
    }

    /// Gibt alle Job-Executions zurück
    pub fn get_all_job_executions(&self, limit: usize) -> Vec<&JobExecution> {
        self.job_executions.iter().rev().take(limit).collect()
    }

    /// Entfernt alte Job-Executions (älter als days Tage)
    pub fn cleanup_old_executions(&mut self, days: u64) -> usize {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days as i64);
        let cutoff_str = cutoff.to_rfc3339();

        let original_len = self.job_executions.len();
        self.job_executions.retain(|e| e.started_at > cutoff_str);

        original_len - self.job_executions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = AppConfig::default();

        assert!(config.repositories.is_empty());
        assert!(config.backup_jobs.is_empty());
        assert_eq!(config.settings.theme, "system");
        assert_eq!(config.settings.log_level, "info");
        assert!(config.settings.check_updates);
    }

    #[test]
    fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Temporäres Überschreiben des Config-Pfads für Test
        unsafe {
            std::env::set_var("RUSTIC_GUI_CONFIG_PATH", config_path.to_str().unwrap());
        }

        let mut config = AppConfig::default();
        config.settings.theme = "dark".to_string();

        // Repository hinzufügen
        config.add_repository(RepositoryConfig {
            id: "test-repo".to_string(),
            name: "Test Repository".to_string(),
            path: "/tmp/test".to_string(),
            backend_type: BackendType::Local,
            password_stored: false,
            backend_options: None,
        });

        // Backup-Job hinzufügen
        config.add_backup_job(BackupJobConfig {
            id: "test-job".to_string(),
            name: "Test Job".to_string(),
            repository_id: "test-repo".to_string(),
            source_paths: vec![PathBuf::from("/home/user")],
            exclude_patterns: vec!["*.tmp".to_string()],
            tags: vec!["test".to_string()],
            schedule: Some("0 2 * * *".to_string()),
            retention: RetentionPolicy {
                keep_last: Some(10),
                keep_daily: Some(7),
                keep_weekly: Some(4),
                keep_monthly: Some(12),
                keep_yearly: Some(2),
            },
            enabled: true,
        });

        // Speichern
        config.save().unwrap();

        // Laden
        let loaded_config = AppConfig::load().unwrap();

        assert_eq!(loaded_config.repositories.len(), 1);
        assert_eq!(loaded_config.backup_jobs.len(), 1);
        assert_eq!(loaded_config.settings.theme, "dark");

        // Repository prüfen
        let repo = loaded_config.get_repository("test-repo").unwrap();
        assert_eq!(repo.name, "Test Repository");
        assert_eq!(repo.backend_type, BackendType::Local);

        // Job prüfen
        let job = loaded_config.get_backup_job("test-job").unwrap();
        assert_eq!(job.name, "Test Job");
        assert_eq!(job.schedule.as_ref().unwrap(), "0 2 * * *");

        unsafe {
            std::env::remove_var("RUSTIC_GUI_CONFIG_PATH");
        }
    }

    #[test]
    fn test_repository_operations() {
        let mut config = AppConfig::default();

        let repo_config = RepositoryConfig {
            id: "repo1".to_string(),
            name: "Repository 1".to_string(),
            path: "/path/to/repo1".to_string(),
            backend_type: BackendType::Local,
            password_stored: true,
            backend_options: None,
        };

        // Hinzufügen
        config.add_repository(repo_config.clone());
        assert_eq!(config.repositories.len(), 1);

        // Finden
        let found = config.get_repository("repo1").unwrap();
        assert_eq!(found.name, "Repository 1");

        // Zweites Repository hinzufügen
        let repo2_config = RepositoryConfig {
            id: "repo2".to_string(),
            name: "Repository 2".to_string(),
            path: "/path/to/repo2".to_string(),
            backend_type: BackendType::Sftp,
            password_stored: false,
            backend_options: Some(serde_json::json!({"host": "example.com"})),
        };

        config.add_repository(repo2_config);
        assert_eq!(config.repositories.len(), 2);

        // Entfernen
        assert!(config.remove_repository("repo1"));
        assert_eq!(config.repositories.len(), 1);
        assert!(!config.remove_repository("nonexistent"));
    }

    #[test]
    fn test_backup_job_operations() {
        let mut config = AppConfig::default();

        let job_config = BackupJobConfig {
            id: "job1".to_string(),
            name: "Job 1".to_string(),
            repository_id: "repo1".to_string(),
            source_paths: vec![PathBuf::from("/home")],
            exclude_patterns: vec![],
            tags: vec!["daily".to_string()],
            schedule: Some("0 2 * * *".to_string()),
            retention: RetentionPolicy::default(),
            enabled: true,
        };

        // Hinzufügen
        config.add_backup_job(job_config.clone());
        assert_eq!(config.backup_jobs.len(), 1);

        // Finden
        let found = config.get_backup_job("job1").unwrap();
        assert_eq!(found.name, "Job 1");

        // Jobs für Repository finden
        let jobs = config.get_backup_jobs_for_repository("repo1");
        assert_eq!(jobs.len(), 1);

        // Entfernen
        assert!(config.remove_backup_job("job1"));
        assert_eq!(config.backup_jobs.len(), 0);
    }

    #[test]
    fn test_config_path() {
        let path = AppConfig::config_path().unwrap();
        assert!(path.ends_with("rustic-gui/config.toml"));
    }
}
