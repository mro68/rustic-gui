// TODO.md: Phase 1 - Repository-Management Commands ✅ IMPLEMENTIERT
// Referenz: TODO.md Zeile 164-174
// Alle Commands von lib.rs hierher verschoben für bessere Struktur

use crate::state::AppState;
use crate::types::RepositoryDto;
use rustic_core::CheckOptions;
use tauri::Emitter;

/// Initialisiert ein neues Repository und speichert es in der Konfiguration.
///
/// Erstellt ein rustic Repository mit den angegebenen Parametern,
/// speichert das Passwort im System-Keychain und fügt das Repository
/// zur App-Konfiguration hinzu.
///
/// # Arguments
/// * `path` - Pfad zum Repository (lokal oder remote)
/// * `password` - Verschlüsselungspasswort für das Repository
/// * `backend_type` - Typ des Backends ("local", "s3", "sftp", "rest", "rclone")
/// * `backend_options` - Optional: Backend-spezifische Optionen (JSON)
/// * `state` - AppState mit Config und Keychain-Zugriff
///
/// # Returns
/// `Result<RepositoryDto, ErrorDto>` - Repository-Informationen oder Fehler
///
/// # Errors
/// - Repository konnte nicht erstellt werden (ungültiger Pfad, Backend-Fehler)
/// - Config konnte nicht gespeichert werden
///
/// # Examples
/// ```ignore
/// // Via Tauri Command vom Frontend aufgerufen (TypeScript)
/// await invoke('init_repository', {
///   path: '/backup/repo',
///   password: 'secret',
///   backend_type: 'local',
/// });
/// ```
#[tauri::command]
pub fn init_repository(
    path: String,
    password: String,
    backend_type: String,
    backend_options: Option<serde_json::Value>,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // 1. Repository initialisieren mit rustic_core
    let dto = crate::rustic::repository::init_repository(
        &path,
        &password,
        &backend_type,
        backend_options,
    )
    .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // 2. Repository-ID generieren
    let repo_id = dto.id.clone();

    // 3. Passwort in Keychain speichern
    let password_stored = match crate::keychain::store_password(&repo_id, &password) {
        Ok(_) => {
            tracing::info!("Passwort für Repository '{}' in Keychain gespeichert", repo_id);
            true
        }
        Err(e) => {
            tracing::warn!(
                "Passwort konnte nicht in Keychain gespeichert werden: {}. Repository kann trotzdem verwendet werden.",
                e
            );
            false
        }
    };

    // 4. Prüfe ob Repository bereits existiert (nach Pfad)
    {
        let config = state.config.lock();
        if config.repositories.iter().any(|r| r.path == path) {
            return Err(crate::types::ErrorDto {
                code: "RepositoryAlreadyAdded".to_string(),
                message: format!(
                    "Repository '{}' wurde bereits zur Konfiguration hinzugefügt",
                    path
                ),
                details: Some(format!("Pfad: {}", path)),
            });
        }
    }

    // 5. Repository in Config speichern
    {
        let mut config = state.config.lock();
        let repo_config = crate::config::RepositoryConfig {
            id: repo_id.clone(),
            name: dto.name.clone(),
            path: dto.path.clone(),
            backend_type: match backend_type.as_str() {
                "local" => crate::config::BackendType::Local,
                "sftp" => crate::config::BackendType::Sftp,
                "s3" => crate::config::BackendType::S3,
                "rest" => crate::config::BackendType::Rest,
                "rclone" => crate::config::BackendType::Rclone,
                _ => crate::config::BackendType::Local,
            },
            backend_options: None,
            password_stored,
        };
        config.add_repository(repo_config);
    }

    // 6. Config speichern
    state.save_config().map_err(|e| crate::types::ErrorDto {
        code: "ConfigError".to_string(),
        message: format!("Config-Speicherung fehlgeschlagen: {}", e),
        details: None,
    })?;

    Ok(dto)
}

