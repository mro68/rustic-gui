# Troubleshooting Instructions

> Problemlösung, Debugging und häufige Fehler für Rustic GUI

---

## 🎯 Troubleshooting-Workflow

### Schritt 1: Problem identifizieren

**Fragen stellen:**
- Was genau funktioniert nicht?
- Wann tritt das Problem auf?
- Ist es reproduzierbar?
- Frontend- oder Backend-Problem?

**Logs sammeln:**
```bash
# Development-Modus mit vollem Logging
RUST_LOG=debug npm run tauri:dev

# Logs beobachten
# Frontend: Browser DevTools Console
# Backend: Terminal-Output
```

### Schritt 2: Logs analysieren

**Frontend-Logs (Browser Console):**
```typescript
// Debug-Logging aktivieren
console.log('[DEBUG]', 'Context:', { variable1, variable2 });
console.error('[ERROR]', 'Fehler:', error);
```

**Backend-Logs (Rust):**
```rust
// Verschiedene Log-Level
tracing::trace!("Sehr detailliert");
tracing::debug!("Debug-Info: {:?}", value);
tracing::info!("Wichtiges Event");
tracing::warn!("Warnung");
tracing::error!("Fehler: {}", error);
```

### Schritt 3: Problem eingrenzen

**Minimal-Reproducer erstellen:**
```typescript
// Frontend: Isoliertes Test-Snippet
async function testIssue() {
  console.log('1. Start');
  try {
    const result = await invoke('problematic_command', { param: 'test' });
    console.log('2. Success:', result);
  } catch (error) {
    console.error('3. Error:', error);
  }
}
```

```rust
// Backend: Isolierter Test
#[tauri::command]
pub async fn test_issue() -> Result<String, String> {
    tracing::debug!("1. Start");
    
    let result = problematic_function()
        .map_err(|e| {
            tracing::error!("2. Error: {}", e);
            e.to_string()
        })?;
    
    tracing::debug!("3. Success");
    Ok(result)
}
```

---

## 🔥 Häufige Probleme & Lösungen

### Problem 1: "Repository nicht gefunden"

**Symptom:**
```
Error: Repository nicht gefunden: /path/to/repo
rustic_backend::error: Repository does not exist
```

**Mögliche Ursachen:**
1. Repository existiert nicht
2. Falscher Pfad
3. Keine Berechtigung
4. Bei Cloud: rclone nicht konfiguriert

**Diagnose:**
```bash
# Prüfe ob Pfad existiert
ls -la /path/to/repo

# Prüfe Berechtigungen
ls -ld /path/to/repo

# Bei Cloud: Prüfe rclone
rclone listremotes
rclone lsd remote:
```

**Lösung 1: Repository initialisieren**
```rust
// Backend
#[tauri::command]
pub async fn init_repository(
    path: String,
    password: String,
) -> Result<(), String> {
    crate::rustic::repository::init(&path, &password, false)
        .await
        .map_err(|e| e.to_string())
}
```

**Lösung 2: Pfad korrigieren**
```typescript
// Frontend: Pfad normalisieren
function normalizePath(path: string): string {
  // Windows: Backslashes -> Forward Slashes
  return path.replace(/\\/g, '/');
}
```

**Lösung 3: Berechtigungen prüfen**
```bash
# Linux/macOS
sudo chown -R $USER:$USER /path/to/repo
chmod -R 755 /path/to/repo

# Windows: Als Administrator ausführen
```

---

### Problem 2: "Repository ist gesperrt"

**Symptom:**
```
Error: Repository locked by another process
Lock file: /path/to/repo/locks/...
```

**Ursache:**
- Vorheriger Backup wurde nicht sauber beendet
- App ist abgestürzt
- Anderer rustic-Prozess läuft

**Diagnose:**
```bash
# Prüfe ob rustic-Prozesse laufen
ps aux | grep rustic

# Prüfe Lock-Dateien
ls -la /path/to/repo/locks/
```

**Lösung 1: Force Unlock (Command)**
```rust
/// ⚠️ NUR verwenden wenn sicher ist, dass kein Prozess läuft!
#[tauri::command]
pub async fn force_unlock_repository(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let repo = state.get_current_repo()?;
    
    tracing::warn!("Force-Unlock wurde angefordert");
    
    // Alle Locks entfernen
    repo.unlock_all()
        .await
        .map_err(|e| format!("Unlock fehlgeschlagen: {}", e))?;
    
    tracing::info!("Repository entsperrt");
    Ok(())
}
```

