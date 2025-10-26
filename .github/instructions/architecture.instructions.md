# Architecture Instructions

> System-Architektur, Komponentenstruktur und Datenflüsse für Rustic GUI

---

## 🏗️ System-Architektur

### High-Level-Übersicht

```
┌─────────────────────────────────────────────┐
│           Frontend (Svelte + TS)            │
│  ┌─────────────┐  ┌───────────────────────┐ │
│  │   Pages     │  │    Components         │ │
│  │  Dashboard  │  │  - Dialogs            │ │
│  │  Snapshots  │  │  - FileTree           │ │
│  │  Jobs       │  │  - ProgressBar        │ │
│  └──────┬──────┘  └──────────┬────────────┘ │
│         │                    │              │
│         └────────┬───────────┘              │
│                  │                          │
│         ┌────────▼──────────┐               │
│         │   Svelte Stores   │               │
│         │  - repositories   │               │
│         │  - snapshots      │               │
│         │  - jobs           │               │
│         └────────┬──────────┘               │
│                  │                          │
│         ┌────────▼──────────┐               │
│         │   API Wrapper     │               │
│         │  (TypeScript)     │               │
│         └────────┬──────────┘               │
└──────────────────┼──────────────────────────┘
                   │ IPC (Tauri invoke/emit)
┌──────────────────▼──────────────────────────┐
│          Backend (Tauri + Rust)             │
│  ┌──────────────────────────────────────────┐│
│  │        Tauri Commands                    ││
│  │  - run_backup                            ││
│  │  - list_snapshots                        ││
│  │  - restore_files                         ││
│  └──────────────┬───────────────────────────┘│
│                 │                            │
│  ┌──────────────▼───────────────────────────┐│
│  │      Rustic Integration                  ││
│  │  ┌────────────┐  ┌────────────────────┐ ││
│  │  │rustic_core │  │ rustic_backend     │ ││
│  │  │ - Backup   │  │ - OpenDAL          │ ││
│  │  │ - Restore  │  │ - rclone FFI       │ ││
│  │  │ - Snapshots│  │                    │ ││
│  │  └────────────┘  └────────────────────┘ ││
│  └──────────────────────────────────────────┘│
│                 │                            │
│  ┌──────────────▼───────────────────────────┐│
│  │      Storage Layer                       ││
│  │  - Config (TOML)                         ││
│  │  - Keychain (Passwords)                  ││
│  │  - Scheduler (Cron Jobs)                 ││
│  └──────────────────────────────────────────┘│
└──────────────────┬──────────────────────────┘
                   │
                   ▼
        ┌──────────────────┐
        │  rustic Repository│
        │  (local/cloud)    │
        └──────────────────┘
```

---

## 📦 Komponenten-Architektur

### Frontend-Layer

#### 1. UI-Komponenten (Svelte)

**Hierarchie:**
```
Pages (Routen)
  └─> Layout-Komponenten (Sidebar, Header)
      └─> Feature-Komponenten (SnapshotList, JobCard)
          └─> Shared-Komponenten (Button, Modal, Toast)
```

**Verantwortlichkeiten:**
- **Pages**: Routing, Daten-Loading, Layout-Komposition
- **Feature-Komponenten**: Feature-spezifische UI-Logik
- **Shared-Komponenten**: Wiederverwendbare UI-Elemente

**Kommunikation:**
- Props down (Parent → Child)
- Events up (Child → Parent via dispatch)
- Stores für globalen State

#### 2. State Management (Stores)

**Store-Typen:**

```typescript
// Writable Stores: App-interner State
const _repositories = writable<Repository[]>([]);

// Derived Stores: Berechneter State
const activeRepository = derived(
  [repositories, activeRepositoryId],
  ([$repos, $activeId]) => ...
);

// Custom Stores: Mit Business-Logik
export const repositories = {
  subscribe: _repositories.subscribe,
  load: async () => { ... },
  add: async (repo) => { ... },
};
```

**Store-Organisation:**
```
stores/
├── repositories.ts    # Repository-Verwaltung
├── snapshots.ts       # Snapshot-Liste & Details
├── backup-jobs.ts     # Job-Konfiguration & Status
├── ui.ts              # UI-State (Modals, Sidebar, etc.)
└── toasts.ts          # Notifications
```

#### 3. API-Layer (TypeScript Wrapper)

**Zweck:**
- Kapselt Tauri IPC-Aufrufe
- Type-Safety für Backend-Kommunikation
- Zentrale Error-Handling-Logik
- Event-Listener-Management

