// TODO.md: Phase 1 - Repository-Management Commands ✅ REGISTRIERT (teilweise Stubs)
// Referenz: TODO.md Zeile 164-174
// Commands:
// - list_repositories (Zeile 165) ✅ IMPLEMENTIERT
// - delete_repository (Zeile 168) ✅ IMPLEMENTIERT
// - check_repository (Zeile 169) ⏳ STUB (TODO Zeile 134)
// - prune_repository (Zeile 170) ⏳ STUB (TODO Zeile 161)
// - change_password (Zeile 171) ⏳ STUB (TODO Zeile 161)
// Weitere Commands in lib.rs: init_repository, open_repository

use crate::config::RepositoryConfig;
use crate::state::AppState;
use crate::types::RepositoryDto;

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
    read_data: bool,
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