**Lösung 2: UI mit Bestätigung**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { showToast } from '$lib/stores/toasts';
  
  async function handleForceUnlock() {
    const confirmed = await confirm(
      'Repository entsperren?\n\n' +
      'Nur fortfahren wenn sicher ist, dass kein anderer Backup läuft!\n\n' +
      'Andernfalls können Daten beschädigt werden.'
    );
    
    if (confirmed) {
      try {
        await invoke('force_unlock_repository');
        showToast('success', 'Repository entsperrt');
      } catch (error) {
        showToast('error', 'Fehler', error.message);
      }
    }
  }
</script>

<button on:click={handleForceUnlock} class="btn-danger">
  Repository entsperren (Force)
</button>
```

---

### Problem 3: Passwort-Fehler

**Symptom:**
```
Error: Wrong password or corrupted repository
rustic_core::error: Decryption failed
```

**Mögliche Ursachen:**
1. Falsches Passwort
2. Repository ist korrupt
3. Passwort wird nicht korrekt übergeben

**Diagnose:**
```rust
// Test ob Repository grundsätzlich erreichbar ist
#[tauri::command]
pub async fn test_repository_access(
    path: String,
) -> Result<bool, String> {
    let exists = std::path::Path::new(&path).exists();
    
    if !exists {
        return Err("Pfad existiert nicht".to_string());
    }
    
    // Prüfe ob config-Datei existiert
    let config_exists = std::path::Path::new(&path)
        .join("config")
        .exists();
    
    Ok(config_exists)
}
```

**Lösung 1: Passwort-Übergabe prüfen**
```rust
// Backend: Passwort korrekt setzen
pub async fn open_repository(path: &str, password: &str) -> Result<Repository> {
    // Env-Var setzen (rustic_core nutzt diese)
    std::env::set_var("RUSTIC_PASSWORD", password);
    
    let backend_opts = BackendOptions::default()
        .repository(path);
    
    let repo_opts = RepositoryOptions::default()
        .password(Some(password.to_string())); // Auch direkt übergeben
    
    let repo = Repository::new(&repo_opts, &backend_opts)?;
    
    // WICHTIG: Nach Verwendung entfernen
    std::env::remove_var("RUSTIC_PASSWORD");
    
    Ok(repo)
}
```

**Lösung 2: Passwort-Dialog mit Retry**
```svelte
<script lang="ts">
  let password = $state('');
  let error = $state<string | null>(null);
  let attempts = $state(0);
  const MAX_ATTEMPTS = 3;
  
  async function handleSubmit() {
    error = null;
    
    try {
      await invoke('switch_repository', {
        repoId: currentRepoId,
        password: password,
      });
      
      showToast('success', 'Repository geöffnet');
      close();
      
    } catch (err) {
      attempts++;
      
      if (attempts >= MAX_ATTEMPTS) {
        error = 'Maximale Anzahl Versuche erreicht. Bitte Passwort überprüfen.';
      } else {
        error = `Falsches Passwort (Versuch ${attempts}/${MAX_ATTEMPTS})`;
      }
      
      password = ''; // Reset für neuen Versuch
    }
  }
</script>
```

---

### Problem 4: Hoher Memory-Verbrauch

**Symptom:**
- App verbraucht mehrere GB RAM
- System wird langsam
- Out-of-Memory-Fehler

**Ursache:**
- Große Repositories
- Viele Snapshots geladen
- Keine Garbage Collection

**Diagnose:**
```rust
// Memory-Usage loggen
#[tauri::command]
pub async fn get_memory_usage() -> Result<u64, String> {
    use sysinfo::{System, SystemExt};
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let memory_used = sys.used_memory();
    
    tracing::info!("Memory usage: {} MB", memory_used / 1024 / 1024);
    
    Ok(memory_used)
}
```

**Lösung 1: Kleinere Pack-Sizes**
```rust
// Backend: Backup-Optionen optimieren
let opts = BackupOptions::default()
    .pack_size(32 * 1024 * 1024)  // 32 MB statt 128 MB
    .with_atime(false); // Weniger Metadata
```

**Lösung 2: Paginierung im Frontend**
```typescript
// Frontend: Snapshots paginiert laden
interface SnapshotPage {
  items: Snapshot[];
  page: number;
  totalPages: number;
}

