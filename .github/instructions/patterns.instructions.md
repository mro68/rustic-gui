# Patterns Instructions

> Wiederverwendbare Code-Patterns und Lösungen für Rustic GUI

---

## 🔄 Repository-Switching-Pattern

### Problem
Sicheres Wechseln zwischen verschiedenen Repositories.

### Lösung

```rust
#[tauri::command]
pub async fn switch_repository(
    repo_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<RepositoryInfo, String> {
    // 1. Altes Repository schließen
    {
        let mut current = state.current_repo.lock().unwrap();
        if let Some(old_repo) = current.take() {
            drop(old_repo); // Automatisch cleanup via Drop
            tracing::debug!("Altes Repository geschlossen");
        }
    }
    
    // 2. Repo-Config laden
    let config = load_repository_config(&repo_id).await
        .map_err(|e| format!("Config-Fehler: {}", e))?;
    
    // 3. Neues Repository öffnen
    let repo = open_repository(&config.path, &password)
        .await
        .map_err(|e| format!("Öffnen fehlgeschlagen: {}", e))?;
    
    // 4. Repository-Info für Frontend
    let info = RepositoryInfo {
        id: repo_id.clone(),
        path: config.path.clone(),
        backend_type: detect_backend_type(&config.path),
        snapshot_count: repo.get_snapshot_count()?,
        total_size: repo.get_total_size()?,
    };
    
    // 5. In State speichern
    *state.current_repo.lock().unwrap() = Some(repo);
    
    tracing::info!("Repository gewechselt: {}", repo_id);
    
    Ok(info)
}
```

---

## 📊 Batch-Operations mit Progress

### Problem
Mehrere Operationen parallel ausführen mit Progress-Updates.

### Lösung

```rust
use futures::stream::{self, StreamExt};

#[tauri::command]
pub async fn delete_snapshots_batch(
    snapshot_ids: Vec<String>,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<usize, String> {
    let repo = state.get_current_repo()?;
    let total = snapshot_ids.len();
    
    // Stream für parallele Verarbeitung (max 3 gleichzeitig)
    let results = stream::iter(snapshot_ids)
        .enumerate()
        .map(|(idx, snap_id)| {
            let repo = repo.clone();
            let app_handle = app_handle.clone();
            async move {
                let result = repo.forget_snapshot(&snap_id).await;
                
                // Progress-Event senden
                app_handle.emit_all("delete-progress", json!({
                    "completed": idx + 1,
                    "total": total,
                })).ok();
                
                result
            }
        })
        .buffer_unordered(3)
        .collect::<Vec<_>>()
        .await;
    
    // Zähle Erfolge
    let successful = results.iter().filter(|r| r.is_ok()).count();
    
    tracing::info!("Gelöscht: {} von {} Snapshots", successful, total);
    
    Ok(successful)
}
```

---

## 🌳 FileTree für Restore-Browser

### Problem
Hierarchischer Dateibaum mit Lazy-Loading.

### Lösung

**Backend:**

```rust
#[derive(Serialize)]
struct FileTreeNode {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    children: Option<Vec<FileTreeNode>>,
}

#[tauri::command]
pub async fn get_snapshot_file_tree(
    snapshot_id: String,
    path_prefix: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<FileTreeNode>, String> {
    let repo = state.get_current_repo()?;
    
    // Snapshot laden
    let snapshot = repo.get_snapshot(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;
    
    // Tree aus Snapshot laden
    let tree = repo.node_from_snapshot(&snapshot)
        .map_err(|e| format!("Tree laden fehlgeschlagen: {}", e))?;
    
    // Pfad-Filter anwenden
    let nodes = if let Some(prefix) = path_prefix {
        repo.get_node_at_path(&tree, &prefix)
            .map_err(|e| format!("Pfad nicht gefunden: {}", e))?
            .nodes
    } else {
        tree.nodes
    };
    
    // In Frontend-Format konvertieren
    let file_nodes: Vec<FileTreeNode> = nodes
        .into_iter()
        .map(|node| FileTreeNode {
            name: node.name().to_string(),
            path: node.path().to_string(),
            is_dir: matches!(node.node_type(), NodeType::Dir),
            size: node.meta().size,
            children: None, // Lazy-Loading
        })
        .collect();
    
    Ok(file_nodes)
}
```

