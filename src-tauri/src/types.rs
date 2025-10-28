use serde::{Deserialize, Serialize};

/// DTO für Repository-Informationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryDto {
    /// Eindeutige ID des Repositories
    pub id: String,
    /// Anzeigename
    pub name: String,
    /// Pfad oder URL zum Repository
    pub path: String,
    /// Typ des Repositories (local, sftp, s3, etc.)
    pub repository_type: RepositoryType,
    /// Aktueller Status
    pub status: RepositoryStatus,
    /// Anzahl der Snapshots
    pub snapshot_count: u32,
    /// Gesamtgröße in Bytes
    pub total_size: u64,
    /// Wann zuletzt zugegriffen
    pub last_accessed: Option<String>, // ISO 8601
    /// Wann erstellt
    pub created_at: String, // ISO 8601
}

/// Typen von Repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepositoryType {
    Local,
    Sftp,
    S3,
    Rest,
    Rclone,
}

/// Status eines Repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RepositoryStatus {
    /// Repository ist verfügbar und gesund
    Healthy,
    /// Repository ist verfügbar aber hat Warnungen
    Warning,
    /// Repository ist nicht verfügbar
    Unavailable,
    /// Repository ist gesperrt
    Locked,
}

/// DTO für Snapshot-Informationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDto {
    /// Eindeutige ID des Snapshots
    pub id: String,
    /// Zeitstempel der Erstellung
    pub time: String, // ISO 8601
    /// Hostname wo der Snapshot erstellt wurde
    pub hostname: String,
    /// Tags des Snapshots
    pub tags: Vec<String>,
    /// Pfade die gesichert wurden
    pub paths: Vec<String>,
    /// Anzahl der Dateien
    pub file_count: u64,
    /// Gesamtgröße in Bytes
    pub total_size: u64,
    /// Repository ID
    pub repository_id: String,
}

/// DTO für Backup-Job-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobDto {
    /// Eindeutige ID des Jobs
    pub id: String,
    /// Anzeigename
    pub name: String,
    /// Repository ID
    pub repository_id: String,
    /// Pfade die gesichert werden sollen
    pub source_paths: Vec<String>,
    /// Tags für den Snapshot
    pub tags: Vec<String>,
    /// Cron-Expression für Scheduling
    pub schedule: Option<String>,
    /// Ob der Job aktiviert ist
    pub enabled: bool,
    /// Wann der Job zuletzt lief
    pub last_run: Option<String>, // ISO 8601
    /// Wann der Job das nächste Mal läuft
    pub next_run: Option<String>, // ISO 8601
    /// Retention-Policy
    pub retention: Option<RetentionPolicy>,
}

/// Retention-Policy für Snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Behalte die letzten N Snapshots
    pub keep_last: Option<u32>,
    /// Behalte tägliche Snapshots für N Tage
    pub keep_daily: Option<u32>,
    /// Behalte wöchentliche Snapshots für N Wochen
    pub keep_weekly: Option<u32>,
    /// Behalte monatliche Snapshots für N Monate
    pub keep_monthly: Option<u32>,
    /// Behalte jährliche Snapshots für N Jahre
    pub keep_yearly: Option<u32>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_last: Some(10),
            keep_daily: Some(7),
            keep_weekly: Some(4),
            keep_monthly: Some(12),
            keep_yearly: Some(2),
        }
    }
}

/// DTO für Restore-Optionen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOptionsDto {
    /// Snapshot ID zum Wiederherstellen
    pub snapshot_id: String,
    /// Zielpfad für die Wiederherstellung
    pub target_path: String,
    /// Spezifische Dateien/Pfade zum Wiederherstellen (leer = alles)
    pub paths: Vec<String>,
    /// Ob vorhandene Dateien überschrieben werden sollen
    pub overwrite: bool,
    /// Ob Berechtigungen wiederhergestellt werden sollen
    pub restore_permissions: bool,
    /// Ob Zeitstempel wiederhergestellt werden sollen
    pub restore_timestamps: bool,
    /// Ob der Restore als Dry-Run ausgeführt werden soll
    pub dry_run: bool,
}

/// Knoten im Dateibaum für File-Browser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeNode {
    /// Name der Datei/Verzeichnis
    pub name: String,
    /// Vollständiger Pfad
    pub path: String,
    /// Ist es ein Verzeichnis?
    pub is_directory: bool,
    /// Größe in Bytes (0 für Verzeichnisse)
    pub size: u64,
    /// Änderungszeit (ISO 8601)
    pub modified: Option<String>,
    /// Kinder (None für Dateien, Some(vec![]) für leere Verzeichnisse)
    pub children: Option<Vec<FileTreeNode>>,
}

/// Progress-Informationen für Restore-Operationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreProgress {
    /// Anzahl verarbeiteter Dateien
    pub files_processed: u64,
    /// Anzahl wiederhergestellter Dateien
    pub files_restored: u64,
    /// Anzahl übersprungener Dateien
    pub files_skipped: u64,
    /// Anzahl fehlgeschlagener Dateien
    pub files_failed: u64,
    /// Verarbeitete Bytes
    pub bytes_processed: u64,
    /// Geschätzter Fortschritt in Prozent (0-100)
    pub percent_complete: Option<f32>,
    /// Aktuell verarbeitete Datei
    pub current_file: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_dto_serialization() {
        let repo = RepositoryDto {
            id: "repo-1".to_string(),
            name: "My Local Repo".to_string(),
            path: "/home/user/backup".to_string(),
            repository_type: RepositoryType::Local,
            status: RepositoryStatus::Healthy,
            snapshot_count: 42,
            total_size: 1024 * 1024 * 1024, // 1GB
            last_accessed: Some("2025-10-26T10:00:00Z".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: RepositoryDto = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "repo-1");
        assert_eq!(deserialized.name, "My Local Repo");
    }

    #[test]
    fn test_snapshot_dto_serialization() {
        let snapshot = SnapshotDto {
            id: "snapshot-1".to_string(),
            time: "2025-10-26T10:00:00Z".to_string(),
            hostname: "myhost".to_string(),
            tags: vec!["daily".to_string(), "important".to_string()],
            paths: vec!["/home/user".to_string()],
            file_count: 1337,
            total_size: 512 * 1024 * 1024, // 512MB
            repository_id: "repo-1".to_string(),
        };

        let json = serde_json::to_string(&snapshot).unwrap();
        let deserialized: SnapshotDto = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "snapshot-1");
        assert_eq!(deserialized.tags.len(), 2);
    }
}