# Backend Instructions

> Rust + Tauri Backend-Entwicklung für Rustic GUI

---

## 🎯 Backend-Übersicht

Das Backend ist verantwortlich für:
- Integration mit **rustic_core** (Backup-Engine)
- **Tauri Commands** (IPC-Schnittstelle zum Frontend)
- **State-Management** (aktives Repository, laufende Jobs)
- **Config-Persistenz** (TOML-Dateien)
- **Job-Scheduling** (Cron-basierte Backups)
- **Passwort-Verwaltung** (System-Keychain)

---

## 🏗️ Projekt-Struktur

```
src-tauri/
├── Cargo.toml                    # Dependencies
├── tauri.conf.json               # Tauri-Konfiguration
├── build.rs                      # Build-Script
└── src/
    ├── main.rs                   # Entry Point + Command-Registrierung
    ├── state.rs                  # AppState-Definition
    ├── config.rs                 # Config-Management (TOML)
    ├── error.rs                  # Custom Error-Types
    ├── utils.rs                  # Helper-Funktionen
    ├── commands/                 # Tauri Commands
    │   ├── mod.rs
    │   ├── repository.rs         # Repository-Commands
    │   ├── backup.rs             # Backup-Commands
    │   ├── restore.rs            # Restore-Commands
    │   ├── snapshot.rs           # Snapshot-Commands
    │   └── system.rs             # System-Commands (Health, etc.)
    ├── rustic/                   # rustic_core Integration
    │   ├── mod.rs
    │   ├── repository.rs         # Repository-Handling
    │   ├── backup.rs             # Backup-Execution
    │   ├── restore.rs            # Restore-Execution
    │   └── snapshot.rs           # Snapshot-Operations
    ├── scheduler/                # Job-Scheduling
    │   ├── mod.rs
    │   └── cron.rs
    └── keychain/                 # Passwort-Management
        └── mod.rs
```

---

## 🚀 Main Entry Point

```rust
// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod error;
mod keychain;
mod rustic;
mod scheduler;
mod state;
mod utils;

use state::AppState;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Logging initialisieren
    setup_logging();
    
    tracing::info!("Rustic GUI wird gestartet...");
    
    // App-State erstellen
    let app_state = AppState::new().expect("AppState initialisieren fehlgeschlagen");
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Repository
            commands::repository::add_repository,
            commands::repository::list_repositories,
            commands::repository::switch_repository,
            commands::repository::remove_repository,
            commands::repository::init_repository,
            
            // Backup
            commands::backup::run_backup,
            commands::backup::cancel_backup,
            commands::backup::list_backup_jobs,
            commands::backup::create_backup_job,
            commands::backup::update_backup_job,
            commands::backup::delete_backup_job,
            
            // Restore
            commands::restore::restore_files,
            commands::restore::list_snapshot_files,
            
            // Snapshots
            commands::snapshot::list_snapshots,
            commands::snapshot::get_snapshot_details,
            commands::snapshot::compare_snapshots,
            commands::snapshot::delete_snapshot,
            commands::snapshot::forget_snapshots,
            
            // System
            commands::system::check_repository_health,
            commands::system::force_unlock_repository,
            
            // Config & Keychain
            commands::repository::save_repository_password,
            commands::repository::load_repository_password,
        ])
        .setup(|app| {
            tracing::info!("Tauri-App erfolgreich gestartet");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Tauri-Anwendung");
}

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info")
                .add_directive("rustic_gui=debug".parse().unwrap())
                .add_directive("rustic_core=info".parse().unwrap())
                .add_directive("rustic_backend=info".parse().unwrap())
        });
    
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_line_number(true)
        .with_file(false)
        .init();
}
```

---

## 🔧 AppState Pattern

