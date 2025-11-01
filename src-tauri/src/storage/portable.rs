use crate::config::AppConfig;
use crate::error::{Result as RusticResult, RusticGuiError};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.toml";
const PORTABLE_WRITE_TEST_FILE: &str = ".rustic_gui_portable_write_test";
const PORTABLE_KEY_FILE: &str = ".rustic_gui_portable.key";
const PORTABLE_FORMAT_VERSION: u8 = 1;
const NONCE_SIZE: usize = 12;

/// Statusinformationen für den portablen Speicherpfad.
#[derive(Debug, Clone, Serialize)]
pub struct PortableStoreStatus {
    /// Ursprüngliches Verzeichnis (neben Binary)
    pub portable_dir: String,
    /// Effektiv genutztes Verzeichnis (Fallback bei Read-Only)
    pub effective_dir: String,
    /// Gibt an, ob das Ursprungsverzeichnis schreibgeschützt ist
    pub read_only: bool,
    /// Wurde ein Fallback-Verzeichnis verwendet?
    pub fallback_used: bool,
    /// Ist die Konfiguration im portablen Store verschlüsselt?
    pub encrypted: bool,
}

/// Verwaltet Pfade und Dateien für den portablen Konfigurationsspeicher.
#[derive(Clone)]
pub struct PortableStore {
    portable_dir: PathBuf,
    effective_dir: PathBuf,
    read_only: bool,
    fallback_used: bool,
    encryption: PortableEncryption,
}

#[derive(Clone)]
struct PortableEncryption {
    key_bytes: [u8; 32],
}

impl fmt::Debug for PortableStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortableStore")
            .field("portable_dir", &self.portable_dir)
            .field("effective_dir", &self.effective_dir)
            .field("read_only", &self.read_only)
            .field("fallback_used", &self.fallback_used)
            .finish()
    }
}

impl fmt::Debug for PortableEncryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PortableEncryption").finish_non_exhaustive()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedConfigFile {
    version: u8,
    nonce: String,
    ciphertext: String,
}

impl PortableStore {
    /// Initialisiert den portablen Store und legt Verzeichnisse an.
    pub fn initialize() -> RusticResult<Self> {
        let portable_dir = Self::determine_portable_dir();

        let (effective_dir, read_only, fallback_used) =
            if Self::is_directory_writable(&portable_dir) {
                tracing::info!(
                    "Portabler Konfigurationsspeicher wird in {} verwendet",
                    portable_dir.display()
                );
                (portable_dir.clone(), false, false)
            } else {
                tracing::warn!(
                    "Portabler Pfad {} ist schreibgeschützt – weiche auf Fallback aus",
                    portable_dir.display()
                );
                let fallback_dir = Self::determine_fallback_dir();
                if !Self::is_directory_writable(&fallback_dir) {
                    return Err(RusticGuiError::ConfigError {
                        message: format!(
                            "Fallback-Verzeichnis {} konnte nicht angelegt werden",
                            fallback_dir.display()
                        ),
                    });
                }

                // Kopiere bestehende Konfiguration einmalig ins Fallback, falls vorhanden
                let original_config = portable_dir.join(CONFIG_FILE_NAME);
                let fallback_config = fallback_dir.join(CONFIG_FILE_NAME);
                if original_config.exists() && !fallback_config.exists() {
                    if let Err(err) = fs::copy(&original_config, &fallback_config) {
                        tracing::warn!(
                            "Konfiguration konnte nicht in Fallback kopiert werden: {}",
                            err
                        );
                    }
                }

                // Kopiere vorhandenen Schlüssel, damit bestehende Installationen funktionieren
                let original_key = portable_dir.join(PORTABLE_KEY_FILE);
                let fallback_key = fallback_dir.join(PORTABLE_KEY_FILE);
                if original_key.exists() && !fallback_key.exists() {
                    if let Err(err) = fs::copy(&original_key, &fallback_key) {
                        tracing::warn!(
                            "Schlüsseldatei konnte nicht in Fallback kopiert werden: {}",
                            err
                        );
                    }
                }

                (fallback_dir, true, true)
            };

        if let Err(err) = fs::create_dir_all(&effective_dir) {
            tracing::error!(
                "Konfigurationsverzeichnis {} konnte nicht erstellt werden: {}",
                effective_dir.display(),
                err
            );
            return Err(RusticGuiError::ConfigError {
                message: format!(
                    "Konfigurationsverzeichnis {} konnte nicht erstellt werden",
                    effective_dir.display()
                ),
            });
        }

        let encryption = PortableEncryption::initialize(&effective_dir)?;

        Ok(Self { portable_dir, effective_dir, read_only, fallback_used, encryption })
    }

