# Milestone 3: Job-Scheduler ⏰

> **Automatisierte Backup-Jobs mit Cron-Scheduling**

**Dauer:** 30h (1 Woche) | **Status:** 0% - BLOCKING Automation  
**Priorität:** 🟠 HIGH  
**Dependencies:** M1 (run_backup muss funktionieren)

---

## Übersicht

**Problem:** Aktuell nur manuelle Backups. Keine Automatisierung.

**Lösung:** tokio-cron-scheduler Integration für zeitgesteuerte Backup-Jobs.

**UI Status:** ✅ Backup Jobs Page, CreateJobDialog mit Cron-Builder vorhanden.

**Betroffene Dateien:**

- `src-tauri/src/state.rs:12` - BackupScheduler TODO
- `src-tauri/src/commands/backup.rs:263-264` - Job-History TODO
- Neue Datei: `src-tauri/src/scheduler/mod.rs`

---

## 3.1 Scheduler-Setup

**Dauer:** 12h | **Priorität:** 🔴 HIGHEST

### Task 3.1.1: BackupScheduler-Struktur (4h)

**Datei:** `src-tauri/src/scheduler/mod.rs`

```rust
use tokio_cron_scheduler::{JobScheduler, Job};
use std::collections::HashMap;

pub struct BackupScheduler {
    scheduler: JobScheduler,
    jobs: HashMap<String, uuid::Uuid>, // job_id -> scheduler_uuid
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

    pub fn list_scheduled_jobs(&self) -> Vec<String> {
        self.jobs.keys().cloned().collect()
    }
}
```

**Tests:**

- [ ] Scheduler erstellen
- [ ] Job hinzufügen (mit "\* \* \* \* \* \*" - jede Sekunde)
- [ ] Job wird ausgeführt
- [ ] Job entfernen

---

### Task 3.1.2: AppState-Integration (2h)

**Datei:** `src-tauri/src/state.rs:12`

```rust
pub struct AppState {
    // ... existing fields ...
    pub scheduler: Arc<Mutex<Option<BackupScheduler>>>, // TODO: Aktuell None
}

impl AppState {
    pub fn new() -> Self {
        // Scheduler wird async initialisiert in main.rs
        Self {
            // ... existing fields ...
            scheduler: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn init_scheduler(&self) -> Result<()> {
        let mut scheduler = self.scheduler.lock().unwrap();
        *scheduler = Some(BackupScheduler::new().await?);
        Ok(())
    }
}
```

**Datei:** `src-tauri/src/main.rs` - Async-Initialisierung

```rust
#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    // Scheduler async initialisieren
    app_state.init_scheduler().await.expect("Scheduler init failed");

    tauri::Builder::default()
        .manage(app_state)
        // ... rest ...
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Task 3.1.3: schedule_backup Command (6h)

**Datei:** `src-tauri/src/commands/backup.rs` (erweitern)

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

    let mut scheduler_lock = state.scheduler.lock().unwrap();
    let scheduler = scheduler_lock.as_mut()
        .ok_or_else(|| "Scheduler nicht initialisiert".to_string())?;

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

fn validate_cron_expression(expr: &str) -> Result<()> {
    use cron::Schedule;
    use std::str::FromStr;

    Schedule::from_str(expr)
        .map_err(|e| anyhow!("Ungültiger Cron-Ausdruck: {}", e))?;

    Ok(())
}
```

**Tests:**

- [ ] Job schedulen mit "_/5 _ \* \* \* \*" (alle 5 Sekunden)
- [ ] Job wird 3x ausgeführt (Events prüfen)
- [ ] Job entfernen, keine weiteren Ausführungen

---

## 3.2 Job-State-Persistence

**Dauer:** 10h | **Priorität:** 🟠 HIGH

### Task 3.2.1: Job-History-Tracking (5h)

**Datei:** `src-tauri/src/commands/backup.rs:263-264`

**Neue Struktur:**

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct JobExecution {
    pub job_id: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: JobExecutionStatus,
    pub snapshot_id: Option<String>,
    pub files_processed: u64,
    pub bytes_processed: u64,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum JobExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

pub struct JobHistory {
    executions: Vec<JobExecution>,
}

impl JobHistory {
    pub fn new() -> Self {
        Self { executions: Vec::new() }
    }

    pub fn add_execution(&mut self, execution: JobExecution) {
        self.executions.push(execution);

        // Nur letzte 100 behalten
        if self.executions.len() > 100 {
            self.executions.remove(0);
        }
    }

    pub fn get_executions_for_job(&self, job_id: &str) -> Vec<&JobExecution> {
        self.executions.iter()
            .filter(|e| e.job_id == job_id)
            .collect()
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.executions)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let executions = serde_json::from_str(&json)?;
        Ok(Self { executions })
    }
}
```

**AppState erweitern:**

```rust
pub struct AppState {
    // ... existing fields ...
    pub job_history: Arc<Mutex<JobHistory>>,
}
```

**run_backup erweitern:**

```rust
// Vor Backup
let execution = JobExecution {
    job_id: job_id.clone(),
    started_at: Utc::now(),
    finished_at: None,
    status: JobExecutionStatus::Running,
    snapshot_id: None,
    files_processed: 0,
    bytes_processed: 0,
    error_message: None,
};

