use crate::types::RepositoryDto;
use crate::state::AppState;

/// Listet alle Repositories auf
#[tauri::command]
pub async fn list_repositories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RepositoryDto>, String> {
    // TODO: Implementieren
    Err("list_repositories: Noch nicht implementiert".into())
}

/// Löscht ein Repository
#[tauri::command]
pub async fn delete_repository(
    id: String,
    delete_data: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren
    Err("delete_repository: Noch nicht implementiert".into())
}

/// Prüft ein Repository (Health-Check)
#[tauri::command]
pub async fn check_repository(
    id: String,
    read_data: bool,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementieren
    Err("check_repository: Noch nicht implementiert".into())
}

/// Prune-Operation für ein Repository
#[tauri::command]
pub async fn prune_repository(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implementieren
    Err("prune_repository: Noch nicht implementiert".into())
}

/// Passwort ändern für ein Repository
#[tauri::command]
pub async fn change_password(
    id: String,
    old_pass: String,
    new_pass: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // TODO: Implementieren
    Err("change_password: Noch nicht implementiert".into())
}