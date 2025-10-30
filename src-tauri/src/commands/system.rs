use crate::state::AppState;

/// Health-Check für ein Repository
#[tauri::command]
pub async fn check_repository_health(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementieren
    Err("check_repository_health: Noch nicht implementiert".into())
}

/// Erzwingt das Entsperren eines Repositories
#[tauri::command]
pub async fn force_unlock_repository(
    repository_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren
    Err("force_unlock_repository: Noch nicht implementiert".into())
}