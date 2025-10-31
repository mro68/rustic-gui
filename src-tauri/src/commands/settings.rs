use crate::config::AppSettings;
use crate::state::AppState;

/// Lädt die aktuellen App-Einstellungen
/// M4.4: Settings-Backend-Integration
#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    let config = state.config.lock();
    Ok(config.settings.clone())
}

/// Speichert App-Einstellungen
/// M4.4: Settings-Backend-Integration
#[tauri::command]
pub async fn save_settings(
    settings: AppSettings,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    tracing::debug!(
        "Speichere Einstellungen: Theme={}, LogLevel={}",
        settings.theme,
        settings.log_level
    );

    // Update Config
    {
        let mut config = state.config.lock();
        config.settings = settings.clone();
    }

    // Persistiere Config
    state
        .save_config()
        .map_err(|e| format!("Fehler beim Speichern der Config: {}", e))?;

    tracing::info!("Einstellungen erfolgreich gespeichert");

    Ok(())
}

/// Setzt Einstellungen auf Standardwerte zurück
/// M4.4: Settings-Backend-Integration
#[tauri::command]
pub async fn reset_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    tracing::info!("Setze Einstellungen auf Standard zurück");

    let default_settings = AppSettings {
        theme: "system".to_string(),
        log_level: "info".to_string(),
        check_updates: true,
        max_concurrent_backups: 1,
        notifications_enabled: true,
        language: "de".to_string(),
        password_storage: "system_keychain".to_string(),
        lock_timeout: 15,
    };

    // Update Config
    {
        let mut config = state.config.lock();
        config.settings = default_settings.clone();
    }

    // Persistiere Config
    state
        .save_config()
        .map_err(|e| format!("Fehler beim Speichern der Config: {}", e))?;

    Ok(default_settings)
}

/// Aktualisiert nur das Theme
/// M4.4: Settings-Backend-Integration
#[tauri::command]
pub async fn update_theme(
    theme: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    tracing::debug!("Aktualisiere Theme: {}", theme);

    {
        let mut config = state.config.lock();
        config.settings.theme = theme;
    }

    state
        .save_config()
        .map_err(|e| format!("Fehler beim Speichern: {}", e))?;

    Ok(())
}