**Frontend:**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { FileTreeNode } from '$lib/types';
  
  let { snapshotId }: { snapshotId: string } = $props();
  
  let nodes = $state<FileTreeNode[]>([]);
  let expandedPaths = $state(new Set<string>());
  let loadingPaths = $state(new Set<string>());
  
  async function loadChildren(path: string): Promise<FileTreeNode[]> {
    return await invoke('get_snapshot_file_tree', {
      snapshotId,
      pathPrefix: path,
    });
  }
  
  async function toggleExpand(node: FileTreeNode) {
    if (!node.isDir) return;
    
    if (expandedPaths.has(node.path)) {
      expandedPaths.delete(node.path);
      expandedPaths = expandedPaths;
    } else {
      if (!node.children) {
        loadingPaths.add(node.path);
        loadingPaths = loadingPaths;
        
        try {
          node.children = await loadChildren(node.path);
        } finally {
          loadingPaths.delete(node.path);
          loadingPaths = loadingPaths;
        }
      }
      
      expandedPaths.add(node.path);
      expandedPaths = expandedPaths;
    }
  }
</script>
```

---

## ⏰ Scheduled Backups mit Cron

### Problem
Zeitgesteuerte Backup-Jobs.

### Lösung

```rust
use tokio_cron_scheduler::{JobScheduler, Job};

pub struct BackupScheduler {
    scheduler: JobScheduler,
    jobs: HashMap<String, uuid::Uuid>,
}

impl BackupScheduler {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
            jobs: HashMap::new(),
        })
    }
    
    pub async fn add_job<F>(
        &mut self,
        job_id: String,
        cron_expression: &str,
        callback: F,
    ) -> Result<()>
    where
        F: Fn() -> BoxFuture<'static, ()> + Send + Sync + 'static,
    {
        let job = Job::new_async(cron_expression, move |_uuid, _lock| {
            Box::pin(callback())
        })?;
        
        let uuid = self.scheduler.add(job).await?;
        self.jobs.insert(job_id, uuid);
        
        Ok(())
    }
}

#[tauri::command]
pub async fn schedule_backup_job(
    job_id: String,
    cron_expression: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let mut scheduler = state.scheduler.lock().await;
    
    let job_id_clone = job_id.clone();
    let app_handle_clone = app_handle.clone();
    
    scheduler.add_job(
        job_id.clone(),
        &cron_expression,
        move || {
            let job_id = job_id_clone.clone();
            let app_handle = app_handle_clone.clone();
            
            Box::pin(async move {
                tracing::info!("Scheduled Backup: {}", job_id);
                
                app_handle.emit_all("scheduled-backup-started", json!({
                    "job_id": job_id,
                })).ok();
                
                // Backup ausführen
                if let Err(e) = run_backup_internal(&job_id).await {
                    app_handle.emit_all("scheduled-backup-failed", json!({
                        "job_id": job_id,
                        "error": e.to_string(),
                    })).ok();
                }
            })
        },
    ).await
    .map_err(|e| format!("Scheduling fehlgeschlagen: {}", e))?;
    
    Ok(())
}
```

---

## 💾 Config-Persistence mit TOML

### Problem
App-Konfiguration persistent speichern.

### Lösung

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub repositories: Vec<RepositoryConfig>,
    pub backup_jobs: Vec<BackupJobConfig>,
    pub settings: AppSettings,
}

impl AppConfig {
    /// Lädt Config von Disk.
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&config_path)?;
        let config: Self = toml::from_str(&content)?;
        
        Ok(config)
    }
    
    /// Speichert Config auf Disk.
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, toml)?;
        
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Config-Dir nicht gefunden"))?;
        
        Ok(config_dir.join("rustic-gui").join("config.toml"))
    }
}
```

---

## 🔔 Toast-Notification-Pattern

### Problem
Einheitliche Notifications im UI.

### Lösung

**Store:**

