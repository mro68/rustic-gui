# Code-Style Instructions

> Code-Konventionen, Naming und Best Practices für Rustic GUI

---

## 🌍 Sprachkonventionen: Hybrid-Ansatz

### Grundregel

```
💬 Kommunikation/Kommentare: DEUTSCH
💻 Code-Elemente: ENGLISCH
📚 Dokumentation: DEUTSCH
```

### Im Detail

#### ✅ DEUTSCH verwenden für:

- **Alle Kommentare** im Code
  ```typescript
  // Validiere Job-Konfiguration vor dem Speichern
  function validateJob(job: BackupJob) { ... }
  ```

- **Docstrings** (TSDoc, Rustdoc)
  ```typescript
  /**
   * Startet einen Backup-Job und gibt das Ergebnis zurück.
   * 
   * @param jobId - Eindeutige ID des Backup-Jobs
   */
  ```

- **Error Messages für User** (UI)
  ```typescript
  throw new Error('Repository nicht gefunden');
  showToast('error', 'Backup fehlgeschlagen');
  ```

- **UI-Texte** (Buttons, Labels, etc.)
  ```svelte
  <button>Backup starten</button>
  <label>Quellpfade auswählen</label>
  ```

- **Log-Ausgaben** für User-facing Logs
  ```rust
  tracing::info!("Backup erfolgreich abgeschlossen");
  tracing::error!("Fehler beim Öffnen des Repositories");
  ```

- **Git Commit Messages**
  ```bash
  git commit -m "feat(backup): Job-Scheduling implementiert"
  ```

- **Dokumentation** (README, Instructions)
  ```markdown
  ## Installation
  
  Lade die AppImage herunter und mache sie ausführbar:
  ```

#### ✅ ENGLISCH verwenden für:

- **Variablen**
  ```typescript
  const backupJobs = [];
  let currentSnapshot = null;
  const snapshotId = '123';
  ```

- **Funktionen**
  ```typescript
  function runBackup() {}
  async function listSnapshots() {}
  function validateRepository() {}
  ```

- **Typen/Interfaces**
  ```typescript
  interface BackupJob {}
  type SnapshotId = string;
  enum BackupStatus {}
  ```

- **Klassen/Structs**
  ```rust
  struct BackupJob {}
  struct SnapshotInfo {}
  enum BackupError {}
  ```

- **Dateien**
  ```
  backup-service.ts
  snapshot-store.ts
  create-job-dialog.svelte
  ```

- **Branches**
  ```bash
  feature/snapshot-compare
  fix/restore-permissions
  ```

- **Technische Begriffe** (keine Übersetzung!)
  - Repository (nicht: Ablageort)
  - Snapshot (nicht: Momentaufnahme)
  - Restore (nicht: Wiederherstellen)
  - Backup (nicht: Sicherung)

---

## 💻 TypeScript / Svelte Conventions

### Naming Conventions

```typescript
// Variablen: camelCase
const backupJobs = [];
let currentSnapshot = null;
const isBackupRunning = false;

// Konstanten: SCREAMING_SNAKE_CASE (nur für echte Konstanten)
const MAX_RETRIES = 3;
const DEFAULT_TIMEOUT_MS = 5000;
const API_BASE_URL = 'http://localhost:3000';

// Funktionen: camelCase, mit Verb beginnen
function createBackup() {}
function validateRepository() {}
async function fetchSnapshots() {}

// Typen/Interfaces: PascalCase
interface BackupJob {
  id: string;
  name: string;
}

type SnapshotId = string;
type BackupStatus = 'idle' | 'running' | 'completed' | 'failed';

// Enums: PascalCase (Keys auch PascalCase)
enum BackupStatus {
  Idle = 'idle',
  Running = 'running',
  Completed = 'completed',
  Failed = 'failed',
}

// Svelte Components: PascalCase
// Datei: CreateJobDialog.svelte
export let jobName: string;

// Svelte Stores: camelCase
export const backupJobs = writable<BackupJob[]>([]);
export const activeRepository = writable<Repository | null>(null);

// Private Funktionen/Variablen: _ Prefix (optional)
function _internalHelper() {}
const _privateData = {};
```