**Struktur:**
```typescript
// api/backup.ts
export async function runBackup(jobId: string): Promise<BackupResult> {
  return await invoke('run_backup', { jobId });
}

export async function onBackupProgress(
  callback: (progress: BackupProgress) => void
): Promise<UnlistenFn> {
  return await listen('backup-progress', callback);
}
```

### Backend-Layer

#### 1. Tauri Commands (Rust)

**Command-Pattern:**
```rust
#[tauri::command]
pub async fn command_name(
    param: Type,
    state: State<'_, AppState>,
) -> Result<ReturnType, String> {
    // 1. Validierung
    // 2. Business-Logik
    // 3. Error-Handling
    // 4. Return
}
```

**Command-Organisation:**
```
src/
├── main.rs              # Command-Registrierung
├── backup/
│   ├── mod.rs
│   └── commands.rs      # run_backup, cancel_backup
├── restore/
│   └── commands.rs      # restore_files, list_files
├── snapshot/
│   └── commands.rs      # list_snapshots, compare_snapshots
└── repository/
    └── commands.rs      # add_repository, switch_repository
```

#### 2. Business-Logik (Rust)

**Schichten:**

```rust
// 1. Command-Layer (Tauri Interface)
#[tauri::command]
pub async fn run_backup(...) -> Result<BackupResult> {
    let executor = BackupExecutor::new(...);
    executor.execute().await
}

// 2. Service-Layer (Business-Logik)
pub struct BackupExecutor {
    repo: Repository,
    config: BackupConfig,
}

impl BackupExecutor {
    pub async fn execute(&self) -> Result<BackupResult> {
        // Orchestriert den Backup-Prozess
    }
}

// 3. Integration-Layer (rustic_core)
use rustic_core::{Repository, BackupOptions};

async fn run_rustic_backup(
    repo: &Repository,
    paths: &[PathBuf],
) -> Result<()> {
    repo.backup(paths, &BackupOptions::default()).await
}
```

#### 3. State Management (Rust)

**AppState-Pattern:**
```rust
pub struct AppState {
    // Aktuell geöffnetes Repository
    pub current_repo: Arc<Mutex<Option<Repository>>>,
    
    // Laufende Backups (für Cancellation)
    pub cancellation_tokens: Arc<Mutex<HashMap<String, CancellationToken>>>,
    
    // Scheduler für zeitgesteuerte Jobs
    pub scheduler: Arc<Mutex<BackupScheduler>>,
    
    // Config
    pub config: Arc<Mutex<AppConfig>>,
}
```

**State-Zugriff in Commands:**
```rust
#[tauri::command]
pub async fn some_command(
    state: State<'_, AppState>,
) -> Result<()> {
    // Repository holen
    let repo = state.current_repo.lock().unwrap()
        .as_ref()
        .ok_or("Kein Repository geöffnet")?;
    
    // Mit Repository arbeiten
    Ok(())
}
```

---

## 🔄 Datenflüsse

### 1. Backup-Flow

```
User klickt "Backup starten"
         │
         ▼
┌────────────────────┐
│ Frontend           │
│ BackupJob.svelte   │
└────────┬───────────┘
         │ invoke('run_backup', { jobId })
         ▼
┌────────────────────┐
│ Tauri Command      │
│ run_backup()       │
└────────┬───────────┘
         │ 1. Validierung
         │ 2. Lade Job-Config
         │ 3. Öffne Repository
         ▼
┌────────────────────┐
│ BackupExecutor     │
│ execute()          │
└────────┬───────────┘
         │ 4. Starte rustic_core Backup
         ▼
┌────────────────────┐
│ rustic_core        │
│ repo.backup()      │
└────────┬───────────┘
         │ Progress-Callbacks
         ├──────────────────────┐
         │                      │
         ▼                      ▼
┌─────────────────┐  ┌────────────────┐
│ Tauri Events    │  │ Daten schreiben│
│ emit_all()      │  │ ins Repository │
└────────┬────────┘  └────────────────┘
         │ event: 'backup-progress'
         ▼
┌────────────────────┐
│ Frontend Updates   │
│ Progress-Bar       │
└────────────────────┘
```

### 2. Restore-Flow

