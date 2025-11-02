# Milestone 5: Testing & QA 🧪

> **Produktionsreife durch Testing absichern**

**Dauer:** 54h (1.5 Wochen) | **Status:** 25% - IN ARBEIT  
**Priorität:** 🔴 HIGHEST  
**Dependencies:** M1-M4 (Features müssen funktionieren)  
**Begonnen:** 2025-10-31

---

## Übersicht

**Problem:** Frontend Build-Fehler blockieren Testing. Tests müssen erweitert werden.

**Ziel:**

- **60% Backend-Coverage** (Unit-Tests) - 0% (Build-Problem)
- **40% Frontend-Coverage** (Component-Tests) - ~30% ✅
- **100% kritische Pfade** (E2E-Tests) - 0%

**Strategie:** Build-System zuerst stabilisieren, dann Tests erweitern!

**Fortschritt:**
- ✅ Build-System stabilisiert
- ✅ 63 Store-Tests hinzugefügt
- ⏸️ Backend-Tests blockiert (GTK-Dependencies)
- ⏸️ Component-Tests (Svelte 5 Testing-Library Probleme)

---

## ✅ 5.0 Build-System stabilisiert (KOMPLETT - 2025-10-31)

**Status:** 100% ✅

### Erledigte Arbeiten:

- [x] **Svelte 5 Syntax-Fehler behoben** (10 Komponenten)
  - [x] ContextMenu.svelte: `on:keydown` → `onkeydown`
  - [x] RepositoryCard.svelte: `$state()` für reaktive Variablen
  - [x] CompareSnapshotsDialog.svelte: `$state()` für isComparing
  - [x] TagEditorDialog.svelte: Accessibility mit `role="presentation"`
  - [x] CreateJobDialog.svelte: `$:` → `$derived()` / `$effect()`
  - [x] DeleteJobDialog.svelte: `$state()` und `$derived()`
  - [x] DeleteRepoDialog.svelte: `$derived()` für Validierung
  - [x] EditJobDialog.svelte: `$effect()` für Form-Init
  - [x] UnlockRepositoryDialog.svelte: `$effect()` für Passwort-Stärke
  - [x] ChangePasswordDialog.svelte: `$effect()` für Passwort-Stärke
  - [x] CreateJobDialog.svelte: Each-Block-Bindings auf Index-basiert

**Resultat:** `npm run build` erfolgreich! ✅

---

## 5.1 Backend Unit-Tests

**Dauer:** 24h | **Priorität:** 🔴 HIGHEST | **Target: 60% Coverage**

### Tasks

#### 5.1.1 rustic_core Integration Tests (10h)

**Datei:** `src-tauri/src/rustic/repository.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_local_repository() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().to_str().unwrap();

        let result = init_repository(path, "test-password", "local", None);

        assert!(result.is_ok());
        assert!(temp_dir.path().join(".rustic").exists());
    }

    #[tokio::test]
    async fn test_open_repository_wrong_password() {
        // Setup: Create repo with password "correct"
        // ...

        let result = open_repository(path, "wrong-password").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid password"));
    }

    #[tokio::test]
    async fn test_backup_with_progress() {
        // Create test files
        // Run backup
        // Verify progress events
        // Verify snapshot created
    }

    // ... mehr Tests ...
}
```

**Coverage-Ziele:**

- `repository.rs` - 80% (init, open, check, prune)
- `backup.rs` - 70% (run, cancel, progress)
- `restore.rs` - 70% (restore, list_files)
- `snapshot.rs` - 60% (list, get, delete, tags)

---

#### 5.1.2 Config & State Tests (6h)

**Datei:** `src-tauri/src/config.rs`, `src-tauri/src/state.rs`

**Tests:**

- [ ] Config laden/speichern
- [ ] Config-Migration (alte Version → neue)
- [ ] AppState-Synchronisation
- [ ] Keychain-Integration

---

#### 5.1.3 Scheduler Tests (4h)

**Datei:** `src-tauri/src/scheduler/mod.rs`

**Tests:**

- [ ] Job schedulen
- [ ] Job wird ausgeführt
- [ ] Job-History wird gespeichert
- [ ] Retry-Logic funktioniert