### File Naming

```
Svelte Components: PascalCase.svelte
  - CreateJobDialog.svelte
  - SnapshotList.svelte
  - RestoreBrowser.svelte

TypeScript Files: kebab-case.ts
  - backup-service.ts
  - snapshot-store.ts
  - api-client.ts

Types: kebab-case.types.ts
  - backup.types.ts
  - snapshot.types.ts

Tests: gleicher Name + .test.ts
  - backup-service.test.ts
  - snapshot-store.test.ts
```

### TypeScript Best Practices

#### Type-Safety

```typescript
// ✅ GUT: Explizite Typen
function runBackup(jobId: string): Promise<BackupResult> {
  // ...
}

// ❌ SCHLECHT: Implizite any
function runBackup(jobId) {
  // ...
}

// ✅ GUT: Union Types statt any
type Result = BackupResult | RestoreResult;

// ❌ SCHLECHT: any verwenden
function process(data: any) {
  // ...
}

// ✅ GUT: Generics
function getById<T>(id: string, items: T[]): T | undefined {
  return items.find(item => item.id === id);
}

// ✅ GUT: Type Guards
function isBackupJob(obj: unknown): obj is BackupJob {
  return typeof obj === 'object' 
    && obj !== null 
    && 'id' in obj 
    && 'name' in obj;
}
```

#### Optional Chaining & Nullish Coalescing

```typescript
// ✅ GUT: Optional Chaining
const snapshotCount = repository?.snapshots?.length ?? 0;

// ❌ SCHLECHT: Lange if-Ketten
let snapshotCount = 0;
if (repository) {
  if (repository.snapshots) {
    snapshotCount = repository.snapshots.length;
  }
}

// ✅ GUT: Nullish Coalescing
const timeout = config.timeout ?? DEFAULT_TIMEOUT;

// ❌ SCHLECHT: OR-Operator (fängt auch 0, false, '' ab!)
const timeout = config.timeout || DEFAULT_TIMEOUT;
```

#### Error Handling

```typescript
// ✅ GUT: Eigene Error-Klassen
export class BackupError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'BackupError';
  }
}

// Verwendung
try {
  await runBackup(jobId);
} catch (error) {
  if (error instanceof BackupError) {
    // Spezifische Behandlung
    console.error(`Backup-Fehler [${error.code}]:`, error.message);
    showToast('error', 'Backup fehlgeschlagen', error.message);
  } else {
    // Generische Behandlung
    console.error('Unerwarteter Fehler:', error);
    showToast('error', 'Fehler', 'Ein unerwarteter Fehler ist aufgetreten');
  }
}

// ✅ GUT: Result-Type-Pattern (alternativ)
type Result<T, E = Error> = 
  | { ok: true; value: T }
  | { ok: false; error: E };

async function runBackup(jobId: string): Promise<Result<BackupResult>> {
  try {
    const result = await executeBackup(jobId);
    return { ok: true, value: result };
  } catch (error) {
    return { ok: false, error: error as BackupError };
  }
}

// Verwendung
const result = await runBackup(jobId);
if (result.ok) {
  console.log('Erfolg:', result.value);
} else {
  console.error('Fehler:', result.error);
}
```

### Svelte 5 Conventions

#### Component Structure