```
User wählt Dateien im FileTree
         │
         ▼
┌────────────────────┐
│ Frontend           │
│ RestoreBrowser     │
└────────┬───────────┘
         │ 1. Lazy-load Verzeichnisse
         │    invoke('list_snapshot_files', { path })
         ▼
┌────────────────────┐
│ Backend            │
│ list_files()       │
└────────┬───────────┘
         │ 2. rustic_core Tree-Traversierung
         ▼
┌────────────────────┐
│ Frontend           │
│ FileTree aktualisiert│
└────────────────────┘

User klickt "Restore"
         │
         ▼
┌────────────────────┐
│ RestoreDialog      │
│ Ziel auswählen     │
└────────┬───────────┘
         │ invoke('restore_files', { files, target })
         ▼
┌────────────────────┐
│ Backend            │
│ restore_files()    │
└────────┬───────────┘
         │ rustic_core Restore
         │ mit Progress-Events
         ▼
┌────────────────────┐
│ Frontend           │
│ Progress-Anzeige   │
└────────────────────┘
```

### 3. Snapshot-Listing-Flow

```
User öffnet Snapshot-Seite
         │
         ▼
┌────────────────────┐
│ Snapshots.svelte   │
│ onMount()          │
└────────┬───────────┘
         │ loadSnapshots() (Store)
         ▼
┌────────────────────┐
│ snapshots Store    │
└────────┬───────────┘
         │ invoke('list_snapshots')
         ▼
┌────────────────────┐
│ Backend            │
│ list_snapshots()   │
└────────┬───────────┘
         │ 1. Hole aktuelles Repo
         │ 2. rustic_core snapshots()
         │ 3. Parse zu SnapshotDto[]
         ▼
┌────────────────────┐
│ snapshots Store    │
│ _snapshots.set()   │
└────────┬───────────┘
         │ Reaktiv
         ▼
┌────────────────────┐
│ Snapshots.svelte   │
│ $snapshots → Liste │
└────────────────────┘
```

### 4. Event-Flow (Backend → Frontend)

```
Backend (Rust)
         │
         │ app_handle.emit_all("event-name", payload)
         ▼
Tauri Event-Bus
         │
         ▼
Frontend (TypeScript)
         │
         │ listen("event-name", callback)
         ▼
Event-Handler (Component)
         │
         │ State-Update
         ▼
UI-Update (Svelte Reaktivität)
```

**Beispiel-Events:**
- `backup-progress-{jobId}` → Progress-Updates
- `backup-completed-{jobId}` → Backup fertig
- `backup-failed-{jobId}` → Backup-Fehler
- `scheduled-backup-started-{jobId}` → Scheduled Backup gestartet

---

## 🗂️ Datenmodelle

### TypeScript Types (Frontend)

```typescript
// types/repository.types.ts
export interface Repository {
  id: string;
  name: string;
  path: string;
  backendType: 'local' | 'sftp' | 's3' | 'rclone';
  passwordStored: boolean;
}

// types/snapshot.types.ts
export interface Snapshot {
  id: string;
  time: string;
  hostname: string;
  username: string;
  paths: string[];
  tags: string[];
  summary: {
    filesNew: number;
    filesChanged: number;
    filesUnmodified: number;
    dataAdded: number;
    totalFilesProcessed: number;
    totalBytesProcessed: number;
  };
}

// types/backup.types.ts
export interface BackupJob {
  id: string;
  name: string;
  repositoryId: string;
  sourcePaths: string[];
  excludePatterns: string[];
  tags: string[];
  schedule?: string; // Cron expression
  retention: RetentionPolicy;
}

export interface BackupProgress {
  filesProcessed: number;
  filesTotal: number;
  bytesProcessed: number;
  bytesTotal: number;
  currentFile?: string;
}

export interface BackupResult {
  snapshotId: string;
  duration: number;
  filesProcessed: number;
  bytesProcessed: number;
}
```

### Rust Structs (Backend)