state.job_history.lock().unwrap().add_execution(execution.clone());

// Nach Backup (Erfolg)
let mut history = state.job_history.lock().unwrap();
history.update_execution(&job_id, |e| {
    e.finished_at = Some(Utc::now());
    e.status = JobExecutionStatus::Completed;
    e.snapshot_id = Some(result.snapshot_id.clone());
    e.files_processed = result.files_processed;
    e.bytes_processed = result.bytes_processed;
});

// Auf Disk speichern
history.save_to_file(&config_dir().join("job_history.json"))?;
```

---

### Task 3.2.2: Job-Config-Persistence (3h)

**Beim App-Start:** Scheduled Jobs aus Config laden und re-schedulen.

**Datei:** `src-tauri/src/main.rs`

```rust
async fn restore_scheduled_jobs(state: &AppState) -> Result<()> {
    let config = state.config.lock().unwrap();

    for job in &config.backup_jobs {
        if let Some(schedule) = &job.schedule {
            // Job re-schedulen
            schedule_backup(
                job.id.clone(),
                schedule.clone(),
                // ... state, app_handle ...
            ).await?;

            tracing::info!("Restored scheduled job: {}", job.name);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    app_state.init_scheduler().await.expect("Scheduler init failed");

    // Jobs wiederherstellen
    restore_scheduled_jobs(&app_state).await.expect("Job restore failed");

    // ... rest ...
}
```

---

### Task 3.2.3: list_job_history Command (2h)

**Datei:** `src-tauri/src/commands/backup.rs`

```rust
#[tauri::command]
pub fn list_job_history(
    job_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<JobExecution>, String> {
    let history = state.job_history.lock().unwrap();

    let executions = if let Some(id) = job_id {
        history.get_executions_for_job(&id)
            .into_iter()
            .cloned()
            .collect()
    } else {
        history.executions.clone()
    };

    Ok(executions)
}
```

---

## 3.3 Retry-Logic & Error-Handling

**Dauer:** 8h | **Priorität:** 🟡 MEDIUM

### Task 3.3.1: Retry-Strategie (5h)

**Konfigurierbar:**

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub retry_delay_secs: u64,
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_secs: 60,
            exponential_backoff: true,
        }
    }
}
```

**Implementierung:**

```rust
async fn run_backup_with_retry(
    job_id: String,
    config: &BackupJobConfig,
    retry_config: &RetryConfig,
    // ... state, app_handle ...
) -> Result<BackupResult> {
    let mut attempts = 0;
    let mut last_error = None;

    while attempts <= retry_config.max_retries {
        match run_backup(job_id.clone(), /* ... */).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                last_error = Some(e.clone());

                if attempts <= retry_config.max_retries {
                    let delay = if retry_config.exponential_backoff {
                        retry_config.retry_delay_secs * 2_u64.pow(attempts as u32)
                    } else {
                        retry_config.retry_delay_secs
                    };

                    tracing::warn!("Backup attempt {} failed, retrying in {}s: {}",
                        attempts, delay, e);

                    tokio::time::sleep(Duration::from_secs(delay)).await;
                } else {
                    tracing::error!("Backup failed after {} attempts", attempts);
                }
            }
        }
    }

    Err(last_error.unwrap())
}
```

---

### Task 3.3.2: Error-Notifications (3h)

**Events:**

- `scheduled-backup-retry` - Retry wird durchgeführt
- `scheduled-backup-failed-final` - Alle Retries fehlgeschlagen

**UI-Integration:** Toast-Notifications bereits vorhanden, Events abonnieren.

---

## Zusammenfassung & Deliverables

### Gesamt-Dauer

**30 Stunden (1 Woche Vollzeit)**

### Deliverables Checklist

- [ ] **tokio-cron-scheduler integriert**
- [ ] **Jobs können scheduliert werden** (Cron-Expression)
- [ ] **Job-State-Persistence** (nach Neustart wiederhergestellt)
- [ ] **Job-History** (letzte 100 Ausführungen gespeichert)
- [ ] **Retry-Logic** (konfigurierbar, exponential backoff)
- [ ] **Error-Notifications** (Events für Frontend)
- [ ] **Unit-Tests** (Scheduler, History, Retry)

### Akzeptanz-Kriterien

| Kriterium                                   | Test               | Status |
| ------------------------------------------- | ------------------ | ------ |
| Job wird alle 5 Sekunden ausgeführt         | Integration-Test   | [ ]    |
| Jobs werden nach Neustart wiederhergestellt | App restart        | [ ]    |
| Job-History wird gespeichert                | Config-File prüfen | [ ]    |
| Retry funktioniert bei Netzwerk-Fehler      | Mock-Fehler        | [ ]    |

---

**[← Zurück zu M2](M2-cloud-backends.md)** | **[Zurück zur Roadmap](../../ROADMAP.md)** | **[Weiter zu M4 →](M4-advanced-features.md)**
