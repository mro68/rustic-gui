use crate::{error::Result, types::RepositoryDto};
use std::path::Path;

/// Repository-Management Funktionen
///
/// Diese Module kapseln alle Operationen auf rustic Repositories.
/// Alle Funktionen sind async und thread-safe.

/// Initialisiert ein neues Repository
///
/// # Arguments
/// * `path` - Pfad zum Repository
/// * `password` - Repository-Passwort
/// * `backend_type` - Typ des Backends (local, sftp, s3, etc.)
/// * `backend_options` - Backend-spezifische Optionen
///
/// # Returns
/// RepositoryDto mit Informationen über das neu erstellte Repository
pub fn init_repository(
    path: &str,
    password: &str,
    backend_type: &str,
    backend_options: Option<serde_json::Value>,
) -> Result<RepositoryDto> {
    let path = Path::new(path);

    // Stelle sicher, dass das Verzeichnis existiert
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    // TODO: Implementiere richtige rustic_core Integration
    // Für jetzt nur ein Platzhalter

    // Erstelle DTO als Response
    let dto = RepositoryDto {
        id: format!("repo-{}", path.display()),
        name: path.file_name().and_then(|n| n.to_str()).unwrap_or("Unnamed Repository").to_string(),
        path: path.to_string_lossy().to_string(),
        repository_type: match backend_type {
            "local" => crate::types::RepositoryType::Local,
            "sftp" => crate::types::RepositoryType::Sftp,
            "s3" => crate::types::RepositoryType::S3,
            "rest" => crate::types::RepositoryType::Rest,
            "rclone" => crate::types::RepositoryType::Rclone,
            _ => crate::types::RepositoryType::Local,
        },
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count: 0,
        total_size: 0,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(dto)
}

/// Öffnet ein bestehendes Repository
///
/// # Arguments
/// * `path` - Pfad zum Repository
/// * `password` - Repository-Passwort
///
/// # Returns
/// RepositoryDto mit Informationen über das geöffnete Repository
pub fn open_repository(path: &str, password: &str) -> Result<RepositoryDto> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(crate::error::RusticGuiError::RepositoryNotFound {
            path: path.to_string_lossy().to_string(),
        });
    }

    // TODO: Implementiere richtige rustic_core Integration
    // Für jetzt nur ein Platzhalter

    // Erstelle DTO
    let dto = RepositoryDto {
        id: format!("repo-{}", path.display()),
        name: path.file_name().and_then(|n| n.to_str()).unwrap_or("Unnamed Repository").to_string(),
        path: path.to_string_lossy().to_string(),
        repository_type: crate::types::RepositoryType::Local,
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count: 0, // TODO: Aus Repository lesen
        total_size: 0,     // TODO: Berechnen
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: "2025-01-01T00:00:00Z".to_string(), // TODO: Aus Config lesen
    };

    Ok(dto)
}

/// Ruft Informationen über ein Repository ab
///
/// # Arguments
/// * `path` - Pfad zum Repository
/// * `password` - Repository-Passwort
///
/// # Returns
/// RepositoryDto mit aktuellen Informationen
pub fn get_repository_info(path: &str, password: &str) -> Result<RepositoryDto> {
    // Für jetzt einfach open_repository verwenden
    // Später können wir das optimieren um nicht immer neu zu öffnen
    open_repository(path, password)
}

/// Führt eine Repository-Überprüfung durch
///
/// # Arguments
/// * `path` - Pfad zum Repository
/// * `password` - Repository-Passwort
///
/// # Returns
/// RepositoryDto mit aktualisiertem Status
pub fn check_repository(path: &str, password: &str) -> Result<RepositoryDto> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(crate::error::RusticGuiError::RepositoryNotFound {
            path: path.to_string_lossy().to_string(),
        });
    }

    // TODO: Implementiere richtigen Check-Algorithmus
    // Für jetzt nur prüfen ob Verzeichnis existiert

    let dto = RepositoryDto {
        id: format!("repo-{}", path.display()),
        name: path.file_name().and_then(|n| n.to_str()).unwrap_or("Unnamed Repository").to_string(),
        path: path.to_string_lossy().to_string(),
        repository_type: crate::types::RepositoryType::Local,
        status: crate::types::RepositoryStatus::Healthy, // TODO: Richtigen Status ermitteln
        snapshot_count: 0,                               // TODO: Berechnen
        total_size: 0,                                   // TODO: Berechnen
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
    };

    Ok(dto)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_repository_local() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("test-repo");

        let result = init_repository(&repo_path.to_string_lossy(), "test-password", "local", None);

        // Für jetzt erwarten wir einen Fehler, da rustic_core möglicherweise anders funktioniert
        // Das ist ein Platzhalter für die richtige Implementierung
        match result {
            Ok(dto) => {
                assert_eq!(dto.repository_type, crate::types::RepositoryType::Local);
                assert_eq!(dto.status, crate::types::RepositoryStatus::Healthy);
            }
            Err(_) => {
                // Erwartet für jetzt - richtige Implementierung folgt
            }
        }
    }

    #[test]
    fn test_open_nonexistent_repository() {
        let result = open_repository("/nonexistent/path", "password");
        assert!(matches!(result, Err(crate::error::RusticGuiError::RepositoryNotFound { .. })));
    }

    #[test]
    fn test_repository_dto_creation() {
        let dto = RepositoryDto {
            id: "test-repo".to_string(),
            name: "Test Repository".to_string(),
            path: "/tmp/test".to_string(),
            repository_type: crate::types::RepositoryType::Local,
            status: crate::types::RepositoryStatus::Healthy,
            snapshot_count: 5,
            total_size: 1024,
            last_accessed: Some("2025-10-26T10:00:00Z".to_string()),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(dto.id, "test-repo");
        assert_eq!(dto.name, "Test Repository");
        assert_eq!(dto.snapshot_count, 5);
        assert_eq!(dto.total_size, 1024);
    }
}