```rust
// src/state.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;
use crate::config::AppConfig;
use crate::scheduler::BackupScheduler;

/// Globaler Application-State.
///
/// Wird über Tauri's `.manage()` geteilt und ist in allen Commands verfügbar.
pub struct AppState {
    /// Aktuell geöffnetes Repository
    pub current_repo: Arc<Mutex<Option<rustic_core::Repository>>>,
    
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
    pub fn new() -> anyhow::Result<Self> {
        let config = AppConfig::load().unwrap_or_default();
        
        Ok(Self {
            current_repo: Arc::new(Mutex::new(None)),
            cancellation_tokens: Arc::new(Mutex::new(HashMap::new())),
            scheduler: Arc::new(AsyncMutex::new(
                BackupScheduler::new().expect("Scheduler erstellen fehlgeschlagen")
            )),
            config: Arc::new(Mutex::new(config)),
        })
    }
    
    /// Helper: Hole aktuelles Repository oder gib Fehler zurück.
    pub fn get_current_repo(&self) -> Result<rustic_core::Repository, String> {
        self.current_repo
            .lock()
            .clone()
            .ok_or_else(|| "Kein Repository geöffnet".to_string())
    }
    
    /// Helper: Speichert Config auf Disk.
    pub fn save_config(&self) -> Result<(), String> {
        let config = self.config.lock().clone();
        config.save()
            .map_err(|e| format!("Config speichern fehlgeschlagen: {}", e))
    }
}
```

---

## 📝 Tauri Command Pattern

### Standard-Command-Template