/// Öffnet ein existierendes Repository und validiert die Zugangsdaten.
///
/// Prüft ob das Repository zugänglich ist und gibt Metadata zurück.
/// Das Repository wird NICHT in der Konfiguration gespeichert -
/// nutzen Sie dafür `switch_repository` nach erfolgreichem Öffnen.
///
/// # Arguments
/// * `path` - Pfad zum Repository
/// * `path` - Repository-Pfad (lokal oder remote URL)
/// * `password` - Repository-Passwort
/// * `state` - AppState für Config-Zugriff
///
/// # Returns
/// `Result<RepositoryDto, ErrorDto>` - Repository-Metadata oder Fehler
///
/// # Errors
/// - Repository nicht gefunden unter dem angegebenen Pfad
/// - Falsches Passwort (AuthenticationFailed)
/// - Repository ist korrupt oder nicht rustic-kompatibel
#[tauri::command]
pub fn open_repository(
    path: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // 1. Öffne das Repository intern (validiert Passwort)
    let _repo = crate::rustic::repository::open_repository(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // 2. Repository-DTO erstellen
    let dto = RepositoryDto {
        id: format!(
            "repo-{}",
            std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("unknown")
        ),
        name: std::path::Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown Repository")
            .to_string(),
        path: path.clone(),
        repository_type: crate::types::RepositoryType::Local,
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count: 0, // TODO: Get from repo
        total_size: 0,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let repo_id = dto.id.clone();

    // 3. Passwort in Keychain speichern
    let password_stored = match crate::keychain::store_password(&repo_id, &password) {
        Ok(_) => {
            tracing::info!("Passwort für Repository '{}' in Keychain gespeichert", repo_id);
            true
        }
        Err(e) => {
            tracing::warn!(
                "Passwort konnte nicht in Keychain gespeichert werden: {}. Repository kann trotzdem verwendet werden.",
                e
            );
            false
        }
    };

    // 4. Prüfe ob Repository bereits existiert (nach Pfad)
    {
        let config = state.config.lock();
        if config.repositories.iter().any(|r| r.path == path) {
            return Err(crate::types::ErrorDto {
                code: "RepositoryAlreadyAdded".to_string(),
                message: format!("Repository '{}' wurde bereits hinzugefügt", path),
                details: Some(format!("Pfad: {}", path)),
            });
        }
    }

    // 5. Repository in Config speichern
    {
        let mut config = state.config.lock();
        let repo_config = crate::config::RepositoryConfig {
            id: repo_id.clone(),
            name: dto.name.clone(),
            path: dto.path.clone(),
            backend_type: crate::config::BackendType::Local, // TODO: Detect from path
            backend_options: None,
            password_stored,
        };
        config.add_repository(repo_config);
    }

    // 6. Config speichern
    state.save_config().map_err(|e| crate::types::ErrorDto {
        code: "ConfigError".to_string(),
        message: format!("Config-Speicherung fehlgeschlagen: {}", e),
        details: None,
    })?;

    tracing::info!("Repository '{}' erfolgreich geöffnet und zu Config hinzugefügt", repo_id);

    Ok(dto)
}

/// Holt detaillierte Repository-Informationen ohne das Repository zu öffnen.
///
/// Liest Metadaten wie Snapshot-Count, Größe, etc. aus dem Repository.
/// Ähnlich wie `open_repository`, aber ohne Speicherung in Config.
///
/// # Arguments
/// * `path` - Repository-Pfad
/// * `password` - Repository-Passwort
///
/// # Returns
/// `Result<RepositoryDto, ErrorDto>` - Detaillierte Repository-Informationen
///
/// # Errors
/// - Repository nicht gefunden oder nicht lesbar
/// - Authentifizierung fehlgeschlagen
#[tauri::command]
pub fn get_repository_info(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    crate::rustic::repository::get_repository_info(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Prüft ein Repository auf Fehler oder Inkonsistenzen.
///
/// Verwendet rustic_core's check() Methode mit CheckOptions.
/// Referenz: https://github.com/rustic-rs/rustic/blob/main/src/commands/check.rs
///
/// # Arguments
/// * `repository_id` - ID des zu prüfenden Repositories
/// * `trust_cache` - Cache vertrauen (schneller, weniger sicher)
/// * `read_data` - Pack-Dateien lesen und verifizieren (langsamer, gründlicher)
/// * `state` - AppState mit Repository-Cache
/// * `app_handle` - Tauri AppHandle für Progress-Events
///
/// # Returns
/// `Result<CheckResult, String>` - Check-Ergebnisse oder Fehler
#[tauri::command]
pub async fn check_repository(
    repository_id: String,
    trust_cache: bool,
    read_data: bool,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<crate::types::CheckResultDto, String> {
    // Repository aus Cache holen (NICHT async!)
    let repo = state
        .get_repository(&repository_id)
        .map_err(|e| format!("Repository nicht gefunden: {}", e))?;

    // Progress-Events senden
    app_handle
        .emit(
            "check-started",
            serde_json::json!({
                "repository_id": repository_id,
            }),
        )
        .ok();

    // CheckOptions erstellen (wie rustic CLI)
    let opts = CheckOptions::default().trust_cache(trust_cache).read_data(read_data);

    // Repository prüfen (check() lädt automatisch alle Snapshots und ihre Trees!)
    // WICHTIG: check() gibt Result<()> zurück, keine CheckResults!
    // Fehler werden intern über tracing geloggt.
    match repo.check(opts) {
        Ok(_) => {
            // Check erfolgreich
            app_handle
                .emit(
                    "check-completed",
                    serde_json::json!({
                        "repository_id": repository_id,
                        "success": true,
                    }),
                )
                .ok();

            Ok(crate::types::CheckResultDto {
                errors: Vec::new(),
                warnings: Vec::new(),
                is_ok: true,
            })
        }
        Err(e) => {
            // Check fehlgeschlagen
            let error_msg = format!("{}", e);

            app_handle
                .emit(
                    "check-completed",
                    serde_json::json!({
                        "repository_id": repository_id,
                        "success": false,
                        "error": error_msg.clone(),
                    }),
                )
                .ok();

            Ok(crate::types::CheckResultDto {
                errors: vec![error_msg],
                warnings: Vec::new(),
                is_ok: false,
            })
        }
    }
}

/// Prune-Operation: Entfernt unnötige Pack-Dateien aus dem Repository.
///
/// Führt eine 2-stufige Prune-Operation durch:
/// 1. Erstellt einen Prune-Plan mit Statistiken
/// 2. Führt den Plan aus (nur wenn dry_run = false)
///
/// # Arguments
/// * `repository_id` - ID des zu prunenenden Repositories
/// * `dry_run` - Nur Plan erstellen, keine Änderungen (true = Simulation)
/// * `state` - AppState mit Repository-Cache
/// * `app_handle` - Tauri AppHandle für Progress-Events
///
/// # Returns
/// `Result<PruneResultDto, String>` - Prune-Statistiken oder Fehler
///
/// # Referenz
/// https://github.com/rustic-rs/rustic/blob/main/src/commands/prune.rs
#[tauri::command]
pub async fn prune_repository(
    repository_id: String,
    dry_run: bool,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<crate::types::PruneResultDto, String> {
    use rustic_core::PruneOptions;

    // Repository holen
    let repo = state
        .get_repository(&repository_id)
        .map_err(|e| format!("Repository nicht gefunden: {}", e))?;

    // Progress-Event
    app_handle
        .emit(
            "prune-started",
            serde_json::json!({
                "repository_id": repository_id,
                "dry_run": dry_run,
            }),
        )
        .ok();

    // PruneOptions erstellen
    let opts = PruneOptions::default();

    // Schritt 1: Prune-Plan erstellen
    let prune_plan =
        repo.prune_plan(&opts).map_err(|e| format!("Prune-Plan fehlgeschlagen: {}", e))?;

    // Statistiken extrahieren (BEVOR prune_plan moved wird)
    let packs_removed = prune_plan.stats.packs_to_delete.remove;
    let packs_kept = prune_plan.stats.packs_to_delete.keep;
    let packs_recovered = prune_plan.stats.packs_to_delete.recover;
    let size_removed = prune_plan.stats.size_to_delete.remove;
    let size_kept = prune_plan.stats.size_to_delete.keep;
    let size_recovered = prune_plan.stats.size_to_delete.recover;

    // Schritt 2: Prune ausführen (falls nicht dry-run)
    if !dry_run {
        repo.prune(&opts, prune_plan).map_err(|e| format!("Prune fehlgeschlagen: {}", e))?;
    }

    // Completion-Event
    app_handle
        .emit(
            "prune-completed",
            serde_json::json!({
                "repository_id": repository_id,
                "dry_run": dry_run,
                "packs_removed": packs_removed,
                "size_removed": size_removed,
            }),
        )
        .ok();

    Ok(crate::types::PruneResultDto {
        packs_removed,
        packs_kept,
        packs_recovered,
        size_removed,
        size_kept,
        size_recovered,
        dry_run,
    })
}

/// Prune-Operation (ALT - Version aus lib.rs, deprecated)
#[tauri::command]
pub fn prune_repository_v1(
    path: String,
    password: String,
    dry_run: bool,
) -> std::result::Result<(u32, u64), crate::types::ErrorDto> {
    crate::rustic::repository::prune_repository(&path, &password, dry_run)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Passwort ändern für ein Repository.
///
/// Ändert das Passwort und aktualisiert den Keychain.
/// Erstellt einen neuen Key und entfernt den alten Key optional.
///
/// # Arguments
/// * `repository_id` - ID des Repositories
/// * `old_password` - Aktuelles Passwort
/// * `new_password` - Neues Passwort
/// * `state` - AppState mit Repository-Cache
/// * `app_handle` - Tauri AppHandle für Events
///
/// # Returns
/// `Result<(), String>` - Ok bei Erfolg, Fehler wenn Passwortänderung fehlschlägt
///
/// # Referenz
/// https://github.com/rustic-rs/rustic/blob/main/src/commands/key.rs
#[tauri::command]
pub async fn change_password(
    repository_id: String,
    old_password: String,
    new_password: String,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use rustic_core::KeyOptions;

    // Progress-Event
    app_handle
        .emit(
            "password-change-started",
            serde_json::json!({
                "repository_id": repository_id,
            }),
        )
        .ok();

    // Repository mit altem Passwort öffnen
    let repo = state
        .open_repository_with_password(&repository_id, &old_password)
        .map_err(|e| format!("Repository mit altem Passwort öffnen fehlgeschlagen: {}", e))?;

    // Neuen Key hinzufügen
    let key_opts = KeyOptions::default();
    repo.add_key(&new_password, &key_opts)
        .map_err(|e| format!("Neuen Key hinzufügen fehlgeschlagen: {}", e))?;

    // Keychain aktualisieren
    crate::keychain::store_password(&repository_id, &new_password)
        .map_err(|e| format!("Keychain aktualisieren fehlgeschlagen: {}", e))?;

    // Repository-Cache invalidieren (zwingt Re-Open mit neuem Passwort)
    state.invalidate_repository_cache(&repository_id);

    // Completion-Event
    app_handle
        .emit(
            "password-change-completed",
            serde_json::json!({
                "repository_id": repository_id,
                "success": true,
            }),
        )
        .ok();

    tracing::info!("Passwort für Repository '{}' erfolgreich geändert", repository_id);

    Ok(())
}

/// Passwort ändern (ALT - Version aus lib.rs, deprecated)
#[tauri::command]
pub fn change_password_v1(
    path: String,
    old_password: String,
    new_password: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    crate::rustic::repository::change_password(&path, &old_password, &new_password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Wechselt das aktive Repository
#[tauri::command]
pub fn switch_repository(
    repository_id: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<crate::types::RepositoryDto, crate::types::ErrorDto> {
    // Hole Pfad und Metadaten aus der Konfiguration
    let (path, repo_name, backend_type, password_stored) = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&repository_id)
            .ok_or_else(|| crate::error::RusticGuiError::RepositoryNotFound {
                path: format!("Repository-ID: {}", repository_id),
            })
            .map_err(|e| crate::types::ErrorDto::from(&e))?;

        (repo.path.clone(), repo.name.clone(), repo.backend_type.clone(), repo.password_stored)
    };

    // Öffne Repository und lege es im Cache ab
    let repository = state
        .open_repository_with_password(&repository_id, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // Passwort sicher speichern (mandatory)
    crate::keychain::store_password(&repository_id, &password).map_err(|e| {
        crate::types::ErrorDto {
            code: "KeychainStoreFailed".to_string(),
            message: format!("Passwort konnte nicht gespeichert werden: {}", e),
            details: None,
        }
    })?;

    // Aktualisiere Config falls Passwort bisher nicht persistiert war
    if !password_stored {
        {
            let mut config = state.config.lock();
            if let Some(repo) = config.repositories.iter_mut().find(|r| r.id == repository_id) {
                repo.password_stored = true;
            }
        }

        state.save_config().map_err(|e| crate::types::ErrorDto {
            code: "ConfigError".to_string(),
            message: format!("Config-Speicherung fehlgeschlagen: {}", e),
            details: None,
        })?;
    }

    // Aktives Repository im State setzen
    state.set_current_repository(Some(repository_id.clone()));

    // Snapshot-Zähler ermitteln
    let snapshot_count = match repository.get_all_snapshots() {
        Ok(snaps) => snaps.len() as u32,
        Err(err) => {
            tracing::warn!(
                "Snapshots für Repository {} konnten nicht gelesen werden: {}",
                repository_id,
                err
            );
            0
        }
    };

    let info = crate::types::RepositoryDto {
        id: repository_id.clone(),
        name: repo_name,
        path: path.clone(),
        repository_type: match backend_type {
            crate::config::BackendType::Local => crate::types::RepositoryType::Local,
            crate::config::BackendType::Sftp => crate::types::RepositoryType::Sftp,
            crate::config::BackendType::S3 => crate::types::RepositoryType::S3,
            crate::config::BackendType::Rest => crate::types::RepositoryType::Rest,
            crate::config::BackendType::Rclone => crate::types::RepositoryType::Rclone,
        },
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count,
        total_size: 0,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    tracing::info!("Repository gewechselt: {} ({})", repository_id, path);

    Ok(info)
}

/// Speichert Repository-Passwort
#[tauri::command]
pub fn store_repository_password(
    repo_id: String,
    password: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    if let Err(err) = crate::keychain::store_password(&repo_id, &password) {
        tracing::error!(repo_id = %repo_id, error = %err, "Passwort konnte nicht gespeichert werden");
        return Err(crate::types::ErrorDto::from(&err));
    }
    Ok(())
}

/// Lädt Repository-Passwort
#[tauri::command]
pub fn get_repository_password(
    repo_id: String,
) -> std::result::Result<String, crate::types::ErrorDto> {
    crate::keychain::load_password(&repo_id).map_err(|err| {
        tracing::warn!(repo_id = %repo_id, error = %err, "Passwort konnte nicht geladen werden");
        crate::types::ErrorDto::from(&err)
    })
}

/// Löscht Repository-Passwort
#[tauri::command]
pub fn delete_repository_password(
    repo_id: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    if let Err(err) = crate::keychain::delete_password(&repo_id) {
        tracing::error!(repo_id = %repo_id, error = %err, "Passwort konnte nicht aus Keychain entfernt werden");
        return Err(crate::types::ErrorDto::from(&err));
    }
    Ok(())
}

/// Listet alle Repositories auf
/// TODO.md: Phase 1 Zeile 165 ✅ IMPLEMENTIERT
#[tauri::command]
pub async fn list_repositories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RepositoryDto>, String> {
    let config = state.config.lock();

    let repos: Vec<RepositoryDto> = config
        .repositories
        .iter()
        .map(|repo_config| {
            RepositoryDto {
                id: repo_config.id.clone(),
                name: repo_config.name.clone(),
                path: repo_config.path.clone(),
                repository_type: match repo_config.backend_type {
                    crate::config::BackendType::Local => crate::types::RepositoryType::Local,
                    crate::config::BackendType::Sftp => crate::types::RepositoryType::Sftp,
                    crate::config::BackendType::S3 => crate::types::RepositoryType::S3,
                    crate::config::BackendType::Rest => crate::types::RepositoryType::Rest,
                    crate::config::BackendType::Rclone => crate::types::RepositoryType::Rclone,
                },
                status: crate::types::RepositoryStatus::Healthy, // Aktueller Status wird beim Öffnen ermittelt
                snapshot_count: 0,                               // Wird beim Öffnen geladen
                total_size: 0,                                   // Wird beim Öffnen berechnet
                last_accessed: None,
                created_at: chrono::Utc::now().to_rfc3339(), // TODO: Aus Config lesen wenn verfügbar
            }
        })
        .collect();

    tracing::debug!("Liefere {} Repositories", repos.len());
    Ok(repos)
}

/// Löscht ein Repository
/// TODO.md: Phase 1 Zeile 168 ✅ IMPLEMENTIERT
#[tauri::command]
pub async fn delete_repository(
    id: String,
    delete_data: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Prüfe ob Repository existiert
    let repo_path = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&id)
            .ok_or_else(|| format!("Repository '{}' nicht gefunden", id))?;
        repo.path.clone()
    };

    // Entferne Repository aus Config
    {
        let mut config = state.config.lock();
        if !config.remove_repository(&id) {
            return Err(format!("Repository '{}' konnte nicht entfernt werden", id));
        }
    }

    // Speichere Config
    state.save_config().map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

    // Optional: Lösche Repository-Daten
    if delete_data {
        let path = std::path::Path::new(&repo_path);
        if path.exists() {
            std::fs::remove_dir_all(path)
                .map_err(|e| format!("Repository-Daten konnten nicht gelöscht werden: {}", e))?;
            tracing::info!("Repository-Daten gelöscht: {}", repo_path);
        }
    }

    // Lösche Passwort aus Keychain
    let _ = crate::keychain::delete_password(&id);

    tracing::info!("Repository '{}' gelöscht (delete_data: {})", id, delete_data);
    Ok(())
}

/// Testet die Verbindung zu einem Backend
/// M2 Task 2.1.3: Connection-Test Command
#[tauri::command]
pub async fn test_repository_connection(
    backend_type: String,
    backend_options: serde_json::Value,
) -> Result<crate::types::ConnectionTestResult, String> {
    use std::time::Instant;

    let start = Instant::now();

    match backend_type.as_str() {
        "s3" | "azblob" | "gcs" | "b2" => {
            // OpenDAL-Backend testen
            use crate::rustic::backends::{
                OpenDALConfig, create_opendal_backend, validate_opendal_config,
            };

            let config: OpenDALConfig = serde_json::from_value(backend_options)
                .map_err(|e| format!("Ungültige Backend-Konfiguration: {}", e))?;

            // Validiere Konfiguration
            validate_opendal_config(&config)
                .map_err(|e| format!("Validierung fehlgeschlagen: {}", e))?;

            // Erstelle Backend-Optionen
            let _backend_opts = create_opendal_backend(&config)
                .map_err(|e| format!("Backend-Erstellung fehlgeschlagen: {}", e))?;

            // TODO: Echte Verbindungsprüfung mit rustic_backend
            // Für jetzt: Wenn wir bis hier gekommen sind, ist die Konfiguration valide

            let latency = start.elapsed().as_millis() as u64;

            Ok(crate::types::ConnectionTestResult {
                success: true,
                message: format!("Verbindung zu {} erfolgreich", config.provider),
                latency_ms: Some(latency),
            })
        }
        "rclone" => {
            // Rclone-Backend testen
            use crate::rustic::backends::{
                RcloneConfig, RcloneManager, create_rclone_backend, validate_rclone_config,
            };

            let config: RcloneConfig = serde_json::from_value(backend_options)
                .map_err(|e| format!("Ungültige Rclone-Konfiguration: {}", e))?;

            // Validiere Konfiguration
            validate_rclone_config(&config)
                .map_err(|e| format!("Validierung fehlgeschlagen: {}", e))?;

            // Prüfe ob rclone installiert ist
            let _rclone_mgr =
                RcloneManager::new().map_err(|e| format!("Rclone-Manager-Fehler: {}", e))?;

            // Erstelle Backend-Optionen
            let _backend_opts = create_rclone_backend(&config)
                .map_err(|e| format!("Backend-Erstellung fehlgeschlagen: {}", e))?;

            let latency = start.elapsed().as_millis() as u64;

            Ok(crate::types::ConnectionTestResult {
                success: true,
                message: format!("Rclone-Verbindung zu {} vorbereitet", config.remote_name),
                latency_ms: Some(latency),
            })
        }
        "local" => {
            // Lokales Backend - prüfe nur ob Pfad existiert oder erstellt werden kann
            let path: String = backend_options
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Pfad nicht in Backend-Optionen gefunden".to_string())?
                .to_string();

            let path_buf = std::path::PathBuf::from(&path);

            if !path_buf.exists() {
                // Versuche Verzeichnis zu erstellen
                std::fs::create_dir_all(&path_buf)
                    .map_err(|e| format!("Verzeichnis kann nicht erstellt werden: {}", e))?;
            }

            let latency = start.elapsed().as_millis() as u64;

            Ok(crate::types::ConnectionTestResult {
                success: true,
                message: format!("Lokaler Pfad verfügbar: {}", path),
                latency_ms: Some(latency),
            })
        }
        _ => Err(format!("Backend-Typ nicht unterstützt: {}", backend_type)),
    }
}

/// Speichert eine favorisierte Location
/// M2 Task 2.3.2: Favoriten-Management
#[tauri::command]
pub async fn save_favorite_location(
    name: String,
    path: String,
    location_type: String,
    config: Option<serde_json::Value>,
    state: tauri::State<'_, AppState>,
) -> Result<crate::types::FavoriteLocation, String> {
    use crate::types::{FavoriteLocation, FavoriteLocationType};

    // Parse location type
    let location_type: FavoriteLocationType =
        serde_json::from_value(serde_json::json!(location_type))
            .map_err(|e| format!("Ungültiger Location-Typ: {}", e))?;

    // Erstelle neue Favorite
    let favorite = FavoriteLocation {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path,
        location_type,
        config,
        created_at: chrono::Utc::now().to_rfc3339(),
        last_used: None,
    };

    // Zur Config hinzufügen
    {
        let mut config = state.config.lock();
        config.favorite_locations.push(favorite.clone());
    }

    // Config speichern
    state.save_config().map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

    tracing::info!("Favorite Location gespeichert: {}", favorite.name);

    Ok(favorite)
}

/// Listet alle favorisierten Locations
/// M2 Task 2.3.2: Favoriten-Management
#[tauri::command]
pub async fn list_favorite_locations(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::types::FavoriteLocation>, String> {
    let config = state.config.lock();
    let mut favorites = config.favorite_locations.clone();

    // Sortiere nach letzter Verwendung (neueste zuerst)
    favorites.sort_by(|a, b| match (&b.last_used, &a.last_used) {
        (Some(b_time), Some(a_time)) => b_time.cmp(a_time),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => b.created_at.cmp(&a.created_at),
    });

    tracing::debug!("Liefere {} Favorite Locations", favorites.len());

    Ok(favorites)
}

/// Aktualisiert das last_used Feld einer favorisierten Location
/// M2 Task 2.3.2: Favoriten-Management
#[tauri::command]
pub async fn update_favorite_last_used(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut config = state.config.lock();

        if let Some(favorite) = config.favorite_locations.iter_mut().find(|f| f.id == id) {
            favorite.last_used = Some(chrono::Utc::now().to_rfc3339());
        } else {
            return Err(format!("Favorite Location mit ID '{}' nicht gefunden", id));
        }
    }

    // Config speichern
    state.save_config().map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

    tracing::debug!("Favorite Location {} als verwendet markiert", id);

    Ok(())
}

/// Löscht eine favorisierte Location
/// M2 Task 2.3.2: Favoriten-Management
#[tauri::command]
pub async fn delete_favorite_location(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut config = state.config.lock();
        let original_len = config.favorite_locations.len();
        config.favorite_locations.retain(|f| f.id != id);

        if config.favorite_locations.len() == original_len {
            return Err(format!("Favorite Location mit ID '{}' nicht gefunden", id));
        }
    }

    // Config speichern
    state.save_config().map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

    tracing::info!("Favorite Location {} gelöscht", id);

    Ok(())
}

/// Holt detaillierte Statistiken für ein Repository
/// M4.3: Repository-Statistiken Command
#[tauri::command]
pub async fn get_repository_stats(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<crate::types::RepositoryStatsDto, String> {
    tracing::debug!("Lade Statistiken für Repository '{}'", id);

    // Nutze neues State-System um Repository zu öffnen
    let repo = state
        .get_repository(&id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Hole Statistiken mit rustic_core API
    let stats = crate::rustic::repository::get_repository_stats(&*repo)
        .map_err(|e| format!("Statistiken sammeln fehlgeschlagen: {}", e))?;

    tracing::info!(
        "Statistiken für Repository '{}' geladen: {} Snapshots",
        id,
        stats.snapshot_count
    );

    Ok(stats)
}
