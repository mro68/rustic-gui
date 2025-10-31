use crate::config::AppConfig;
use crate::scheduler::BackupScheduler;
use parking_lot::Mutex;
use rustic_backend::BackendOptions;
use rustic_core::{NoProgressBars, OpenStatus, Repository, RepositoryOptions};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex as AsyncMutex;
use tokio_util::sync::CancellationToken;

/// Cached Repository mit Timeout
struct CachedRepository {
    repository: Arc<Repository<NoProgressBars, OpenStatus>>,
    last_accessed: Instant,
}

/// Cache-Timeout: 5 Minuten
const CACHE_TIMEOUT: Duration = Duration::from_secs(300);

/// Globaler Application-State.
///
/// Wird über Tauri's `.manage()` geteilt und ist in allen Commands verfügbar.
/// Verwaltet das aktuell geöffnete Repository, laufende Backups und den Scheduler.
#[derive(Clone)]
pub struct AppState {
    /// ID des aktuell ausgewählten Repositories
    pub current_repository_id: Arc<Mutex<Option<String>>>,

    /// Cache für geöffnete Repositories (ID -> Repository)
    /// Repositories werden für 5 Minuten gecacht um wiederholte Opens zu vermeiden
    repository_cache: Arc<Mutex<HashMap<String, CachedRepository>>>,

    /// Cancellation-Tokens für laufende Backups
    /// Key: Job-ID, Value: CancellationToken
    pub cancellation_tokens: Arc<Mutex<HashMap<String, CancellationToken>>>,

    /// Job-Scheduler für zeitgesteuerte Backups (M3)
    pub scheduler: Arc<AsyncMutex<Option<BackupScheduler>>>,

    /// App-Konfiguration (TOML)
    pub config: Arc<Mutex<AppConfig>>,
}

impl AppState {
    /// Repository-Cache Timeout (5 Minuten)
    const CACHE_TIMEOUT: Duration = Duration::from_secs(300);

    /// Erstellt neuen AppState und lädt Config.
    ///
    /// Der Scheduler wird NICHT in new() initialisiert, da dies async sein muss.
    /// Verwende stattdessen init_scheduler() nach der Erstellung.
    pub fn new() -> crate::error::Result<Self> {
        let config = AppConfig::load().unwrap_or_default();

        Ok(Self {
            current_repository_id: Arc::new(Mutex::new(None)),
            repository_cache: Arc::new(Mutex::new(HashMap::new())),
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
        self.current_repository_id.lock().is_some()
    }

    /// Setzt das aktuelle Repository
    pub fn set_current_repository(&self, repo_id: Option<String>) {
        let mut current = self.current_repository_id.lock();
        *current = repo_id;
    }

    /// Gibt die ID des aktuellen Repositories zurück
    pub fn get_current_repository_id(&self) -> Option<String> {
        self.current_repository_id.lock().clone()
    }

    /// Öffnet ein Repository und gibt eine Referenz zurück
    ///
    /// Verwendet den Cache für bessere Performance. Repositories werden
    /// für 5 Minuten gecacht und automatisch aus dem Cache entfernt wenn älter.
    ///
    /// # Arguments
    /// * `repository_id` - ID des Repositories (aus Config)
    ///
    /// # Returns
    /// Result mit Arc<Repository>-Referenz
    ///
    /// # Errors
    /// - RepositoryNotFound: Repository-ID existiert nicht in Config
    /// - AuthenticationFailed: Passwort falsch oder fehlt
    /// - RusticError: Fehler beim Öffnen des Repositories
    pub fn get_repository(
        &self,
        repository_id: &str,
    ) -> crate::error::Result<Arc<Repository<NoProgressBars, OpenStatus>>> {
        // 1. Prüfe Cache
        {
            let mut cache = self.repository_cache.lock();
            
            // Cleanup: Entferne alte Einträge
            cache.retain(|_, cached| {
                cached.last_accessed.elapsed() < CACHE_TIMEOUT
            });
            
            // Prüfe ob gecacht und noch gültig
            if let Some(cached) = cache.get_mut(repository_id) {
                cached.last_accessed = Instant::now();
                tracing::debug!("Repository {} aus Cache geladen", repository_id);
                return Ok(cached.repository.clone());
            }
        }

        // 2. Nicht im Cache - Repository öffnen
        let config = self.config.lock();
        let repo_config = config
            .get_repository(repository_id)
            .ok_or_else(|| crate::error::RusticGuiError::Internal(
                format!("Repository {} nicht in Config gefunden", repository_id)
            ))?;

        // 3. Passwort aus Keychain holen (falls gespeichert)
        let password = if repo_config.password_stored {
            crate::keychain::load_password(repository_id)
                .map_err(|e| crate::error::RusticGuiError::Internal(
                    format!("Passwort konnte nicht geladen werden: {}", e)
                ))?
        } else {
            return Err(crate::error::RusticGuiError::Internal(
                "Passwort nicht gespeichert - Repository muss erst entsperrt werden".to_string()
            ));
        };

        // 4. Backend erstellen
        let backend_opts = BackendOptions::default().repository(&repo_config.path);
        let backends = backend_opts.to_backends().map_err(|e| {
            crate::error::RusticGuiError::RusticError {
                message: format!("Backend-Erstellung fehlgeschlagen: {}", e),
            }
        })?;

        // 5. Repository öffnen
        let repo_opts = RepositoryOptions::default().password(password);
        let repository = Repository::<NoProgressBars, _>::new(&repo_opts, &backends)
            .map_err(|e| crate::error::RusticGuiError::RusticError {
                message: format!("Repository erstellen fehlgeschlagen: {}", e),
            })?
            .open()
            .map_err(|e| crate::error::RusticGuiError::RusticError {
                message: format!("Repository öffnen fehlgeschlagen: {}", e),
            })?;

        // 6. In Cache speichern
        let repository_arc = Arc::new(repository);
        {
            let mut cache = self.repository_cache.lock();
            cache.insert(
                repository_id.to_string(),
                CachedRepository {
                    repository: repository_arc.clone(),
                    last_accessed: Instant::now(),
                },
            );
            tracing::info!("Repository {} geöffnet und gecacht", repository_id);
        }

        Ok(repository_arc)
    }

    /// Helper: Führt eine Funktion mit dem aktuellen Repository aus
    ///
    /// # Arguments
    /// * `f` - Funktion die das Repository nutzt
    ///
    /// # Returns
    /// Result mit Rückgabewert der Funktion
    ///
    /// # Errors
    /// - NoRepositorySelected: Kein Repository ausgewählt
    /// - Weitere Fehler von get_repository()
    pub fn with_current_repo<F, R>(&self, f: F) -> crate::error::Result<R>
    where
        F: FnOnce(&Repository<NoProgressBars, OpenStatus>) -> crate::error::Result<R>,
    {
        let repo_id = self.get_current_repository_id().ok_or_else(|| {
            crate::error::RusticGuiError::Internal("Kein Repository ausgewählt".to_string())
        })?;

        let repository = self.get_repository(&repo_id)?;
        f(&*repository)
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
        let result = state.with_current_repo(|_repo| Ok(()));
        assert!(matches!(
            result,
            Err(crate::error::RusticGuiError::Internal(_))
        ));
    }

    #[test]
    fn test_save_config_placeholder() {
        let state = AppState::new().unwrap();
        let result = state.save_config();
        assert!(result.is_ok());
    }
}
