use thiserror::Error;

/// Haupt-Fehlertypen für rustic-gui
///
/// Alle Fehler implementieren std::error::Error und können
/// in Strings konvertiert werden für Tauri.
#[derive(Debug, Error)]
pub enum RusticGuiError {
    #[error("Repository nicht gefunden: {path}")]
    RepositoryNotFound { path: String },

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

    #[error("IO-Fehler: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON-Fehler: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML-Fehler: {0}")]
    TomlError(String),

    #[error("Interner Fehler: {0}")]
    Internal(String),
}

/// Typ-Alias für Results mit RusticGuiError
pub type Result<T> = std::result::Result<T, RusticGuiError>;

/// Konvertierung für Tauri (braucht String)
impl From<RusticGuiError> for String {
    fn from(error: RusticGuiError) -> String {
        error.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let error = RusticGuiError::RepositoryNotFound {
            path: "/tmp/repo".into(),
        };
        assert_eq!(
            error.to_string(),
            "Repository nicht gefunden: /tmp/repo"
        );
    }

    #[test]
    fn test_error_to_string_conversion() {
        let error = RusticGuiError::AuthenticationFailed;
        let error_string: String = error.into();
        assert_eq!(error_string, "Authentifizierung fehlgeschlagen");
    }
}