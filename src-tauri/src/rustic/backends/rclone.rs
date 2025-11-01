/// Rclone-Backend-Integration für 70+ Cloud-Provider
///
/// Unterstützt SFTP, Google Drive, Dropbox, OneDrive, pCloud, Mega,
/// FTP/FTPS, WebDAV und viele weitere Provider via rclone.
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Konfiguration für Rclone-Backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcloneConfig {
    /// Name des Rclone-Remote
    pub remote_name: String,
    /// Rclone-Provider-Typ (sftp, gdrive, dropbox, etc.)
    pub provider: String,
    /// Pfad im Remote
    pub path: String,
    /// Provider-spezifische Optionen
    pub options: BTreeMap<String, String>,
}

/// Manager für Rclone-Operationen
pub struct RcloneManager;

impl RcloneManager {
    /// Erstellt einen neuen RcloneManager
    ///
    /// # Returns
    /// Ok(RcloneManager) wenn rclone installiert ist
    ///
    /// # Errors
    /// Gibt einen Fehler zurück wenn rclone nicht installiert ist
    pub fn new() -> Result<Self> {
        // Prüfe ob rclone binary verfügbar ist
        match std::process::Command::new("rclone").arg("version").output() {
            Ok(output) => {
                if !output.status.success() {
                    return Err(crate::error::RusticGuiError::RcloneNotFound);
                }
                tracing::debug!("Rclone gefunden: {}", String::from_utf8_lossy(&output.stdout));
            }
            Err(_) => {
                return Err(crate::error::RusticGuiError::RcloneNotFound);
            }
        }

        Ok(Self)
    }

    /// Prüft ob ein Remote bereits konfiguriert ist
    ///
    /// # Arguments
    /// * `remote_name` - Name des zu prüfenden Remote
    ///
    /// # Returns
    /// true wenn Remote existiert, false sonst
    pub fn remote_exists(&self, remote_name: &str) -> bool {
        match std::process::Command::new("rclone").args(&["listremotes"]).output() {
            Ok(output) => {
                let remotes = String::from_utf8_lossy(&output.stdout);
                remotes.lines().any(|line| line.trim_end_matches(':') == remote_name)
            }
            Err(_) => false,
        }
    }

    /// Listet alle konfigurierten Rclone-Remotes
    ///
    /// # Returns
    /// Vec mit Remote-Namen
    pub fn list_remotes(&self) -> Result<Vec<String>> {
        let output =
            std::process::Command::new("rclone").args(&["listremotes"]).output().map_err(|e| {
                crate::error::RusticGuiError::RcloneError {
                    message: format!("Fehler beim Auflisten von Remotes: {}", e),
                }
            })?;

        if !output.status.success() {
            return Err(crate::error::RusticGuiError::RcloneError {
                message: format!("Rclone-Fehler: {}", String::from_utf8_lossy(&output.stderr)),
            });
        }

        let remotes: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim_end_matches(':').to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(remotes)
    }
}

/// Erstellt ein Rclone-Backend für rustic
///
/// # Arguments
/// * `config` - Rclone-Konfiguration
///
/// # Returns
/// BTreeMap mit Optionen für rustic_backend::RcloneBackend
///
/// # Errors
/// Gibt einen Fehler zurück wenn Konfiguration ungültig ist
pub fn create_rclone_backend(config: &RcloneConfig) -> Result<BTreeMap<String, String>> {
    validate_rclone_config(config)?;

    let mut options = config.options.clone();

    // Typ setzen
    options.insert("type".to_string(), config.provider.clone());

    tracing::debug!(
        "Rclone-Backend erstellt: Remote={}, Provider={}, Path={}",
        config.remote_name,
        config.provider,
        config.path
    );

    Ok(options)
}

