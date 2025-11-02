# Backup, Restore & Snapshot Instructions

> Feature-spezifische Implementierungs-Guidelines für Rustic GUI

---

## 🎯 WICHTIG: Referenz-Implementierung

**Bevor du irgendeine rustic_core API implementierst:**

1. ✅ **Schaue IMMER zuerst in das [rustic CLI Repository](https://github.com/rustic-rs/rustic/)**
2. ✅ **Nutze die Commands in `src/commands/` als Referenz:**
   - **Check**: [`check.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/check.rs)
   - Restore: [`restore.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/restore.rs)
   - Backup: [`backup.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/backup.rs)
   - Diff/Compare: [`diff.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/diff.rs)
   - Forget/Delete: [`forget.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/forget.rs)
   - Tag-Verwaltung: [`tag.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/tag.rs)
   - Snapshots: [`snapshots.rs`](https://github.com/rustic-rs/rustic/blob/main/src/commands/snapshots.rs)
3. ✅ **Übernimme die Patterns 1:1** - Die rustic CLI nutzt die API korrekt
4. ✅ **Bei Problemen**: Vergleiche deinen Code mit der entsprechenden Command-Datei

**Warum?** Die rustic_core Dokumentation ist unvollständig. Das CLI zeigt die **korrekte** API-Nutzung.

---

## 🛠️ Repository-Wartung

### Check Repository

**WICHTIG:** `Repository::check()` gibt `Result<()>` zurück, **keine** `CheckResults`!

Fehler werden intern über `tracing` geloggt. Für UI-Feedback nutze Success/Failure:

```rust
#[tauri::command]
pub async fn check_repository(
    repository_id: String,
    trust_cache: bool,
    read_data: bool,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<CheckResultDto, String> {
    use rustic_core::CheckOptions;

    // Repository holen (NICHT async!)
    let repo = state.get_repository(&repository_id)?;

    // Progress-Event
    app_handle.emit("check-started", json!({
        "repository_id": repository_id,
    })).ok();

    // CheckOptions erstellen
    let opts = CheckOptions::default()
        .trust_cache(trust_cache)  // Cache vertrauen (schneller)
        .read_data(read_data);     // Pack-Files lesen (gründlicher)

    // Check ausführen (lädt automatisch alle Snapshots!)
    match repo.check(opts) {
        Ok(_) => {
            app_handle.emit("check-completed", json!({
                "repository_id": repository_id,
                "success": true,
            })).ok();

            Ok(CheckResultDto {
                errors: Vec::new(),
                warnings: Vec::new(),
                is_ok: true,
            })
        }
        Err(e) => {
            let error_msg = format!("{}", e);

            app_handle.emit("check-completed", json!({
                "repository_id": repository_id,
                "success": false,
                "error": error_msg.clone(),
            })).ok();

            Ok(CheckResultDto {
                errors: vec![error_msg],
                warnings: Vec::new(),
                is_ok: false,
            })
        }
    }
}
```

**Referenz:** [rustic CLI check.rs](https://github.com/rustic-rs/rustic/blob/main/src/commands/check.rs)

---

### Prune Repository

**2-stufiger Prozess:** `prune_plan()` → `prune()`

```rust
#[tauri::command]
pub async fn prune_repository(
    repository_id: String,
    dry_run: bool,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<PruneResultDto, String> {
    use rustic_core::PruneOptions;

    let repo = state.get_repository(&repository_id)?;

    // Progress-Event
    app_handle.emit("prune-started", json!({
        "repository_id": repository_id,
        "dry_run": dry_run,
    })).ok();

    // PruneOptions erstellen
    let opts = PruneOptions::default();

    // Schritt 1: Prune-Plan erstellen
    let prune_plan = repo.prune_plan(&opts)
        .map_err(|e| format!("Prune-Plan fehlgeschlagen: {}", e))?;

    // Statistiken extrahieren (BEVOR prune_plan moved wird)
    let packs_removed = prune_plan.stats.packs_to_delete.remove;
    let packs_kept = prune_plan.stats.packs_to_delete.keep;
    let packs_recovered = prune_plan.stats.packs_to_delete.recover;
    let size_removed = prune_plan.stats.size_to_delete.remove;
    let size_kept = prune_plan.stats.size_to_delete.keep;
    let size_recovered = prune_plan.stats.size_to_delete.recover;

    // Schritt 2: Prune ausführen (falls nicht dry-run)
    if !dry_run {
        repo.prune(&opts, prune_plan)
            .map_err(|e| format!("Prune fehlgeschlagen: {}", e))?;
    }

    // Completion-Event
    app_handle.emit("prune-completed", json!({
        "repository_id": repository_id,
        "packs_removed": packs_removed,
        "size_removed": size_removed,
    })).ok();

    Ok(PruneResultDto {
        packs_removed,
        packs_kept,
        packs_recovered,
        size_removed,
        size_kept,
        size_recovered,
        dry_run,
    })
}
```

**Referenz:** [rustic CLI prune.rs](https://github.com/rustic-rs/rustic/blob/main/src/commands/prune.rs)

---

### Change Password

**Erstellt neuen Key, aktualisiert Keychain:**

```rust
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
    app_handle.emit("password-change-started", json!({
        "repository_id": repository_id,
    })).ok();

    // Repository mit altem Passwort öffnen
    let repo = state
        .open_repository_with_password(&repository_id, &old_password)
        .map_err(|e| format!("Repository öffnen fehlgeschlagen: {}", e))?;

    // Neuen Key hinzufügen
    let key_opts = KeyOptions::default();
    repo.add_key(&new_password, &key_opts)
        .map_err(|e| format!("Neuen Key hinzufügen fehlgeschlagen: {}", e))?;

    // Keychain aktualisieren
    crate::keychain::store_password(&repository_id, &new_password)
        .map_err(|e| format!("Keychain aktualisieren fehlgeschlagen: {}", e))?;

    // Repository-Cache invalidieren (erzwingt Re-Open)
    state.invalidate_repository_cache(&repository_id);

    // Completion-Event
    app_handle.emit("password-change-completed", json!({
        "repository_id": repository_id,
        "success": true,
    })).ok();

    Ok(())
}
```

**Referenz:** [rustic CLI key.rs](https://github.com/rustic-rs/rustic/blob/main/src/commands/key.rs)

---

## 🔄 Backup-Implementation

### Backup-Job-Struktur

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobConfig {
    pub id: String,
    pub name: String,
    pub repository_id: String,
    pub source_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub tags: Vec<String>,
    pub schedule: Option<String>, // Cron expression
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
```

### Backup ausführen mit Progress

```rust
#[tauri::command]
pub async fn run_backup(
    job_id: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<BackupResult, String> {
    // 1. Job-Config laden
    let config = state.config.lock().unwrap()
        .backup_jobs
        .iter()
        .find(|j| j.id == job_id)
        .ok_or("Job nicht gefunden")?
        .clone();

    // 2. Repository öffnen
    let repo = state.get_current_repo()?;

    // 3. Cancellation-Token erstellen
    let token = CancellationToken::new();
    state.cancellation_tokens.lock().unwrap()
        .insert(job_id.clone(), token.clone());

    // 4. Backup-Task mit Cancellation
    let job_id_clone = job_id.clone();
    let result = tokio::select! {
        res = execute_backup_with_progress(
            &repo,
            &config,
            &app_handle,
            &job_id
        ) => res,

        _ = token.cancelled() => {
            Err("Backup abgebrochen".into())
        }
    };

    // 5. Cleanup
    state.cancellation_tokens.lock().unwrap()
        .remove(&job_id_clone);

    result
}

async fn execute_backup_with_progress(
    repo: &Repository,
    config: &BackupJobConfig,
    app_handle: &tauri::AppHandle,
    job_id: &str,
) -> Result<BackupResult, String> {
    let app_handle = app_handle.clone();
    let job_id = job_id.to_string();

    // Progress-Callback
    let progress_callback = move |info: BackupProgress| {
        app_handle.emit_all(&format!("backup-progress-{}", job_id), json!({
            "files_processed": info.files_done,
            "files_total": info.files_total,
            "bytes_processed": info.bytes_done,
            "bytes_total": info.bytes_total,
            "current_file": info.current_file,
        })).ok();
    };

    // Backup-Optionen
    let opts = BackupOptions::default()
        .with_tags(config.tags.clone())
        .with_excludes(config.exclude_patterns.clone());

    // Backup ausführen
    let snapshot = repo.backup(
        &config.source_paths,
        &opts,
        Some(Box::new(progress_callback))
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

### Backup-Validierung

```rust
pub fn validate_backup_config(config: &BackupJobConfig) -> Result<()> {
    // Name prüfen
    if config.name.trim().is_empty() {
        anyhow::bail!("Job-Name darf nicht leer sein");
    }

    // Source-Pfade prüfen
    if config.source_paths.is_empty() {
        anyhow::bail!("Mindestens ein Quellpfad erforderlich");
    }

    for path in &config.source_paths {
        if !path.exists() {
            anyhow::bail!("Pfad existiert nicht: {}", path.display());
        }
    }

    // Cron-Expression validieren (falls vorhanden)
    if let Some(schedule) = &config.schedule {
        validate_cron_expression(schedule)?;
    }

    Ok(())
}
```

---

## 🔄 Restore-Implementation

### Restore mit Progress (2-step API)

**WICHTIG:** `prepare_restore()` → `restore()` wie in rustic CLI:

```rust
#[tauri::command]
pub async fn restore_files(
    snapshot_id: String,
    files: Vec<String>, // Leer = alle Dateien
    target: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use rustic_core::{LocalDestination, LsOptions, RestoreOptions};

    let repo = state.get_repository()?;

    // Snapshot laden
    let snapshot = repo.get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Ziel validieren
    let target_path = PathBuf::from(target);
    if !target_path.exists() {
        std::fs::create_dir_all(&target_path)
            .map_err(|e| format!("Ziel erstellen fehlgeschlagen: {}", e))?;
    }

    // Event: Started
    app_handle.emit("restore-started", json!({
        "snapshot_id": snapshot_id,
        "target_path": target_path,
    })).ok();

    // Tree laden
    let tree = repo.node_from_snapshot_path("", |sn| sn.id == snapshot.id)
        .map_err(|e| format!("Tree laden fehlgeschlagen: {}", e))?;

    // Destination erstellen (wie in rustic CLI)
    let dest = LocalDestination::new(
        target_path.to_str().unwrap(),
        true,  // create
        !tree.is_dir()  // flatten
    ).map_err(|e| format!("Destination ungültig: {}", e))?;

    // Restore-Optionen
    let restore_opts = RestoreOptions::default()
        .delete(overwrite)
        .no_ownership(!restore_permissions);

    // LsOptions für node_streamer
    let mut ls_opts = LsOptions::default();
    ls_opts.recursive = true;

    // Schritt 1: Restore-Plan erstellen
    let node_streamer = repo.ls(&tree, &ls_opts)?;
    let plan = repo.prepare_restore(&restore_opts, node_streamer, &dest, false)
        .map_err(|e| format!("Plan erstellen fehlgeschlagen: {}", e))?;

    // Total-Files extrahieren (VOR move!)
    let total_files = plan.stats.files.restore
                    + plan.stats.files.unchanged
                    + plan.stats.files.verified;

    // Event: Progress
    app_handle.emit("restore-progress", json!({
        "current": 0,
        "total": total_files,
        "percentage": 0.0,
    })).ok();

    // Schritt 2: Restore ausführen
    let node_streamer_restore = repo.ls(&tree, &ls_opts)?;
    repo.restore(plan, &restore_opts, node_streamer_restore, &dest)
        .map_err(|e| format!("Restore fehlgeschlagen: {}", e))?;

    // Event: Completed
    app_handle.emit("restore-completed", json!({
        "snapshot_id": snapshot_id,
        "files_restored": total_files,
    })).ok();

    Ok(())
}
```

**Referenz:** [rustic CLI restore.rs](https://github.com/rustic-rs/rustic/blob/main/src/commands/restore.rs)

**Hinweis:** rustic_core bietet **keinen Progress-Callback** für `restore()` - muss simuliert werden.

---

### Restore mit File-Selection (alte Version - Referenz)

```rust
#[tauri::command]
pub async fn restore_files(
    snapshot_id: String,
    files: Vec<String>, // Leer = alle Dateien
    target: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let repo = state.get_current_repo()?;

    // Snapshot laden
    let snapshot = repo.get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    // Ziel validieren
    let target_path = PathBuf::from(target);
    if !target_path.exists() {
        std::fs::create_dir_all(&target_path)
            .map_err(|e| format!("Ziel erstellen fehlgeschlagen: {}", e))?;
    }

    // Progress-Callback
    let app_handle_clone = app_handle.clone();
    let progress_callback = move |info: RestoreProgress| {
        app_handle_clone.emit_all("restore-progress", json!({
            "files_processed": info.files_done,
            "files_total": info.files_total,
            "bytes_processed": info.bytes_done,
            "bytes_total": info.bytes_total,
        })).ok();
    };

    // Restore-Optionen
    let mut opts = RestoreOptions::default()
        .set_progress(Some(Box::new(progress_callback)));

    if !files.is_empty() {
        opts = opts.with_files(files);
    }

    // Destination
    let dest = LocalDestination::new(target_path)
        .map_err(|e| format!("Destination ungültig: {}", e))?;

    // Restore ausführen
    repo.restore(&snapshot, dest, &opts)
        .await
        .map_err(|e| format!("Restore fehlgeschlagen: {}", e))?;

    // Completion-Event
    app_handle.emit_all("restore-completed", json!({
        "snapshot_id": snapshot_id,
    })).ok();

    Ok(())
}
```

### File-Tree für Restore-Browser

```rust
#[derive(Serialize)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
    pub permissions: String,
}

#[tauri::command]
pub async fn list_snapshot_files(
    snapshot_id: String,
    path: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<FileTreeNode>, String> {
    let repo = state.get_current_repo()?;

    let snapshot = repo.get_snapshot_from_str(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;

    let tree = repo.node_from_snapshot(&snapshot)
        .map_err(|e| format!("Tree laden fehlgeschlagen: {}", e))?;

    // Navigiere zu Pfad (falls angegeben)
    let nodes = if let Some(p) = path {
        repo.get_node_at_path(&tree, &p)
            .map_err(|e| format!("Pfad nicht gefunden: {}", e))?
            .nodes
    } else {
        tree.nodes
    };

    // Konvertiere zu Frontend-Format
    let file_nodes = nodes
        .into_iter()
        .map(|node| FileTreeNode {
            name: node.name().to_string(),
            path: node.path().to_string(),
            is_dir: matches!(node.node_type(), NodeType::Dir),
            size: node.meta().size,
            modified: node.meta().mtime.to_rfc3339(),
            permissions: format!("{:o}", node.meta().mode),
        })
        .collect();

    Ok(file_nodes)
}
```

---

## 📸 Snapshot-Management

### Snapshots auflisten mit Filtering

```rust
#[derive(Deserialize)]
pub struct SnapshotFilter {
    pub hostname: Option<String>,
    pub tags: Option<Vec<String>>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[tauri::command]
pub async fn list_snapshots(
    filter: Option<SnapshotFilter>,
    state: State<'_, AppState>,
) -> Result<Vec<SnapshotDto>, String> {
    let repo = state.get_current_repo()?;

    // Alle Snapshots laden
    let mut snapshots = repo.get_all_snapshots()
        .map_err(|e| format!("Snapshots laden fehlgeschlagen: {}", e))?;

    // Filtering anwenden
    if let Some(filter) = filter {
        snapshots = snapshots
            .into_iter()
            .filter(|snap| {
                // Hostname-Filter
                if let Some(ref hostname) = filter.hostname {
                    if &snap.hostname != hostname {
                        return false;
                    }
                }

                // Tag-Filter
                if let Some(ref tags) = filter.tags {
                    if !tags.iter().any(|t| snap.tags.contains(t)) {
                        return false;
                    }
                }

                // Datums-Filter
                if let Some(ref from) = filter.date_from {
                    if snap.time < from.parse().unwrap() {
                        return false;
                    }
                }

                if let Some(ref to) = filter.date_to {
                    if snap.time > to.parse().unwrap() {
                        return false;
                    }
                }

                true
            })
            .collect();
    }

    // Nach Datum sortieren (neueste zuerst)
    snapshots.sort_by(|a, b| b.time.cmp(&a.time));

    // In DTOs konvertieren
    let dtos = snapshots
        .into_iter()
        .map(|snap| snapshot_to_dto(&snap))
        .collect();

    Ok(dtos)
}
```

### Snapshot-Vergleich

```rust
#[derive(Serialize)]
pub struct SnapshotDiff {
    pub added_files: Vec<String>,
    pub removed_files: Vec<String>,
    pub modified_files: Vec<String>,
    pub added_size: u64,
    pub removed_size: u64,
}

#[tauri::command]
pub async fn compare_snapshots(
    snapshot_a_id: String,
    snapshot_b_id: String,
    state: State<'_, AppState>,
) -> Result<SnapshotDiff, String> {
    let repo = state.get_current_repo()?;

    let snap_a = repo.get_snapshot_from_str(&snapshot_a_id)
        .map_err(|e| format!("Snapshot A nicht gefunden: {}", e))?;

    let snap_b = repo.get_snapshot_from_str(&snapshot_b_id)
        .map_err(|e| format!("Snapshot B nicht gefunden: {}", e))?;

    // rustic_core Diff nutzen
    let diff = repo.diff_snapshots(&snap_a, &snap_b)
        .map_err(|e| format!("Diff fehlgeschlagen: {}", e))?;

    Ok(SnapshotDiff {
        added_files: diff.added.iter().map(|f| f.path.clone()).collect(),
        removed_files: diff.removed.iter().map(|f| f.path.clone()).collect(),
        modified_files: diff.modified.iter().map(|f| f.path.clone()).collect(),
        added_size: diff.stats.data_added,
        removed_size: diff.stats.data_removed,
    })
}
```

### Snapshot löschen mit Prune

```rust
#[tauri::command]
pub async fn delete_snapshot(
    snapshot_id: String,
    prune: bool, // Ob unused data entfernt werden soll
    state: State<'_, AppState>,
) -> Result<(), String> {
    let repo = state.get_current_repo()?;

    // Snapshot vergessen
    repo.forget_snapshot(&snapshot_id)
        .await
        .map_err(|e| format!("Snapshot löschen fehlgeschlagen: {}", e))?;

    // Optional: Prune unused data
    if prune {
        repo.prune()
            .await
            .map_err(|e| format!("Prune fehlgeschlagen: {}", e))?;
    }

    Ok(())
}
```

---

## ⏰ Job-Scheduling

### Scheduler-Setup

```rust
use tokio_cron_scheduler::{JobScheduler, Job};

pub struct BackupScheduler {
    scheduler: JobScheduler,
    jobs: HashMap<String, uuid::Uuid>,
}

impl BackupScheduler {
    pub async fn new() -> Result<Self> {
        let scheduler = JobScheduler::new().await?;
        scheduler.start().await?;

        Ok(Self {
            scheduler,
            jobs: HashMap::new(),
        })
    }

    pub async fn schedule_job(
        &mut self,
        job_id: String,
        cron_expr: &str,
        callback: impl Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static,
    ) -> Result<()> {
        let job = Job::new_async(cron_expr, move |_uuid, _lock| {
            Box::pin(callback())
        })?;

        let uuid = self.scheduler.add(job).await?;
        self.jobs.insert(job_id, uuid);

        Ok(())
    }

    pub async fn remove_job(&mut self, job_id: &str) -> Result<()> {
        if let Some(uuid) = self.jobs.remove(job_id) {
            self.scheduler.remove(&uuid).await?;
        }
        Ok(())
    }
}
```

### Schedule-Command

```rust
#[tauri::command]
pub async fn schedule_backup(
    job_id: String,
    cron_expression: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Validiere Cron-Expression
    validate_cron_expression(&cron_expression)
        .map_err(|e| format!("Ungültiger Cron-Ausdruck: {}", e))?;

    let mut scheduler = state.scheduler.lock().await;

    let job_id_clone = job_id.clone();
    let app_handle_clone = app_handle.clone();
    let state_clone = state.inner().clone();

    scheduler.schedule_job(
        job_id.clone(),
        &cron_expression,
        move || {
            let job_id = job_id_clone.clone();
            let app_handle = app_handle_clone.clone();
            let state = state_clone.clone();

            Box::pin(async move {
                tracing::info!("Scheduled backup started: {}", job_id);

                // Event senden
                app_handle.emit_all("scheduled-backup-started", json!({
                    "job_id": job_id,
                    "time": chrono::Utc::now().to_rfc3339(),
                })).ok();

                // Backup ausführen
                match run_backup(job_id.clone(), state, app_handle.clone()).await {
                    Ok(result) => {
                        app_handle.emit_all("scheduled-backup-completed", json!({
                            "job_id": job_id,
                            "result": result,
                        })).ok();
                    }
                    Err(e) => {
                        tracing::error!("Scheduled backup failed: {}", e);
                        app_handle.emit_all("scheduled-backup-failed", json!({
                            "job_id": job_id,
                            "error": e,
                        })).ok();
                    }
                }
            })
        },
    ).await
    .map_err(|e| format!("Scheduling fehlgeschlagen: {}", e))?;

    tracing::info!("Backup {} scheduled with: {}", job_id, cron_expression);

    Ok(())
}
```

### Cron-Validation

```rust
fn validate_cron_expression(expr: &str) -> Result<()> {
    use cron::Schedule;
    use std::str::FromStr;

    Schedule::from_str(expr)
        .map_err(|e| anyhow!("Ungültiger Cron-Ausdruck: {}", e))?;

    Ok(())
}
```

---

## 📊 Retention-Policy

### Retention anwenden

```rust
#[tauri::command]
pub async fn apply_retention_policy(
    policy: RetentionPolicy,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let repo = state.get_current_repo()?;

    // Alle Snapshots laden
    let snapshots = repo.get_all_snapshots()
        .map_err(|e| format!("Snapshots laden fehlgeschlagen: {}", e))?;

    // rustic_core's Retention-Logic
    let to_keep = repo.apply_retention_policy(&snapshots, &policy)
        .map_err(|e| format!("Retention-Policy fehlgeschlagen: {}", e))?;

    // Ermittle zu löschende Snapshots
    let to_delete: Vec<String> = snapshots
        .iter()
        .filter(|s| !to_keep.contains(&s.id))
        .map(|s| s.id.to_string())
        .collect();

    // Lösche Snapshots
    for id in &to_delete {
        repo.forget_snapshot(id).await
            .map_err(|e| format!("Snapshot {} löschen fehlgeschlagen: {}", id, e))?;
    }

    // Prune unused data
    repo.prune().await
        .map_err(|e| format!("Prune fehlgeschlagen: {}", e))?;

    Ok(to_delete)
}
```

---

## ✅ Feature-Checkliste

### Backup

- [ ] Job-Konfiguration validiert
- [ ] Progress-Events implementiert
- [ ] Cancellation unterstützt
- [ ] Error-Handling vollständig
- [ ] Logs aussagekräftig

### Restore

- [ ] File-Selection funktioniert
- [ ] Progress-Anzeige
- [ ] Permissions werden wiederhergestellt
- [ ] Ziel-Validierung
- [ ] Error-Handling

### Snapshots

- [ ] Filtering implementiert
- [ ] Sorting korrekt
- [ ] Vergleich funktioniert
- [ ] Löschen mit Confirmation
- [ ] Retention-Policy

### Scheduler

- [ ] Cron-Validation
- [ ] Job-Management (Add/Remove)
- [ ] Events bei Start/Ende
- [ ] Error-Handling
- [ ] Persistenz in Config

---

**Version**: 1.0  
**Letzte Aktualisierung**: 2025-10-26