```svelte
<script lang="ts">
  /**
   * Dialog zum Erstellen eines neuen Backup-Jobs.
   * 
   * Zeigt ein mehrstufiges Formular mit Tabs für:
   * - Allgemeine Einstellungen
   * - Pfade und Exclusions
   * - Zeitplan-Konfiguration
   */
  
  // 1. Imports
  import { createEventDispatcher } from 'svelte';
  import type { BackupJob } from '$lib/types';
  import Button from '$lib/components/shared/Button.svelte';
  
  // 2. Props (mit $props() in Svelte 5)
  let { 
    isOpen = $bindable(false),
    initialData = undefined 
  }: {
    isOpen?: boolean;
    initialData?: Partial<BackupJob>;
  } = $props();
  
  // 3. State
  let activeTab = $state(0);
  let jobName = $state(initialData?.name ?? '');
  let sourcePaths = $state<string[]>(initialData?.sourcePaths ?? []);
  
  // 4. Derived State
  let isValid = $derived(
    jobName.trim().length > 0 && sourcePaths.length > 0
  );
  
  // 5. Event Dispatcher
  const dispatch = createEventDispatcher<{
    save: BackupJob;
    cancel: void;
  }>();
  
  // 6. Functions
  function handleSave() {
    if (!isValid) return;
    
    const job: BackupJob = {
      id: crypto.randomUUID(),
      name: jobName,
      sourcePaths,
      // ... weitere Felder
    };
    
    dispatch('save', job);
    isOpen = false;
  }
  
  function handleCancel() {
    dispatch('cancel');
    isOpen = false;
  }
</script>

<!-- 7. Template -->
{#if isOpen}
  <dialog open>
    <h2>Neuen Backup-Job erstellen</h2>
    
    <form on:submit|preventDefault={handleSave}>
      <!-- Form-Inhalt -->
      
      <div class="actions">
        <Button 
          variant="secondary" 
          on:click={handleCancel}
        >
          Abbrechen
        </Button>
        
        <Button 
          variant="primary" 
          type="submit"
          disabled={!isValid}
        >
          Speichern
        </Button>
      </div>
    </form>
  </dialog>
{/if}

<!-- 8. Styles -->
<style>
  dialog {
    padding: 1.5rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }
  
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
  }
</style>
```

#### Reactive Statements (Svelte 5)

```svelte
<script lang="ts">
  // Svelte 5: $state() für reactive Variablen
  let count = $state(0);
  let name = $state('');
  
  // Svelte 5: $derived() für berechnete Werte
  let doubled = $derived(count * 2);
  let greeting = $derived(`Hallo, ${name}!`);
  
  // Svelte 5: $effect() für Side Effects
  $effect(() => {
    console.log('Count changed:', count);
    
    // Cleanup (optional)
    return () => {
      console.log('Cleanup');
    };
  });
  
  // Svelte 5: $props() für Component Props
  let { 
    initialCount = 0,
    onUpdate 
  }: {
    initialCount?: number;
    onUpdate?: (value: number) => void;
  } = $props();
  
  // Initialisierung
  $effect(() => {
    count = initialCount;
  });
</script>
```

---

## 🦀 Rust Conventions

### Naming Conventions

```rust
// Variablen: snake_case
let backup_result = run_backup();
let snapshot_count = 42;
let is_running = false;

// Konstanten: SCREAMING_SNAKE_CASE
const MAX_CONCURRENT_BACKUPS: usize = 3;
const DEFAULT_BUFFER_SIZE: usize = 8192;

// Funktionen: snake_case
fn create_backup() {}
fn validate_repository() {}
async fn fetch_snapshots() {}

// Structs: PascalCase
struct BackupJob {
    id: String,
    name: String,
}

struct SnapshotInfo {
    id: String,
    timestamp: DateTime<Utc>,
}

// Enums: PascalCase (Variants auch PascalCase)
enum BackupStatus {
    Idle,
    Running,
    Completed,
    Failed(String),
}

// Traits: PascalCase
trait BackupExecutor {
    fn execute(&self) -> Result<BackupResult>;
}

// Module: snake_case
mod backup_service;
mod snapshot_parser;
mod config_manager;

// Type Aliases: PascalCase
type Result<T> = std::result::Result<T, BackupError>;
type SnapshotId = String;
```

### File Naming

```
Rust Files: snake_case.rs
  - backup_service.rs
  - snapshot_parser.rs
  - config_manager.rs

Modules: snake_case/
  - backup/
    - mod.rs
    - executor.rs
    - validator.rs
```

### Rust Best Practices

#### Error Handling