```typescript
// src/lib/stores/toasts.ts
import { writable } from 'svelte/store';

interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message?: string;
  duration?: number;
}

const _toasts = writable<Toast[]>([]);

export const toasts = { subscribe: _toasts.subscribe };

export function showToast(
  type: Toast['type'],
  title: string,
  message?: string,
  duration = 3000
): void {
  const id = crypto.randomUUID();
  
  const toast: Toast = { id, type, title, message, duration };
  
  _toasts.update(t => [...t, toast]);
  
  if (duration > 0) {
    setTimeout(() => removeToast(id), duration);
  }
}

export function removeToast(id: string): void {
  _toasts.update(t => t.filter(toast => toast.id !== id));
}
```

**Component:**

```svelte
<!-- src/lib/components/ToastContainer.svelte -->
<script lang="ts">
  import { toasts, removeToast } from '$lib/stores/toasts';
  import Toast from './shared/Toast.svelte';
</script>

<div class="toast-container">
  {#each $toasts as toast (toast.id)}
    <Toast
      {...toast}
      onClose={() => removeToast(toast.id)}
    />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 1rem;
    right: 1rem;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
</style>
```

**Usage:**

```typescript
import { showToast } from '$lib/stores/toasts';

try {
  await runBackup(jobId);
  showToast('success', 'Backup abgeschlossen');
} catch (error) {
  showToast('error', 'Backup fehlgeschlagen', error.message);
}
```

---

## 🔄 Retry-Logic für Network-Fehler

### Problem
Transiente Netzwerk-Fehler automatisch wiederholen.

### Lösung

```rust
use tokio::time::{sleep, Duration};

async fn with_retry<F, T>(
    operation: F,
    max_retries: usize,
) -> Result<T>
where
    F: Fn() -> BoxFuture<'static, Result<T>>,
{
    let mut attempts = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_retries && is_retriable(&e) => {
                attempts += 1;
                let delay = Duration::from_secs(2u64.pow(attempts as u32));
                
                tracing::warn!(
                    "Versuch {}/{} fehlgeschlagen: {}. Retry in {:?}",
                    attempts,
                    max_retries,
                    e,
                    delay
                );
                
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

fn is_retriable(error: &anyhow::Error) -> bool {
    // Prüfe ob Fehler transient ist (Netzwerk, Timeout, etc.)
    error.to_string().contains("timeout") 
        || error.to_string().contains("connection")
        || error.to_string().contains("network")
}

// Usage
let snapshots = with_retry(
    || Box::pin(list_snapshots(&repo)),
    3
).await?;
```

---

## 🎣 Custom Svelte Hook Pattern

### Problem
Wiederverwendbare Logik in Components.

### Lösung

```typescript
// src/lib/hooks/use-backup-progress.ts
import { onDestroy } from 'svelte';
import { onBackupProgress } from '$lib/api/backup';
import type { BackupProgress } from '$lib/types';

export function useBackupProgress(jobId: string) {
  let progress = $state<BackupProgress>({
    filesProcessed: 0,
    filesTotal: 0,
    bytesProcessed: 0,
    bytesTotal: 0,
  });
  
  let unlisten: (() => void) | null = null;
  
  async function start() {
    unlisten = await onBackupProgress(jobId, (p) => {
      progress = p;
    });
  }
  
  function stop() {
    unlisten?.();
    unlisten = null;
  }
  
  onDestroy(() => {
    stop();
  });
  
  return {
    get progress() { return progress; },
    start,
    stop,
  };
}
```

**Usage:**

```svelte
<script lang="ts">
  import { useBackupProgress } from '$lib/hooks/use-backup-progress';
  
  let { jobId }: { jobId: string } = $props();
  
  const { progress, start, stop } = useBackupProgress(jobId);
  
  onMount(() => {
    start();
  });
</script>

<ProgressBar
  value={progress.bytesProcessed}
  max={progress.bytesTotal}
/>
```

---

## 🔐 Secure Password Input Pattern

### Problem
Passwort-Eingabe mit Validation und Visibility-Toggle.

### Lösung