    /// Liefert den Pfad zur aktiven Konfigurationsdatei.
    pub fn config_path(&self) -> PathBuf {
        self.effective_dir.join(CONFIG_FILE_NAME)
    }

    /// Liefert den Pfad zum Ursprungs-Verzeichnis (neben Binary).
    pub fn portable_dir(&self) -> PathBuf {
        self.portable_dir.clone()
    }

    /// Speicherung im portablen Store.
    pub fn save_config(&self, config: &AppConfig) -> RusticResult<()> {
        if self.read_only && !self.fallback_used {
            return Err(RusticGuiError::ConfigError {
                message: "Portabler Speicher ist schreibgeschützt".to_string(),
            });
        }

        let toml = toml::to_string_pretty(config).map_err(|e| RusticGuiError::ConfigError {
            message: format!("Konfiguration konnte nicht serialisiert werden: {}", e),
        })?;

        let encrypted = self.encryption.encrypt(toml.as_bytes())?;
        let payload =
            serde_json::to_vec_pretty(&encrypted).map_err(|e| RusticGuiError::ConfigError {
                message: format!(
                    "Verschlüsseltes Konfigurationsformat konnte nicht erstellt werden: {}",
                    e
                ),
            })?;

        fs::write(self.config_path(), payload).map_err(|e| RusticGuiError::ConfigError {
            message: format!("Konfiguration konnte nicht geschrieben werden: {}", e),
        })
    }

    /// Lädt die Konfiguration aus dem Store.
    pub fn load_config(&self) -> RusticResult<AppConfig> {
        let path = self.config_path();

        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&path).map_err(|e| RusticGuiError::ConfigError {
            message: format!("Konfiguration konnte nicht gelesen werden: {}", e),
        })?;

        match serde_json::from_str::<EncryptedConfigFile>(&content) {
            Ok(encrypted) => {
                let plaintext = self.encryption.decrypt(&encrypted)?;
                let toml_str =
                    String::from_utf8(plaintext).map_err(|e| RusticGuiError::ConfigError {
                        message: format!("Konfiguration konnte nicht interpretiert werden: {}", e),
                    })?;
                toml::from_str::<AppConfig>(&toml_str).map_err(|e| RusticGuiError::ConfigError {
                    message: format!("Konfiguration konnte nicht entschlüsselt werden: {}", e),
                })
            }
            Err(_) => {
                // Fallback: Legacy-Konfiguration ohne Verschlüsselung
                AppConfig::load_from_path(&path)
                    .map_err(|e| RusticGuiError::ConfigError { message: e.to_string() })
            }
        }
    }

    /// Gibt den Status für das Frontend zurück.
    pub fn status(&self) -> PortableStoreStatus {
        PortableStoreStatus {
            portable_dir: self.portable_dir.display().to_string(),
            effective_dir: self.effective_dir.display().to_string(),
            read_only: self.read_only,
            fallback_used: self.fallback_used,
            encrypted: true,
        }
    }

    fn determine_portable_dir() -> PathBuf {
        if let Ok(custom) = std::env::var("RUSTIC_GUI_PORTABLE_PATH") {
            return PathBuf::from(custom);
        }

        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                return parent.to_path_buf();
            }
        }

        tracing::warn!(
            "Konnte Verzeichnis neben Binary nicht ermitteln – nutze Standard-Config-Verzeichnis"
        );
        dirs::config_dir().unwrap_or_else(std::env::temp_dir).join("rustic-gui")
    }

    fn determine_fallback_dir() -> PathBuf {
        if let Ok(custom) = std::env::var("RUSTIC_GUI_FALLBACK_PATH") {
            return PathBuf::from(custom);
        }

        std::env::temp_dir().join("rustic-gui-portable")
    }

    fn is_directory_writable(dir: &Path) -> bool {
        if !dir.exists() {
            if let Err(err) = fs::create_dir_all(dir) {
                tracing::debug!(
                    "Verzeichnis {} konnte nicht erstellt werden: {}",
                    dir.display(),
                    err
                );
                return false;
            }
        }

        let test_path = dir.join(PORTABLE_WRITE_TEST_FILE);
        match OpenOptions::new().create(true).write(true).truncate(true).open(&test_path) {
            Ok(_) => {
                let _ = fs::remove_file(&test_path);
                true
            }
            Err(err) => {
                tracing::debug!("Schreibtest in {} fehlgeschlagen: {}", dir.display(), err);
                false
            }
        }
    }
}