```rust
// src/types.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryDto {
    pub id: String,
    pub name: String,
    pub path: String,
    pub backend_type: BackendType,
    pub password_stored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendType {
    Local,
    Sftp,
    S3,
    Rclone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDto {
    pub id: String,
    pub time: String,
    pub hostname: String,
    pub username: String,
    pub paths: Vec<String>,
    pub tags: Vec<String>,
    pub summary: SnapshotSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSummary {
    pub files_new: usize,
    pub files_changed: usize,
    pub files_unmodified: usize,
    pub data_added: u64,
    pub total_files_processed: usize,
    pub total_bytes_processed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobConfig {
    pub id: String,
    pub name: String,
    pub repository_id: String,
    pub source_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub tags: Vec<String>,
    pub schedule: Option<String>,
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

---

## 🔐 Security-Architektur

### Passwort-Handling

```
┌─────────────────────────────────────────┐
│ Frontend (TypeScript)                   │
│                                         │
│ User gibt Passwort ein                  │
│         │                               │
│         ▼                               │
│ In-Memory (während Session)             │
│         │                               │
│         │ invoke() mit Passwort-Param   │
└─────────┼─────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────┐
│ Backend (Rust)                          │
│                                         │
│ std::env::set_var("RUSTIC_PASSWORD")   │
│         │                               │
│         ▼                               │
│ rustic_core Aufruf                      │
│         │                               │
│         ▼                               │
│ std::env::remove_var("RUSTIC_PASSWORD")│
│                                         │
│ Optional: Keychain-Storage              │
│   (Linux: Secret Service)               │
│   (Windows: Credential Manager)         │
└─────────────────────────────────────────┘
```

**Regeln:**
1. ✅ Passwort nie loggen
2. ✅ Nur in Memory halten (Session)
3. ✅ Optional in System-Keychain speichern
4. ✅ Nach Verwendung aus Env entfernen
5. ❌ Nie in Config/TOML speichern

---

## ⚡ Performance-Architektur

### Lazy-Loading-Strategie

**FileTree (Restore-Browser):**
```
1. Initial: Nur Root-Level laden
2. On-Expand: Unterverzeichnis nachladen
3. Cache: Geladene Nodes in Memory halten
4. Virtualisierung: Bei >1000 Items
```

**Snapshot-Liste:**
```
1. Paginierung: 50 Snapshots pro Seite
2. Virtual Scrolling: Bei >100 Items
3. Thumbnail-Cache: Snapshot-Previews cachen
```

### Concurrency

**Frontend:**
```typescript
// Parallele API-Calls
const [repos, jobs, snapshots] = await Promise.all([
  loadRepositories(),
  loadBackupJobs(),
  loadSnapshots(),
]);

// Rate-Limiting für Search
const debouncedSearch = debounce(searchSnapshots, 300);
```

**Backend:**
```rust
// Parallele Verarbeitung mit tokio
let results = stream::iter(items)
    .map(|item| process_item(item))
    .buffer_unordered(3) // Max 3 parallel
    .collect::<Vec<_>>()
    .await;
```

---

## 🔄 Error-Handling-Architektur

### Error-Propagation

```
Frontend Error
     │
     ▼
API-Wrapper catch
     │
     ├─> Log to console
     ├─> Show Toast
     └─> Return/Throw
     
Backend Error
     │
     ▼
Service-Layer catch
     │
     ├─> Log (tracing)
     ├─> Map to user-friendly message
     └─> Return Result<T, String>
     
Tauri Command
     │
     ▼
Frontend receives Err
     │
     └─> Display to user
```

### Error-Types

**Frontend:**
```typescript
class BackupError extends Error {
  constructor(
    message: string,
    public code: string,
    public recoverable: boolean
  ) {}
}
```

**Backend:**
```rust
#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Repository nicht gefunden: {0}")]
    RepositoryNotFound(String),
    
    #[error("Authentifizierung fehlgeschlagen")]
    AuthenticationFailed,
    
    // ... weitere
}
```

---

## 📊 Monitoring & Logging

### Logging-Levels

```
TRACE → Sehr detailliert (nur Development)
DEBUG → Debug-Infos (Development)
INFO  → Wichtige Events (Production)
WARN  → Warnungen (Production)
ERROR → Fehler (Production)
```

### Log-Struktur

**Backend:**
```rust
tracing::info!(
    job_id = %job_id,
    snapshot_id = %result.snapshot_id,
    duration_secs = result.duration,
    "Backup erfolgreich abgeschlossen"
);
```

**Frontend:**
```typescript
console.log('[Backup]', {
  jobId,
  status: 'completed',
  duration: result.duration,
});
```

---

## ✅ Architektur-Checkliste

### Design-Principles
- [ ] Separation of Concerns (UI / Logic / Data)
- [ ] Single Responsibility
- [ ] DRY (Don't Repeat Yourself)
- [ ] KISS (Keep It Simple, Stupid)
- [ ] YAGNI (You Aren't Gonna Need It)

### Patterns
- [ ] Repository-Pattern für Data-Access
- [ ] Service-Pattern für Business-Logic
- [ ] Store-Pattern für State-Management
- [ ] Command-Pattern für Tauri-Commands
- [ ] Event-Pattern für Async-Communication

### Security
- [ ] Input-Validierung (Frontend + Backend)
- [ ] Passwort-Handling sicher
- [ ] Keine Secrets in Logs
- [ ] Error-Messages user-friendly (keine Details)

### Performance
- [ ] Lazy-Loading wo möglich
- [ ] Caching wo sinnvoll
- [ ] Debouncing bei User-Input
- [ ] Parallele Ausführung nutzen

---

**Version**: 1.0  
**Letzte Aktualisierung**: 2025-10-26