async function loadSnapshotsPage(page: number, pageSize = 50): Promise<SnapshotPage> {
  const allSnapshots = await invoke<Snapshot[]>('list_snapshots');
  
  const start = page * pageSize;
  const end = start + pageSize;
  const items = allSnapshots.slice(start, end);
  
  return {
    items,
    page,
    totalPages: Math.ceil(allSnapshots.length / pageSize),
  };
}
```

**Lösung 3: Caching mit LRU**
```rust
// Backend: Cache mit max. Größe
use lru::LruCache;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct SnapshotCache {
    cache: Arc<Mutex<LruCache<String, SnapshotDto>>>,
}

impl SnapshotCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }
    
    pub fn get(&self, id: &str) -> Option<SnapshotDto> {
        self.cache.lock().get(id).cloned()
    }
    
    pub fn insert(&self, id: String, snapshot: SnapshotDto) {
        self.cache.lock().put(id, snapshot);
    }
}
```

---

### Problem 5: Backup bleibt hängen

**Symptom:**
- Progress-Bar bewegt sich nicht
- UI reagiert nicht mehr
- Backup läuft ewig

**Ursache:**
- Deadlock
- Blocking-Operation im UI-Thread
- Netzwerk-Timeout bei Cloud-Storage

**Diagnose:**
```rust
// Backend: Timeout für Operationen
use tokio::time::{timeout, Duration};

#[tauri::command]
pub async fn run_backup_with_timeout(
    job_id: String,
    state: tauri::State<'_, AppState>,
) -> Result<BackupResult, String> {
    let backup_future = run_backup_internal(&job_id, &state);
    
    // Timeout nach 1 Stunde
    match timeout(Duration::from_secs(3600), backup_future).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Backup-Timeout nach 1 Stunde");
            Err("Backup-Timeout".to_string())
        }
    }
}
```

**Lösung 1: Progress-Events prüfen**
```rust
// Backend: Sicherstellen dass Progress gesendet wird
let last_progress = Arc::new(Mutex::new(std::time::Instant::now()));

let progress_fn = move |info: BackupProgress| {
    let mut last = last_progress.lock();
    let now = std::time::Instant::now();
    
    // Max alle 500ms senden (nicht öfter)
    if now.duration_since(*last) > Duration::from_millis(500) {
        app_handle.emit_all("backup-progress", &info).ok();
        *last = now;
    }
};
```

**Lösung 2: Keepalive für UI**
```typescript
// Frontend: Zeige dass Prozess läuft
let lastUpdate = $state(Date.now());

$effect(() => {
  const interval = setInterval(() => {
    const timeSinceUpdate = Date.now() - lastUpdate;
    
    // Warnung wenn > 30 Sekunden kein Update
    if (timeSinceUpdate > 30000) {
      console.warn('Kein Progress-Update seit 30 Sekunden');
    }
  }, 5000);
  
  return () => clearInterval(interval);
});

// Progress-Listener
unlisten = await listen('backup-progress', (event) => {
  lastUpdate = Date.now();
  progress = event.payload;
});
```

---

### Problem 6: Windows-Pfade funktionieren nicht

**Symptom:**
```
Error: Invalid path: C:\Users\...
```

**Ursache:**
- Backslashes werden nicht korrekt escaped
- Relative vs. absolute Pfade
- UNC-Pfade (\\server\share)

**Lösung: Pfad-Normalisierung**
```rust
// Backend: Utils für Pfade
pub fn normalize_path(path: &str) -> String {
    use std::path::Path;
    
    // Konvertiere zu PathBuf
    let path = Path::new(path);
    
    // Canonicalize (löst relative Pfade auf)
    if let Ok(canonical) = path.canonicalize() {
        // Konvertiere zu String mit Forward Slashes
        canonical
            .to_string_lossy()
            .replace('\\', "/")
    } else {
        // Fallback: Einfach Backslashes ersetzen
        path.to_string_lossy()
            .replace('\\', "/")
    }
}

// Verwendung in Command
#[tauri::command]
pub async fn add_source_path(path: String) -> Result<String, String> {
    let normalized = normalize_path(&path);
    
    // Validiere
    if !std::path::Path::new(&normalized).exists() {
        return Err(format!("Pfad existiert nicht: {}", normalized));
    }
    
    Ok(normalized)
}
```

```typescript
// Frontend: Path-Helper
export function normalizePath(path: string): string {
  // Windows: Backslashes -> Forward Slashes
  return path.replace(/\\/g, '/');
}

