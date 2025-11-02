/// Retention-Policy Commands
/// Task 4.3: Retention-Policy Apply
///
/// Referenz: rustic CLI forget.rs (Lines 107-177)
use crate::state::AppState;
use crate::types::RetentionPolicy;
use rustic_core::{KeepOptions, repofile::SnapshotId};

/// Wendet eine Retention-Policy auf alle Snapshots an und gibt zurück welche gelöscht würden.
///
/// **Dry-Run Modus** - Diese Funktion löscht KEINE Snapshots, sondern simuliert nur die Policy!
///
/// Task 4.3: Retention-Policy Apply
/// Referenz: rustic CLI forget.rs Lines 107-177
///
/// # Arguments
/// * `policy` - Retention-Policy mit keep_last, keep_daily, etc.
/// * `state` - App-State
///
/// # Returns
/// * `snapshots_to_keep` - IDs der Snapshots die behalten werden
/// * `snapshots_to_delete` - IDs der Snapshots die gelöscht würden
#[tauri::command]
pub async fn preview_retention_policy(
    policy: RetentionPolicy,
    state: tauri::State<'_, AppState>,
) -> Result<RetentionPolicyPreview, String> {
    tracing::info!("Preview Retention-Policy: {:?}", policy);

    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Alle Snapshots laden
    let all_snapshots =
        repo.get_all_snapshots().map_err(|e| format!("Snapshots laden fehlgeschlagen: {}", e))?;

    tracing::debug!("Geladene Snapshots: {}", all_snapshots.len());

    // Konvertiere RetentionPolicy -> rustic_core::KeepOptions
    let keep_opts = rustic_retention_policy_to_keep_options(&policy);

    // Wende Retention-Policy an (mit aktuellem Zeitpunkt)
    let now = chrono::Local::now();
    let forget_result = keep_opts
        .apply(all_snapshots.clone(), now)
        .map_err(|e| format!("Retention-Policy fehlgeschlagen: {}", e))?;

    // Ermittle zu behaltende und zu löschende Snapshots
    let mut snapshots_to_keep: Vec<String> = Vec::new();
    let mut snapshots_to_delete: Vec<String> = Vec::new();

    for snap_result in forget_result {
        let snap_id = snap_result.snapshot.id.to_string();
        if snap_result.keep {
            snapshots_to_keep.push(snap_id);
        } else {
            snapshots_to_delete.push(snap_id);
        }
    }

    tracing::info!(
        "Retention-Policy Preview: {} behalten, {} löschen",
        snapshots_to_keep.len(),
        snapshots_to_delete.len()
    );

    Ok(RetentionPolicyPreview { snapshots_to_keep, snapshots_to_delete })
}

/// Wendet eine Retention-Policy an und löscht Snapshots gemäß Policy.
///
/// **⚠️ ACHTUNG**: Diese Funktion löscht WIRKLICH Snapshots!
///
/// Task 4.3: Retention-Policy Apply
/// Referenz: rustic CLI forget.rs Lines 158-177
///
/// # Arguments
/// * `policy` - Retention-Policy
/// * `state` - App-State
///
/// # Returns
/// Liste der gelöschten Snapshot-IDs
#[tauri::command]
pub async fn apply_retention_policy(
    policy: RetentionPolicy,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    tracing::warn!("Wende Retention-Policy an (löscht Snapshots!): {:?}", policy);

    let repo_id = state.get_current_repository_id().ok_or("Kein Repository ausgewählt")?;

    let repo = state
        .get_repository(&repo_id)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Alle Snapshots laden
    let all_snapshots =
        repo.get_all_snapshots().map_err(|e| format!("Snapshots laden fehlgeschlagen: {}", e))?;

    // Konvertiere RetentionPolicy -> rustic_core::KeepOptions
    let keep_opts = rustic_retention_policy_to_keep_options(&policy);

    // Wende Retention-Policy an
    let now = chrono::Local::now();
    let forget_result = keep_opts
        .apply(all_snapshots, now)
        .map_err(|e| format!("Retention-Policy fehlgeschlagen: {}", e))?;

    // Sammle zu löschende Snapshot-IDs
    let mut snapshots_to_delete: Vec<SnapshotId> = Vec::new();
    let mut deleted_ids: Vec<String> = Vec::new();

    for snap_result in forget_result {
        if !snap_result.keep {
            deleted_ids.push(snap_result.snapshot.id.to_string());
            snapshots_to_delete.push(snap_result.snapshot.id);
        }
    }

    if snapshots_to_delete.is_empty() {
        tracing::info!("Keine Snapshots zum Löschen gemäß Retention-Policy");
        return Ok(vec![]);
    }

    tracing::warn!("Lösche {} Snapshots gemäß Retention-Policy", snapshots_to_delete.len());

    // Lösche Snapshots (Referenz: forget.rs Line 169)
    repo.delete_snapshots(&snapshots_to_delete)
        .map_err(|e| format!("Snapshots löschen fehlgeschlagen: {}", e))?;

    tracing::info!("{} Snapshots erfolgreich gelöscht", deleted_ids.len());

    Ok(deleted_ids)
}

/// Hilfsfunktion: Konvertiert unsere RetentionPolicy in rustic_core::KeepOptions
///
/// Referenz: rustic CLI forget.rs Lines 62-88
fn rustic_retention_policy_to_keep_options(policy: &RetentionPolicy) -> KeepOptions {
    let mut opts = KeepOptions::default();

    // Mappe unsere Policy-Felder auf rustic_core KeepOptions (i32)
    opts.keep_last = policy.keep_last.map(|n| n as i32);
    opts.keep_daily = policy.keep_daily.map(|n| n as i32);
    opts.keep_weekly = policy.keep_weekly.map(|n| n as i32);
    opts.keep_monthly = policy.keep_monthly.map(|n| n as i32);
    opts.keep_yearly = policy.keep_yearly.map(|n| n as i32);

    opts
}

/// DTO für Retention-Policy Preview
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetentionPolicyPreview {
    /// Snapshot-IDs die behalten werden
    pub snapshots_to_keep: Vec<String>,
    /// Snapshot-IDs die gelöscht würden
    pub snapshots_to_delete: Vec<String>,
}