```rust
use thiserror::Error;

// ✅ GUT: Eigene Error-Types mit thiserror
#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Repository nicht gefunden: {0}")]
    RepositoryNotFound(String),
    
    #[error("Authentifizierung fehlgeschlagen")]
    AuthenticationFailed,
    
    #[error("Backup wurde abgebrochen")]
    Cancelled,
    
    #[error("IO-Fehler: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON-Parse-Fehler: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Interner Fehler: {0}")]
    Internal(String),
}

// ✅ GUT: Result-Type verwenden
pub type Result<T> = std::result::Result<T, BackupError>;

// ✅ GUT: ? Operator nutzen
fn run_backup(job_id: &str) -> Result<BackupResult> {
    let config = load_config()?;  // Fehler wird propagiert
    let repo = open_repository(&config.repo_path)?;
    let result = repo.backup(&config.source_paths)?;
    Ok(result)
}

// ❌ SCHLECHT: unwrap() ohne guten Grund
fn bad_example() {
    let config = load_config().unwrap();  // Panic bei Fehler!
}

// ✅ GUT: unwrap() nur mit Kommentar warum es sicher ist
fn good_example() {
    // Sicher: UUID ist immer gültig
    let id = Uuid::parse_str("...").unwrap();
    
    // Oder besser: expect() mit Nachricht
    let id = Uuid::parse_str("...")
        .expect("Hart-codierte UUID muss gültig sein");
}
```

#### Ownership & Borrowing

```rust
// ✅ GUT: Borrowing bevorzugen
fn process_job(job: &BackupJob) -> Result<()> {
    println!("Processing: {}", job.name);
    Ok(())
}

// ❌ SCHLECHT: Ownership übernehmen wenn nicht nötig
fn process_job_bad(job: BackupJob) -> Result<()> {
    println!("Processing: {}", job.name);
    // job wird hier gedropped!
    Ok(())
}

// ✅ GUT: Mutable Borrowing wenn Änderung nötig
fn update_job_status(job: &mut BackupJob, status: BackupStatus) {
    job.status = status;
}

// ✅ GUT: Clone nur wenn nötig (und dokumentiert)
fn start_backup(job: &BackupJob) -> tokio::task::JoinHandle<Result<()>> {
    // Clone nötig da Job in separaten Thread verschoben wird
    let job = job.clone();
    
    tokio::spawn(async move {
        execute_backup(&job).await
    })
}
```

#### Async/Await

```rust
// ✅ GUT: Async Functions
async fn fetch_snapshots(repo_path: &str) -> Result<Vec<Snapshot>> {
    let output = tokio::process::Command::new("rustic")
        .args(&["snapshots", "-r", repo_path, "--json"])
        .output()
        .await?;
    
    let snapshots = serde_json::from_slice(&output.stdout)?;
    Ok(snapshots)
}

// ✅ GUT: Parallele Ausführung mit tokio::join!
async fn load_all_data(repo_path: &str) -> Result<(Vec<Snapshot>, RepoInfo)> {
    let snapshots_future = fetch_snapshots(repo_path);
    let info_future = fetch_repo_info(repo_path);
    
    let (snapshots, info) = tokio::try_join!(snapshots_future, info_future)?;
    
    Ok((snapshots, info))
}

// ✅ GUT: Streams für große Datenmengen
use futures::stream::{self, StreamExt};

async fn process_snapshots_batch(snapshot_ids: Vec<String>) -> Result<()> {
    stream::iter(snapshot_ids)
        .map(|id| async move {
            process_snapshot(&id).await
        })
        .buffer_unordered(3)  // Max 3 parallele Tasks
        .collect::<Vec<_>>()
        .await;
    
    Ok(())
}
```

#### Tauri Command Pattern

