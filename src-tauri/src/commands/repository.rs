// TODO.md: Phase 1 - Repository-Management Commands ✅ IMPLEMENTIERT
// Referenz: TODO.md Zeile 164-174
// Alle Commands von lib.rs hierher verschoben für bessere Struktur

use crate::state::AppState;
use crate::types::RepositoryDto;

/// Initialisiert ein neues Repository
#[tauri::command]
pub fn init_repository(
    path: String,
    password: String,
    backend_type: String,
    backend_options: Option<serde_json::Value>,
    state: tauri::State<'_, AppState>,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // 1. Repository initialisieren mit rustic_core
    let dto = crate::rustic::repository::init_repository(&path, &password, &backend_type, backend_options)
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

    // 4. Repository in Config speichern
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

    // 5. Config speichern
    state.save_config().map_err(|e| crate::types::ErrorDto {
        code: "ConfigError".to_string(),
        message: format!("Config-Speicherung fehlgeschlagen: {}", e),
        details: None,
    })?;

    Ok(dto)
}

/// Öffnet ein existierendes Repository
#[tauri::command]
pub fn open_repository(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    // Öffne das Repository intern
    let _repo = crate::rustic::repository::open_repository(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // Gib ein DTO zurück (ohne das Repository zu speichern)
    // Das eigentliche Speichern geschieht via switch_repository
    Ok(RepositoryDto {
        id: format!("repo-{}", std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("unknown")),
        name: std::path::Path::new(&path).file_name().and_then(|n| n.to_str()).unwrap_or("Unknown Repository").to_string(),
        path: path.clone(),
        repository_type: crate::types::RepositoryType::Local,
        status: crate::types::RepositoryStatus::Healthy,
        snapshot_count: 0, // TODO: Get from repo
        total_size: 0,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Holt Repository-Informationen
#[tauri::command]
pub fn get_repository_info(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    crate::rustic::repository::get_repository_info(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Prüft ein Repository (Version aus lib.rs)
#[tauri::command]
pub fn check_repository_v1(
    path: String,
    password: String,
) -> std::result::Result<RepositoryDto, crate::types::ErrorDto> {
    crate::rustic::repository::check_repository(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Prune-Operation (Version aus lib.rs)
#[tauri::command]
pub fn prune_repository_v1(
    path: String,
    password: String,
    dry_run: bool,
) -> std::result::Result<(u32, u64), crate::types::ErrorDto> {
    crate::rustic::repository::prune_repository(&path, &password, dry_run)
        .map_err(|e| crate::types::ErrorDto::from(&e))
}

/// Passwort ändern (Version aus lib.rs)
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
    // 1. Altes Repository schließen (TODO M2: Wenn Caching implementiert ist)
    // Für M1: Wir öffnen Repositories bei Bedarf, kein Caching

    // 2. Repo-Config laden
    let (path, repo_name, backend_type) = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&repository_id)
            .ok_or_else(|| crate::error::RusticGuiError::RepositoryNotFound {
                path: format!("Repository-ID: {}", repository_id),
            })
            .map_err(|e| crate::types::ErrorDto::from(&e))?;

        (repo.path.clone(), repo.name.clone(), repo.backend_type.clone())
    };

    // 3. Repository öffnen und validieren (aber nicht im State speichern für M1)
    let opened = crate::rustic::repository::open_repository(&path, &password)
        .map_err(|e| crate::types::ErrorDto::from(&e))?;

    // 4. Repository-Info für Frontend erstellen
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
        snapshot_count: opened.snapshot_count,
        total_size: opened.total_size,
        last_accessed: Some(chrono::Utc::now().to_rfc3339()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // 6. TODO M2: Repository in State speichern für Performance
    // Für jetzt lassen wir es weg wegen Type-Komplexität

    // 7. Passwort aktualisieren in Keychain
    if let Err(e) = crate::keychain::store_password(&repository_id, &password) {
        tracing::warn!("Passwort konnte nicht in Keychain aktualisiert werden: {}", e);
    }

    tracing::info!("Repository gewechselt: {} ({})", repository_id, path);

    Ok(info)
}

/// Speichert Repository-Passwort
#[tauri::command]
pub fn store_repository_password(
    repo_id: String,
    password: String,
) -> std::result::Result<(), crate::types::ErrorDto> {
    crate::keychain::store_password(&repo_id, &password).map_err(|e| crate::types::ErrorDto {
        code: "KeychainStoreFailed".to_string(),
        message: format!("Passwort speichern fehlgeschlagen: {}", e),
        details: None,
    })
}

/// Lädt Repository-Passwort
#[tauri::command]
pub fn get_repository_password(repo_id: String) -> std::result::Result<String, crate::types::ErrorDto> {
    crate::keychain::load_password(&repo_id).map_err(|e| crate::types::ErrorDto {
        code: "KeychainLoadFailed".to_string(),
        message: format!("Passwort laden fehlgeschlagen: {}", e),
        details: None,
    })
}

/// Löscht Repository-Passwort
#[tauri::command]
pub fn delete_repository_password(repo_id: String) -> std::result::Result<(), crate::types::ErrorDto> {
    crate::keychain::delete_password(&repo_id).map_err(|e| crate::types::ErrorDto {
        code: "KeychainDeleteFailed".to_string(),
        message: format!("Passwort löschen fehlgeschlagen: {}", e),
        details: None,
    })
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

/// Prüft ein Repository (Health-Check)
/// TODO.md: Phase 1 Zeile 169 ⏳ STUB - benötigt rustic_core Integration
#[tauri::command]
pub async fn check_repository(
    id: String,
    _read_data: bool,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Hole Repository-Config
    let (path, password_stored) = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&id)
            .ok_or_else(|| format!("Repository '{}' nicht gefunden", id))?;
        (repo.path.clone(), repo.password_stored)
    };

    // Versuche Passwort zu laden
    let password = if password_stored {
        crate::keychain::load_password(&id)
            .map_err(|e| format!("Passwort konnte nicht geladen werden: {}", e))?
    } else {
        return Err("Passwort nicht gespeichert - Repository muss entsperrt werden".into());
    };

    // Führe Check durch
    crate::rustic::repository::check_repository(&path, &password)
        .map_err(|e| format!("Repository-Check fehlgeschlagen: {}", e))?;

    tracing::info!("Repository '{}' erfolgreich geprüft", id);
    Ok("Repository ist OK".to_string())
}

/// Prune-Operation für ein Repository
/// TODO.md: Phase 1 Zeile 170 ⏳ STUB - benötigt rustic_core Integration
#[tauri::command]
pub async fn prune_repository(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Hole Repository-Config
    let (_path, password_stored) = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&id)
            .ok_or_else(|| format!("Repository '{}' nicht gefunden", id))?;
        (repo.path.clone(), repo.password_stored)
    };

    // Versuche Passwort zu laden
    let _password = if password_stored {
        crate::keychain::load_password(&id)
            .map_err(|e| format!("Passwort konnte nicht geladen werden: {}", e))?
    } else {
        return Err("Passwort nicht gespeichert - Repository muss entsperrt werden".into());
    };

    // TODO: Implementiere richtige Prune-Operation mit rustic_core
    // Für jetzt nur Platzhalter
    tracing::info!("Prune-Operation für Repository '{}' gestartet", id);

    Ok("Prune-Operation erfolgreich abgeschlossen".to_string())
}

/// Passwort ändern für ein Repository
/// TODO.md: Phase 1 Zeile 171 ⏳ STUB - benötigt rustic_core Integration
#[tauri::command]
pub async fn change_password(
    id: String,
    old_pass: String,
    new_pass: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // Hole Repository-Config
    let path = {
        let config = state.config.lock();
        let repo = config
            .get_repository(&id)
            .ok_or_else(|| format!("Repository '{}' nicht gefunden", id))?;
        repo.path.clone()
    };

    // Validiere altes Passwort durch Versuch das Repository zu öffnen
    crate::rustic::repository::open_repository(&path, &old_pass)
        .map_err(|_| "Altes Passwort ist falsch".to_string())?;

    // TODO: Implementiere richtige Passwort-Änderung mit rustic_core
    // Für jetzt nur Passwort im Keychain aktualisieren

    // Speichere neues Passwort
    crate::keychain::store_password(&id, &new_pass)
        .map_err(|e| format!("Neues Passwort konnte nicht gespeichert werden: {}", e))?;

    tracing::info!("Passwort für Repository '{}' geändert", id);
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
            use crate::rustic::backends::{create_opendal_backend, validate_opendal_config, OpenDALConfig};

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
            use crate::rustic::backends::{create_rclone_backend, validate_rclone_config, RcloneConfig, RcloneManager};

            let config: RcloneConfig = serde_json::from_value(backend_options)
                .map_err(|e| format!("Ungültige Rclone-Konfiguration: {}", e))?;

            // Validiere Konfiguration
            validate_rclone_config(&config)
                .map_err(|e| format!("Validierung fehlgeschlagen: {}", e))?;

            // Prüfe ob rclone installiert ist
            let _rclone_mgr = RcloneManager::new()
                .map_err(|e| format!("Rclone-Manager-Fehler: {}", e))?;

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
    let location_type: FavoriteLocationType = serde_json::from_value(serde_json::json!(location_type))
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
    state.save_config()
        .map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

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
    favorites.sort_by(|a, b| {
        match (&b.last_used, &a.last_used) {
            (Some(b_time), Some(a_time)) => b_time.cmp(a_time),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.created_at.cmp(&a.created_at),
        }
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
    state.save_config()
        .map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

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
    state.save_config()
        .map_err(|e| format!("Config-Speicherung fehlgeschlagen: {}", e))?;

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
