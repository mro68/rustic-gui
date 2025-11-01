# Testing Instructions

> Test-Strategien und Best Practices für Rustic GUI

---

## 🔧 Toolchain & Setup

- **Vitest** (Node.js 20, jsdom) für Frontend-Komponenten und Stores. `npm test` führt alle Tests aus, `npm run test:watch` startet den Watch-Modus. Projektweite Mocks und Assertions werden in `src/test-setup.ts` gepflegt.
- **Rust Tests** via `cargo test`. Asynchrone Pfade verwenden `#[tokio::test]`; portable-spezifische Logik liegt unter `src-tauri/src/storage`.
- **Coverage**: `npm run test:coverage` erzeugt Vitest-Berichte, `cargo tarpaulin --out Html` generiert Rust-Coverage.
- **E2E**: Keine automatisierte End-to-End-Suite vorhanden. Neue E2E-Ansätze bitte vorab hier dokumentieren (z.B. Playwright-Konzept).

## 🧪 Test-Pyramide

```
       /\
      /E2E\        (wenige - nur kritische User-Flows)
     /------\
    /  Inte \      (einige - API-Integration, rustic_core)
   / gration\
  /------------\
 /    Unit      \  (viele - Business-Logik, Utils)
/________________\
```

### Verteilung

- **Unit-Tests**: 70% (schnell, isoliert)
- **Integration-Tests**: 25% (Backend ↔ rustic_core)
- **E2E-Tests**: 5% (komplette User-Flows) — _derzeit nicht implementiert_

---

## 🎯 Was testen?

### ✅ IMMER testen

**Business-Logik:**

- Backup-Job-Validierung
- Snapshot-Filtering/Sorting
- Retention-Policy-Berechnung
- Path-Normalisierung
- Cron-Expression-Parsing

**Utils:**

- Formatierungs-Funktionen (Bytes, Datum)
- Validierungs-Funktionen
- Parser (Config, JSON)

**Kritische Pfade:**

- Repository-Öffnen
- Backup-Ausführung
- Restore-Operation
- Passwort-Handling

### ❌ NICHT unbedingt testen

- Triviale Getter/Setter
- Reine UI-Komponenten ohne Logik
- Third-Party-Library-Code
- Generated Code (Tauri Commands)

---

## 💻 Frontend-Tests (TypeScript/Svelte)

### Setup Vitest

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/test-setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'src/test-setup.ts'],
    },
  },
});
```

```typescript
// src/test-setup.ts
import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
  emit: vi.fn(),
}));
```

### Unit-Tests (Utils)

```typescript
// src/lib/utils/format.test.ts
import { describe, it, expect } from 'vitest';
import { formatBytes, formatDuration, formatDate } from './format';

describe('formatBytes', () => {
  it('formatiert 0 Bytes', () => {
    expect(formatBytes(0)).toBe('0 Bytes');
  });

  it('formatiert Bytes zu KB', () => {
    expect(formatBytes(1024)).toBe('1.0 KB');
  });

  it('formatiert Bytes zu MB', () => {
    expect(formatBytes(1048576)).toBe('1.0 MB');
  });

  it('formatiert Bytes zu GB', () => {
    expect(formatBytes(1073741824)).toBe('1.0 GB');
  });

  it('rundet auf 1 Dezimalstelle', () => {
    expect(formatBytes(1536)).toBe('1.5 KB');
  });
});