impl PortableEncryption {
    fn initialize(dir: &Path) -> RusticResult<Self> {
        let key_path = dir.join(PORTABLE_KEY_FILE);

        if key_path.exists() {
            let encoded =
                fs::read_to_string(&key_path).map_err(|e| RusticGuiError::ConfigError {
                    message: format!("Schlüsseldatei konnte nicht gelesen werden: {}", e),
                })?;
            let key_bytes = Self::decode_key(encoded.trim())?;
            Ok(Self { key_bytes })
        } else {
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);
            let encoded = general_purpose::STANDARD.encode(key);
            Self::write_key_file(&key_path, &encoded)?;
            Ok(Self { key_bytes: key })
        }
    }

    fn encrypt(&self, plaintext: &[u8]) -> RusticResult<EncryptedConfigFile> {
        let cipher = Aes256Gcm::new_from_slice(&self.key_bytes).map_err(|e| {
            RusticGuiError::ConfigError {
                message: format!("Verschlüsselung konnte nicht initialisiert werden: {}", e),
            }
        })?;

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext =
            cipher.encrypt(nonce, plaintext).map_err(|e| RusticGuiError::ConfigError {
                message: format!("Verschlüsselung der Konfiguration fehlgeschlagen: {}", e),
            })?;

        Ok(EncryptedConfigFile {
            version: PORTABLE_FORMAT_VERSION,
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            ciphertext: general_purpose::STANDARD.encode(ciphertext),
        })
    }

    fn decrypt(&self, payload: &EncryptedConfigFile) -> RusticResult<Vec<u8>> {
        if payload.version != PORTABLE_FORMAT_VERSION {
            return Err(RusticGuiError::ConfigError {
                message: format!("Unbekannte Konfigurationsversion: {}", payload.version),
            });
        }

        let nonce_bytes = general_purpose::STANDARD.decode(&payload.nonce).map_err(|e| {
            RusticGuiError::ConfigError {
                message: format!("Nonce der verschlüsselten Konfiguration ungültig: {}", e),
            }
        })?;

        if nonce_bytes.len() != NONCE_SIZE {
            return Err(RusticGuiError::ConfigError {
                message: "Nonce-Länge der verschlüsselten Konfiguration ist ungültig".to_string(),
            });
        }

        let ciphertext = general_purpose::STANDARD.decode(&payload.ciphertext).map_err(|e| {
            RusticGuiError::ConfigError {
                message: format!(
                    "Verschlüsselte Konfiguration konnte nicht dekodiert werden: {}",
                    e
                ),
            }
        })?;

        let cipher = Aes256Gcm::new_from_slice(&self.key_bytes).map_err(|e| {
            RusticGuiError::ConfigError {
                message: format!("Entschlüsselung konnte nicht initialisiert werden: {}", e),
            }
        })?;

        cipher.decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref()).map_err(|e| {
            RusticGuiError::ConfigError {
                message: format!("Konfiguration konnte nicht entschlüsselt werden: {}", e),
            }
        })
    }

    fn decode_key(encoded: &str) -> RusticResult<[u8; 32]> {
        let decoded =
            general_purpose::STANDARD.decode(encoded).map_err(|e| RusticGuiError::ConfigError {
                message: format!("Schlüsseldatei ist beschädigt: {}", e),
            })?;

        let key: [u8; 32] = decoded.try_into().map_err(|_| RusticGuiError::ConfigError {
            message: "Schlüsseldatei hat eine ungültige Länge".to_string(),
        })?;

        Ok(key)
    }

    fn write_key_file(path: &Path, encoded: &str) -> RusticResult<()> {
        let mut file =
            OpenOptions::new().create(true).write(true).truncate(true).open(path).map_err(|e| {
                RusticGuiError::ConfigError {
                    message: format!("Schlüsseldatei konnte nicht angelegt werden: {}", e),
                }
            })?;

        file.write_all(encoded.as_bytes()).map_err(|e| RusticGuiError::ConfigError {
            message: format!("Schlüsseldatei konnte nicht geschrieben werden: {}", e),
        })?;
        file.flush().map_err(|e| RusticGuiError::ConfigError {
            message: format!("Schlüsseldatei konnte nicht synchronisiert werden: {}", e),
        })?;

        #[cfg(unix)]
        {
            if let Err(err) = fs::set_permissions(path, fs::Permissions::from_mode(0o600)) {
                tracing::warn!(
                    "Schlüsseldatei {} konnte nicht auf 0600 gesetzt werden: {}",
                    path.display(),
                    err
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    struct PortableTestGuard {
        _dir: TempDir,
        previous: Option<std::ffi::OsString>,
    }

    impl PortableTestGuard {
        fn new() -> Self {
            let dir = TempDir::new().expect("TempDir konnte nicht erstellt werden");
            let previous = env::var_os("RUSTIC_GUI_PORTABLE_PATH");
            unsafe {
                env::set_var("RUSTIC_GUI_PORTABLE_PATH", dir.path());
            }
            Self { _dir: dir, previous }
        }
    }

    impl Drop for PortableTestGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                unsafe {
                    env::set_var("RUSTIC_GUI_PORTABLE_PATH", value);
                }
            } else {
                unsafe {
                    env::remove_var("RUSTIC_GUI_PORTABLE_PATH");
                }
            }
        }
    }

    #[test]
    fn test_encrypted_roundtrip() {
        let _guard = PortableTestGuard::new();
        let store =
            PortableStore::initialize().expect("PortableStore konnte nicht initialisiert werden");

        let mut config = AppConfig::default();
        config.settings.theme = "dark".to_string();

        store.save_config(&config).expect("Konfiguration konnte nicht gespeichert werden");

        let content = fs::read_to_string(store.config_path())
            .expect("Konfiguration konnte nicht gelesen werden");
        assert!(content.contains("ciphertext"));
        assert!(serde_json::from_str::<EncryptedConfigFile>(&content).is_ok());

        let loaded = store.load_config().expect("Konfiguration konnte nicht geladen werden");
        assert_eq!(loaded.settings.theme, "dark");
    }

    #[test]
    fn test_plain_config_migration() {
        let _guard = PortableTestGuard::new();
        let store =
            PortableStore::initialize().expect("PortableStore konnte nicht initialisiert werden");

        let config_path = store.config_path();
        let legacy = toml::to_string(&AppConfig::default())
            .expect("Legacy-Konfiguration konnte nicht serialisiert werden");
        fs::write(&config_path, legacy)
            .expect("Legacy-Konfiguration konnte nicht geschrieben werden");

        let loaded = store.load_config().expect("Legacy-Konfiguration konnte nicht geladen werden");
        assert_eq!(loaded.settings.theme, "system");

        store.save_config(&loaded).expect("Konfiguration konnte nicht erneut gespeichert werden");
        let content =
            fs::read_to_string(config_path).expect("Konfiguration konnte nicht gelesen werden");
        assert!(serde_json::from_str::<EncryptedConfigFile>(&content).is_ok());
    }
}