```rust
/// Kurzbeschreibung was der Command macht.
///
/// Längere Beschreibung mit Details zur Funktionsweise.
/// Erklärt Parameter, Return-Werte und mögliche Fehler.
///
/// # Frontend-Usage
///
/// ```typescript
/// const result = await invoke('command_name', { param1: 'value' });
/// ```
#[tauri::command]
pub async fn command_name(
    param1: String,
    param2: Option<i32>,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<ReturnType, String> {
    // 1. VALIDIERUNG
    if param1.is_empty() {
        return Err("Parameter darf nicht leer sein".to_string());
    }
    
    // 2. LOGGING
    tracing::debug!(
        param1 = %param1,
        param2 = ?param2,
        "Command gestartet"
    );
    
    // 3. STATE-ZUGRIFF
    let repo = state.get_current_repo()?;
    
    // 4. BUSINESS-LOGIK
    let result = execute_logic(&repo, &param1)
        .await
        .map_err(|e| {
            tracing::error!("Fehler: {}", e);
            format!("Operation fehlgeschlagen: {}", e)
        })?;
    
    // 5. EVENTS (optional)
    app_handle.emit_all("event-name", &result).ok();
    
    // 6. LOGGING & RETURN
    tracing::info!("Command erfolgreich abgeschlossen");
    Ok(result)
}
```

### Command mit Progress-Events

```rust
#[tauri::command]
pub async fn run_backup(
    job_id: String,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<BackupResult, String> {
    tracing::info!("Backup gestartet: {}", job_id);
    
    // Cancellation-Token erstellen
    let token = CancellationToken::new();
    state.cancellation_tokens.lock()
        .insert(job_id.clone(), token.clone());
    
    // Backup mit Cancellation
    let result = tokio::select! {
        res = execute_backup(&job_id, &state, &app_handle) => res,
        _ = token.cancelled() => {
            tracing::warn!("Backup abgebrochen: {}", job_id);
            Err("Backup wurde abgebrochen".to_string())
        }
    };
    
    // Cleanup
    state.cancellation_tokens.lock().remove(&job_id);
    
    result
}

async fn execute_backup(
    job_id: &str,
    state: &tauri::State<'_, AppState>,
    app_handle: &tauri::AppHandle,
) -> Result<BackupResult, String> {
    // Job-Config laden
    let config = state.config.lock()
        .backup_jobs
        .iter()
        .find(|j| j.id == job_id)
        .ok_or("Job nicht gefunden")?
        .clone();
    
    let repo = state.get_current_repo()?;
    
    // Progress-Callback
    let app_handle = app_handle.clone();
    let job_id = job_id.to_string();
    let progress_fn = move |info: rustic_core::BackupProgress| {
        app_handle.emit_all(
            &format!("backup-progress-{}", job_id),
            serde_json::json!({
                "files_processed": info.files_done,
                "files_total": info.files_total,
                "bytes_processed": info.bytes_done,
                "bytes_total": info.bytes_total,
            })
        ).ok();
    };
    
    // Backup ausführen
    let snapshot = crate::rustic::backup::run(
        &repo,
        &config.source_paths,
        &config.tags,
        &config.exclude_patterns,
        progress_fn,
    )
    .await
    .map_err(|e| format!("Backup fehlgeschlagen: {}", e))?;
    
    Ok(BackupResult {
        snapshot_id: snapshot.id.to_string(),
        duration: snapshot.summary.total_duration_secs,
        files_processed: snapshot.summary.files_new + snapshot.summary.files_changed,
        bytes_processed: snapshot.summary.data_added,
    })
}
```

---

## 🔗 rustic_core Integration

### Repository öffnen

```rust
// src/rustic/repository.rs

use rustic_core::{Repository, RepositoryOptions};
use rustic_backend::BackendOptions;
use anyhow::{Context, Result};

/// Öffnet ein rustic Repository.
///
/// Unterstützt lokale und Cloud-Backends (via rclone).
pub async fn open(path: &str, password: &str) -> Result<Repository> {
    // Passwort in Env setzen (rustic_core liest daraus)
    std::env::set_var("RUSTIC_PASSWORD", password);
    
    // Backend-Optionen
    let backend_opts = BackendOptions::default()
        .repository(path);
    
    // Repository-Optionen
    let repo_opts = RepositoryOptions::default()
        .password(Some(password.to_string()));
    
    // Repository öffnen
    let repo = Repository::new(&repo_opts, &backend_opts)
        .context("Repository öffnen fehlgeschlagen")?;
    
    // Passwort IMMER aus Env entfernen
    std::env::remove_var("RUSTIC_PASSWORD");
    
    tracing::debug!("Repository erfolgreich geöffnet: {}", path);
    
    Ok(repo)
}

/// Initialisiert ein neues Repository.
pub async fn init(path: &str, password: &str, is_cloud: bool) -> Result<()> {
    std::env::set_var("RUSTIC_PASSWORD", password);
    
    let backend_opts = BackendOptions::default()
        .repository(path);
    
    let repo_opts = RepositoryOptions::default()
        .password(Some(password.to_string()));
    
    // Für Cloud: rclone-spezifische Config
    if is_cloud {
        // TODO: rclone-Backend-Konfiguration
    }
    
    Repository::init(&repo_opts, &backend_opts)
        .context("Repository initialisieren fehlgeschlagen")?;
    
    std::env::remove_var("RUSTIC_PASSWORD");
    
    tracing::info!("Repository initialisiert: {}", path);
    
    Ok(())
}
```

### Backup ausführen

```rust
// src/rustic/backup.rs

use rustic_core::{BackupOptions, SnapshotFile};
use std::path::PathBuf;
use anyhow::Result;

/// Führt Backup mit rustic_core aus.
pub async fn run<F>(
    repo: &Repository,
    source_paths: &[PathBuf],
    tags: &[String],
    excludes: &[String],
    progress_callback: F,
) -> Result<SnapshotFile>
where
    F: Fn(rustic_core::BackupProgress) + Send + Sync + 'static,
{
    // Validiere Pfade
    for path in source_paths {
        if !path.exists() {
            anyhow::bail!("Pfad existiert nicht: {}", path.display());
        }
    }
    
    // Backup-Optionen
    let opts = BackupOptions::default()
        .with_tags(tags.to_vec())
        .with_excludes(excludes.to_vec())
        .with_atime(false); // Reduziert Metadata
    
    tracing::info!(
        "Backup gestartet: {} Pfade, {} Tags",
        source_paths.len(),
        tags.len()
    );
    
    // Backup ausführen
    let snapshot = repo.backup(
        source_paths,
        &opts,
        Some(Box::new(progress_callback))
    )
    .await?;
    
    tracing::info!(
        "Backup abgeschlossen: Snapshot {}",
        snapshot.id
    );
    
    Ok(snapshot)
}
```

### Restore ausführen

```rust
// src/rustic/restore.rs

use rustic_core::{RestoreOptions, LocalDestination};
use std::path::PathBuf;
use anyhow::Result;

/// Stellt Dateien aus Snapshot wieder her.
pub async fn run<F>(
    repo: &Repository,
    snapshot_id: &str,
    files: Vec<String>,
    target: PathBuf,
    progress_callback: F,
) -> Result<()>
where
    F: Fn(rustic_core::RestoreProgress) + Send + Sync + 'static,
{
    // Snapshot laden
    let snapshot = repo.get_snapshot_from_str(snapshot_id)?;
    
    // Ziel vorbereiten
    if !target.exists() {
        std::fs::create_dir_all(&target)?;
    }
    
    let dest = LocalDestination::new(target)?;
    
    // Restore-Optionen
    let mut opts = RestoreOptions::default()
        .set_progress(Some(Box::new(progress_callback)));
    
    if !files.is_empty() {
        opts = opts.with_files(files);
    }
    
    tracing::info!("Restore gestartet: Snapshot {}", snapshot_id);
    
    // Restore ausführen
    repo.restore(&snapshot, dest, &opts).await?;
    
    tracing::info!("Restore abgeschlossen");
    
    Ok(())
}
```

---

## ⚙️ Config-Management

```rust
// src/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub repositories: Vec<RepositoryConfig>,
    pub backup_jobs: Vec<BackupJobConfig>,
    pub settings: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub id: String,
    pub name: String,
    pub path: String,
    pub backend_type: BackendType,
    pub password_stored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobConfig {
    pub id: String,
    pub name: String,
    pub repository_id: String,
    pub source_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub tags: Vec<String>,
    pub schedule: Option<String>,
    pub retention: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_last: Option<usize>,
    pub keep_daily: Option<usize>,
    pub keep_weekly: Option<usize>,
    pub keep_monthly: Option<usize>,
    pub keep_yearly: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub log_level: String,
    pub check_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendType {
    Local,
    Sftp,
    S3,
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
            },
        }
    }
}