describe('formatDuration', () => {
  it('formatiert Sekunden', () => {
    expect(formatDuration(45)).toBe('45s');
  });

  it('formatiert Minuten und Sekunden', () => {
    expect(formatDuration(125)).toBe('2m 5s');
  });

  it('formatiert Stunden, Minuten und Sekunden', () => {
    expect(formatDuration(3665)).toBe('1h 1m 5s');
  });

  it('behandelt 0 korrekt', () => {
    expect(formatDuration(0)).toBe('0s');
  });
});
```

### Component-Tests (Svelte)

```typescript
// src/lib/components/shared/Button.test.ts
import { render, fireEvent, screen } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('rendert Button mit Label', () => {
    render(Button, { props: { label: 'Klick mich' } });
    expect(screen.getByText('Klick mich')).toBeInTheDocument();
  });

  it('rendert Button mit Slot-Content', () => {
    const { container } = render(Button, {
      props: {},
      context: new Map([
        ['$$slots', { default: true }],
        ['$$scope', {}],
      ]),
    });
    // Slot-Testing in Svelte 5 ist etwas anders
  });

  it('ruft onClick-Handler auf', async () => {
    const handleClick = vi.fn();
    const { component } = render(Button, {
      props: { label: 'Test' },
    });

    component.$on('click', handleClick);

    const button = screen.getByRole('button');
    await fireEvent.click(button);

    expect(handleClick).toHaveBeenCalledOnce();
  });

  it('ist disabled wenn disabled=true', () => {
    render(Button, { props: { label: 'Test', disabled: true } });
    const button = screen.getByRole('button') as HTMLButtonElement;
    expect(button.disabled).toBe(true);
  });

  it('zeigt Loading-Spinner', () => {
    const { container } = render(Button, {
      props: { label: 'Test', loading: true },
    });
    expect(container.querySelector('.spinner')).toBeInTheDocument();
  });

  it('feuert kein Event wenn disabled', async () => {
    const handleClick = vi.fn();
    const { component } = render(Button, {
      props: { label: 'Test', disabled: true },
    });

    component.$on('click', handleClick);

    const button = screen.getByRole('button');
    await fireEvent.click(button);

    expect(handleClick).not.toHaveBeenCalled();
  });
});
```

### Store-Tests

```typescript
// src/lib/stores/repositories.test.ts
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import {
  repositories,
  activeRepository,
  loadRepositories,
  addRepository,
  removeRepository,
  switchRepository,
  reset,
} from './repositories';

vi.mock('@tauri-apps/api/core');

describe('repositories store', () => {
  beforeEach(() => {
    reset();
    vi.clearAllMocks();
  });

  it('startet mit leerem Array', () => {
    const repos = get(repositories);
    expect(repos).toEqual([]);
  });

  it('lädt Repositories vom Backend', async () => {
    const mockRepos = [
      { id: '1', name: 'Repo 1', path: '/path/1', backendType: 'local' },
      { id: '2', name: 'Repo 2', path: '/path/2', backendType: 'sftp' },
    ];

    vi.mocked(invoke).mockResolvedValue(mockRepos);

    await loadRepositories();

    const repos = get(repositories);
    expect(repos).toEqual(mockRepos);
  });

  it('fügt Repository hinzu', async () => {
    const newRepo = {
      id: '3',
      name: 'New Repo',
      path: '/path/3',
      backendType: 'local' as const,
    };

    vi.mocked(invoke).mockResolvedValue(undefined);

    await addRepository(newRepo);

    const repos = get(repositories);
    expect(repos).toContainEqual(newRepo);
  });

  it('entfernt Repository', async () => {
    const mockRepos = [
      { id: '1', name: 'Repo 1', path: '/path/1', backendType: 'local' },
      { id: '2', name: 'Repo 2', path: '/path/2', backendType: 'sftp' },
    ];

    vi.mocked(invoke).mockResolvedValue(mockRepos);
    await loadRepositories();

    vi.mocked(invoke).mockResolvedValue(undefined);
    await removeRepository('1');

    const repos = get(repositories);
    expect(repos).toHaveLength(1);
    expect(repos[0].id).toBe('2');
  });

  it('setzt aktives Repository', async () => {
    const mockRepos = [{ id: '1', name: 'Repo 1', path: '/path/1', backendType: 'local' }];

    vi.mocked(invoke).mockResolvedValue(mockRepos);
    await loadRepositories();

    vi.mocked(invoke).mockResolvedValue(undefined);
    await switchRepository('1', 'password');

    const active = get(activeRepository);
    expect(active?.id).toBe('1');
  });

  it('behandelt Fehler beim Laden', async () => {
    vi.mocked(invoke).mockRejectedValue(new Error('Backend-Fehler'));

    await expect(loadRepositories()).rejects.toThrow('Backend-Fehler');
  });
});
```

### API-Wrapper-Tests

```typescript
// src/lib/api/backup.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { runBackup, onBackupProgress, cancelBackup } from './backup';

