use crate::types::ErrorDto;
// Konvertierung von RusticGuiError zu ErrorDto
impl From<&RusticGuiError> for ErrorDto {
    fn from(error: &RusticGuiError) -> Self {
        use RusticGuiError::*;
        let (code, message, details) = match error {
            RepositoryNotFound { path } => {
                ("RepositoryNotFound", error.to_string(), Some(format!("path: {}", path)))
            }
            RepositoryAlreadyExists { path } => {
                ("RepositoryAlreadyExists", error.to_string(), Some(format!("path: {}", path)))
            }
            RepositoryLocked => ("RepositoryLocked", error.to_string(), None),
            AuthenticationFailed => ("AuthenticationFailed", error.to_string(), None),
            SnapshotNotFound { id } => {
                ("SnapshotNotFound", error.to_string(), Some(format!("id: {}", id)))
            }
            BackupFailed { reason } => ("BackupFailed", error.to_string(), Some(reason.clone())),
            RestoreFailed { reason } => ("RestoreFailed", error.to_string(), Some(reason.clone())),
            InvalidConfig { field } => {
                ("InvalidConfig", error.to_string(), Some(format!("field: {}", field)))
            }
            ConfigError { message } => ("ConfigError", error.to_string(), Some(message.clone())),
            RusticError { message } => ("RusticError", error.to_string(), Some(message.clone())),
            IoError(e) => ("IoError", error.to_string(), Some(e.to_string())),
            JsonError(e) => ("JsonError", error.to_string(), Some(e.to_string())),
            TomlError(msg) => ("TomlError", error.to_string(), Some(msg.clone())),
            Internal(msg) => ("Internal", error.to_string(), Some(msg.clone())),
            UnsupportedBackend { backend_type } => (
                "UnsupportedBackend",
                error.to_string(),
                Some(format!("backend_type: {}", backend_type)),
            ),
            InvalidConfiguration { message } => {
                ("InvalidConfiguration", error.to_string(), Some(message.clone()))
            }
            RcloneNotFound => ("RcloneNotFound", error.to_string(), None),
            RcloneError { message } => ("RcloneError", error.to_string(), Some(message.clone())),
        };
        ErrorDto { code: code.to_string(), message, details }
    }
}
use thiserror::Error;

/// Haupt-Fehlertypen für rustic-gui
///
/// Alle Fehler implementieren std::error::Error und können
/// in Strings konvertiert werden für Tauri.
#[derive(Debug, Error)]
pub enum RusticGuiError {
    #[error("Repository nicht gefunden: {path}")]
    RepositoryNotFound { path: String },

    #[error("Repository existiert bereits: {path}")]
    RepositoryAlreadyExists { path: String },

    #[error("Repository ist gesperrt")]
    RepositoryLocked,

    #[error("Authentifizierung fehlgeschlagen")]
    AuthenticationFailed,

    #[error("Snapshot nicht gefunden: {id}")]
    SnapshotNotFound { id: String },

    #[error("Backup fehlgeschlagen: {reason}")]
    BackupFailed { reason: String },

    #[error("Restore fehlgeschlagen: {reason}")]
    RestoreFailed { reason: String },

    #[error("Konfiguration ungültig: {field}")]
    InvalidConfig { field: String },

    #[error("Konfigurationsfehler: {message}")]
    ConfigError { message: String },

    #[error("rustic-Fehler: {message}")]
    RusticError { message: String },

    #[error("IO-Fehler: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON-Fehler: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML-Fehler: {0}")]
    TomlError(String),

    #[error("Interner Fehler: {0}")]
    Internal(String),

    #[error("Backend nicht unterstützt: {backend_type}")]
    UnsupportedBackend { backend_type: String },

    #[error("Ungültige Konfiguration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Rclone nicht gefunden. Bitte installieren Sie rclone von https://rclone.org")]
    RcloneNotFound,

    #[error("Rclone-Fehler: {message}")]
    RcloneError { message: String },
}

/// Typ-Alias für Results mit RusticGuiError
pub type Result<T> = std::result::Result<T, RusticGuiError>;

// Konvertierung von rustic_core Fehlern
impl From<rustic_core::RusticError> for RusticGuiError {
    fn from(error: rustic_core::RusticError) -> Self {
        RusticGuiError::RusticError { message: error.to_string() }
    }
}

/// Konvertierung für Tauri (braucht String)
impl From<RusticGuiError> for String {
    fn from(error: RusticGuiError) -> String {
        error.to_string()
    }
}

// Für Tauri InvokeError
impl From<RusticGuiError> for tauri::ipc::InvokeError {
    fn from(error: RusticGuiError) -> Self {
        tauri::ipc::InvokeError::from(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let error = RusticGuiError::RepositoryNotFound { path: "/tmp/repo".into() };
        assert_eq!(error.to_string(), "Repository nicht gefunden: /tmp/repo");
    }

    #[test]
    fn test_error_to_string_conversion() {
        let error = RusticGuiError::AuthenticationFailed;
        let error_string: String = error.into();
        assert_eq!(error_string, "Authentifizierung fehlgeschlagen");
    }

    #[test]
    fn test_snapshot_not_found_error() {
        let error = RusticGuiError::SnapshotNotFound { id: "snapshot-123".into() };
        assert_eq!(error.to_string(), "Snapshot nicht gefunden: snapshot-123");
    }

    #[test]
    fn test_backup_failed_error() {
        let error = RusticGuiError::BackupFailed { reason: "Disk full".into() };
        assert_eq!(error.to_string(), "Backup fehlgeschlagen: Disk full");
    }

    #[test]
    fn test_restore_failed_error() {
        let error = RusticGuiError::RestoreFailed { reason: "Permission denied".into() };
        assert_eq!(error.to_string(), "Restore fehlgeschlagen: Permission denied");
    }

    #[test]
    fn test_invalid_config_error() {
        let error = RusticGuiError::InvalidConfig { field: "repository.path".into() };
        assert_eq!(error.to_string(), "Konfiguration ungültig: repository.path");
    }

    #[test]
    fn test_io_error_conversion() {
        use std::io;
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let rustic_error: RusticGuiError = io_error.into();
        match rustic_error {
            RusticGuiError::IoError(_) => {}
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<String>("invalid json").unwrap_err();
        let rustic_error: RusticGuiError = json_error.into();
        match rustic_error {
            RusticGuiError::JsonError(_) => {}
            _ => panic!("Expected JsonError"),
        }
    }

    #[test]
    fn test_toml_error_creation() {
        let error = RusticGuiError::TomlError("invalid toml".into());
        assert_eq!(error.to_string(), "TOML-Fehler: invalid toml");
    }

    #[test]
    fn test_internal_error() {
        let error = RusticGuiError::Internal("something went wrong".into());
        assert_eq!(error.to_string(), "Interner Fehler: something went wrong");
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = RusticGuiError::RepositoryLocked;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("RepositoryLocked"));
    }
}