```rust
/// Tauri Command für Snapshot-Auflistung.
///
/// Dieser Command wird vom Frontend via `invoke('list_snapshots')` aufgerufen.
///
/// # Security
///
/// - Passwort wird nicht geloggt
/// - Repository-Pfad wird validiert
/// 
/// # Errors
///
/// Returns error string wenn:
/// - Repository nicht existiert
/// - Authentifizierung fehlschlägt
/// - rustic Fehler zurückgibt
#[tauri::command]
pub async fn list_snapshots(
    repo_path: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SnapshotDto>, String> {
    // Validierung
    if repo_path.is_empty() {
        return Err("Repository-Pfad darf nicht leer sein".into());
    }
    
    // Passwort in Env (wird nach Aufruf gelöscht)
    std::env::set_var("RUSTIC_PASSWORD", &password);
    
    // rustic aufrufen
    let output = tokio::process::Command::new("rustic")
        .args(&["snapshots", "-r", &repo_path, "--json"])
        .output()
        .await
        .map_err(|e| format!("Fehler beim Ausführen von rustic: {}", e))?;
    
    // Passwort aus Env entfernen
    std::env::remove_var("RUSTIC_PASSWORD");
    
    // Erfolg prüfen
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Rustic-Fehler: {}", stderr));
    }
    
    // JSON parsen
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshots = serde_json::from_str(&stdout)
        .map_err(|e| format!("JSON-Parse-Fehler: {}", e))?;
    
    Ok(snapshots)
}
```

---

## 📝 Dokumentations-Style

### TSDoc (TypeScript)

```typescript
/**
 * Startet einen Backup-Job und gibt das Ergebnis zurück.
 * 
 * Diese Funktion kommuniziert mit dem Backend via Tauri IPC
 * und zeigt Progress-Updates während der Ausführung an.
 * 
 * @param jobId - Eindeutige ID des auszuführenden Jobs
 * @param options - Optionale Konfigurations-Parameter
 * @param options.dryRun - Wenn true, wird nur simuliert
 * @param options.verbose - Aktiviert detaillierte Logs
 * @returns Promise mit dem Backup-Ergebnis
 * @throws {BackupError} Wenn Backup fehlschlägt
 * @throws {ValidationError} Wenn Job-Konfiguration ungültig
 * 
 * @example
 * ```typescript
 * const result = await runBackup('job-123', { dryRun: true });
 * console.log(`Backup abgeschlossen: ${result.snapshotId}`);
 * ```
 * 
 * @see {@link BackupJob} für Job-Konfiguration
 * @see {@link BackupResult} für Ergebnis-Format
 */
export async function runBackup(
  jobId: string,
  options?: BackupOptions
): Promise<BackupResult> {
  // Implementation
}
```

### Rustdoc

```rust
/// Führt einen Backup-Job aus und gibt das Ergebnis zurück.
///
/// Diese Funktion ruft rustic als Subprocess auf und parst die JSON-Ausgabe.
/// Der Backup-Prozess läuft asynchron und kann mit einem CancellationToken
/// abgebrochen werden.
///
/// # Arguments
///
/// * `job_id` - Eindeutige ID des auszuführenden Jobs
/// * `password` - Repository-Passwort für Authentifizierung
/// * `cancellation_token` - Optional: Token zum Abbrechen des Backups
///
/// # Returns
///
/// `Result<BackupResult, BackupError>` - Erfolg mit Details oder Fehler
///
/// # Errors
///
/// Diese Funktion gibt Fehler zurück wenn:
/// - Repository nicht existiert ([`BackupError::RepositoryNotFound`])
/// - Authentifizierung fehlschlägt ([`BackupError::AuthenticationFailed`])
/// - Backup abgebrochen wurde ([`BackupError::Cancelled`])
/// - Ein IO-Fehler auftritt ([`BackupError::IoError`])
///
/// # Examples
///
/// ```rust
/// use rustic_gui::backup::run_backup;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let result = run_backup("job-123", "secret", None).await?;
///     println!("Backup abgeschlossen: {:?}", result);
///     Ok(())
/// }
/// ```
///
/// # Panics
///
/// Diese Funktion panict nicht.
///
/// # Safety
///
/// Passwort wird in Umgebungsvariable gesetzt und nach Verwendung gelöscht.
/// Es wird nie geloggt oder anderweitig gespeichert.
#[tauri::command]
pub async fn run_backup(
    job_id: String,
    password: String,
    cancellation_token: Option<CancellationToken>,
) -> Result<BackupResult, BackupError> {
    // Implementation
}
```

### Inline-Kommentare

```typescript
// ✅ GUT: Erklärt WARUM, nicht WAS
// Wir verwenden ein Set statt Array für O(1) Lookup,
// da wir häufig prüfen müssen ob ein Pfad bereits existiert
const excludedPaths = new Set<string>();

