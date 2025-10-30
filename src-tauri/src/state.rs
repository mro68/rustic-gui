use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;
use crate::config::AppConfig;

/// Platzhalter für Repository-Typ bis rustic_core Integration fertig ist
type Repository = (); // TODO: Ersetze mit rustic_core::Repository<...>

/// Platzhalter für Scheduler bis Job-Scheduling implementiert ist
type BackupScheduler = (); // TODO: Ersetze mit richtiger Scheduler-Implementierung

/// Globaler Application-State.
///
/// Wird über Tauri's `.manage()` geteilt und ist in allen Commands verfügbar.
/// Verwaltet das aktuell geöffnete Repository, laufende Backups und den Scheduler.
#[derive(Clone)]
pub struct AppState {
    /// Aktuell geöffnetes Repository
    pub current_repo: Arc<Mutex<Option<Repository>>>,

    /// Cancellation-Tokens für laufende Backups
    /// Key: Job-ID, Value: CancellationToken
    pub cancellation_tokens: Arc<Mutex<HashMap<String, CancellationToken>>>,

    /// Job-Scheduler für zeitgesteuerte Backups
    pub scheduler: Arc<AsyncMutex<BackupScheduler>>,

    /// App-Konfiguration (TOML)
    pub config: Arc<Mutex<AppConfig>>,
}

impl AppState {
    /// Erstellt neuen AppState und lädt Config.
    pub fn new() -> crate::error::Result<Self> {
        let config = AppConfig::load().unwrap_or_default();

        Ok(Self {
            current_repo: Arc::new(Mutex::new(None)),
            cancellation_tokens: Arc::new(Mutex::new(HashMap::new())),
            scheduler: Arc::new(AsyncMutex::new(())), // Platzhalter für BackupScheduler
            config: Arc::new(Mutex::new(config)),
        })
    }

    /// Helper: Hole aktuelles Repository oder gib Fehler zurück.
    pub fn get_current_repo(&self) -> crate::error::Result<Repository> {
        self.current_repo
            .lock()
            .clone()
            .ok_or_else(|| crate::error::RusticGuiError::RepositoryNotFound {
                path: "Kein Repository geöffnet".to_string(),
            })
    }

    /// Helper: Speichert Config auf Disk.
    pub fn save_config(&self) -> crate::error::Result<()> {
        let config = self.config.lock().clone();
        config.save().map_err(|e| {
            crate::error::RusticGuiError::ConfigError {
                message: format!("Config speichern fehlgeschlagen: {}", e),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new().unwrap();
        assert!(state.current_repo.lock().is_none());
        assert!(state.cancellation_tokens.lock().is_empty());
    }

    #[test]
    fn test_get_current_repo_when_none() {
        let state = AppState::new().unwrap();
        let result = state.get_current_repo();
        assert!(matches!(result, Err(crate::error::RusticGuiError::RepositoryNotFound { .. })));
    }

    #[test]
    fn test_save_config_placeholder() {
        let state = AppState::new().unwrap();
        let result = state.save_config();
        assert!(result.is_ok());
    }
}