/// Erstellt ein SFTP-Backend via Rclone
///
/// # Arguments
/// * `host` - SFTP-Server-Host
/// * `port` - SFTP-Server-Port (normalerweise 22)
/// * `user` - SFTP-Benutzername
/// * `password` - SFTP-Passwort
/// * `path` - Pfad auf dem Server
///
/// # Returns
/// RcloneConfig für SFTP
///
/// # Example
/// ```no_run
/// use rustic_gui_lib::rustic::backends::rclone::create_sftp_backend;
///
/// let config = create_sftp_backend(
///     "sftp.example.com",
///     22,
///     "username",
///     "password",
///     "/backup"
/// ).unwrap();
/// ```
pub fn create_sftp_backend(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
    path: &str,
) -> Result<RcloneConfig> {
    if host.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "SFTP-Host darf nicht leer sein".to_string(),
        });
    }

    if user.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "SFTP-Benutzername darf nicht leer sein".to_string(),
        });
    }

    let mut options = BTreeMap::new();
    options.insert("host".to_string(), host.to_string());
    options.insert("port".to_string(), port.to_string());
    options.insert("user".to_string(), user.to_string());

    // Passwort nur setzen wenn nicht leer
    if !password.is_empty() {
        options.insert("pass".to_string(), password.to_string());
    }

    // Remote-Name basierend auf Host generieren
    let remote_name = format!("rustic_sftp_{}", host.replace(".", "_").replace(":", "_"));

    Ok(RcloneConfig { remote_name, provider: "sftp".to_string(), path: path.to_string(), options })
}

/// Validiert eine Rclone-Konfiguration
///
/// # Arguments
/// * `config` - Zu validierende Konfiguration
///
/// # Returns
/// Ok(()) wenn Konfiguration valide ist
pub fn validate_rclone_config(config: &RcloneConfig) -> Result<()> {
    if config.remote_name.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Remote-Name darf nicht leer sein".to_string(),
        });
    }

    if config.provider.is_empty() {
        return Err(crate::error::RusticGuiError::InvalidConfiguration {
            message: "Provider darf nicht leer sein".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sftp_backend() {
        let config =
            create_sftp_backend("sftp.example.com", 22, "testuser", "testpass", "/backup").unwrap();

        assert_eq!(config.provider, "sftp");
        assert_eq!(config.path, "/backup");
        assert_eq!(config.options.get("host"), Some(&"sftp.example.com".to_string()));
        assert_eq!(config.options.get("port"), Some(&"22".to_string()));
        assert_eq!(config.options.get("user"), Some(&"testuser".to_string()));
        assert_eq!(config.options.get("pass"), Some(&"testpass".to_string()));
    }

    #[test]
    fn test_create_sftp_backend_without_password() {
        let config =
            create_sftp_backend("sftp.example.com", 22, "testuser", "", "/backup").unwrap();

        assert!(config.options.get("pass").is_none());
    }

    #[test]
    fn test_create_sftp_backend_empty_host() {
        let result = create_sftp_backend("", 22, "user", "pass", "/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_sftp_backend_empty_user() {
        let result = create_sftp_backend("host", 22, "", "pass", "/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rclone_config_success() {
        let config = RcloneConfig {
            remote_name: "test_remote".to_string(),
            provider: "sftp".to_string(),
            path: "/backup".to_string(),
            options: BTreeMap::new(),
        };

        assert!(validate_rclone_config(&config).is_ok());
    }

    #[test]
    fn test_validate_rclone_config_empty_remote_name() {
        let config = RcloneConfig {
            remote_name: "".to_string(),
            provider: "sftp".to_string(),
            path: "/backup".to_string(),
            options: BTreeMap::new(),
        };

        assert!(validate_rclone_config(&config).is_err());
    }

    #[test]
    fn test_validate_rclone_config_empty_provider() {
        let config = RcloneConfig {
            remote_name: "test".to_string(),
            provider: "".to_string(),
            path: "/backup".to_string(),
            options: BTreeMap::new(),
        };

        assert!(validate_rclone_config(&config).is_err());
    }

    #[test]
    fn test_create_rclone_backend() {
        let mut options = BTreeMap::new();
        options.insert("host".to_string(), "example.com".to_string());

        let config = RcloneConfig {
            remote_name: "test_remote".to_string(),
            provider: "sftp".to_string(),
            path: "/backup".to_string(),
            options: options.clone(),
        };

        let backend_options = create_rclone_backend(&config).unwrap();
        assert_eq!(backend_options.get("type"), Some(&"sftp".to_string()));
        assert_eq!(backend_options.get("host"), Some(&"example.com".to_string()));
    }
}