vi.mock('@tauri-apps/api/core');
vi.mock('@tauri-apps/api/event');

describe('Backup API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('runBackup', () => {
    it('ruft Backend-Command auf', async () => {
      const mockResult = {
        snapshotId: 'abc123',
        duration: 42,
        filesProcessed: 100,
        bytesProcessed: 1024000,
      };

      vi.mocked(invoke).mockResolvedValue(mockResult);

      const result = await runBackup('job-1', 'password');

      expect(invoke).toHaveBeenCalledWith('run_backup', {
        jobId: 'job-1',
        password: 'password',
      });
      expect(result).toEqual(mockResult);
    });

    it('wirft Fehler bei Backend-Fehler', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Backup fehlgeschlagen'));

      await expect(runBackup('job-1', 'password')).rejects.toThrow('Backup fehlgeschlagen');
    });
  });

  describe('onBackupProgress', () => {
    it('registriert Progress-Listener', async () => {
      const mockUnlisten = vi.fn();
      vi.mocked(listen).mockResolvedValue(mockUnlisten);

      const callback = vi.fn();
      const unlisten = await onBackupProgress('job-1', callback);

      expect(listen).toHaveBeenCalledWith('backup-progress-job-1', expect.any(Function));
      expect(unlisten).toBe(mockUnlisten);
    });
  });

  describe('cancelBackup', () => {
    it('ruft Cancel-Command auf', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      await cancelBackup('job-1');

      expect(invoke).toHaveBeenCalledWith('cancel_backup', {
        jobId: 'job-1',
      });
    });
  });
});
```

---

## 🦀 Backend-Tests (Rust)

### Unit-Tests

```rust
// src/backup/mod.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_config_validation() {
        let valid_config = BackupJobConfig {
            id: "test".into(),
            name: "Test Job".into(),
            repository_id: "repo-1".into(),
            source_paths: vec![PathBuf::from("/home/user")],
            exclude_patterns: vec![],
            tags: vec![],
            schedule: None,
            retention: RetentionPolicy::default(),
        };

        assert!(validate_backup_config(&valid_config).is_ok());

        // Leerer Name
        let mut invalid = valid_config.clone();
        invalid.name = "".into();
        assert!(validate_backup_config(&invalid).is_err());

        // Keine Source-Pfade
        let mut invalid = valid_config.clone();
        invalid.source_paths = vec![];
        assert!(validate_backup_config(&invalid).is_err());
    }

    #[test]
    fn test_snapshot_to_dto_conversion() {
        let snapshot = create_test_snapshot();
        let dto = snapshot_to_dto(&snapshot);

        assert_eq!(dto.id, snapshot.id.to_string());
        assert_eq!(dto.hostname, snapshot.hostname);
        assert_eq!(dto.tags, snapshot.tags);
    }

    #[test]
    fn test_path_normalization() {
        // Windows-Pfade
        let windows_path = r"C:\Users\Max\Documents";
        let normalized = normalize_path(windows_path);
        assert_eq!(normalized, "C:/Users/Max/Documents");

        // Unix-Pfade (unverändert)
        let unix_path = "/home/user/documents";
        let normalized = normalize_path(unix_path);
        assert_eq!(normalized, "/home/user/documents");
    }

    #[test]
    fn test_cron_expression_validation() {
        // Gültige Expressions
        assert!(validate_cron("0 0 2 * * *").is_ok()); // Täglich 2:00
        assert!(validate_cron("0 */15 * * * *").is_ok()); // Alle 15 Min

        // Ungültige Expressions
        assert!(validate_cron("invalid").is_err());
        assert!(validate_cron("0 0 25 * * *").is_err()); // 25. Stunde
    }
}
```

### Async-Tests

```rust
#[cfg(test)]
mod async_tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_backup_execution() {
        let config = create_test_config();
        let repo = create_test_repository().await;

