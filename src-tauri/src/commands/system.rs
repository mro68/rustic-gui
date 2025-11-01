// TODO.md: Phase 1 - System Commands ✅ TEILWEISE IMPLEMENTIERT
// Status: prepare_shutdown verschoben von lib.rs
// Referenz: TODO.md Integration-Zusammenfassung Zeile 338

use crate::state::AppState;
use crate::storage::PortableStoreStatus;

/// Bereitet sauberen Shutdown vor (prüft laufende Operationen)
#[tauri::command]
pub fn prepare_shutdown(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<bool, crate::types::ErrorDto> {
    // Prüfe ob laufende Backups existieren
    let running_backups = state.cancellation_tokens.lock().len();

    if running_backups > 0 {
        tracing::warn!("Shutdown verhindert: {} laufende Backups", running_backups);
        return Err(crate::types::ErrorDto {
            code: "ShutdownBlocked".to_string(),
            message: format!("Shutdown verhindert: {} laufende Backups", running_backups),
            details: None,
        });
    }

    // TODO: Weitere Cleanup-Logik (Scheduler stoppen, etc.)

    tracing::info!("Shutdown vorbereitet - keine laufenden Operationen");
    Ok(true) // Shutdown erlaubt
}

/// Liefert Statusinformationen zum portablen Konfigurationsspeicher.
#[tauri::command]
pub fn get_portable_status(state: tauri::State<'_, AppState>) -> PortableStoreStatus {
    state.portable_status()
}

/// Health-Check für ein Repository
#[tauri::command]
pub async fn check_repository_health(
    _repository_id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: lib.rs:424 (auskommentiert, noch nicht registriert)
    // Wird benötigt für erweiterte Repository-Diagnostik
    Err("check_repository_health: Noch nicht implementiert".into())
}

/// Erzwingt das Entsperren eines Repositories
#[tauri::command]
pub async fn force_unlock_repository(
    _repository_id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren mit rustic_core
    // TODO.md: lib.rs:425 (auskommentiert, noch nicht registriert)
    // Wird benötigt wenn Repository-Locks manuell entfernt werden müssen
    Err("force_unlock_repository: Noch nicht implementiert".into())
}