impl AppConfig {
    /// Lädt Config von Disk.
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            tracing::info!("Keine Config gefunden, erstelle Default");
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&path)
            .context("Config-Datei lesen fehlgeschlagen")?;
        
        let config: Self = toml::from_str(&content)
            .context("Config parsen fehlgeschlagen")?;
        
        tracing::debug!("Config geladen: {} Repositories, {} Jobs",
            config.repositories.len(),
            config.backup_jobs.len()
        );
        
        Ok(config)
    }
    
    /// Speichert Config auf Disk.
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        
        // Parent-Verzeichnis erstellen
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let toml = toml::to_string_pretty(self)
            .context("Config serialisieren fehlgeschlagen")?;
        
        std::fs::write(&path, toml)
            .context("Config schreiben fehlgeschlagen")?;
        
        tracing::debug!("Config gespeichert nach: {}", path.display());
        
        Ok(())
    }
    
    /// Gibt Config-Pfad zurück.
    ///
    /// - Linux: `~/.config/rustic-gui/config.toml`
    /// - Windows: `%APPDATA%\rustic-gui\config.toml`
    /// - macOS: `~/Library/Application Support/rustic-gui/config.toml`
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Config-Verzeichnis nicht gefunden")?;
        
        Ok(config_dir.join("rustic-gui").join("config.toml"))
    }
}
```

---

## 🔐 Passwort-Management

```rust
// src/keychain/mod.rs

use keyring::Entry;
use anyhow::{Context, Result};

const SERVICE_NAME: &str = "rustic-gui";

/// Speichert Passwort in System-Keychain.
pub fn store_password(repo_id: &str, password: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, repo_id)
        .context("Keychain-Entry erstellen fehlgeschlagen")?;
    
    entry.set_password(password)
        .context("Passwort speichern fehlgeschlagen")?;
    
    tracing::debug!("Passwort gespeichert für: {}", repo_id);
    
    Ok(())
}

/// Lädt Passwort aus System-Keychain.
pub fn load_password(repo_id: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, repo_id)
        .context("Keychain-Entry erstellen fehlgeschlagen")?;
    
    let password = entry.get_password()
        .context("Passwort laden fehlgeschlagen")?;
    
    tracing::debug!("Passwort geladen für: {}", repo_id);
    
    Ok(password)
}

/// Löscht Passwort aus System-Keychain.
pub fn delete_password(repo_id: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, repo_id)
        .context("Keychain-Entry erstellen fehlgeschlagen")?;
    
    entry.delete_password()
        .context("Passwort löschen fehlgeschlagen")?;
    
    tracing::debug!("Passwort gelöscht für: {}", repo_id);
    
    Ok(())
}
```

---

## ⏰ Job-Scheduling

```rust
// src/scheduler/mod.rs

use tokio_cron_scheduler::{JobScheduler, Job};
use std::collections::HashMap;
use anyhow::Result;
use futures::future::BoxFuture;

pub struct BackupScheduler {
    scheduler: JobScheduler,
    jobs: HashMap<String, uuid::Uuid>,
}

impl BackupScheduler {
    pub fn new() -> Result<Self> {
        let scheduler = tokio::runtime::Handle::current()
            .block_on(JobScheduler::new())?;
        
        Ok(Self {
            scheduler,
            jobs: HashMap::new(),
        })
    }
    