---

#### 5.1.4 Cloud-Backend Tests (4h)

**Setup:** Lokale Test-Backends (MinIO für S3, Azurite für Azure)

**Tests:**

- [ ] S3-Backend-Connection
- [ ] Repository init auf S3
- [ ] Backup auf S3
- [ ] Restore von S3

---

## 5.2 Frontend Component-Tests

**Dauer:** 16h | **Priorität:** 🟠 HIGH | **Target: 40% Coverage** | **Status:** 50% ✅

### Tasks

#### 5.2.1 Store-Tests (Vitest) (6h) - ✅ KOMPLETT (2025-10-31)

**Status:** 100% ✅

**Datei:** `src/lib/stores/*.test.ts`

**Implementiert:**

- [x] **repositories.test.ts** (22 Tests)
  - Basic Store Operations
  - setRepositories/setActiveRepository
  - activeRepository derived store  
  - setLoading/setError
  - loadRepositories mit Backend-Mocking
  - reset() Funktionalität

- [x] **snapshots.test.ts** (22 Tests)
  - Basic Store Operations
  - setSnapshots/addSnapshots (Duplikat-Vermeidung)
  - removeSnapshot
  - Filter/Sort Funktionalität
  - loadSnapshots mit Backend-Integration
  - reset() Funktionalität

- [x] **backup-jobs.test.ts** (19 Tests)
  - Basic Store Operations
  - addJob/updateJob/removeJob
  - setLoading/setError
  - loadJobs mit Backend-Mocking
  - reset() Funktionalität

**Verbleibend:**
- [ ] settings.test.ts
- [ ] toast.test.ts (optional - einfacher Store)

**Coverage:**

- repositories.ts - 90%+ ✅
- snapshots.ts - 85%+ ✅
- backup-jobs.ts - 90%+ ✅
- settings.ts - 0%
- toast.ts - 0%

```typescript
import { describe, it, expect } from 'vitest';
import { get } from 'svelte/store';
import { repositories, addRepository } from './repositories';

describe('repositories store', () => {
  it('should add repository', () => {
    const repo = { id: '1', name: 'Test', path: '/tmp' };
    addRepository(repo);

    const repos = get(repositories);
    expect(repos).toContainEqual(repo);
  });

  it('should switch current repository', () => {
    // ...
  });
});
```

**Coverage:**

- `repositories.ts` - 60%
- `snapshots.ts` - 50%
- `backup-jobs.ts` - 50%
- `settings.ts` - 40%

---

#### 5.2.2 API-Wrapper-Tests (4h)

**Datei:** `src/lib/api/*.test.ts`

**Mock Tauri-Commands:**

```typescript
import { vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Backup API', () => {
  it('should call run_backup command', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    (invoke as any).mockResolvedValue({ snapshot_id: '123' });

    const result = await runBackup('job-1');

    expect(invoke).toHaveBeenCalledWith('run_backup', { jobId: 'job-1' });
    expect(result.snapshot_id).toBe('123');
  });
});
```

---

#### 5.2.3 Dialog-Component-Tests (6h)

**Datei:** `src/lib/components/dialogs/*.test.ts`

**Testing-Library:**

```svelte
<script lang="ts">
import { render, fireEvent } from '@testing-library/svelte';
import AddRepositoryDialog from './AddRepositoryDialog.svelte';

test('should open dialog and submit', async () => {
  const { getByText, getByLabelText } = render(AddRepositoryDialog, {
    props: { open: true },
  });

  const nameInput = getByLabelText('Repository Name');
  await fireEvent.input(nameInput, { target: { value: 'My Repo' } });

  const submitButton = getByText('Hinzufügen');
  await fireEvent.click(submitButton);

  // Verify event emitted
});
```

**Tests:**

- [ ] AddRepositoryDialog
- [ ] CreateJobDialog (Cron-Builder)
- [ ] RestoreDialog (File-Selection)
- [ ] LocationPickerDialog (Tab-Switching)

---

## 5.3 Integration-Tests

**Dauer:** 8h | **Priorität:** 🟠 HIGH

### Tasks

#### 5.3.1 Backend ↔ Frontend Integration (4h)