        let result = execute_backup(&repo, &config).await;

        assert!(result.is_ok());
        let backup_result = result.unwrap();
        assert!(backup_result.files_processed > 0);
    }

    #[tokio::test]
    async fn test_backup_cancellation() {
        use tokio::time::{sleep, Duration};
        use tokio_util::sync::CancellationToken;

        let token = CancellationToken::new();
        let token_clone = token.clone();

        // Backup-Task starten
        let backup_task = tokio::spawn(async move {
            execute_backup_with_cancellation(token_clone).await
        });

        // Nach 100ms abbrechen
        sleep(Duration::from_millis(100)).await;
        token.cancel();

        // Task sollte mit Cancelled-Error enden
        let result = backup_task.await.unwrap();
        assert!(matches!(result, Err(BackupError::Cancelled)));
    }

    #[tokio::test]
    async fn test_parallel_snapshot_loading() {
        let repo = create_test_repository().await;
        let snapshot_ids = vec!["id1", "id2", "id3"];

        let results = load_snapshots_parallel(&repo, snapshot_ids).await;

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
```

### Integration-Tests

```rust
// tests/backup_integration_test.rs

use rustic_gui::backup::*;
use rustic_gui::repository::*;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_full_backup_restore_workflow() {
    // Setup: Temp-Verzeichnisse
    let temp_repo = TempDir::new().unwrap();
    let temp_source = TempDir::new().unwrap();
    let temp_restore = TempDir::new().unwrap();

    // 1. Erstelle Test-Dateien
    let test_file = temp_source.path().join("test.txt");
    fs::write(&test_file, "Hello World").unwrap();

    let test_dir = temp_source.path().join("subdir");
    fs::create_dir(&test_dir).unwrap();
    fs::write(test_dir.join("nested.txt"), "Nested content").unwrap();

    // 2. Initialisiere Repository
    init_repository(
        temp_repo.path().to_str().unwrap(),
        "test-password",
        false,
    )
    .await
    .unwrap();

    // 3. Öffne Repository
    let repo = open_repository(
        temp_repo.path().to_str().unwrap(),
        "test-password",
    )
    .await
    .unwrap();

    // 4. Führe Backup aus
    let snapshot = execute_backup(
        &repo,
        &[temp_source.path().to_path_buf()],
        vec!["test".into()],
        dummy_app_handle(),
    )
    .await
    .unwrap();

    assert!(snapshot.id.len() > 0);

    // 5. Liste Snapshots
    let snapshots = list_snapshots(&repo).await.unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].tags, vec!["test"]);

    // 6. Restore Dateien
    restore_files(
        &repo,
        &snapshot.id.to_string(),
        vec![],
        temp_restore.path().to_path_buf(),
        dummy_app_handle(),
    )
    .await
    .unwrap();

    // 7. Verify restored files
    let restored_file = temp_restore.path().join("test.txt");
    assert!(restored_file.exists());
    let content = fs::read_to_string(restored_file).unwrap();
    assert_eq!(content, "Hello World");

    let restored_nested = temp_restore.path().join("subdir/nested.txt");
    assert!(restored_nested.exists());
}

