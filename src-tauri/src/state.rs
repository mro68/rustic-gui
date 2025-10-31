use crate::config::AppConfig;
use crate::scheduler::BackupScheduler;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;

// Für M1: Wir speichern das Repository nicht im State, da der Type komplex ist.
// Stattdessen öffnen wir es bei Bedarf neu (Performance-Trade-off für M1).
// TODO M2: Optimiere durch Caching des geöffneten Repositories
pub type RusticRepository = (); // Platzhalter

/// Globaler Application-State.
///
/// Wird über Tauri's `.manage()` geteilt und ist in allen Commands verfügbar.
/// Verwaltet das aktuell geöffnete Repository, laufende Backups und den Scheduler.
#[derive(Clone)]
pub struct AppState {
    /// Aktuell geöffnetes Repository
    pub current_repo: Arc<Mutex<Option<RusticRepository>>>,

    /// Cancellation-Tokens für laufende Backups
    /// Key: Job-ID, Value: CancellationToken
    pub cancellation_tokens: Arc<Mutex<HashMap<String, CancellationToken>>>,

    /// Job-Scheduler für zeitgesteuerte Backups (M3)
    pub scheduler: Arc<AsyncMutex<Option<BackupScheduler>>>,

    /// App-Konfiguration (TOML)
    pub config: Arc<Mutex<AppConfig>>,
}

impl AppState {
    /// Erstellt neuen AppState und lädt Config.
    ///
    /// Der Scheduler wird NICHT in new() initialisiert, da dies async sein muss.
    /// Verwende stattdessen init_scheduler() nach der Erstellung.
    pub fn new() -> crate::error::Result<Self> {
        let config = AppConfig::load().unwrap_or_default();

        Ok(Self {
            current_repo: Arc::new(Mutex::new(None)),
            cancellation_tokens: Arc::new(Mutex::new(HashMap::new())),
            scheduler: Arc::new(AsyncMutex::new(None)), // Wird async initialisiert
            config: Arc::new(Mutex::new(config)),
        })
    }

    /// Initialisiert den Scheduler (async)
    ///
    /// Muss nach new() aufgerufen werden, idealerweise beim App-Start.
    ///
    /// # Returns
    /// Result mit () bei Erfolg
    ///
    /// # Errors
    /// Gibt einen Fehler zurück wenn Scheduler-Initialisierung fehlschlägt
    pub async fn init_scheduler(&self) -> crate::error::Result<()> {
        let mut scheduler = self.scheduler.lock().await;
        *scheduler = Some(BackupScheduler::new().await.map_err(|e| {
            crate::error::RusticGuiError::Internal(format!(
                "Scheduler-Initialisierung fehlgeschlagen: {}",
                e
            ))
        })?);

        tracing::info!("AppState: Scheduler erfolgreich initialisiert");
        Ok(())
    }

    /// Helper: Prüft ob ein Repository geöffnet ist.
    pub fn has_current_repo(&self) -> bool {
        // TODO M2: Echte Repository-Prüfung
        false
    }

    /// Helper: TODO M2 - Repository-Zugriff muss neu designed werden
    pub fn with_current_repo<F, R>(&self, _f: F) -> crate::error::Result<R>
    where
        F: FnOnce() -> crate::error::Result<R>,
    {
        // TODO M2: Implementiere wenn Repository-Caching fertig ist
        Err(crate::error::RusticGuiError::Internal(
            "Repository-Zugriff noch nicht implementiert (M2)".to_string(),
        ))
    }

    /// Helper: Speichert Config auf Disk.
    pub fn save_config(&self) -> crate::error::Result<()> {
        let config = self.config.lock().clone();
        config.save().map_err(|e| crate::error::RusticGuiError::ConfigError {
            message: format!("Config speichern fehlgeschlagen: {}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new().unwrap();
        assert!(!state.has_current_repo());
        assert!(state.cancellation_tokens.lock().is_empty());
    }

    #[tokio::test]
    async fn test_scheduler_initialization() {
        let state = AppState::new().unwrap();
        let result = state.init_scheduler().await;
        assert!(result.is_ok());

        // Prüfe ob Scheduler gesetzt ist
        let scheduler = state.scheduler.lock().await;
        assert!(scheduler.is_some());
    }

    #[test]
    fn test_get_current_repo_when_none() {
        let state = AppState::new().unwrap();
        let result = state.with_current_repo(|| Ok(()));
        assert!(matches!(
            result,
            Err(crate::error::RusticGuiError::Internal { .. })
        ));
    }

    #[test]
    fn test_save_config_placeholder() {
        let state = AppState::new().unwrap();
        let result = state.save_config();
        assert!(result.is_ok());
    }
}