**Datei:** `src-tauri/tests/integration_tests.rs`

```rust
#[tokio::test]
async fn test_full_backup_restore_flow() {
    // 1. Init Repository
    let repo = init_repository("/tmp/test-repo", "password", "local", None).await?;

    // 2. Open Repository
    open_repository(&repo.id).await?;

    // 3. Run Backup
    let result = run_backup("test-job").await?;
    assert!(!result.snapshot_id.is_empty());

    // 4. List Snapshots
    let snapshots = list_snapshots(None).await?;
    assert_eq!(snapshots.len(), 1);

    // 5. Restore Files
    restore_files(&result.snapshot_id, vec![], "/tmp/restore").await?;
    assert!(Path::new("/tmp/restore").exists());
}
```

---

#### 5.3.2 Cloud-Backend Integration (4h)

**Tests:**

- [ ] Backup auf S3 → Restore
- [ ] Backup auf SFTP (via Rclone) → Restore
- [ ] Job-Scheduler mit Cloud-Repo

---

## 5.4 End-to-End Tests

**Dauer:** 6h | **Priorität:** 🔴 HIGHEST | **Target: 100% kritische Pfade**

### Tasks

#### 5.4.1 Happy-Path Test (4h)

**Tool:** Playwright oder Tauri WebDriver

```typescript
import { test, expect } from '@playwright/test';

test('complete backup workflow', async ({ page }) => {
  // 1. Add Repository
  await page.click('[data-testid="add-repo-button"]');
  await page.fill('[name="repo-name"]', 'Test Repo');
  await page.fill('[name="repo-path"]', '/tmp/test');
  await page.click('[data-testid="submit"]');

  // 2. Create Backup Job
  await page.click('[data-testid="create-job-button"]');
  await page.fill('[name="job-name"]', 'Daily Backup');
  await page.click('[data-testid="add-source-path"]');
  // ...

  // 3. Run Backup
  await page.click('[data-testid="run-backup"]');
  await expect(page.locator('[data-testid="backup-progress"]')).toBeVisible();
  await expect(page.locator('[data-testid="backup-success"]')).toBeVisible({ timeout: 30000 });

  // 4. Verify Snapshot
  await page.goto('/snapshots');
  await expect(page.locator('[data-testid="snapshot-row"]')).toHaveCount(1);

  // 5. Restore Files
  await page.click('[data-testid="snapshot-row"]:first-child >> [data-testid="restore"]');
  // ...
});
```

**Kritische Pfade:**

- [ ] Add Repo → Create Job → Run Backup → Restore
- [ ] Add Cloud Repo (S3) → Run Backup
- [ ] Schedule Job → Wait for execution
- [ ] Compare 2 Snapshots

---

#### 5.4.2 Error-Szenarien (2h)

**Tests:**

- [ ] Wrong password → Error-Message angezeigt
- [ ] Disk full während Backup → Error-Handling
- [ ] Network-Error bei Cloud-Backup → Retry-Logic
- [ ] Cancel Backup → Graceful Stop

---

## Zusammenfassung

**Gesamt-Dauer:** 54h

**Coverage-Ziele:**

- Backend: 60% (Unit-Tests)
- Frontend: 40% (Component-Tests)
- Kritische Pfade: 100% (E2E-Tests)

**Deliverables:**

- [ ] **Unit-Tests** (Cargo Test + Vitest)
- [ ] **Integration-Tests** (Backend ↔ Frontend)
- [ ] **E2E-Tests** (Playwright/WebDriver)
- [ ] **CI/CD-Integration** (GitHub Actions)

**CI/CD Pipeline:**

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo test
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npm run test:coverage
      - uses: codecov/codecov-action@v3

  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm run build
      - run: npm run test:e2e
```

**Akzeptanz-Kriterien:**

- [ ] CI-Pipeline grün
- [ ] Coverage-Report generiert
- [ ] Alle kritischen Pfade getestet
- [ ] Keine flaky Tests

---

**[← Zurück zu M4](M4-advanced-features.md)** | **[Zurück zur Roadmap](../../ROADMAP.md)** | **[Weiter zu M6 →](M6-documentation-release.md)**