// ❌ SCHLECHT: Redundant, erklärt nur WAS
// Erstellt ein neues Set
const excludedPaths = new Set<string>();

// ✅ GUT: Erklärt nicht-offensichtlichen Trick
// Timeout nötig da rustic manchmal verzögert Ausgabe liefert
await new Promise(resolve => setTimeout(resolve, 100));

// ✅ GUT: Warnung vor Fallstrick
// ACHTUNG: Darf nicht parallel ausgeführt werden, da Repository locked wird!
async function pruneRepository() {}

// ✅ GUT: TODO mit Details
// TODO: Implementiere retry-logic für transiente Netzwerk-Fehler
//       → Issue #234
async function uploadToCloud() {}
```

---

## 🎨 Code-Formatierung

### Prettier (TypeScript/Svelte)

```json
// .prettierrc
{
  "semi": true,
  "singleQuote": true,
  "trailingComma": "es5",
  "printWidth": 100,
  "tabWidth": 2,
  "useTabs": false,
  "arrowParens": "avoid",
  "endOfLine": "lf",
  
  "plugins": ["prettier-plugin-svelte"],
  "overrides": [
    {
      "files": "*.svelte",
      "options": {
        "parser": "svelte",
        "svelteSortOrder": "scripts-markup-styles",
        "svelteStrictMode": true,
        "svelteIndentScriptAndStyle": true
      }
    }
  ]
}
```

### rustfmt (Rust)

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
use_small_heuristics = "Max"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
wrap_comments = true
format_code_in_doc_comments = true
normalize_comments = true
```

---

## 🚫 Anti-Patterns

### Was vermeiden?

#### TypeScript/Svelte

```typescript
// ❌ SCHLECHT: any verwenden
function process(data: any) {
  return data.something;
}

// ✅ GUT: Korrekte Typen
function process(data: BackupJob) {
  return data.name;
}

// ❌ SCHLECHT: Non-null assertion ohne Prüfung
const name = repository!.name;

// ✅ GUT: Optional chaining
const name = repository?.name ?? 'Unknown';

// ❌ SCHLECHT: Nested callbacks
fetchRepo(id, (repo) => {
  fetchSnapshots(repo, (snapshots) => {
    processSnapshots(snapshots, (result) => {
      // ...
    });
  });
});

// ✅ GUT: async/await
const repo = await fetchRepo(id);
const snapshots = await fetchSnapshots(repo);
const result = await processSnapshots(snapshots);

// ❌ SCHLECHT: Mutation von Props in Svelte
export let items = [];
function addItem() {
  items.push(newItem);  // Mutiert Parent-State!
}

// ✅ GUT: Event dispatch
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();

function addItem() {
  dispatch('add', newItem);
}
```

#### Rust