// Bei File-Selection
async function handleSelectPath() {
  const selected = await open({
    directory: true,
    multiple: false,
  });
  
  if (selected) {
    const normalized = normalizePath(selected as string);
    sourcePaths = [...sourcePaths, normalized];
  }
}
```

---

### Problem 7: Restore-Berechtigungen falsch

**Symptom:**
- Dateien werden wiederhergestellt
- Aber: Permissions sind falsch
- Oder: Owner/Group stimmt nicht

**Ursache:**
- rustic_core stellt Permissions nicht automatisch wieder her
- Benutzer hat keine Rechte für chown

**Lösung:**
```rust
// Backend: Restore mit Permission-Wiederherstellung
pub async fn restore_with_permissions(
    repo: &Repository,
    snapshot_id: &str,
    target: PathBuf,
) -> Result<()> {
    // Restore ausführen
    restore_files(repo, snapshot_id, vec![], target.clone()).await?;
    
    // Permissions wiederherstellen (nur Linux/macOS)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        // Tree aus Snapshot laden
        let snapshot = repo.get_snapshot_from_str(snapshot_id)?;
        let tree = repo.node_from_snapshot(&snapshot)?;
        
        // Für jede Datei Permissions setzen
        for node in tree.walk() {
            let file_path = target.join(&node.path);
            
            if file_path.exists() {
                let mode = node.meta.mode;
                let perms = std::fs::Permissions::from_mode(mode);
                
                std::fs::set_permissions(&file_path, perms)
                    .ok(); // Fehler ignorieren (evtl. keine Rechte)
            }
        }
    }
    
    Ok(())
}
```

---

### Problem 8: Cloud-Storage langsam

**Symptom:**
- Backup dauert sehr lange
- Progress ruckelt
- Timeouts

**Ursache:**
- Langsame Netzwerkverbindung
- Zu kleine Pack-Sizes
- Kein Caching

**Lösung 1: Pack-Size erhöhen**
```rust
// Größere Packs = weniger Uploads
let opts = BackupOptions::default()
    .pack_size(128 * 1024 * 1024); // 128 MB
```

**Lösung 2: Local-Cache nutzen**
```rust
// Backend: Cache-Verzeichnis für Cloud-Repos
let backend_opts = BackendOptions::default()
    .repository(path)
    .cache_dir(Some("/tmp/rustic-cache".into()));
```

**Lösung 3: Parallele Uploads**
```rust
// rustic_core unterstützt parallele Uploads
let opts = BackupOptions::default()
    .with_upload_threads(4); // 4 parallele Uploads