```svelte
<!-- src/lib/components/shared/PasswordInput.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  let {
    value = $bindable(''),
    placeholder = 'Passwort eingeben',
    required = false,
    minLength = 0,
    validate = undefined
  }: {
    value?: string;
    placeholder?: string;
    required?: boolean;
    minLength?: number;
    validate?: (password: string) => string | null;
  } = $props();
  
  let isVisible = $state(false);
  let error = $state<string | null>(null);
  
  const dispatch = createEventDispatcher<{
    change: string;
  }>();
  
  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    value = target.value;
    
    // Validation
    if (required && value.length === 0) {
      error = 'Passwort erforderlich';
    } else if (minLength > 0 && value.length < minLength) {
      error = `Mindestens ${minLength} Zeichen`;
    } else if (validate) {
      error = validate(value);
    } else {
      error = null;
    }
    
    dispatch('change', value);
  }
  
  function toggleVisibility() {
    isVisible = !isVisible;
  }
</script>

<div class="password-input">
  <div class="input-wrapper">
    <input
      type={isVisible ? 'text' : 'password'}
      value={value}
      {placeholder}
      {required}
      on:input={handleInput}
      class:error={error !== null}
    />
    
    <button
      type="button"
      class="toggle-visibility"
      on:click={toggleVisibility}
      aria-label={isVisible ? 'Passwort verbergen' : 'Passwort anzeigen'}
    >
      {isVisible ? '👁️' : '👁️‍🗨️'}
    </button>
  </div>
  
  {#if error}
    <span class="error-message">{error}</span>
  {/if}
</div>

<style>
  .password-input {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .input-wrapper {
    position: relative;
  }
  
  input {
    width: 100%;
    padding: 0.5rem 2.5rem 0.5rem 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
  }
  
  input.error {
    border-color: var(--color-error);
  }
  
  .toggle-visibility {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
  }
  
  .error-message {
    color: var(--color-error);
    font-size: 0.875rem;
  }
</style>
```

---

## 📋 Form-Validation-Pattern

### Problem
Einheitliche Form-Validierung.

### Lösung

```typescript
// src/lib/utils/validation.ts

export interface ValidationRule<T> {
  validate: (value: T) => boolean;
  message: string;
}

export class FormValidator<T extends Record<string, any>> {
  private rules: Map<keyof T, ValidationRule<any>[]> = new Map();
  private errors: Map<keyof T, string> = new Map();
  
  addRule<K extends keyof T>(
    field: K,
    validate: (value: T[K]) => boolean,
    message: string
  ): this {
    if (!this.rules.has(field)) {
      this.rules.set(field, []);
    }
    this.rules.get(field)!.push({ validate, message });
    return this;
  }
  
  validate(data: T): boolean {
    this.errors.clear();
    
    for (const [field, rules] of this.rules) {
      const value = data[field];
      
      for (const rule of rules) {
        if (!rule.validate(value)) {
          this.errors.set(field, rule.message);
          break;
        }
      }
    }
    
    return this.errors.size === 0;
  }
  
  getError(field: keyof T): string | null {
    return this.errors.get(field) ?? null;
  }
  
  hasErrors(): boolean {
    return this.errors.size > 0;
  }
}

// Vordefinierte Validatoren
export const validators = {
  required: (message = 'Feld erforderlich') => ({
    validate: (value: any) => {
      if (typeof value === 'string') return value.trim().length > 0;
      if (Array.isArray(value)) return value.length > 0;
      return value != null;
    },
    message,
  }),
  
  minLength: (min: number, message?: string) => ({
    validate: (value: string) => value.length >= min,
    message: message ?? `Mindestens ${min} Zeichen`,
  }),
  
  maxLength: (max: number, message?: string) => ({
    validate: (value: string) => value.length <= max,
    message: message ?? `Maximal ${max} Zeichen`,
  }),
  
  pattern: (regex: RegExp, message: string) => ({
    validate: (value: string) => regex.test(value),
    message,
  }),
  
  email: () => ({
    validate: (value: string) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value),
    message: 'Ungültige E-Mail-Adresse',
  }),
};
```

**Usage:**