```rust
// ❌ SCHLECHT: unwrap() ohne Grund
fn bad_example() {
    let config = load_config().unwrap();
}

// ✅ GUT: Fehler propagieren
fn good_example() -> Result<()> {
    let config = load_config()?;
    Ok(())
}

// ❌ SCHLECHT: Clones überall
fn process_items(items: &Vec<Item>) {
    for item in items {
        let cloned = item.clone();  // Unnötig!
        process(cloned);
    }
}

// ✅ GUT: Borrowing nutzen
fn process_items(items: &[Item]) {
    for item in items {
        process(item);  // Borrow statt Clone
    }
}

// ❌ SCHLECHT: String-Fehler
fn bad_error() -> Result<(), String> {
    Err("Something went wrong".into())
}

// ✅ GUT: Eigener Error-Type
fn good_error() -> Result<(), BackupError> {
    Err(BackupError::Internal("Something went wrong".into()))
}

// ❌ SCHLECHT: Blocking in async
async fn bad_async() {
    std::thread::sleep(Duration::from_secs(1));  // Blockiert!
}

// ✅ GUT: Async sleep
async fn good_async() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

---

## ✅ Code-Review-Checkliste

### TypeScript/Svelte

- [ ] Alle Variablen/Funktionen haben korrekte Typen (kein `any`)
- [ ] Fehler werden korrekt behandelt (try-catch)
- [ ] Keine `!` non-null assertions ohne Prüfung
- [ ] Optional chaining (`?.`) und nullish coalescing (`??`) genutzt
- [ ] Async/await statt Callbacks
- [ ] Props werden nicht mutiert (Svelte)
- [ ] Events werden korrekt dispatched
- [ ] Kommentare auf Deutsch, Code auf Englisch
- [ ] ESLint zeigt keine Warnungen
- [ ] Prettier-formatiert

### Rust

- [ ] Keine `unwrap()` ohne guten Grund (oder `expect()` mit Erklärung)
- [ ] Fehler werden mit `?` propagiert
- [ ] Eigene Error-Types statt String
- [ ] Borrowing bevorzugt über Cloning
- [ ] Async/await korrekt verwendet (kein Blocking)
- [ ] Ownership/Lifetime-Regeln befolgt
- [ ] Kommentare auf Deutsch, Code auf Englisch
- [ ] Clippy zeigt keine Warnungen
- [ ] rustfmt-formatiert

### Allgemein

- [ ] Funktionen sind klein und fokussiert (Single Responsibility)
- [ ] DRY-Prinzip: Keine Code-Duplikation
- [ ] KISS-Prinzip: Einfache Lösungen bevorzugt
- [ ] Docstrings für öffentliche API
- [ ] Inline-Kommentare für komplexe Logik
- [ ] Tests vorhanden (siehe testing.instructions.md)
- [ ] UI folgt Mockups (siehe mockups/*)

---

## 🎓 Best Practices Zusammenfassung

### Code-Qualität

1. **Type-Safety first**: Nutze TypeScript strict mode und Rust's Type-System
2. **Explicit over implicit**: Lieber verbose als unklar
3. **Fail fast**: Validiere früh, gib klare Fehler zurück
4. **DRY**: Wiederverwendbare Funktionen/Komponenten extrahieren
5. **KISS**: Einfache Lösungen bevorzugen

### Performance

1. **Lazy loading**: Lade nur was nötig ist
2. **Memoization**: Cache teure Berechnungen
3. **Debouncing**: Bei User-Input (z.B. Search)
4. **Virtual scrolling**: Bei großen Listen
5. **Parallele Ausführung**: Nutze async/await, tokio::join!

### Security

1. **Keine Passwörter in Logs**
2. **Input-Validierung** immer
3. **Escape User-Input** in UI
4. **Keine SQL-Injection** (wenn DB)
5. **Keine Command-Injection** (bei Shell-Calls)

### Wartbarkeit

1. **Kleine Funktionen**: Max 50 Zeilen
2. **Kleine Dateien**: Max 300 Zeilen
3. **Klare Namen**: Selbst-dokumentierend
4. **Kommentare für "Warum"**: Nicht "Was"
5. **Tests schreiben**: Siehe testing.instructions.md

---

## 📚 Style-Guide-Referenzen

### TypeScript/JavaScript

- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
- [Google TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
- [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)

### Svelte

- [Svelte Style Guide](https://github.com/sveltejs/svelte/blob/master/STYLE.md)
- [Svelte Best Practices](https://svelte.dev/docs/svelte/best-practices)

### Rust

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

---

**Version**: 1.0  
**Letzte Aktualisierung**: 2025-10-26