```

---

## 🔍 Debug-Tools & Helpers

### Backend-Debug-Command

```rust
#[tauri::command]
pub async fn get_debug_info(
    state: tauri::State<'_, AppState>,
) -> Result<DebugInfo, String> {
    let config = state.config.lock().clone();
    let has_repo = state.current_repo.lock().is_some();
    let running_backups = state.cancellation_tokens.lock().len();
    
    Ok(DebugInfo {
        repositories_count: config.repositories.len(),
        jobs_count: config.backup_jobs.len(),
        has_active_repo: has_repo,
        running_backups,
        rust_version: env!("CARGO_PKG_VERSION").to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[derive(serde::Serialize)]
pub struct DebugInfo {
    pub repositories_count: usize,
    pub jobs_count: usize,
    pub has_active_repo: bool,
    pub running_backups: usize,
    pub rust_version: String,
    pub os: String,
    pub arch: String,
}
```

### Health-Check-Command

```rust
#[tauri::command]
pub async fn check_repository_health(
    state: tauri::State<'_, AppState>,
) -> Result<HealthReport, String> {
    let repo = state.get_current_repo()?;
    
    let mut report = HealthReport {
        is_accessible: false,
        index_ok: false,
        packs_ok: false,
        snapshots_ok: false,
        snapshot_count: 0,
        has_locks: false,
        errors: Vec::new(),
    };
    
    // Test 1: Repository erreichbar
    report.is_accessible = repo.test_connection().await.is_ok();
    
    if !report.is_accessible {
        report.errors.push("Repository nicht erreichbar".to_string());
        return Ok(report);
    }
    
    // Test 2: Index
    if let Err(e) = repo.check_index().await {
        report.errors.push(format!("Index-Fehler: {}", e));
    } else {
        report.index_ok = true;
    }
    
    // Test 3: Packs
    if let Err(e) = repo.check_packs().await {
        report.errors.push(format!("Pack-Fehler: {}", e));
    } else {
        report.packs_ok = true;
    }
    
    // Test 4: Snapshots
    match repo.get_all_snapshots().await {
        Ok(snapshots) => {
            report.snapshots_ok = true;
            report.snapshot_count = snapshots.len();
        }
        Err(e) => {
            report.errors.push(format!("Snapshot-Fehler: {}", e));
        }
    }
    
    // Test 5: Locks
    if let Ok(locks) = repo.list_locks().await {
        report.has_locks = !locks.is_empty();
    }
    
    Ok(report)
}

#[derive(serde::Serialize)]
pub struct HealthReport {
    pub is_accessible: bool,
    pub index_ok: bool,
    pub packs_ok: bool,
    pub snapshots_ok: bool,
    pub snapshot_count: usize,
    pub has_locks: bool,
    pub errors: Vec<String>,
}
```

### Logging-Helper

```rust
// Log-Level zur Laufzeit ändern
#[tauri::command]
pub async fn set_log_level(level: String) -> Result<(), String> {
    use tracing_subscriber::EnvFilter;
    
    let filter = EnvFilter::try_new(&level)
        .map_err(|e| format!("Ungültiges Log-Level: {}", e))?;
    
    // TODO: Filter zur Laufzeit ändern (nicht trivial)
    // Für jetzt: Nur validieren
    
    tracing::info!("Log-Level auf {} gesetzt", level);
    Ok(())
}
```

---

## 📊 Performance-Debugging

### Profiling aktivieren

```bash
# CPU-Profiling mit flamegraph
cargo install flamegraph
cargo flamegraph --bin rustic-gui

# Memory-Profiling mit valgrind
valgrind --tool=massif --massif-out-file=massif.out ./target/release/rustic-gui
ms_print massif.out

# Chrome DevTools für Frontend
npm run tauri:dev
# → F12 → Performance-Tab → Record
```

### Performance-Metriken loggen

```rust
use std::time::Instant;

#[tauri::command]
pub async fn run_backup(/*...*/) -> Result<BackupResult, String> {
    let start = Instant::now();
    
    // Backup ausführen
    let result = execute_backup(/*...*/).await?;
    
    let duration = start.elapsed();
    tracing::info!(
        "Backup abgeschlossen in {:?} ({} Dateien, {} MB)",
        duration,
        result.files_processed,
        result.bytes_processed / 1024 / 1024
    );
    
    Ok(result)
}
```

---

## 📝 Issue-Template

Wenn du ein Issue erstellen musst:

```markdown
## Problem-Beschreibung
[Was funktioniert nicht?]

## Schritte zur Reproduktion
1. Öffne App
2. Navigiere zu...
3. Klicke auf...
4. Fehler tritt auf

## Erwartetes Verhalten
[Was sollte passieren?]

## Aktuelles Verhalten
[Was passiert stattdessen?]

## Logs
```
[Relevante Logs hier einfügen]
RUST_LOG=debug Output:
...
```

## Umgebung
- OS: [z.B. Ubuntu 22.04]
- rustic-gui Version: [z.B. 0.1.0]
- rustic Version: [z.B. 0.7.0]
- Rust Version: [rustc --version]

## Screenshots
[Falls relevant]

## Zusätzliche Informationen
- Tritt nur bei bestimmten Repositories auf?
- Reproduzierbar oder sporadisch?
- Error-Code (falls vorhanden)?
```

---

## ✅ Troubleshooting-Checkliste

Vor Issue-Erstellung:

- [ ] Logs gesammelt (RUST_LOG=debug)
- [ ] Browser-Console geprüft
- [ ] Minimal-Reproducer erstellt
- [ ] GitHub Issues durchsucht
- [ ] Mit aktueller Version getestet
- [ ] Permissions geprüft
- [ ] Disk-Space geprüft
- [ ] Netzwerk-Verbindung getestet (bei Cloud)

---

## 🆘 Hilfe bekommen

### Dokumentation

1. Diese Troubleshooting-Docs
2. `README.md`
3. Andere `*.instructions.md`

### Community

1. **GitHub Issues**: https://github.com/your-org/rustic-gui/issues
2. **rustic Discord**: https://discord.gg/WRUWENZnzQ
3. **Tauri Discord**: https://discord.com/invite/tauri

### Logs mitschicken

```bash
# Alle Logs sammeln
RUST_LOG=debug npm run tauri:dev > debug.log 2>&1

# Dann debug.log zum Issue hinzufügen
```

---

**Version**: 2.0  
**Letzte Aktualisierung**: 2025-10-26