    /// Fügt zeitgesteuerten Job hinzu.
    pub async fn add_job<F>(
        &mut self,
        job_id: String,
        cron_expr: &str,
        callback: F,
    ) -> Result<()>
    where
        F: Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        let job = Job::new_async(cron_expr, move |_uuid, _lock| {
            Box::pin(callback())
        })?;
        
        let uuid = self.scheduler.add(job).await?;
        self.jobs.insert(job_id.clone(), uuid);
        
        tracing::info!("Job {} geplant mit: {}", job_id, cron_expr);
        
        Ok(())
    }
    
    /// Entfernt Job.
    pub async fn remove_job(&mut self, job_id: &str) -> Result<()> {
        if let Some(uuid) = self.jobs.remove(job_id) {
            self.scheduler.remove(&uuid).await?;
            tracing::info!("Job {} entfernt", job_id);
        }
        
        Ok(())
    }
    
    /// Startet Scheduler.
    pub async fn start(&self) -> Result<()> {
        self.scheduler.start().await?;
        tracing::info!("Scheduler gestartet");
        Ok(())
    }
}
```

---

## 🐛 Error-Handling

```rust
// src/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Repository nicht gefunden: {0}")]
    RepositoryNotFound(String),
    
    #[error("Authentifizierung fehlgeschlagen")]
    AuthenticationFailed,
    
    #[error("Backup wurde abgebrochen")]
    Cancelled,
    
    #[error("Pfad nicht gefunden: {0}")]
    PathNotFound(String),
    
    #[error("Keine Berechtigung: {0}")]
    PermissionDenied(String),
    
    #[error("IO-Fehler: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("rustic-Fehler: {0}")]
    Rustic(#[from] rustic_core::error::RusticError),
    
    #[error("{0}")]
    Internal(String),
}

impl BackupError {
    /// Konvertiert zu user-friendly Message.
    pub fn to_user_message(&self) -> String {
        match self {
            Self::RepositoryNotFound(path) => {
                format!("Repository nicht gefunden: {}", path)
            }
            Self::AuthenticationFailed => {
                "Authentifizierung fehlgeschlagen. Bitte Passwort prüfen.".to_string()
            }
            Self::Cancelled => {
                "Vorgang wurde abgebrochen.".to_string()
            }
            Self::PathNotFound(path) => {
                format!("Pfad existiert nicht: {}", path)
            }
            Self::PermissionDenied(path) => {
                format!("Keine Berechtigung für: {}", path)
            }
            Self::Io(e) => {
                format!("Dateisystem-Fehler: {}", e)
            }
            Self::Rustic(e) => {
                format!("Backup-Fehler: {}", e)
            }
            Self::Internal(msg) => {
                format!("Interner Fehler: {}", msg)
            }
        }
    }
}

// Verwendung in Commands
#[tauri::command]
pub async fn example() -> Result<Data, String> {
    do_something()
        .await
        .map_err(|e: BackupError| e.to_user_message())
}
```

---

## 📦 Dependencies

```toml
# Cargo.toml

[package]
name = "rustic-gui"
version = "0.1.0"
edition = "2021"

[dependencies]
# Tauri
tauri = { version = "2.0", features = ["protocol-asset"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# rustic
rustic-core = "0.3"
rustic-backend = "0.3"

# Async
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"

# Serialization
toml = "0.8"

# Error-Handling
thiserror = "1"
anyhow = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Scheduling
tokio-cron-scheduler = "0.10"

# Keychain
keyring = "2"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dirs = "5"
parking_lot = "0.12"

[build-dependencies]
tauri-build = { version = "2.0", features = [] }
```

---

## ✅ Backend-Checkliste

### Bei jedem Command
- [ ] Docstring mit Beschreibung
- [ ] Parameter-Validierung
- [ ] Error-Handling (Result<T, String>)
- [ ] Logging (tracing)
- [ ] Keine Passwörter in Logs
- [ ] User-friendly Error-Messages

### Async-Code
- [ ] Tokio für Async
- [ ] Cancellation-Support wo nötig
- [ ] Progress-Events bei langen Ops
- [ ] Keine Blocking-Calls

### State-Management
- [ ] AppState korrekt nutzen
- [ ] Locks minimal halten
- [ ] Keine Deadlocks
- [ ] Thread-Safety gewährleistet

### Integration
- [ ] rustic_core korrekt integriert
- [ ] Passwort-Handling sicher
- [ ] Config-Persistenz
- [ ] Events an Frontend

---

**Version**: 2.0  
**Letzte Aktualisierung**: 2025-10-26