```svelte
<script lang="ts">
  import { FormValidator, validators } from '$lib/utils/validation';
  
  interface JobForm {
    name: string;
    sourcePaths: string[];
    schedule: string;
  }
  
  let formData = $state<JobForm>({
    name: '',
    sourcePaths: [],
    schedule: '',
  });
  
  const validator = new FormValidator<JobForm>()
    .addRule('name', ...validators.required('Job-Name erforderlich'))
    .addRule('name', ...validators.minLength(3))
    .addRule('sourcePaths', ...validators.required('Mindestens ein Pfad'))
    .addRule('schedule', (v) => !v || isValidCron(v), 'Ungültiger Cron-Ausdruck');
  
  function handleSubmit() {
    if (validator.validate(formData)) {
      // Form ist valid
      saveJob(formData);
    } else {
      // Zeige Errors
      console.log('Validation errors:', validator.hasErrors());
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="form-group">
    <label for="name">Job-Name</label>
    <input
      id="name"
      bind:value={formData.name}
      class:error={validator.getError('name')}
    />
    {#if validator.getError('name')}
      <span class="error">{validator.getError('name')}</span>
    {/if}
  </div>
  
  <button type="submit">Speichern</button>
</form>
```

---

## 🎨 Theme-Switching-Pattern

### Problem
Dark/Light Mode dynamisch wechseln.

### Lösung

```typescript
// src/lib/stores/theme.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark' | 'system';

function createThemeStore() {
  const { subscribe, set } = writable<Theme>('system');
  
  function applyTheme(theme: Theme) {
    if (!browser) return;
    
    let effectiveTheme: 'light' | 'dark';
    
    if (theme === 'system') {
      effectiveTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
    } else {
      effectiveTheme = theme;
    }
    
    document.documentElement.setAttribute('data-theme', effectiveTheme);
    localStorage.setItem('theme', theme);
  }
  
  function init() {
    if (!browser) return;
    
    const stored = localStorage.getItem('theme') as Theme | null;
    const theme = stored ?? 'system';
    
    set(theme);
    applyTheme(theme);
    
    // System-Theme-Changes beobachten
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', () => {
      const currentTheme = localStorage.getItem('theme') as Theme;
      if (currentTheme === 'system') {
        applyTheme('system');
      }
    });
  }
  
  function setTheme(theme: Theme) {
    set(theme);
    applyTheme(theme);
  }
  
  return {
    subscribe,
    setTheme,
    init,
  };
}

export const theme = createThemeStore();
```

**CSS:**

```css
/* app.css */
:root[data-theme="light"] {
  --bg-primary: #ffffff;
  --text-primary: #111827;
  /* ... */
}

:root[data-theme="dark"] {
  --bg-primary: #111827;
  --text-primary: #f9fafb;
  /* ... */
}
```

**Usage:**

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { theme } from '$lib/stores/theme';
  
  onMount(() => {
    theme.init();
  });
</script>

<select bind:value={$theme} on:change={(e) => theme.setTheme(e.target.value)}>
  <option value="light">Hell</option>
  <option value="dark">Dunkel</option>
  <option value="system">System</option>
</select>
```

---

## 🔍 Debounced Search Pattern

### Problem
Search-Input ohne zu viele API-Calls.

### Lösung

```typescript
// src/lib/utils/debounce.ts
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  
  return function executedFunction(...args: Parameters<T>) {
    const later = () => {
      timeout = null;
      func(...args);
    };
    
    if (timeout !== null) {
      clearTimeout(timeout);
    }
    
    timeout = setTimeout(later, wait);
  };
}
```

**Usage:**

```svelte
<script lang="ts">
  import { debounce } from '$lib/utils/debounce';
  import { searchSnapshots } from '$lib/api/snapshots';
  
  let searchTerm = $state('');
  let results = $state([]);
  
  const debouncedSearch = debounce(async (term: string) => {
    if (term.length < 2) {
      results = [];
      return;
    }
    
    results = await searchSnapshots(term);
  }, 300);
  
  $effect(() => {
    debouncedSearch(searchTerm);
  });
</script>

<input
  type="search"
  bind:value={searchTerm}
  placeholder="Snapshots durchsuchen..."
/>

{#each results as result}
  <div>{result.name}</div>
{/each}
```

---

## ✅ Pattern-Checkliste

Beim Implementieren neuer Features prüfe:

- [ ] Gibt es bereits ein Pattern dafür?
- [ ] Kann bestehendes Pattern wiederverwendet werden?
- [ ] Ist das Pattern dokumentiert?
- [ ] Neues Pattern zu diesem Dokument hinzufügen?

---

**Version**: 1.0  
**Letzte Aktualisierung**: 2025-10-26