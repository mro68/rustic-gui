## 📋 Inhaltsverzeichnis

1. [Projekt-Übersicht](#projekt-übersicht)
2. [Sprachkonventionen](#sprachkonventionen)
3. [Code-Style & Best Practices](#code-style--best-practices)
4. [Dokumentations-Standards](#dokumentations-standards)
5. [Git Workflow](#git-workflow)
6. [Testing-Strategie](#testing-strategie)
7. [Tools & Automation](#tools--automation)
8. [Architektur-Guidelines](#architektur-guidelines)
9. [Häufige Muster](#häufige-muster)
10. [Troubleshooting](#troubleshooting)

---

## 🎯 Projekt-Übersicht

### Was ist Rustic GUI?

Rustic GUI ist eine Desktop-Anwendung für das Backup-Tool [rustic](https://rustic.cli.rs/), entwickelt mit:

- **Frontend**: Svelte 5 + TypeScript
- **Backend**: Rust (Tauri 2.0)
- **Build Targets**: Linux (AppImage), Windows (EXE/MSI)
- **Zielgruppe**: Technisch versierte Nutzer

### Projekt-Ziele

- Intuitive grafische Oberfläche für rustic
- Backup-Job-Scheduling
- Snapshot-Verwaltung und -Vergleich
- Restore-Funktionalität mit File-Browser
- Multi-Repository-Support

---

## 🌍 Sprachkonventionen

### Grundregel: Hybrid-Ansatz

```bash
Kommunikation: Deutsch
Code-Elemente: Englisch
Dokumentation: Deutsch
```

### Im Detail

#### ✅ DEUTSCH

- **Alle Kommentare** im Code
- **Docstrings** und Funktionsbeschreibungen
- **Git Commit Messages** (strukturiert, siehe Git Workflow)
- **README** und Dokumentation
- **Error Messages** für User (UI)
- **Log-Ausgaben** für User-facing Logs

#### ✅ ENGLISCH

- **Variablen-Namen**: `backupJob`, `snapshotList`, `repoConfig`
- **Funktions-Namen**: `runBackup()`, `listSnapshots()`, `restoreFiles()`
- **Typen/Interfaces**: `BackupJobDto`, `SnapshotInfo`, `RestoreOptions`
- **Dateien-Namen**: `backup-service.ts`, `snapshot-store.ts`
- **Branch-Namen**: `feature/snapshot-compare`, `fix/restore-permissions`
- **Technische Begriffe**: "repository", "snapshot", "restore", "backup"

### Beispiel-Code

```typescript
/**
 * Startet einen Backup-Job und gibt das Ergebnis zurück.
 * 
 * @param jobId - Eindeutige ID des Backup-Jobs
 * @param options - Zusätzliche Optionen für den Backup-Lauf
 * @returns Promise mit dem Backup-Ergebnis
 * @throws BackupError wenn der Backup fehlschlägt
 */
async function runBackup(jobId: string, options?: BackupOptions): Promise<BackupResult> {
  // Validiere Job-ID
  if (!jobId) {
    throw new Error('Job-ID darf nicht leer sein');
  }
  
  // Hole Job-Konfiguration aus Store
  const job = await getJobById(jobId);
  
  // Führe Backup aus
  return await executeBackup(job, options);
}
```

---

## 💻 Code-Style & Best Practices

### TypeScript / Svelte

#### Naming Conventions

```typescript
// Variablen: camelCase
const backupJobs = [];
let currentSnapshot = null;

// Konstanten: SCREAMING_SNAKE_CASE für echte Konstanten
const MAX_RETRIES = 3;
const DEFAULT_TIMEOUT = 5000;

// Funktionen: camelCase, Verben verwenden
function createBackup() {}
function validateRepository() {}

// Typen/Interfaces: PascalCase
interface BackupJob {}
type SnapshotId = string;

// Svelte Components: PascalCase
// Datei: CreateJobDialog.svelte
export let jobName: string;

// Stores: camelCase mit beschreibendem Namen
export const backupJobs = writable<BackupJob[]>([]);
export const activeRepository = writable<Repository | null>(null);
```

#### Docstrings (TSDoc)

```typescript
/**
 * Kurzbeschreibung der Funktion auf Deutsch.
 * 
 * Längere Beschreibung mit mehr Details über die Funktionsweise.
 * Kann auch mehrere Zeilen umfassen.
 * 
 * @param repoPath - Absoluter Pfad zum Repository
 * @param password - Passwort für Repository-Zugriff
 * @param options - Optionale Konfigurations-Parameter
 * @returns Promise mit den gefundenen Snapshots
 * @throws RepositoryError wenn Repository nicht erreichbar
 * @throws AuthenticationError bei falschem Passwort
 * 
 * @example
 * ```typescript
 * const snapshots = await listSnapshots('/mnt/backup/repo', 'secret');
 * console.log(`Gefundene Snapshots: ${snapshots.length}`);
 * ```
 */
export async function listSnapshots(
  repoPath: string,
  password: string,
  options?: ListOptions
): Promise<Snapshot[]> {
  // Implementation
}
```

#### Error Handling

```typescript
// Eigene Error-Klassen mit deutschen Nachrichten
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
    showToast('error', 'Backup fehlgeschlagen', error.message);
  } else {
    // Generische Behandlung
    console.error('Unerwarteter Fehler:', error);
    showToast('error', 'Fehler', 'Ein unerwarteter Fehler ist aufgetreten');
  }
}
```

#### Svelte Component Structure

```html
<script lang="ts">
  /**
   * Dialog zum Erstellen eines neuen Backup-Jobs.
   * 
   * Bietet ein mehrstufiges Formular mit Tabs für:
   * - Allgemeine Einstellungen
   * - Pfade und Exclusions
   * - Zeitplan-Konfiguration
   * - Retention-Policy
   */
  
  // Imports
  import { createEventDispatcher } from 'svelte';
  import type { BackupJob } from '$lib/types';
  
  // Props
  export let isOpen = false;
  export let initialData: Partial<BackupJob> | undefined = undefined;
  
  // State
  let activeTab = 0;
  let jobName = initialData?.name ?? '';
  let sourcePaths: string[] = initialData?.sourcePaths ?? [];
  
  // Event Dispatcher
  const dispatch = createEventDispatcher<{
    save: BackupJob;
    cancel: void;
  }>();
  
  // Funktionen
  function handleSave() {
    const job: BackupJob = {
      id: crypto.randomUUID(),
      name: jobName,
      sourcePaths,
      // ... weitere Felder
    };
    dispatch('save', job);
  }
  
  function handleCancel() {
    dispatch('cancel');
  }
</script>

<!-- Template -->
<dialog open={isOpen}>
  <!-- Content -->
</dialog>

<style>
  /* Component-spezifische Styles */
  dialog {
    /* ... */
  }
</style>
```

### Rust / Tauri

#### Naming Conventions

```rust
// Variablen: snake_case
let backup_result = run_backup();
let snapshot_count = 42;

// Konstanten: SCREAMING_SNAKE_CASE
const MAX_CONCURRENT_BACKUPS: usize = 3;
const DEFAULT_BUFFER_SIZE: usize = 8192;

// Funktionen: snake_case
fn create_backup() {}
fn validate_repository() {}

// Structs/Enums: PascalCase
struct BackupJob {}
enum BackupStatus {}

// Traits: PascalCase
trait BackupExecutor {}

// Module: snake_case
mod backup_service;
mod snapshot_parser;
```

#### Docstrings (Rustdoc)

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
/// - `BackupError::RepositoryNotFound` wenn Repository nicht existiert
/// - `BackupError::AuthenticationFailed` bei falschem Passwort
/// - `BackupError::Cancelled` wenn Backup abgebrochen wurde
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
#[tauri::command]
pub async fn run_backup(
    job_id: String,
    password: String,
    cancellation_token: Option<CancellationToken>,
) -> Result<BackupResult, BackupError> {
    // Implementation
}
```

#### Error Handling

```rust
use thiserror::Error;

/// Fehlertypen für Backup-Operationen
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
    
    #[error("Interner Fehler: {0}")]
    Internal(String),
}

// Verwendung
fn run_backup() -> Result<BackupResult, BackupError> {
    let repo_path = std::env::var("REPO_PATH")
        .map_err(|_| BackupError::RepositoryNotFound("Umgebungsvariable fehlt".into()))?;
    
    // ... weitere Logik
    Ok(BackupResult::default())
}
```

#### Tauri Commands Pattern

```rust
/// Tauri Command für Snapshot-Auflistung.
///
/// Dieser Command wird vom Frontend via `invoke('list_snapshots', {...})` aufgerufen.
///
/// # Sicherheit
///
/// - Passwort wird nicht geloggt
/// - Repository-Pfad wird validiert
#[tauri::command]
pub async fn list_snapshots(
    repo_path: String,
    password: String,
) -> Result<Vec<SnapshotDto>, String> {
    // Setze Passwort in Umgebungsvariable (wird nach Aufruf gelöscht)
    std::env::set_var("RUSTIC_PASSWORD", &password);
    
    // Rufe rustic auf
    let output = tokio::process::Command::new("rustic")
        .args(&["snapshots", "-r", &repo_path, "--json"])
        .output()
        .await
        .map_err(|e| format!("Fehler beim Ausführen von rustic: {}", e))?;
    
    // Lösche Passwort aus Umgebung
    std::env::remove_var("RUSTIC_PASSWORD");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Rustic-Fehler: {}", stderr));
    }
    
    // Parse JSON
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshots: Vec<SnapshotDto> = serde_json::from_str(&stdout)
        .map_err(|e| format!("JSON-Parse-Fehler: {}", e))?;
    
    Ok(snapshots)
}
```

---

## 📚 Dokumentations-Standards

### README.md Struktur

```markdown
# Rustic GUI

> Grafische Benutzeroberfläche für das Backup-Tool rustic

## Features

- 📦 Multi-Repository-Verwaltung
- ⏰ Zeitgesteuerte Backups
- 📸 Snapshot-Vergleich
- 🔄 Restore mit File-Browser

## Installation

### Linux (AppImage)

...

### Windows (Portable EXE)

...

## Entwicklung

### Voraussetzungen

- Node.js 20+
- Rust 1.75+
- rustic binary

### Setup

\`\`\`bash
npm install
npm run tauri:dev
\`\`\`

## Dokumentation

Siehe [docs/](./docs/) für detaillierte Dokumentation.

## Lizenz

MIT
```

### Inline-Dokumentation Guidelines

#### Was dokumentieren?

✅ **Immer dokumentieren:**
- Public API-Funktionen
- Komplexe Algorithmen
- Nicht-offensichtliche Logik
- Workarounds und Hacks (mit Erklärung warum)
- Type-Definitionen

❌ **Nicht dokumentieren:**
- Selbst-erklärende Getter/Setter
- Triviale Funktionen
- Offensichtliche Implementierungen

#### Gute vs. Schlechte Kommentare

```typescript
// ❌ SCHLECHT: Redundant
// Setzt den Namen
function setName(name: string) {
  this.name = name;
}

// ✅ GUT: Erklärt Warum/Wie
/**
 * Setzt den Namen und normalisiert ihn.
 * 
 * Trimmt Whitespace und konvertiert zu lowercase, da rustic
 * Job-Namen case-insensitive behandelt.
 */
function setName(name: string) {
  this.name = name.trim().toLowerCase();
}

// ❌ SCHLECHT: Kommentar widerspricht Code
// Gibt true zurück wenn Backup aktiv
function isBackupInactive() { // Inkonsistenz!
  return !this.isActive;
}

// ✅ GUT: Kommentar erklärt nicht-offensichtlichen Trick
/**
 * Wir verwenden ein Set statt Array für O(1) Lookup,
 * da wir häufig prüfen müssen ob ein Pfad bereits existiert.
 */
const excludedPaths = new Set<string>();
```

---

## 🔄 Git Workflow

### Branch-Strategie: GitHub Flow (vereinfacht)

```bash
main
  ├── feature/snapshot-compare
  ├── feature/job-scheduling
  ├── fix/restore-permissions
  └── docs/update-readme
```

#### Branch-Naming

```bash
feature/kurze-beschreibung  # Neue Features
fix/kurze-beschreibung      # Bugfixes
docs/kurze-beschreibung     # Nur Dokumentation
refactor/kurze-beschreibung # Code-Umstrukturierung
chore/kurze-beschreibung    # Build, Dependencies, etc.
```

### Commit Messages: Conventional Commits (Deutsch)

```
<typ>(<scope>): <kurzbeschreibung>

<längere beschreibung>

<footer>
```

#### Typen

- `feat`: Neues Feature
- `fix`: Bugfix
- `docs`: Dokumentation
- `style`: Formatierung (kein Code-Change)
- `refactor`: Code-Umstrukturierung
- `test`: Tests hinzufügen/ändern
- `chore`: Build, Dependencies, etc.

#### Scopes (optional)

- `backup`: Backup-Funktionalität
- `restore`: Restore-Funktionalität
- `ui`: User Interface
- `config`: Konfiguration
- `scheduler`: Job-Scheduling

#### Beispiele

```bash
# Feature
git commit -m "feat(backup): Job-Scheduling implementiert

Backup-Jobs können jetzt mit Cron-Expressions zeitgesteuert werden.
Verwendet tokio-cron-scheduler für asynchrone Ausführung.

Closes #42"

# Bugfix
git commit -m "fix(restore): Berechtigungen werden korrekt wiederhergestellt

Vorher wurden Datei-Permissions ignoriert.
Jetzt wird chmod() korrekt aufgerufen.

Fixes #123"

# Dokumentation
git commit -m "docs: README mit Installation-Anleitung ergänzt"

# Refactoring
git commit -m "refactor(ui): Dialog-Components in separate Dateien ausgelagert"
```

### Workflow

```bash
# 1. Feature-Branch erstellen
git checkout -b feature/snapshot-compare

# 2. Entwickeln und committen
git add .
git commit -m "feat(snapshots): Snapshot-Vergleich UI implementiert"

# 3. Regelmäßig von main pullen
git fetch origin
git rebase origin/main

# 4. Push und Pull Request
git push origin feature/snapshot-compare

# 5. Nach Review: Merge in main
# (via GitHub UI oder:)
git checkout main
git merge --no-ff feature/snapshot-compare
git push origin main

# 6. Feature-Branch löschen
git branch -d feature/snapshot-compare
git push origin --delete feature/snapshot-compare
```

### Pre-Commit Checks (empfohlen)

```bash
# .git/hooks/pre-commit
#!/bin/bash

echo "🔍 Running pre-commit checks..."

# TypeScript/Svelte
npm run lint || exit 1
npm run type-check || exit 1

# Rust
cd src-tauri
cargo fmt --check || exit 1
cargo clippy -- -D warnings || exit 1
cd ..

echo "✅ All checks passed!"
```

---

## 🧪 Testing-Strategie

### Test-Pyramide

```bash
       /\
      /e2e\        (wenige)
     /------\
    /  inte \      (einige)
   /  gration\
  /------------\
 /    unit      \  (viele)
/________________\
```

### TypeScript/Svelte Tests

#### Vitest Setup

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
  },
});
```

#### Unit Test Beispiel

```typescript
// src/lib/utils/format.test.ts
import { describe, it, expect } from 'vitest';
import { formatBytes, formatDuration } from './format';

describe('formatBytes', () => {
  it('formatiert Bytes korrekt in KB', () => {
    expect(formatBytes(1024)).toBe('1.0 KB');
  });

  it('formatiert große Werte in GB', () => {
    expect(formatBytes(1073741824)).toBe('1.0 GB');
  });

  it('behandelt 0 Bytes korrekt', () => {
    expect(formatBytes(0)).toBe('0 Bytes');
  });
});

describe('formatDuration', () => {
  it('formatiert Sekunden in lesbare Zeit', () => {
    expect(formatDuration(3661)).toBe('1h 1m 1s');
  });
});
```

#### Svelte Component Test

```typescript
// src/lib/components/Button.test.ts
import { render, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('rendert Button mit Text', () => {
    const { getByText } = render(Button, {
      props: { label: 'Klick mich' }
    });
    expect(getByText('Klick mich')).toBeInTheDocument();
  });

  it('feuert click-Event beim Klicken', async () => {
    const handleClick = vi.fn();
    const { getByRole } = render(Button, {
      props: { label: 'Test' }
    });
    
    const button = getByRole('button');
    await fireEvent.click(button);
    
    // Event-Handler würde in echter Component gecheckt
    expect(button).toHaveBeenClicked();
  });
});
```

### Rust Tests

#### Unit Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_snapshot_json() {
        let json = r#"{"id":"abc123","time":"2024-01-01T00:00:00Z"}"#;
        let snapshot: Snapshot = serde_json::from_str(json).unwrap();
        
        assert_eq!(snapshot.id, "abc123");
        assert!(!snapshot.time.is_empty());
    }

    #[test]
    fn test_backup_error_formatting() {
        let error = BackupError::RepositoryNotFound("/tmp/repo".into());
        assert_eq!(
            error.to_string(),
            "Repository nicht gefunden: /tmp/repo"
        );
    }

    #[tokio::test]
    async fn test_run_backup_success() {
        // Arrange
        let job_id = "test-job";
        let password = "secret";

        // Act
        let result = run_backup(job_id.into(), password.into(), None).await;

        // Assert
        assert!(result.is_ok());
    }
}
```

#### Integration Test

```rust
// tests/backup_integration_test.rs
use rustic_gui::backup::run_backup;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_full_backup_workflow() {
    // Erstelle temporäres Repo
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path().to_str().unwrap();
    
    // Initialisiere Repository
    // ... (rustic init aufrufen)
    
    // Führe Backup aus
    let result = run_backup("test-job".into(), "password".into(), None).await;
    
    // Prüfe Erfolg
    assert!(result.is_ok());
    
    // Prüfe dass Snapshot erstellt wurde
    // ... (rustic snapshots auflisten)
}
```

### Test Commands

```bash
# TypeScript/Svelte
npm run test              # Alle Tests
npm run test:unit         # Nur Unit-Tests
npm run test:watch        # Watch-Mode
npm run test:coverage     # Mit Coverage

# Rust
cargo test                # Alle Tests
cargo test --lib          # Nur Library-Tests
cargo test --test backup  # Spezifischer Integration-Test
cargo test -- --nocapture # Mit stdout
```

---

## 🛠️ Tools & Automation

### Linting & Formatting

#### TypeScript/Svelte

```json
// package.json scripts
{
  "scripts": {
    "lint": "eslint src --ext .ts,.svelte",
    "lint:fix": "eslint src --ext .ts,.svelte --fix",
    "format": "prettier --write 'src/**/*.{ts,svelte,css}'",
    "format:check": "prettier --check 'src/**/*.{ts,svelte,css}'",
    "type-check": "svelte-check --tsconfig ./tsconfig.json"
  }
}
```

#### ESLint Config

```javascript
// eslint.config.js
export default {
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:svelte/recommended',
  ],
  parser: '@typescript-eslint/parser',
  rules: {
    // Bevorzuge const
    'prefer-const': 'error',
    
    // Keine unused vars
    '@typescript-eslint/no-unused-vars': ['error', {
      argsIgnorePattern: '^_'
    }],
    
    // Explizite Return-Types bei exported functions
    '@typescript-eslint/explicit-module-boundary-types': 'warn',
    
    // Keine any (außer mit Kommentar warum)
    '@typescript-eslint/no-explicit-any': 'error',
  },
};
```

#### Prettier Config

```json
// .prettierrc
{
  "semi": true,
  "singleQuote": true,
  "trailingComma": "es5",
  "printWidth": 100,
  "tabWidth": 2,
  "useTabs": false,
  "svelteSortOrder": "scripts-markup-styles",
  "svelteStrictMode": true
}
```

#### Rust

```toml
# rustfmt.toml
max_width = 100
use_small_heuristics = "Max"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

```bash
# Format & Lint
cargo fmt
cargo clippy -- -D warnings
```

### Dokumentations-Generierung

#### TypeScript: TypeDoc

```bash
npm install --save-dev typedoc

# package.json
{
  "scripts": {
    "docs:generate": "typedoc --out docs/api src/lib"
  }
}
```

```json
// typedoc.json
{
  "entryPoints": ["src/lib"],
  "out": "docs/api",
  "plugin": ["typedoc-plugin-markdown"],
  "readme": "README.md",
  "exclude": ["**/*.test.ts", "**/*.spec.ts"],
  "excludePrivate": true,
  "excludeProtected": true
}
```

#### Rust: cargo doc

```bash
# Generiere Dokumentation
cargo doc --no-deps --open

# Mit private items
cargo doc --no-deps --document-private-items

# Für alle Dependencies
cargo doc --open
```

#### Gesamt-Dokumentation mit mdBook

```bash
cargo install mdbook

# Erstelle Buch
mdbook init docs
cd docs

# Struktur in SUMMARY.md
# src/SUMMARY.md:
# - [Einführung](./introduction.md)
# - [Installation](./installation.md)
# - [Benutzer-Handbuch](./user-guide/README.md)
#   - [Backups erstellen](./user-guide/create-backup.md)
# - [Entwickler-Dokumentation](./developer/README.md)
#   - [Architektur](./developer/architecture.md)

# Build & Serve
mdbook serve
```

### CI/CD: GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

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
      
      - name: Lint
        run: npm run lint
      
      - name: Type check
        run: npm run type-check
      
      - name: Tests
        run: npm run test
      
      - name: Build
        run: npm run build

  test-backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Format check
        run: cargo fmt --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Tests
        run: cargo test

  build:
    needs: [test-frontend, test-backend]
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '20'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build Tauri
        run: npm run tauri:build
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: rustic-gui-${{ matrix.os }}
          path: src-tauri/target/release/bundle/
```

### Weitere nützliche Tools

#### Code-Qualität

```bash
# Dependency-Check
npm audit
cargo audit

# Bundle-Size-Analyse
npm install --save-dev vite-bundle-visualizer

# Rust Code-Coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

#### Entwickler-Produktivität

```bash
# Hot-Reload verbessern
npm install --save-dev vite-plugin-restart

# Tauri Dev mit besserem Logging
RUST_LOG=debug npm run tauri:dev

# Automatische Rust-Docs beim Speichern
cargo watch -x doc
```


## ---

## 🔍 Häufige Muster

### Pattern 1: Repository-Switching

```rust
/// Wechselt das aktive Repository.
///
/// Schließt das alte Repository sauber und öffnet das neue.
#[tauri::command]
pub async fn switch_repository(
    repo_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<RepositoryInfo, String> {
    // Altes Repository schließen
    {
        let mut current = state.current_repo.lock().unwrap();
        if let Some(old_repo) = current.take() {
            // rustic_core Repository implementiert Drop, daher automatisch cleanup
            drop(old_repo);
            tracing::debug!("Altes Repository geschlossen");
        }
    }
    
    // Repo-Config laden
    let config = load_repository_config(&repo_id).await
        .map_err(|e| format!("Fehler beim Laden der Config: {}", e))?;
    
    // Neues Repository öffnen
    let repo = crate::rustic::open_repository(&config.path, &password)
        .await
        .map_err(|e| format!("Fehler beim Öffnen: {}", e))?;
    
    // Repository-Info für Frontend
    let info = RepositoryInfo {
        id: repo_id.clone(),
        path: config.path.clone(),
        backend_type: detect_backend_type(&config.path),
        snapshot_count: repo.get_snapshot_count()?,
        total_size: repo.get_total_size()?,
    };
    
    // In State speichern
    *state.current_repo.lock().unwrap() = Some(repo);
    
    tracing::info!("Repository gewechselt: {}", repo_id);
    
    Ok(info)
}
```

### Pattern 2: Batch-Operations mit Progress

```rust
use futures::stream::{self, StreamExt};

/// Löscht mehrere Snapshots parallel mit Progress-Updates.
///
/// Verwendet Streams für effiziente Parallel-Verarbeitung.
#[tauri::command]
pub async fn delete_snapshots_batch(
    snapshot_ids: Vec<String>,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<usize, String> {
    let repo = state.current_repo.lock().unwrap()
        .as_ref()
        .ok_or("Kein Repository geöffnet")?
        .clone();
    
    let total = snapshot_ids.len();
    let mut completed = 0;
    
    // Stream für parallele Verarbeitung (max 3 gleichzeitig)
    let results = stream::iter(snapshot_ids)
        .map(|snap_id| {
            let repo = repo.clone();
            async move {
                repo.forget_snapshot(&snap_id)
                    .map_err(|e| e.to_string())
            }
        })
        .buffer_unordered(3)
        .collect::<Vec<_>>()
        .await;
    
    // Zähle Erfolge und sende Progress
    for (idx, result) in results.iter().enumerate() {
        if result.is_ok() {
            completed += 1;
        }
        
        // Progress-Event senden
        app_handle.emit_all("delete-progress", json!({
            "completed": idx + 1,
            "total": total,
            "successful": completed,
        })).ok();
    }
    
    tracing::info!("Gelöscht: {} von {} Snapshots", completed, total);
    
    Ok(completed)
}
```

### Pattern 3: File-Tree für Restore-Browser

```rust
use rustic_core::{Node, NodeType};
use std::collections::HashMap;

#[derive(Serialize)]
struct FileTreeNode {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    children: Option<Vec<FileTreeNode>>,
}

/// Lädt den Dateibaum eines Snapshots für den Restore-Browser.
///
/// Baut eine hierarchische Struktur auf für effiziente UI-Darstellung.
#[tauri::command]
pub async fn get_snapshot_file_tree(
    snapshot_id: String,
    path_prefix: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<FileTreeNode>, String> {
    let repo = state.current_repo.lock().unwrap()
        .as_ref()
        .ok_or("Kein Repository geöffnet")?
        .clone();
    
    // Snapshot laden
    let snapshot = repo.get_snapshot(&snapshot_id)
        .map_err(|e| format!("Snapshot nicht gefunden: {}", e))?;
    
    // Tree aus Snapshot laden
    let tree = repo.node_from_snapshot(&snapshot)
        .map_err(|e| format!("Fehler beim Laden des Trees: {}", e))?;
    
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
            children: None, // Lazy-Loading für Unterverzeichnisse
        })
        .collect();
    
    Ok(file_nodes)
}
```

### Pattern 4: Scheduled Backups mit Cron

```rust
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;

/// Backup-Scheduler für zeitgesteuerte Jobs.
pub struct BackupScheduler {
    scheduler: JobScheduler,
    jobs: Arc<Mutex<HashMap<String, uuid::Uuid>>>,
}

impl BackupScheduler {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
            jobs: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Fügt einen zeitgesteuerten Backup-Job hinzu.
    ///
    /// # Arguments
    ///
    /// * `job_id` - Eindeutige Job-ID
    /// * `cron_expression` - Cron-Ausdruck (z.B. "0 0 2 * * *" für täglich 2:00)
    /// * `callback` - Funktion die bei jedem Trigger aufgerufen wird
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
        self.jobs.lock().unwrap().insert(job_id, uuid);
        
        Ok(())
    }
    
    /// Entfernt einen Job aus dem Scheduler.
    pub async fn remove_job(&mut self, job_id: &str) -> Result<()> {
        let mut jobs = self.jobs.lock().unwrap();
        
        if let Some(uuid) = jobs.remove(job_id) {
            self.scheduler.remove(&uuid).await?;
        }
        
        Ok(())
    }
    
    /// Startet den Scheduler.
    pub async fn start(&self) -> Result<()> {
        self.scheduler.start().await?;
        Ok(())
    }
}

// Integration in Tauri
#[tauri::command]
pub async fn schedule_backup_job(
    job_id: String,
    cron_expression: String,
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let mut scheduler = state.scheduler.lock().await;
    
    // Callback für Backup-Ausführung
    let job_id_clone = job_id.clone();
    let app_handle_clone = app_handle.clone();
    
    scheduler.add_job(
        job_id.clone(),
        &cron_expression,
        move || {
            let job_id = job_id_clone.clone();
            let app_handle = app_handle_clone.clone();
            
            Box::pin(async move {
                tracing::info!("Scheduled Backup gestartet: {}", job_id);
                
                // Backup ausführen (via existierendes Command)
                if let Err(e) = run_backup(job_id.clone(), /* ... */).await {
                    tracing::error!("Scheduled Backup fehlgeschlagen: {}", e);
                    
                    // Fehler-Event senden
                    app_handle.emit_all("scheduled-backup-failed", json!({
                        "job_id": job_id,
                        "error": e.to_string(),
                    })).ok();
                }
            })
        },
    ).await
    .map_err(|e| format!("Fehler beim Schedulen: {}", e))?;
    
    tracing::info!("Job {} geplant mit Cron: {}", job_id, cron_expression);
    
    Ok(())
}
```

### Pattern 5: Config-Persistence mit TOML

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Liste aller konfigurierten Repositories
    pub repositories: Vec<RepositoryConfig>,
    
    /// Liste aller Backup-Jobs
    pub backup_jobs: Vec<BackupJobConfig>,
    
    /// Globale App-Einstellungen
    pub settings: AppSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub id: String,
    pub name: String,
    pub path: String,
    pub backend_type: BackendType,
    
    /// Ob Passwort in System-Keychain gespeichert ist
    pub password_stored: bool,
    
    /// Statistiken (Cache)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_snapshot_count: Option<usize>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_total_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupJobConfig {
    pub id: String,
    pub name: String,
    pub repository_id: String,
    pub source_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub tags: Vec<String>,
    
    /// Schedule (None = manuell)
    pub schedule: Option<String>,
    
    /// Retention-Policy
    pub retention: RetentionPolicy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_last: Option<usize>,
    pub keep_daily: Option<usize>,
    pub keep_weekly: Option<usize>,
    pub keep_monthly: Option<usize>,
    pub keep_yearly: Option<usize>,
}

/// Lädt die App-Config aus der TOML-Datei.
pub fn load_config() -> Result<AppConfig> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        // Erstelle Default-Config
        let default_config = AppConfig::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    
    let content = std::fs::read_to_string(&config_path)?;
    let config: AppConfig = toml::from_str(&content)?;
    
    Ok(config)
}

/// Speichert die App-Config in TOML-Datei.
pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_path = get_config_path()?;
    
    // Erstelle Config-Verzeichnis falls nicht vorhanden
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let toml = toml::to_string_pretty(config)?;
    std::fs::write(&config_path, toml)?;
    
    Ok(())
}

/// Gibt den Config-Pfad zurück.
///
/// - Linux: ~/.config/rustic-gui/config.toml
/// - Windows: %APPDATA%\rustic-gui\config.toml
fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Config-Verzeichnis nicht gefunden"))?;
    
    Ok(config_dir.join("rustic-gui").join("config.toml"))
}
```

---

## 🐛 Troubleshooting

### Problem 1: "Repository nicht gefunden" bei Cloud-Storage

**Symptom:**
```bash
Error: Repository nicht gefunden: gdrive:backup
rustic_backend::error: OpenDAL backend error
```

**Ursache:** Repository wurde noch nicht initialisiert, oder rclone-Remote ist nicht konfiguriert.

**Lösung:**

```bash
# 1. Prüfe ob rclone-Remote existiert
rclone listremotes

# 2. Falls nicht: Konfiguriere Remote
rclone config

# 3. Teste Remote
rclone lsd gdrive:

# 4. Initialisiere Repository mit rclone-Flag
# (in GUI: Checkbox "Cloud-Storage (rclone init)" aktivieren)
```

**Code-Check:**
```rust
// Stelle sicher dass is_cloud = true bei Cloud-Pfaden
let is_cloud = repo_path.contains(':') && !repo_path.starts_with('/');
init_repository(repo_path, password, is_cloud).await?;
```

---

### Problem 2: "Repository ist gesperrt"

**Symptom:**
```bash
Error: Repository ist gesperrt (anderer Prozess aktiv)
rustic_core::error: Lock file exists
```

**Ursache:** Vorheriger Backup/Restore wurde nicht sauber beendet (Crash, Abbruch).

**Lösung:**

```rust
/// Entsperrt ein Repository forciert.
///
/// ⚠️ Nur verwenden wenn sicher ist, dass kein anderer Prozess aktiv ist!
#[tauri::command]
pub async fn force_unlock_repository(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let repo = state.current_repo.lock().unwrap()
        .as_ref()
        .ok_or("Kein Repository geöffnet")?;
    
    // Alle Locks entfernen
    repo.unlock_all()
        .map_err(|e| format!("Fehler beim Entsperren: {}", e))?;
    
    tracing::warn!("Repository forciert entsperrt");
    
    Ok(())
}
```

**UI-Pattern:**
```html
<script>
  async function handleUnlock() {
    const confirmed = await confirm(
      'Repository entsperren?\n\n' +
      'Nur fortfahren wenn sicher ist, dass kein anderer Backup läuft!'
    );
    
    if (confirmed) {
      await invoke('force_unlock_repository');
      showToast('success', 'Repository entsperrt');
    }
  }
</script>
```

---

### Problem 3: Passwort-Fehler bei Cloud-Storage

**Symptom:**
```bash
Error: Authentifizierung fehlgeschlagen
rustic_core::error: Wrong password
```

**Ursache:** Passwort wird bei Cloud-Storage manchmal nicht korrekt übergeben.

**Workaround:**

```rust
// Setze RUSTIC_PASSWORD als Env-Var UND übergebe an Repository
std::env::set_var("RUSTIC_PASSWORD", password);

let repo_opts = RepositoryOptions::default()
    .password(password)  // Explizit übergeben
    .password_file(None)  // Keine Passwort-Datei
    .password_command(None);  // Kein Passwort-Command

let repo = Repository::new(&repo_opts, backends)?;

// Nach Verwendung: Entferne aus Env
std::env::remove_var("RUSTIC_PASSWORD");
```

---

### Problem 4: Hoher Memory-Verbrauch bei großen Backups

**Symptom:** App verwendet mehrere GB RAM während Backup.

**Ursache:** rustic_core lädt Index komplett in Memory.

**Lösung:**

```rust
// Verwende streaming statt kompletten Index
let backup_opts = BackupOptions::default()
    .with_atime(false)  // Reduziert Metadata
    .pack_size(32 * 1024 * 1024);  // Kleinere Packs (32 MB statt 128 MB)

// Chunker-Parameter anpassen
let chunker_opts = ChunkerOptions::default()
    .avg_size(1024 * 1024);  // Kleinere Chunks
```

**Zusätzlich: Memory-Limit setzen**

```rust
// In main.rs
use tikv_jemalloc_ctl::{stats, epoch};

// Memory-Limit setzen (z.B. 2 GB)
std::env::set_var("MALLOC_CONF", "narenas:1,lg_tcache_max:13");
```

---

### Problem 5: Restore-Progress zeigt keine Werte

**Symptom:** Progress-Bar bei Restore bleibt bei 0%.

**Ursache:** rustic_core sendet Progress-Events nur bei Callbacks.

**Lösung:**

```rust
use rustic_core::Progress;

pub async fn restore_with_progress(
    repo: &Repository,
    snapshot_id: &str,
    target: PathBuf,
    app_handle: tauri::AppHandle,
) -> Result<()> {
    let snap = repo.get_snapshot(snapshot_id)?;
    let dest = LocalDestination::new(target)?;
    
    // Progress-Callback erstellen
    let progress = Progress::new(move |info| {
        app_handle.emit_all("restore-progress", json!({
            "files_processed": info.files_done,
            "files_total": info.files_total,
            "bytes_processed": info.bytes_done,
            "bytes_total": info.bytes_total,
        })).ok();
    });
    
    // Restore mit Progress
    let restore_opts = RestoreOptions::default()
        .set_progress(progress);
    
    repo.restore(&snap, dest, &restore_opts)?;
    
    Ok(())
}
```

---

### Problem 6: rclone-Init schlägt fehl

**Symptom:**
```
Error: rclone-Fehler: remote not found
```

**Debug-Steps:**

```rust
// Logging aktivieren
std::env::set_var("RUST_LOG", "debug");

// rclone-Config-Pfad prüfen
let rclone_config = dirs::config_dir()
    .unwrap()
    .join("rclone")
    .join("rclone.conf");

tracing::debug!("rclone config: {:?}", rclone_config);

if !rclone_config.exists() {
    return Err("rclone.conf nicht gefunden!".into());
}

// Remote-Liste abrufen
let output = std::process::Command::new("rclone")
    .arg("listremotes")
    .output()?;

tracing::debug!("Verfügbare Remotes: {}", 
    String::from_utf8_lossy(&output.stdout));
```

**Workaround bei rclone-Problemen:**

```rust
// Falls librclone nicht funktioniert: Subprocess verwenden
async fn init_with_rclone_subprocess(
    repo_path: &str,
) -> Result<()> {
    let output = tokio::process::Command::new("rclone")
        .args(&[
            "mkdir",
            &format!("{}/config", repo_path),
        ])
        .output()
        .await?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("rclone mkdir failed: {}", stderr);
    }
    
    // ... weitere Verzeichnisse erstellen
    
    Ok(())
}
```

---

### Problem 7: Snapshot-Vergleich zu langsam

**Symptom:** Compare-Dialog braucht mehrere Sekunden zum Laden.

**Ursache:** Kompletter Dateibaum wird geladen und verglichen.

**Optimierung:**

```rust
/// Lädt nur Diff zwischen zwei Snapshots (effizienter).
pub async fn compare_snapshots_optimized(
    repo: &Repository,
    snapshot_a_id: &str,
    snapshot_b_id: &str,
) -> Result<SnapshotDiff> {
    let snap_a = repo.get_snapshot(snapshot_a_id)?;
    let snap_b = repo.get_snapshot(snapshot_b_id)?;
    
    // Verwende rustic_core's eingebauten Diff
    let diff = repo.diff_snapshots(&snap_a, &snap_b)?;
    
    // Konvertiere zu Frontend-Format (nur Änderungen!)
    let changes: Vec<FileChange> = diff.changes
        .into_iter()
        .map(|change| FileChange {
            path: change.path,
            change_type: match change.typ {
                ChangeType::Added => "added",
                ChangeType::Removed => "removed",
                ChangeType::Modified => "modified",
            }.to_string(),
            size_before: change.size_before,
            size_after: change.size_after,
        })
        .collect();
    
    Ok(SnapshotDiff {
        added_count: diff.stats.added,
        removed_count: diff.stats.removed,
        modified_count: diff.stats.modified,
        size_diff: diff.stats.size_diff,
        changes,
    })
}
```

---

### Problem 8: Windows-Pfade mit Backslashes

**Symptom:** Backup schlägt fehl bei Windows-Pfaden wie `C:\Users\...`.

**Ursache:** Backslashes müssen escaped werden.

**Lösung:**

```rust
use std::path::Path;

/// Normalisiert Pfade für verschiedene OS.
pub fn normalize_path(path: &str) -> String {
    let path = Path::new(path);
    
    // Konvertiere zu UTF-8 String
    path.to_string_lossy()
        .replace('\\', "/")  // Windows Backslashes -> Forward Slashes
}

// Verwendung
let normalized = normalize_path(r"C:\Users\Max\Documents");
// Result: "C:/Users/Max/Documents"
```

**In Tauri Commands:**

```rust
#[tauri::command]
pub async fn add_source_path(
    path: String,
) -> Result<String, String> {
    // Normalisiere Pfad
    let normalized = normalize_path(&path);
    
    // Validiere Pfad
    if !Path::new(&normalized).exists() {
        return Err(format!("Pfad existiert nicht: {}", normalized));
    }
    
    Ok(normalized)
}
```

---

### Debug-Helpers

#### Logging-Setup mit Filtering

```rust
// main.rs
use tracing_subscriber::{EnvFilter, fmt};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info")
                .add_directive("rustic_gui=debug".parse().unwrap())
                .add_directive("rustic_core=info".parse().unwrap())
                .add_directive("rustic_backend=info".parse().unwrap())
        });
    
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}

// Start mit: RUST_LOG=debug cargo run
```

#### Repository-Health-Check Command

```rust
/// Führt einen umfassenden Health-Check des Repositories durch.
#[tauri::command]
pub async fn check_repository_health(
    state: State<'_, AppState>,
) -> Result<HealthReport, String> {
    let repo = state.current_repo.lock().unwrap()
        .as_ref()
        .ok_or("Kein Repository geöffnet")?
        .clone();
    
    let mut report = HealthReport::default();
    
    // 1. Check: Repository erreichbar
    report.is_accessible = repo.test_connection().is_ok();
    
    // 2. Check: Index konsistent
    report.index_ok = repo.check_index().is_ok();
    
    // 3. Check: Pack-Files validieren
    let pack_check = repo.check_packs().await;
    report.packs_ok = pack_check.is_ok();
    report.pack_errors = pack_check.err().map(|e| e.to_string());
    
    // 4. Check: Snapshots laden
    let snapshots = repo.get_all_snapshots();
    report.snapshot_count = snapshots.as_ref().map(|s| s.len()).unwrap_or(0);
    report.snapshots_ok = snapshots.is_ok();
    
    // 5. Check: Locks
    report.has_locks = !repo.list_locks()?.is_empty();
    
    Ok(report)
}

#[derive(Default, Serialize)]
struct HealthReport {
    is_accessible: bool,
    index_ok: bool,
    packs_ok: bool,
    pack_errors: Option<String>,
    snapshot_count: usize,
    snapshots_ok: bool,
    has_locks: bool,
}
```

---

### Performance-Profiling

```rust
// Cargo.toml
[profile.release]
debug = true  // Symbole für Profiling

// Mit cargo flamegraph profilen
# cargo install flamegraph
# cargo flamegraph --bin rustic-gui

// Oder mit perf (Linux)
# cargo build --release
# perf record -g ./target/release/rustic-gui
# perf report
```

---

## 💡 Tipps & Tricks

### 1. Development-Modus erkennen

```rust
#[cfg(debug_assertions)]
const IS_DEV: bool = true;

#[cfg(not(debug_assertions))]
const IS_DEV: bool = false;

// Verwenden
if IS_DEV {
    tracing::warn!("Development-Modus aktiv!");
}
```

### 2. Graceful Shutdown

```rust
// main.rs
fn main() {
    tauri::Builder::default()
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Verhindere sofortiges Schließen
                api.prevent_close();
                
                // Prüfe ob Backups laufen
                let app = event.window().app_handle();
                let state = app.state::<AppState>();
                
                let running = state.cancellation_tokens.lock().unwrap().len();
                
                if running > 0 {
                    // Zeige Bestätigungs-Dialog
                    tauri::api::dialog::blocking::ask(
                        Some(&event.window()),
                        "Backup läuft",
                        format!("{} Backup(s) laufen noch. Trotzdem beenden?", running),
                        |answer| {
                            if answer {
                                // Breche alle Backups ab
                                // ... dann exit
                                std::process::exit(0);
                            }
                        },
                    );
                } else {
                    // Keine Backups -> normal schließen
                    std::process::exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Auto-Save Config

```rust
// Automatisches Speichern bei Änderungen
impl AppState {
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.lock().unwrap();
        updater(&mut config);
        save_config(&config)?;
        Ok(())
    }
}

// Verwendung
state.update_config(|cfg| {
    cfg.backup_jobs.push(new_job);
})?;
```

### 4. Retry-Logic für Network-Fehler

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
            Err(e) if attempts < max_retries => {
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

// Verwendung
let snapshots = with_retry(
    || Box::pin(list_snapshots(&repo)),
    3
).await?;
```

---

## 📚 Weiterführende Ressourcen

### Dokumentation

- **rustic_core Docs**: https://docs.rs/rustic_core
- **rustic_backend Docs**: https://docs.rs/rustic_backend
- **Tauri Docs**: https://tauri.app/v2/
- **Svelte 5 Docs**: https://svelte-5-preview.vercel.app/

### Community

- **rustic Discord**: https://discord.gg/WRUWENZnzQ
- **Tauri Discord**: https://discord.com/invite/tauri

### Beispiel-Projekte

- **rustic CLI Source**: https://github.com/rustic-rs/rustic
- **Tauri Examples**: https://github.com/tauri-apps/tauri/tree/dev/examples

---

## 🎓 Code-Review-Checkliste

Vor jedem Commit/PR diese Punkte prüfen:

### ✅ Code-Qualität

- [ ] Alle neuen Funktionen haben Docstrings (Deutsch)
- [ ] Komplexe Logik hat erklärende Kommentare
- [ ] Keine `unwrap()` ohne guten Grund (nutze `?` oder `unwrap_or`)
- [ ] Error-Handling ist vollständig (alle Fehler werden behandelt)
- [ ] Keine Passwörter/Secrets in Logs
- [ ] Memory-Leaks geprüft (insb. bei async/Callbacks)

### ✅ Tests

- [ ] Unit-Tests für neue Business-Logik
- [ ] Integration-Tests für rustic_core-Integrationen
- [ ] Edge-Cases getestet (leere Inputs, sehr große Dateien, etc.)
- [ ] Alle Tests laufen durch (`cargo test && npm test`)

### ✅ Performance

- [ ] Keine synchronen Blocking-Calls in UI-Thread
- [ ] Große Operations laufen asynchron mit Progress
- [ ] Datenbankabfragen/Disk-I/O optimiert
- [ ] Memory-Profiling bei großen Datenmengen durchgeführt

### ✅ Sicherheit

- [ ] User-Input wird validiert
- [ ] Pfade werden normalisiert/sanitized
- [ ] Passwörter werden sicher gehandhabt (Keychain, kein Plaintext)
- [ ] Keine SQL-Injection (falls DB verwendet)
- [ ] Keine Command-Injection (bei Shell-Calls)

### ✅ UI/UX

- [ ] Loading-States zeigen während Operations
- [ ] Error-Messages sind user-friendly
- [ ] Keyboard-Navigation funktioniert
- [ ] Responsive Design getestet (Window-Resize)
- [ ] Dark-Mode berücksichtigt

### ✅ Dokumentation

- [ ] README.md aktualisiert (bei neuen Features)
- [ ] CHANGELOG.md Eintrag hinzugefügt
- [ ] API-Docs aktualisiert (TypeDoc/Rustdoc)
- [ ] Code-Kommentare sind aktuell

### ✅ Git

- [ ] Commit-Message folgt Conventional Commits
- [ ] Branch-Name ist beschreibend
- [ ] Keine Debug-Prints/Console.logs committed
- [ ] `.gitignore` ist aktuell

---

## 🚀 Release-Prozess

### Vorbereitung

```bash
# 1. Version bumpen
# package.json
"version": "0.2.0"

# src-tauri/Cargo.toml
version = "0.2.0"

# src-tauri/tauri.conf.json
"version": "0.2.0"

# 2. CHANGELOG aktualisieren
# CHANGELOG.md
## [0.2.0] - 2025-10-26

### Added
- Job-Scheduling mit Cron-Expressions
- Snapshot-Vergleich mit visueller Diff-Anzeige

### Fixed
- Restore-Berechtigungen werden korrekt wiederhergestellt

### Changed
- Performance-Verbesserung bei großen Repositories

# 3. Tests laufen lassen
npm test
cargo test

# 4. Commit & Tag
git add .
git commit -m "chore: Release v0.2.0"
git tag v0.2.0
git push origin main --tags
```

### Build-Prozess

```bash
# Lokaler Build für Test
npm run tauri:build

# Artifacts in: src-tauri/target/release/bundle/

# Linux AppImage: rustic-gui_0.2.0_amd64.AppImage
# Windows MSI: rustic-gui_0.2.0_x64_en-US.msi
# Windows Portable: rustic-gui.exe
```

### GitHub Release (via CI)

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: rustic-gui_${{ github.ref_name }}_amd64.AppImage
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: rustic-gui_${{ github.ref_name }}_x64_en-US.msi
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '20'
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build
        run: npm run tauri:build
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/appimage/*.AppImage
            src-tauri/target/release/bundle/msi/*.msi
            src-tauri/target/release/rustic-gui.exe
          body: |
            ## Changes in ${{ github.ref_name }}
            
            See [CHANGELOG.md](CHANGELOG.md) for details.
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## 📖 Glossar

### Backup-Begriffe

- **Repository**: Speicherort für Backups (lokal oder Cloud)
- **Snapshot**: Zustand der Daten zu einem bestimmten Zeitpunkt
- **Pack**: Container-Datei mit Backup-Daten (komprimiert & verschlüsselt)
- **Index**: Metadaten-Struktur für schnelles Suchen
- **Blob**: Einzelner Daten-Chunk innerhalb eines Packs
- **Tree**: Verzeichnis-Struktur eines Snapshots
- **Node**: Datei oder Verzeichnis in einem Tree
- **Retention**: Aufbewahrungsrichtlinie für alte Snapshots
- **Prune**: Löschen ungenutzter Daten aus Repository
- **Forget**: Markieren von Snapshots zum Löschen

### Technische Begriffe

- **Backend**: Speicher-Interface (local, SFTP, S3, etc.)
- **OpenDAL**: Data Access Layer für verschiedene Storage-Backends
- **rclone**: Tool für Cloud-Storage-Zugriff
- **Deduplication**: Speichern identischer Daten nur einmal
- **Chunking**: Aufteilen von Dateien in kleinere Blöcke
- **Content-Defined Chunking**: Variable Chunk-Größen basierend auf Inhalt
- **Rolling Hash**: Algorithmus für CDC (z.B. FastCDC)

### Rustic-Spezifisch

- **rustic_core**: Kern-Logik für Backup/Restore
- **rustic_backend**: Abstraktion für Storage-Backends
- **Config-File**: Repository-Konfiguration (JSON in repo/config)
- **Key-File**: Verschlüsselungs-Schlüssel (in repo/keys/)
- **Lock-File**: Mutex-Datei für Repository-Zugriff

---

## 🎨 UI-Komponenten-Bibliothek

### Wiederverwendbare Components

```bash
src/lib/components/
├── shared/
│   ├── Button.svelte          # Standard-Button mit Varianten
│   ├── Modal.svelte           # Dialog-Wrapper
│   ├── Toast.svelte           # Notification-Component
│   ├── LoadingSpinner.svelte  # Spinner für Loading-States
│   ├── ProgressBar.svelte     # Progress-Indikator
│   ├── FileTree.svelte        # Dateibaum (für Restore)
│   ├── TagList.svelte         # Tag-Anzeige/Eingabe
│   └── ContextMenu.svelte     # Rechtsklick-Menu
├── dialogs/
│   ├── AddRepositoryDialog.svelte
│   ├── CreateJobDialog.svelte
│   ├── RestoreDialog.svelte
│   └── ConfirmDialog.svelte
├── layout/
│   ├── Sidebar.svelte
│   ├── Header.svelte
│   ├── MainLayout.svelte
│   └── TabContainer.svelte
└── pages/
    ├── Dashboard.svelte
    ├── Repositories.svelte
    ├── Snapshots.svelte
    ├── BackupJobs.svelte
    └── Settings.svelte
```

### Component-Props-Pattern

```html
<!-- Button.svelte -->
<script lang="ts">
  /**
   * Wiederverwendbarer Button mit verschiedenen Varianten.
   * 
   * @component
   */
  
  /** Button-Text */
  export let label: string;
  
  /** Button-Variante */
  export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
  
  /** Größe des Buttons */
  export let size: 'small' | 'medium' | 'large' = 'medium';
  
  /** Deaktiviert den Button */
  export let disabled = false;
  
  /** Zeigt Loading-Spinner */
  export let loading = false;
  
  /** Icon links vom Text (optional) */
  export let icon: string | undefined = undefined;
  
  // Event Dispatcher
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher<{ click: MouseEvent }>();
  
  function handleClick(event: MouseEvent) {
    if (!disabled && !loading) {
      dispatch('click', event);
    }
  }
</script>

<button
  class="btn btn-{variant} btn-{size}"
  class:disabled
  class:loading
  on:click={handleClick}
  {disabled}
>
  {#if loading}
    <span class="spinner" />
  {:else if icon}
    <span class="icon">{icon}</span>
  {/if}
  <span>{label}</span>
</button>

<style>
  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    transition: all 0.2s;
  }
  
  .btn-primary {
    background: #3b82f6;
    color: white;
  }
  
  .btn-primary:hover:not(.disabled) {
    background: #2563eb;
  }
  
  .btn.loading {
    pointer-events: none;
    opacity: 0.7;
  }
  
  .spinner {
    /* ... */
  }
</style>
```

### Store-Pattern für State-Management

```typescript
// src/lib/stores/repositories.ts

import { writable, derived } from 'svelte/store';
import type { Repository } from '$lib/types';

/**
 * Store für Repository-Verwaltung.
 * 
 * Enthält alle konfigurierten Repositories und das aktuell aktive.
 */

/** Liste aller Repositories */
export const repositories = writable<Repository[]>([]);

/** ID des aktiven Repositories */
export const activeRepositoryId = writable<string | null>(null);

/** Aktives Repository (derived store) */
export const activeRepository = derived(
  [repositories, activeRepositoryId],
  ([$repos, $activeId]) => {
    if (!$activeId) return null;
    return $repos.find(r => r.id === $activeId) ?? null;
  }
);

/** Lädt Repositories aus Backend */
export async function loadRepositories() {
  const repos = await invoke<Repository[]>('get_all_repositories');
  repositories.set(repos);
}

/** Fügt ein neues Repository hinzu */
export async function addRepository(repo: Repository) {
  await invoke('add_repository', { repo });
  repositories.update(repos => [...repos, repo]);
}

/** Entfernt ein Repository */
export async function removeRepository(id: string) {
  await invoke('remove_repository', { id });
  repositories.update(repos => repos.filter(r => r.id !== id));
}

/** Wechselt das aktive Repository */
export async function switchRepository(id: string, password: string) {
  await invoke('switch_repository', { repoId: id, password });
  activeRepositoryId.set(id);
}
```

---

## 🔧 Entwicklungs-Workflow

### Typischer Feature-Entwicklungs-Zyklus

```bash
# 1. Feature-Branch erstellen
git checkout -b feature/snapshot-tags

# 2. Backend implementieren (Rust)
# src-tauri/src/snapshot/tags.rs
# - Funktion zum Hinzufügen von Tags
# - Tests schreiben

# 3. Tauri Command hinzufügen
# src-tauri/src/main.rs
# - add_snapshot_tag Command registrieren

# 4. TypeScript Types aktualisieren
# src/lib/types.ts
# - SnapshotDto erweitern

# 5. API-Wrapper implementieren
# src/lib/api/snapshots.ts
# - addSnapshotTag() Funktion

# 6. UI Component erstellen
# src/lib/components/TagEditor.svelte
# - Input für neue Tags
# - Tag-Liste mit Remove-Buttons

# 7. In Page integrieren
# src/lib/pages/Snapshots.svelte
# - TagEditor einbinden
# - Event-Handler

# 8. Testen
npm run test
cargo test

# 9. Formatieren & Linten
npm run format
npm run lint
cargo fmt
cargo clippy

# 10. Commit & Push
git add .
git commit -m "feat(snapshots): Tag-Verwaltung implementiert"
git push origin feature/snapshot-tags

# 11. Pull Request erstellen
# - Beschreibung hinzufügen
# - Screenshots einfügen
# - Review anfordern
```

### Hot-Reload während Entwicklung

```bash
# Terminal 1: Backend mit Hot-Reload
cd src-tauri
cargo watch -x run

# Terminal 2: Frontend mit Hot-Reload
npm run dev

# Terminal 3: Logs beobachten
tail -f ~/.local/share/rustic-gui/logs/app.log
```

---

## 🧩 Architektur-Diagramme

### Gesamt-Übersicht

```bash
┌─────────────────────────────────────────────┐
│           Frontend (Svelte + TS)            │
│  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Pages     │  │    Components       │  │
│  │  Dashboard  │  │  - Dialogs          │  │
│  │  Snapshots  │  │  - FileTree         │  │
│  │  Jobs       │  │  - ProgressBar      │  │
│  └──────┬──────┘  └──────────┬──────────┘  │
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
                   │ IPC (invoke)
┌──────────────────▼──────────────────────────┐
│          Backend (Tauri + Rust)             │
│  ┌──────────────────────────────────────┐  │
│  │        Tauri Commands                │  │
│  │  - run_backup                        │  │
│  │  - list_snapshots                    │  │
│  │  - restore_files                     │  │
│  └──────────────┬───────────────────────┘  │
│                 │                           │
│  ┌──────────────▼───────────────────────┐  │
│  │      Rustic Integration              │  │
│  │  ┌────────────┐  ┌────────────────┐ │  │
│  │  │ rustic_core│  │rustic_backend  │ │  │
│  │  │ - Backup   │  │ - OpenDAL      │ │  │
│  │  │ - Restore  │  │ - rclone FFI   │ │  │
│  │  │ - Snapshots│  │                │ │  │
│  │  └────────────┘  └────────────────┘ │  │
│  └─────────────────────────────────────┘  │
│                 │                          │
│  ┌──────────────▼───────────────────────┐ │
│  │      Storage Layer                   │ │
│  │  - Config (TOML)                     │ │
│  │  - Keychain (Passwords)              │ │
│  │  - Scheduler (Cron Jobs)             │ │
│  └──────────────────────────────────────┘ │
└────────────────────────────────────────────┘
                   │
                   ▼
        ┌──────────────────┐
        │  rustic Repository│
        │  (local/cloud)    │
        └──────────────────┘
```

### Backup-Flow

```bash
User klickt "Run Backup"
         │
         ▼
┌────────────────────┐
│ Frontend           │
│ BackupJob.svelte   │
└────────┬───────────┘
         │ invoke('run_backup')
         ▼
┌────────────────────┐
│ Tauri Command      │
│ run_backup()       │
└────────┬───────────┘
         │
         ▼
┌────────────────────┐
│ Repository laden   │
│ (aus AppState)     │
└────────┬───────────┘
         │
         ▼
┌────────────────────┐
│ rustic_core        │
│ repo.backup()      │
└────────┬───────────┘
         │ Progress-Callbacks
         ├──────────────────┐
         │                  │
         ▼                  ▼
┌────────────────┐  ┌──────────────┐
│ Tauri Events   │  │ Data schreiben│
│ emit_all()     │  │ in Repository │
└────────┬───────┘  └──────────────┘
         │
         ▼
┌────────────────────┐
│ Frontend Updates   │
│ Progress-Bar       │
└────────────────────┘
```

---

## 📝 Abschluss-Checkliste für AI

Wenn du als AI an diesem Projekt arbeitest, stelle sicher:

### Vor dem Code schreiben:

- [ ] Instructions.md komplett gelesen
- [ ] Mockups angeschaut (GUI-Design verstanden)
- [ ] Sprachkonventionen verstanden (Deutsch/Englisch Mix)
- [ ] Projekt-Struktur klar

### Während Code-Generierung:

- [ ] Docstrings auf Deutsch
- [ ] Code-Elemente auf Englisch
- [ ] Error-Handling vollständig
- [ ] TypeScript Types korrekt
- [ ] Rust Ownership/Borrowing beachtet
- [ ] Keine Passwörter/Secrets in Logs

### Nach Code-Generierung:

- [ ] Code kompiliert (`cargo check` / `npm run type-check`)
- [ ] Tests laufen durch
- [ ] Formatierung korrekt (`cargo fmt` / `npm run format`)
- [ ] Keine Warnungen (`cargo clippy` / `npm run lint`)

### Bei Unsicherheit:

- [ ] Frage nach bevor du rätst
- [ ] Gebe Alternativen an
- [ ] Erkläre Trade-Offs
- [ ] Verweise auf Best Practices in diesem Dokument

---

## 🎉 Ende der Instructions

Dieses Dokument ist lebendig und sollte bei neuen Erkenntnissen aktualisiert werden!

**Viel Erfolg beim Entwickeln!** 🚀

Bei Fragen: Siehe Troubleshooting-Sektion oder frage den menschlichen Entwickler.

---

*Version: 1.0*  
*Letzte Aktualisierung: 2025-10-26*  
*Autor: Rustic GUI Team*# Rustic GUI - AI Development Instructions

> **Umfassende Anweisungen für KI-gestützte Entwicklung des Rustic GUI Projekts**
> 
> Version: 1.0 | Datum: 2025-10-25 | Sprache: Deutsch/Englisch (hybrid)

---