#[tokio::test]
async fn test_repository_not_found() {
    let result = open_repository("/nonexistent/path", "password").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_wrong_password() {
    let temp_repo = TempDir::new().unwrap();

    // Init mit Passwort
    init_repository(temp_repo.path().to_str().unwrap(), "correct", false)
        .await
        .unwrap();

    // Öffnen mit falschem Passwort
    let result = open_repository(temp_repo.path().to_str().unwrap(), "wrong").await;
    assert!(result.is_err());
}

// Helper-Funktionen
fn dummy_app_handle() -> tauri::AppHandle {
    // Mock AppHandle für Tests
    unimplemented!("Use proper mock in real tests")
}
```

### Mock-Patterns

```rust
// src/testing/mocks.rs

use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait RepositoryInterface {
    async fn backup(&self, paths: &[PathBuf]) -> Result<Snapshot>;
    async fn list_snapshots(&self) -> Result<Vec<Snapshot>>;
    async fn restore(&self, snapshot_id: &str, target: PathBuf) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_mock_repository() {
        let mut mock_repo = MockRepositoryInterface::new();

        // Setup Expectations
        mock_repo
            .expect_backup()
            .with(eq(vec![PathBuf::from("/test")]))
            .times(1)
            .returning(|_| Ok(create_test_snapshot()));

        // Test Code
        let result = mock_repo.backup(&[PathBuf::from("/test")]).await;
        assert!(result.is_ok());
    }
}
```

---

## 🎯 Test-Coverage

### Coverage generieren

```bash
# Frontend (Vitest)
npm run test:coverage

# Backend (Rust mit tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Beide zusammen
npm run test:coverage && cargo tarpaulin
```

### Coverage-Ziele

- **Gesamt**: Minimum 70%
- **Business-Logik**: Minimum 80%
- **Utils**: Minimum 90%
- **UI-Komponenten**: Minimum 50% (weniger kritisch)

### Coverage-Report analysieren

```bash
# HTML-Report öffnen
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
start coverage/index.html  # Windows

# Identifiziere ungetestete Bereiche
# Priorisiere:
# 1. Kritische Pfade (Backup, Restore)
# 2. Error-Handling
# 3. Edge-Cases
```

---

## 🔄 CI/CD-Integration

### GitHub Actions

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run tests
        run: npm test

      - name: Generate coverage
        run: npm run test:coverage

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/coverage-final.json
          flags: frontend

  test-backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test

      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml
          flags: backend
```

---

## 🐛 Debugging Tests

### Frontend

```typescript
// Debug einzelnen Test
npm run test -- -t "sollte Button rendern"

// Watch-Modus
npm run test:watch

// Mit Debug-Logging
DEBUG=* npm test

// In Browser debuggen
npm run test:ui
```

### Backend

```rust
// Debug-Output in Tests
#[test]
fn test_debug() {
    env_logger::init(); // Aktiviert logging in Tests

    let result = some_function();
    dbg!(&result); // Debug-Print

    assert!(result.is_ok());
}

// Nur einen Test ausführen
cargo test test_name -- --nocapture

// Mit stdout
cargo test -- --show-output

// Nur fehlgeschlagene Tests
cargo test -- --test-threads=1
```

---

## 📋 Test-Checklisten

### Vor neuem Feature

- [ ] Überlege welche Tests nötig sind
- [ ] Schreibe Tests WÄHREND der Entwicklung
- [ ] Test-Abdeckung für Happy-Path
- [ ] Test-Abdeckung für Error-Cases
- [ ] Test-Abdeckung für Edge-Cases

### Vor Pull Request

- [ ] Alle Tests laufen durch (Frontend + Backend)
- [ ] Keine Warnungen in Test-Output
- [ ] Coverage hat sich nicht verschlechtert
- [ ] Neue Tests für neue Features
- [ ] Bestehende Tests angepasst (bei Breaking Changes)

### Beim Code-Review

- [ ] Tests sind verständlich
- [ ] Tests sind unabhängig (kein Shared State)
- [ ] Tests sind deterministisch (kein Flaky)
- [ ] Test-Namen beschreiben was getestet wird
- [ ] Assertions sind klar

---

## 🎓 Testing-Best-Practices

### AAA-Pattern (Arrange-Act-Assert)

```typescript
it('sollte Backup erfolgreich ausführen', async () => {
  // Arrange: Setup
  const jobId = 'test-job';
  const mockResult = { snapshotId: 'abc', duration: 42 };
  vi.mocked(invoke).mockResolvedValue(mockResult);

  // Act: Ausführung
  const result = await runBackup(jobId, 'password');

  // Assert: Verifikation
  expect(invoke).toHaveBeenCalledWith('run_backup', {
    jobId,
    password: 'password',
  });
  expect(result).toEqual(mockResult);
});
```

### Test-Namen

```rust
// ✅ GUT: Beschreibend
#[test]
fn test_backup_fails_when_repository_not_found() { }

#[test]
fn test_snapshot_list_is_sorted_by_date_descending() { }

// ❌ SCHLECHT: Unklar
#[test]
fn test_backup() { }

#[test]
fn test_snapshots() { }
```

### Test-Isolation

```typescript
// ✅ GUT: Jeder Test ist unabhängig
describe('Store Tests', () => {
  beforeEach(() => {
    reset(); // State zurücksetzen
    vi.clearAllMocks(); // Mocks zurücksetzen
  });

  it('Test 1', () => {
    // ...
  });

  it('Test 2', () => {
    // Läuft unabhängig von Test 1
  });
});

// ❌ SCHLECHT: Tests beeinflussen sich
let sharedState = [];

it('Test 1', () => {
  sharedState.push('data');
  expect(sharedState.length).toBe(1);
});

it('Test 2', () => {
  // Hängt von Test 1 ab! ❌
  expect(sharedState.length).toBe(1);
});
```

### Flaky Tests vermeiden

```rust
// ❌ SCHLECHT: Time-dependent
#[tokio::test]
async fn flaky_test() {
    let start = Instant::now();
    do_async_work().await;
    let duration = start.elapsed();

    // Kann je nach System-Last fehlschlagen!
    assert!(duration < Duration::from_millis(100));
}

// ✅ GUT: Deterministisch
#[tokio::test]
async fn stable_test() {
    let result = do_async_work().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

// ✅ GUT: Mock für Time-Dependencies
#[tokio::test]
async fn test_with_mock_time() {
    tokio::time::pause(); // Pausiert Zeit

    let start = Instant::now();
    tokio::time::advance(Duration::from_secs(10)).await;
    let duration = start.elapsed();

    assert_eq!(duration, Duration::from_secs(10));
}
```

---

## 🚀 Performance-Tests

### Benchmark-Tests (Rust)

```rust
// Cargo.toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "backup_bench"
harness = false
```

```rust
// benches/backup_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustic_gui::backup::*;

fn benchmark_snapshot_parsing(c: &mut Criterion) {
    let json = include_str!("../fixtures/large_snapshot.json");

    c.bench_function("parse large snapshot", |b| {
        b.iter(|| {
            parse_snapshot(black_box(json))
        });
    });
}

fn benchmark_path_normalization(c: &mut Criterion) {
    let paths: Vec<_> = (0..1000)
        .map(|i| format!("/path/to/file{}", i))
        .collect();

    c.bench_function("normalize 1000 paths", |b| {
        b.iter(|| {
            paths.iter().map(|p| normalize_path(p)).collect::<Vec<_>>()
        });
    });
}

criterion_group!(benches, benchmark_snapshot_parsing, benchmark_path_normalization);
criterion_main!(benches);
```

```bash
# Benchmarks ausführen
cargo bench

# Mit Baseline vergleichen
cargo bench --bench backup_bench -- --save-baseline main
# ... Änderungen machen ...
cargo bench --bench backup_bench -- --baseline main
```

---

## ✅ Testing-Checkliste

### Unit-Tests

- [ ] Business-Logik getestet
- [ ] Utils getestet
- [ ] Error-Cases getestet
- [ ] Edge-Cases getestet
- [ ] Mocks verwendet wo nötig

### Integration-Tests

- [ ] Kritische User-Flows getestet
- [ ] API-Integration getestet
- [ ] rustic_core-Integration getestet
- [ ] Real-World-Scenarios

### Qualität

- [ ] Tests sind lesbar
- [ ] Tests sind wartbar
- [ ] Keine Flaky-Tests
- [ ] Coverage > 70%
- [ ] CI läuft grün

---

**Version**: 1.1  
**Letzte Aktualisierung**: 2025-11-01
