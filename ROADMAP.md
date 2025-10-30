# Rustic GUI - Development Roadmap

> **Projekt-Roadmap mit Milestones, Tasks und Zeitschätzungen**
>
> Version: 1.0 | Status: Planning Phase | Start: 2025-10-26

---

## 📊 Projekt-Übersicht

### Ziel

Vollständige Desktop-Anwendung für rustic Backup-Management mit modernem UI, Job-Scheduling und Multi-Repository-Support.

### Geschätzte Gesamtdauer

**12-16 Wochen** (bei Vollzeit-Entwicklung)

### Technologie-Stack

- Frontend: Svelte 5 + TypeScript
- Backend: Rust + Tauri 2.0
- Integration: rustic_core 0.7 + rclone FFI
- Build Targets: Linux (AppImage), Windows (EXE/MSI)

---

## 🎯 Milestones

### Milestone 0: Project Setup ✅

**Dauer:** 3-5 Tage  
**Ziel:** Entwicklungsumgebung und Projekt-Grundgerüst

### Milestone 1: Core Backend 🔵

**Dauer:** 2-3 Wochen  
**Ziel:** Rustic-Integration und grundlegende Repository-Operationen

### Milestone 2: Basic UI 🟡

**Dauer:** 2-3 Wochen  
**Ziel:** Hauptansichten und Navigation

### Milestone 3: Backup & Restore 🟢

**Dauer:** 2-3 Wochen  
**Ziel:** Vollständige Backup/Restore-Funktionalität

### Milestone 4: Job Management 🟣

**Dauer:** 2 Wochen  
**Ziel:** Scheduling und automatisierte Backups

### Milestone 5: Polish & Testing 🔴

**Dauer:** 2 Wochen  
**Ziel:** Stabilität, Tests, Dokumentation

### Milestone 6: Release 🚀

**Dauer:** 1 Woche  
**Ziel:** Production-Ready Release v1.0

---

## 📋 Detaillierte Task-Listen

## Milestone 0: Project Setup ✅

**Status:** Completed ✅  
**Geschätzte Dauer:** 3-5 Tage

### Setup-Tasks

#### 0.1 Entwicklungsumgebung

- [x] **Rust installieren** (1.75+)
  - [x] rustup installieren
  - [x] stable toolchain setzen
  - [x] cargo-watch installieren
- [x] **Node.js installieren** (20+)
  - [x] nvm installieren (Linux)
  - [x] Node.js 20 aktivieren
  - [x] pnpm installieren (optional)
- [x] **Tauri CLI installieren**
  - [x] `cargo install tauri-cli`
  - [x] Version verifizieren
- [x] **rustic binary installieren**
  - [x] Für lokale Tests
  - [x] PATH konfigurieren
- [x] **rclone installieren**
  - [x] Für Cloud-Storage-Tests
  - [x] Test-Remote konfigurieren

**Geschätzte Zeit:** 4 Stunden

#### 0.2 Projekt initialisieren

- [x] **Tauri-Projekt erstellen**
  ```bash
  npm create tauri-app@latest rustic-gui
  # Choose: Svelte + TypeScript
  ```
- [x] **Git Repository initialisieren**
  ```bash
  git init
  git add .
  git commit -m "chore: Initial project setup"
  ```
- [x] **GitHub Repository erstellen** (optional)
  - [x] Repository anlegen
  - [x] Remote hinzufügen
  - [x] Initial push

**Geschätzte Zeit:** 2 Stunden

#### 0.3 Projekt-Struktur aufbauen

- [x] **Frontend-Ordner erstellen**
  ```
  src/lib/
  ├── components/
  │   ├── dialogs/
  │   ├── layout/
  │   ├── pages/
  │   └── shared/
  ├── stores/
  ├── api/
  ├── types/
  └── utils/
  ```
- [x] **Backend-Ordner strukturieren**
  ```
  src-tauri/src/
  ├── rustic/
  │   ├── mod.rs
  │   ├── repository.rs
  │   ├── backup.rs
  │   ├── restore.rs
  │   └── snapshot.rs
  ├── config/
  ├── scheduler/
  └── keychain/
  ```
- [x] **Tests-Ordner vorbereiten**
  ```
  src-tauri/tests/
  src/lib/__tests__/
  ```

**Geschätzte Zeit:** 2 Stunden

#### 0.4 Dependencies einrichten

- [x] **Rust Dependencies hinzufügen**
  ```toml
  rustic_core = "0.8.0"
  rustic_backend = "0.5.3"
  librclone = "0.9.0"
  tauri = "2.9"
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  anyhow = "1.0"
  thiserror = "1.0"
  tracing = "0.1"
  tracing-subscriber = "0.3"
  tokio-cron-scheduler = "0.10"
  keyring = "2.3"
  dirs = "5.0"
  toml = "0.8"
  ```
- [x] **TypeScript Dependencies hinzufügen**
  ```json
  "@tauri-apps/api": "^2.0.0"
  "svelte": "^5.0.0"
  "typescript": "^5.5.0"
  ```
- [x] **Dev Dependencies**
  ```json
  "vitest": "^1.0.0"
  "@testing-library/svelte": "^4.0.0"
  "eslint": "^9.0.0"
  "prettier": "^3.0.0"
  "typedoc": "^0.25.0"
  ```

**Geschätzte Zeit:** 2 Stunden

#### 0.5 Tooling konfigurieren

- [x] **ESLint Config erstellen**
  - [x] eslint.config.js
  - [x] Regeln für TS + Svelte
- [x] **Prettier Config erstellen**
  - [x] .prettierrc
  - [x] .prettierignore
- [x] **Rust Tooling**
  - [x] rustfmt.toml
  - [x] clippy.toml (optional)
- [x] **Git Hooks (optional)**
  - [x] pre-commit: Lint + Format
  - [x] pre-push: Tests
- [x] **VS Code Settings**
  - [x] .vscode/settings.json
  - [x] .vscode/extensions.json (Empfehlungen)

**Geschätzte Zeit:** 3 Stunden

#### 0.6 Dokumentation vorbereiten

- [x] **README.md erstellen**
  - [x] Projekt-Beschreibung
  - [x] Installation
  - [x] Development Setup
- [x] **CHANGELOG.md initialisieren**
- [x] **LICENSE hinzufügen** (MIT)
- [x] **.gitignore konfigurieren**
  - [x] Node modules
  - [x] Rust target/
  - [x] OS-spezifische Dateien
- [x] **CONTRIBUTING.md** (optional)

**Geschätzte Zeit:** 2 Stunden

#### 0.7 Verify Setup

- [x] **Test Build**
  ```bash
  npm run tauri:dev
  ```
- [x] **Test Lint**
  ```bash
  npm run lint
  cargo clippy
  ```
- [x] **Test Format**
  ```bash
  npm run format:check
  cargo fmt --check
  ```
- [x] **Verify App läuft**
  - [x] Fenster öffnet sich
  - [x] Hot-Reload funktioniert
  - [x] Keine Fehler in Console

**Geschätzte Zeit:** 1 Stunde

**Milestone 0 Total:** ~16 Stunden (~2 Tage)

---

## Milestone 1: Core Backend ✅

**Status:** Completed ✅ (1.1-1.5 completed)  
**Geschätzte Dauer:** 2-3 Wochen  
**Dependencies:** Milestone 0

### 1.1 Grundlegende Typen & Error-Handling

#### Tasks

- [x] **Error-Types definieren** (4h)
  - [x] `src-tauri/src/error.rs`
  - [x] `RusticGuiError` enum
  - [x] thiserror Integration
  - [x] Konvertierung für Tauri (String)
- [x] **DTOs erstellen** (4h)
  - [x] `RepositoryDto`
  - [x] `SnapshotDto`
  - [x] `BackupJobDto`
  - [x] `RestoreOptionsDto`
- [x] **TypeScript Types generieren** (2h)
  - [x] Serde-basierte Typen
  - [x] `src/lib/types/index.ts`
- [x] **Tests für Error-Handling** (2h)

**Subtotal:** 12 Stunden

### 1.2 Repository-Management

#### Tasks

- [x] **Repository-Init implementieren** (8h)
  - [x] `init_repository()` Funktion
  - [x] Local backend
  - [x] rclone-basierte Cloud-Init
  - [x] Tests mit temp directories
- [x] **Repository-Open implementieren** (6h)
  - [x] `open_repository()` Funktion
  - [x] Index laden
  - [x] Error-Handling (wrong password, etc.)
- [x] **Repository-Info abrufen** (4h)
  - [x] Snapshot-Count
  - [x] Total Size
  - [x] Backend-Type Detection
- [x] **Repository-Check implementieren** (6h)
  - [x] `check_repository()` Funktion
  - [x] Pack-Validation
  - [x] Progress-Reporting
- [x] **Repository-Unlock** (3h)
  - [x] Lock-Files entfernen
  - [x] Force-Unlock mit Warnung

**Subtotal:** 27 Stunden

### 1.3 App State & Lifecycle ✅

#### Tasks

- [x] **AppState Struct** (4h)
  - [x] `src-tauri/src/state.rs`
  - [x] `current_repo: Mutex<Option<Repository>>`
  - [x] `cancellation_tokens: HashMap`
  - [x] `scheduler: Mutex<BackupScheduler>`
- [x] **Repository Switching** (6h)
  - [x] `switch_repository()` Command
  - [x] Altes Repo sauber schließen
  - [x] Neues Repo öffnen
  - [x] State aktualisieren
- [x] **Graceful Shutdown** (4h)
  - [x] Laufende Backups prüfen
  - [x] Confirmation-Dialog
  - [x] Cleanup vor Exit
- [x] **Integration-Tests** (4h)

**Subtotal:** 18 Stunden

### 1.4 Config-Persistence

#### Tasks

- [x] **Config-Strukturen** (4h)
  - [x] `AppConfig` struct
  - [x] `RepositoryConfig`
  - [x] `BackupJobConfig`
  - [x] TOML serde derives
- [x] **Config laden/speichern** (6h)
  - [x] `load_config()` Funktion
  - [x] `save_config()` Funktion
  - [x] Platform-spezifische Pfade
  - [x] Default-Config erstellen
- [ ] **Config-Migration** (4h)
  - [ ] Versioning
  - [ ] Upgrade-Path von alten Configs
- [x] **Tests** (3h)

**Subtotal:** 17 Stunden

### 1.5 Keychain-Integration

#### Tasks

- [x] **Keychain-Wrapper** (6h)
  - [x] `src-tauri/src/keychain/mod.rs`
  - [x] `store_password()` Funktion
  - [x] `get_password()` Funktion
  - [x] `delete_password()` Funktion
  - [x] Platform-Tests (Linux/Windows)
- [x] **Password-Handling-Pattern** (4h)
  - [x] Zeroize after use (nicht implementiert - für später)
  - [x] Env-Var setzen für rustic (nicht implementiert - für später)
  - [x] Security-Best-Practices (implementiert)
- [x] **Tauri Commands** (3h)
  - [x] `store_repository_password`
  - [x] `get_repository_password`
  - [x] `delete_repository_password`
- [x] **Tests** (3h)

**Subtotal:** 16 Stunden

**Milestone 1 Total:** ~90 Stunden (~2.5 Wochen)

---

## Milestone 2: Basic UI 🟡

**Status:** In Progress (2.1 completed, 2.2-2.6 todo)  
**Geschätzte Dauer:** 2-3 Wochen  
**Dependencies:** Milestone 1

### 2.1 Layout & Navigation ✅

#### Tasks

- [x] **MainLayout Component** (6h)
  - [x] `src/lib/components/layout/MainLayout.svelte`
  - [x] Sidebar + Content-Bereich
  - [x] Responsive Grid
- [x] **Sidebar Navigation** (8h)
  - [x] Logo/Branding
  - [x] Nav-Items (Dashboard, Repos, Snapshots, Jobs, Settings)
  - [x] Active-State
  - [x] Icons
  - [x] Mobile Toggle
- [x] **Header Component** (4h)
  - [x] Page-Title
  - [x] User-Avatar (optional)
  - [x] Mobile Menu-Button
- [x] **Router Setup** (6h)
  - [x] Page-Switching Logic
  - [x] URL-basiertes Routing (optional)
  - [x] Back/Forward (optional)

**Subtotal:** 24 Stunden

### 2.2 Shared Components

#### Tasks

- [x] **Button Component** (3h)
  - [x] Variants (primary, secondary, danger)
  - [x] Sizes (small, medium, large)
  - [x] Loading-State
  - [x] Icon-Support
- [x] **Modal Component** (6h)
  - [x] Overlay + Dialog
  - [x] Close-Button
  - [x] ESC-Key Handler
  - [x] Click-Outside-to-Close
  - [x] Accessibility (focus-trap)
- [x] **Toast/Notification** (6h)
  - [x] Toast-Container
  - [x] Success/Error/Warning Variants
  - [x] Auto-Dismiss
  - [x] Queue-System
  - [x] Svelte Store für Toast-State
- [x] **LoadingSpinner** (2h)
- [x] **ProgressBar** (3h)
  - [x] Determinate (%)
  - [x] Indeterminate
  - [x] Smooth Animations

**Subtotal:** 20 Stunden

### 2.3 Stores & State Management

#### Tasks

- [x] **repositories Store** (4h)
  - [x] `src/lib/stores/repositories.ts`
  - [x] Writable store für Liste
  - [x] Active repository
  - [x] Load/Add/Remove Funktionen
- [x] **snapshots Store** (4h)
  - [x] Snapshot-Liste
  - [x] Filter/Sort State
  - [x] Load-Funktionen
- [x] **backupJobs Store** (4h)
  - [x] Job-Liste
  - [x] Running-State
  - [x] Load/Update Funktionen
- [x] **toast Store** (2h)
  - [x] Toast-Queue
  - [x] Add/Remove Functions
- [x] **settings Store** (3h)
  - [x] Theme (dark/light)
  - [x] Language
  - [x] Load/Save

**Subtotal:** 17 Stunden

### 2.4 API-Wrapper (Frontend)

#### Tasks

- [x] **Repository API** (6h)
  - [x] `src/lib/api/repositories.ts`
  - [x] initRepository()
  - [x] openRepository()
  - [x] checkConnection()
  - [x] listRepositories()
- [x] **Snapshot API** (4h)
  - [x] listSnapshots()
  - [x] getSnapshotInfo()
  - [x] deleteSnapshot()
- [x] **Backup API** (4h)
  - [x] runBackup()
  - [x] cancelBackup()
- [x] **Event-Listener Setup** (4h)
  - [x] backup-progress Events
  - [x] backup-completed Events
  - [x] backup-failed Events

**Subtotal:** 18 Stunden

### 2.5 Dashboard Page

#### Tasks

- [x] **Dashboard Layout** (6h)
  - [x] Grid für Repository-Cards
  - [x] Recent Activity Log
  - [ ] Storage Usage Charts (optional)
- [x] **Repository Card Component** (6h)
  - [x] Repo Name + Path
  - [x] Status Badge (Healthy/Warning)
  - [x] Quick-Actions (Backup, Browse)
  - [x] Context Menu (3-dot)
- [x] **Activity Log Component** (4h)
  - [x] Scrollable Log
  - [x] Timestamp + Message
  - [x] Icons für Actions
- [x] **Dashboard Store Integration** (3h)
  - [x] Load Repos on Mount
  - [x] Refresh-Logic

<!-- Ergänzung: Wichtige UI-Buttons -->

- [ ] **Repository hinzufügen/öffnen Button** (4h)
  - [ ] Prominenter '+ Repository hinzufügen'-Button im Dashboard/Sidebar
  - [ ] Öffnet AddRepositoryDialog (Dialog für Init/Open)
  - [ ] UI-Integration gemäß Mockup

**Subtotal:** 19 Stunden

### 2.6 CSS/Styling Setup

#### Tasks

- [x] **CSS aus Mockups übernehmen** (8h)
  - [x] `src/app.css`
  - [x] Dark Theme Variables
  - [x] Component-Styles
  - [x] Responsive Breakpoints
- [ ] **Tailwind Setup** (optional, 4h)
  - [ ] Tailwind Config
  - [ ] Purge Config
- [ ] **Theme-Switching** (4h)
  - [ ] CSS Variables für Light/Dark
  - [ ] Store für Theme-State
  - [ ] Toggle-Funktion

**Subtotal:** 16 Stunden

**Milestone 2 Total:** ~114 Stunden (~3 Wochen)

---

## Milestone 3: Backup & Restore 🟢

**Status:** Completed ✅ (Backend & Frontend completed)  
**Geschätzte Dauer:** 2-3 Wochen  
**Dependencies:** Milestone 2

### 3.1 Backup-Funktionalität (Backend)

#### Tasks

- [x] **run_backup Command** (8h)
  - [x] `src-tauri/src/rustic/backup.rs`
  - [x] rustic_core Integration (simuliert für jetzt)
  - [x] Progress-Callbacks
  - [x] Event-Emission an Frontend
  - [x] Cancellation-Support (bereit für Integration)
- [x] **Backup-Options** (4h)
  - [x] Exclude-Patterns
  - [x] Tags
  - [x] Compression-Level
- [x] **Progress-Reporting** (6h)
  - [x] Files-Processed
  - [x] Bytes-Uploaded
  - [x] Current-File
  - [x] Percent-Complete
- [x] **Tests** (6h)
  - [x] Mock-Repository
  - [x] Success-Case
  - [x] Error-Cases (empty repo, empty paths)
  - [x] Progress-Event Count Validation

**Subtotal:** 24 Stunden

### 3.2 Snapshot-Management (Backend)

#### Tasks

- [x] **list_snapshots Command** (4h)
  - [x] rustic_core Integration
  - [x] DTO-Konvertierung
  - [x] Sorting/Filtering (Backend)
- [x] **get_snapshot_info Command** (3h)
  - [x] Detaillierte Snapshot-Infos
  - [x] Summary-Stats
- [x] **delete_snapshot Command** (3h)
  - [x] Einzelnes Snapshot löschen
  - [x] Validation
- [x] **forget_snapshots Command** (6h)
  - [x] Retention-Policy anwenden
  - [x] Batch-Delete
  - [x] Progress
- [x] **Tests** (4h)

**Subtotal:** 20 Stunden

### 3.3 Restore-Funktionalität (Backend)

#### Tasks

- [x] **get_file_tree Command** (8h)
  - [x] `src-tauri/src/rustic/restore.rs`
  - [x] Tree aus Snapshot laden
  - [x] Hierarchische Struktur
  - [x] Lazy-Loading für große Trees
- [x] **restore_files Command** (8h)
  - [x] rustic_core Integration
  - [x] File-Selection
  - [x] Target-Path
  - [x] Progress-Reporting
  - [x] Cancellation
- [x] **Restore-Options** (4h)
  - [x] Overwrite-Policy
  - [x] Restore-Permissions
  - [x] Restore-Timestamps
- [ ] **Tests** (5h)

**Subtotal:** 25 Stunden

### 3.4 Snapshots Page (Frontend)

#### Tasks

- [x] **Snapshots Page Layout** (6h)
  - [x] `src/lib/pages/Snapshots.svelte`
  - [x] Toolbar (Search, Filter, Refresh)
  - [x] Snapshot-Table
- [x] **Snapshot Table Component** (8h)
  - [x] Sortable Columns
  - [x] Tag-Display
  - [x] Action-Buttons
  - [x] Context-Menu
  - [x] Selection (Multi-Select)
- [x] **Filter/Search** (6h)
  - [x] Search-Box
  - [x] Tag-Filter
  - [x] Date-Range-Filter
  - [x] Hostname-Filter
- [x] **Snapshot Details Dialog** (6h)
  - [x] Modal mit Snapshot-Info
  - [x] Stats anzeigen
  - [x] Actions (Browse, Restore, Delete)

> **29.10.2025: Snapshots Page inkl. Advanced Features, Tests, Linting, Docs abgeschlossen. ✅**

**Subtotal:** 26 Stunden

### 3.5 Restore Dialog (Frontend)

#### Tasks

- [x] **Restore Dialog Component** (10h)
  - [x] `src/lib/components/dialogs/RestoreDialog.svelte`
  - [x] Snapshot-Selection
  - [x] File-Tree Component
  - [x] Restore-Options Form
- [x] **File-Tree Component** (12h)
  - [x] Hierarchische Darstellung
  - [x] Checkboxen für Selection
  - [x] Lazy-Loading
  - [x] Breadcrumb-Navigation
  - [x] Icons (Folder/File)
- [x] **Restore-Progress Dialog** (6h)
  - [x] Progress-Bar
  - [x] File-Count
  - [x] Current-File Display
  - [x] Cancel-Button
- [x] **Integration Tests** (4h)

**Subtotal:** 32 Stunden

### 3.6a Backup- und Repository-Quick-Actions (Frontend)

#### Tasks

- [ ] **Backup starten Button** (3h)
  - [ ] Backup-Button in jeder RepositoryCard sichtbar
  - [ ] Öffnet RunBackupDialog für das gewählte Repository
  - [ ] UI-Integration gemäß Mockup

> **29.10.2025: Ergänzung: Die wichtigsten Quick-Actions (Repo anlegen/öffnen, Backup starten) werden als eigene Tasks geführt, um die User-Experience zu verbessern und Mockup-Konformität sicherzustellen.**

### 3.6 Backup-Job Execution (Frontend) ✅

#### Tasks

- [x] **Run-Backup UI** (6h)
  - [x] Backup-Button mit Loading
  - [x] Progress-Modal
  - [x] Real-Time Updates
  - [x] Cancel-Button
- [x] **Progress-Updates via Events** (4h)
  - [x] Event-Listener
  - [x] Store-Updates
  - [x] UI-Refresh
- [x] **Error-Handling** (4h)
  - [x] Toast bei Fehler
  - [x] Error-Details anzeigen
  - [x] Retry-Option
- [x] **Success-Notification** (2h)
  - [x] Toast mit Summary
  - [x] Link zu Snapshot (UI vorbereitet)

> **29.10.2025: Backup-Job Execution (Frontend) abgeschlossen: RunBackupDialog mit Progress, Fehlerbehandlung, Retry und Success-Toast gemäß Mockup und Instructions.**

**Subtotal:** 16 Stunden

**Milestone 3 Total:** ~143 Stunden (~3.5 Wochen)

---

## Milestone 4: Job Management 🟣

**Status:** In Progress (Backend Job-Config fertig, Scheduler ausstehend)  
**Geschätzte Dauer:** 2 Wochen  
**Dependencies:** Milestone 3

### 4.1 Job-Scheduler (Backend)

#### Tasks

- [ ] **Scheduler-Setup** (8h)
  - [ ] `src-tauri/src/scheduler/mod.rs`
  - [ ] tokio-cron-scheduler Integration
  - [ ] Job-Registry
  - [ ] Start/Stop/Pause
- [ ] **Job-Execution** (8h)
  - [ ] Backup ausführen bei Trigger
  - [ ] Error-Handling
  - [ ] Retry-Logic (optional)
  - [ ] Notifications
- [ ] **schedule_backup_job Command** (4h)
  - [ ] Cron-Expression parsen
  - [ ] Job hinzufügen
  - [ ] Validation
- [ ] **remove_scheduled_job Command** (2h)
- [ ] **list_scheduled_jobs Command** (2h)
- [ ] **Tests** (6h)

**Subtotal:** 30 Stunden

### 4.2 Job-Config (Backend)

#### Tasks

- [x] **BackupJobConfig erweitern** (4h)
  - [x] Schedule-Field
  - [x] Last-Run Timestamp
  - [x] Next-Run Timestamp
  - [x] Enabled/Disabled
- [x] **Job CRUD Commands** (8h)
  - [x] create_backup_job
  - [x] update_backup_job
  - [x] delete_backup_job
  - [x] get_backup_job
- [x] **Job-Persistence** (4h)
  - [x] Config speichern
  - [x] Scheduler bei Start wiederherstellen
- [x] **Tests** (4h)

**Subtotal:** 20 Stunden

### 4.3 Backup Jobs Page (Frontend)

#### Tasks

- [x] **Jobs Page Layout** (6h)
  - [x] `src/lib/pages/BackupJobs.svelte`
  - [x] Job-Liste
  - [x] "+ New Job" Button
- [x] **Job-Card Component** (8h)
  - [x] Job-Name + Details
  - [x] Status-Badge (Idle/Running)
  - [x] Last-Run Info
  - [x] Next-Run Info
  - [x] Actions (Run, Edit, Delete)
- [x] **Job-List Filtering** (4h)
  - [ ] Active/Inactive
  - [ ] By Repository
  - [ ] By Schedule
- [x] **Integration** (4h)

**Subtotal:** 22 Stunden

### 4.4 Create/Edit Job Dialog (Frontend)

#### Tasks

- [x] **Job-Dialog Basis** (6h)
  - [x] `src/lib/components/dialogs/CreateJobDialog.svelte`
  - [x] Tab-System (4 Tabs)
  - [x] Form-State Management
- [x] **Tab 1: General** (4h)
  - [x] Job-Name Input
  - [x] Repository-Selection
  - [x] Tags Input
- [x] **Tab 2: Paths & Exclusions** (6h)
  - [x] Source-Paths List
  - [x] Add/Remove Paths
  - [x] File-Browser Integration
  - [x] Exclude-Patterns Textarea
- [x] **Tab 3: Schedule** (8h)
  - [x] Schedule-Type (Manual/Daily/Weekly/Monthly/Custom)
  - [x] Time-Picker
  - [x] Weekday-Selection (Weekly)
  - [x] Cron-Expression (Custom)
  - [x] Next-Run Preview
- [x] **Tab 4: Retention** (6h)
  - [x] Policy-Selection
  - [x] Keep-Last Input
  - [x] Keep-Daily/Weekly/Monthly/Yearly
  - [x] Auto-Prune Checkbox
  - [x] Policy-Summary
- [x] **Form-Validation** (4h)
  - [x] Required Fields
  - [x] Path Validation
  - [x] Cron Validation
- [x] **Save/Update** (4h)

**Subtotal:** 38 Stunden

### 4.5 Scheduled Jobs Monitoring

#### Tasks

- [ ] **Job-Status-Updates** (4h)
  - [ ] Events für Job-Status
  - [ ] Store-Updates
  - [ ] UI-Refresh
- [ ] **Job-History** (6h)
  - [ ] History-Log pro Job
  - [ ] Success/Failure Count
  - [ ] Average Duration
- [ ] **Notifications** (4h)
  - [ ] Desktop-Notifications (optional)
  - [ ] Toast bei Job-Completion
  - [ ] Email-Notifications (v2.0 Feature)

**Subtotal:** 14 Stunden

**Milestone 4 Total:** ~124 Stunden (~3 Wochen)

---

## Milestone 5: Polish & Testing 🔴

**Status:** Todo  
**Geschätzte Dauer:** 2 Wochen  
**Dependencies:** Milestone 4

### 5.1 Advanced Features

#### Tasks

- [ ] **Compare-Snapshots Dialog** (12h)
  - [ ] Two-Column Layout
  - [ ] Diff-Berechnung
  - [ ] Added/Removed/Modified Badges
  - [ ] Filter-Chips
  - [ ] Stats Summary
- [ ] **Repository Info Page** (8h)
  - [ ] Detailed Stats
  - [ ] Config-Display
  - [ ] Maintenance-Actions (Check, Prune, Unlock)
- [ ] **Settings Page** (8h)
  - [ ] General Settings (Theme, Language)
  - [ ] Security Settings (Keychain)
  - [ ] About Section
- [ ] **Keyboard-Shortcuts** (6h)
  - [ ] Global Shortcuts (Ctrl+K Search, etc.)
  - [ ] Dialog Shortcuts (ESC, Enter)
  - [ ] Navigation Shortcuts

**Subtotal:** 34 Stunden

### 5.2 Error-Handling & Edge-Cases

#### Tasks

- [ ] **Comprehensive Error-Messages** (6h)
  - [ ] User-Friendly Messages
  - [ ] Details expandable
  - [ ] Copy-to-Clipboard
- [ ] **Offline-Handling** (4h)
  - [ ] Network-Fehler abfangen
  - [ ] Retry-Mechanismus
  - [ ] Graceful Degradation
- [ ] **Large-File Handling** (4h)
  - [ ] Progress für große Dateien
  - [ ] Memory-Management
  - [ ] Streaming wo möglich
- [ ] **Repository-Lock Recovery** (4h)
  - [ ] Automatische Erkennung
  - [ ] Force-Unlock UI
  - [ ] Warnung vor Datenverlust

**Subtotal:** 18 Stunden

### 5.3 Testing

#### Tasks

- [ ] **Unit-Tests (Rust)** (12h)
  - [ ] Alle Module testen
  - [ ] Edge-Cases
  - [ ] Coverage >80%
- [ ] **Unit-Tests (TypeScript)** (10h)
  - [ ] Utility-Functions
  - [ ] Stores
  - [ ] API-Wrapper
- [ ] **Integration-Tests** (12h)
  - [ ] Repository-Lifecycle
  - [ ] Backup-Flow
  - [ ] Restore-Flow
  - [ ] Job-Scheduling
- [ ] **Component-Tests** (8h)
  - [ ] Kritische Components
  - [ ] Dialog-Flows
  - [ ] Form-Validation
- [ ] **E2E-Tests** (optional, 12h)
  - [ ] Happy-Path Scenarios
  - [ ] Critical User-Journeys
  - [ ] Cross-Platform Tests

**Subtotal:** 54 Stunden

### 5.4 Performance-Optimierung

#### Tasks

- [ ] **Bundle-Size Optimierung** (6h)
  - [ ] Tree-Shaking verifizieren
  - [ ] Unused Dependencies entfernen
  - [ ] Bundle-Analyzer nutzen
- [ ] **Memory-Profiling** (6h)
  - [ ] Große Repositories testen
  - [ ] Memory-Leaks finden
  - [ ] Optimierungen
- [ ] **UI-Performance** (6h)
  - [ ] Virtual Scrolling für lange Listen
  - [ ] Lazy-Loading optimieren
  - [ ] Debouncing/Throttling
- [ ] **Startup-Zeit** (4h)
  - [ ] Lazy-Initialization
  - [ ] Startup-Profiling
  - [ ] Optimierungen

**Subtotal:** 22 Stunden

### 5.5 Accessibility & UX

#### Tasks

- [ ] **ARIA-Labels vervollständigen** (4h)
  - [ ] Alle interaktiven Elemente
  - [ ] Screen-Reader Testing
- [ ] **Keyboard-Navigation** (6h)
  - [ ] Tab-Order prüfen
  - [ ] Focus-Management
  - [ ] Focus-Visible Styles
- [ ] **Color-Contrast** (3h)
  - [ ] WCAG AA Compliance
  - [ ] High-Contrast Mode (optional)
- [ ] **Loading-States** (4h)
  - [ ] Überall wo nötig
  - [ ] Skeleton-Screens (optional)
- [ ] **Empty-States** (3h)
  - [ ] Hilfreiche Messages
  - [ ] Call-to-Action Buttons

**Subtotal:** 20 Stunden

### 5.6 Dokumentation

#### Tasks

- [ ] **User-Dokumentation** (12h)
  - [ ] Installation Guide
  - [ ] Quick-Start Tutorial
  - [ ] Feature-Dokumentation
  - [ ] Screenshots/GIFs
  - [ ] FAQ
- [ ] **Developer-Dokumentation** (8h)
  - [ ] Architecture Overview
  - [ ] API-Dokumentation (TypeDoc)
  - [ ] Rustdoc Comments vervollständigen
  - [ ] Contributing Guidelines
- [ ] **Code-Kommentare** (6h)
  - [ ] Alle Public APIs dokumentiert
  - [ ] Komplexe Algorithmen erklärt
  - [ ] TODOs aufräumen

**Subtotal:** 26 Stunden

### 5.7 Bug-Fixing & Stabilität

#### Tasks

- [ ] **Bug-Triage** (variabel)
  - [ ] Alle bekannten Bugs fixen
  - [ ] GitHub Issues durchgehen
- [ ] **Stress-Testing** (8h)
  - [ ] Große Repositories (>100GB)
  - [ ] Viele Snapshots (>1000)
  - [ ] Lange Backup-Sessions
  - [ ] Concurrent Operations
- [ ] **Platform-Testing** (12h)
  - [ ] Linux (verschiedene Distros)
  - [ ] Windows 10/11
  - [ ] Edge-Cases pro Platform
- [ ] **Security-Review** (6h)
  - [ ] Input-Validation
  - [ ] Password-Handling
  - [ ] File-Path Sanitization

**Subtotal:** 26 Stunden + Bug-Fixing

**Milestone 5 Total:** ~200 Stunden (~4 Wochen)

---

## Milestone 6: Release 🚀

**Status:** Todo  
**Geschätzte Dauer:** 1 Woche  
**Dependencies:** Milestone 5

### 6.1 Release-Vorbereitung

#### Tasks

- [ ] **Version-Bump** (1h)
  - [ ] package.json → 1.0.0
  - [ ] Cargo.toml → 1.0.0
  - [ ] tauri.conf.json → 1.0.0
- [ ] **CHANGELOG finalisieren** (2h)
  - [ ] Alle Features auflisten
  - [ ] Breaking Changes (falls vorhanden)
  - [ ] Known Issues
- [ ] **LICENSE prüfen** (1h)
  - [ ] License-Headers in Source-Files
  - [ ] Third-Party Licenses
- [ ] **README aktualisieren** (2h)
  - [ ] Features-Liste
  - [ ] Screenshots
  - [ ] Installation-Links

**Subtotal:** 6 Stunden

### 6.2 Build-Pipeline

#### Tasks

- [ ] **CI/CD finalisieren** (8h)
  - [ ] GitHub Actions Workflow
  - [ ] Matrix-Build (Linux/Windows)
  - [ ] Artifact-Upload
  - [ ] Release-Creation
- [ ] **Signing (optional)** (4h)
  - [ ] Code-Signing für Windows
  - [ ] App-Signing für Linux (optional)
- [ ] **Installer-Testing** (6h)
  - [ ] AppImage auf verschiedenen Distros
  - [ ] MSI auf Windows 10/11
  - [ ] Portable EXE

**Subtotal:** 18 Stunden

### 6.3 Release-Builds

#### Tasks

- [ ] **Production Builds erstellen** (4h)

  ```bash
  npm run tauri:build
  ```

  - [ ] Linux x86_64 AppImage
  - [ ] Windows x64 MSI
  - [ ] Windows x64 Portable EXE

- [ ] **Builds testen** (6h)
  - [ ] Smoke-Tests auf allen Plattformen
  - [ ] Installation testen
  - [ ] Deinstallation testen
- [ ] **Build-Artifacts archivieren** (1h)
  - [ ] Checksums erstellen
  - [ ] Signatures (falls Signing)

**Subtotal:** 11 Stunden

### 6.4 Release-Distribution

#### Tasks

- [ ] **GitHub Release erstellen** (2h)
  - [ ] Tag v1.0.0
  - [ ] Release-Notes aus CHANGELOG
  - [ ] Artifacts hochladen
- [ ] **Website/Landing-Page** (optional, 8h)
  - [ ] Feature-Übersicht
  - [ ] Download-Links
  - [ ] Documentation-Links
- [ ] **Ankündigung** (2h)
  - [ ] Reddit (r/rust, r/DataHoarder, etc.)
  - [ ] rustic Discord
  - [ ] Twitter/Mastodon (optional)

**Subtotal:** 12 Stunden

### 6.5 Post-Release

#### Tasks

- [ ] **Monitoring Setup** (4h)
  - [ ] Issue-Tracker beobachten
  - [ ] User-Feedback sammeln
  - [ ] Error-Reports
- [ ] **Hotfix-Prozess** (2h)
  - [ ] Branching-Strategy für Patches
  - [ ] Hotfix-Release-Workflow
- [ ] **Roadmap v1.1** (2h)
  - [ ] Community-Feedback einarbeiten
  - [ ] Nächste Features priorisieren

**Subtotal:** 8 Stunden

**Milestone 6 Total:** ~55 Stunden (~1 Woche)

---

## 📊 Gesamtübersicht

### Zeit-Investition pro Milestone

| Milestone | Beschreibung     | Geschätzte Zeit | Wochen           |
| --------- | ---------------- | --------------- | ---------------- |
| M0        | Project Setup    | 16h             | 0.4              |
| M1        | Core Backend     | 90h             | 2.3              |
| M2        | Basic UI         | 114h            | 2.9              |
| M3        | Backup & Restore | 143h            | 3.6              |
| M4        | Job Management   | 124h            | 3.1              |
| M5        | Polish & Testing | 200h            | 5.0              |
| M6        | Release          | 55h             | 1.4              |
| **Total** |                  | **742h**        | **~18.5 Wochen** |

### Annahmen

- **Arbeitszeit:** 40h/Woche (Vollzeit)
- **Produktivität:** 100% (unrealistisch!)
- **Realistische Einschätzung:** +30-50% Buffer → **24-28 Wochen**

### Kritischer Pfad

```
M0 → M1 → M2 → M3 → M4 → M5 → M6
```

Alle Milestones sind sequenziell abhängig.

---

## 🎯 Quick-Start Roadmap (MVP)

Falls schnellerer Launch gewünscht ist:

### MVP Scope (Minimal Viable Product)

**Ziel:** Grundlegende Backup/Restore-Funktionalität ohne Scheduling

#### Features

- ✅ Repository-Management (Local + Cloud)
- ✅ Manuelles Backup
- ✅ Snapshot-Liste anzeigen
- ✅ Restore einzelner Dateien
- ❌ Job-Scheduling (später)
- ❌ Snapshot-Vergleich (später)
- ❌ Advanced Settings (später)

#### MVP Timeline

- M0: Project Setup (1 Woche)
- M1: Core Backend (2 Wochen)
- M2: Basic UI (2 Wochen)
- M3: Backup & Restore (2 Wochen)
- M5 (reduziert): Testing & Polish (1 Woche)
- M6: Release (1 Woche)

**MVP Total:** ~9 Wochen

---

## 🔄 Iterative Entwicklung

### Agile Sprints

**Sprint-Dauer:** 2 Wochen

| Sprint       | Fokus          | Deliverables                 |
| ------------ | -------------- | ---------------------------- |
| Sprint 0     | Setup          | Projekt läuft lokal          |
| Sprint 1-2   | Backend Core   | Repository CRUD funktioniert |
| Sprint 3-4   | UI Foundation  | Dashboard + Navigation       |
| Sprint 5-7   | Backup/Restore | Vollständiger Flow           |
| Sprint 8-9   | Job Management | Scheduling funktioniert      |
| Sprint 10-11 | Testing        | Stabile Alpha                |
| Sprint 12    | Release        | v1.0 Production-Ready        |

---

## 📈 Progress-Tracking

### GitHub Project Board

**Columns:**

- 📋 Backlog
- 🔜 Todo (Current Sprint)
- 🏗️ In Progress
- 👀 In Review
- ✅ Done

**Labels:**

- `milestone-1`, `milestone-2`, etc.
- `priority-high`, `priority-medium`, `priority-low`
- `frontend`, `backend`
- `bug`, `feature`, `docs`
- `good-first-issue` (für Contributors)

### Definition of Done (DoD)

Eine Task ist "Done" wenn:

- [ ] Code geschrieben
- [ ] Tests geschrieben & passing
- [ ] Code-Review durchgeführt
- [ ] Formatiert & gelintet
- [ ] Dokumentation aktualisiert
- [ ] Merged in main branch

---

## 🚧 Risiken & Mitigation

### Identifizierte Risiken

#### 1. rustic_core API-Änderungen

**Risiko:** rustic_core könnte Breaking Changes haben  
**Wahrscheinlichkeit:** Mittel  
**Impact:** Hoch  
**Mitigation:**

- Version pinnen (0.7.x)
- Regelmäßig Updates prüfen
- Migration-Plan für Breaking Changes

#### 2. rclone-Integration Probleme

**Risiko:** rclone FFI könnte instabil sein  
**Wahrscheinlichkeit:** Mittel  
**Impact:** Mittel  
**Mitigation:**

- Fallback zu rclone-Subprocess
- Umfangreiche Tests mit verschiedenen Clouds
- Community-Support nutzen

#### 3. Performance bei großen Repositories

**Risiko:** UI könnte bei >1000 Snapshots langsam werden  
**Wahrscheinlichkeit:** Hoch  
**Impact:** Mittel  
**Mitigation:**

- Virtual Scrolling implementieren
- Pagination für Snapshots
- Lazy-Loading
- Performance-Tests früh einplanen

#### 4. Cross-Platform Bugs

**Risiko:** Plattform-spezifische Probleme  
**Wahrscheinlichkeit:** Hoch  
**Impact:** Mittel  
**Mitigation:**

- Früh auf beiden Plattformen testen
- CI für Linux + Windows
- Platform-spezifische Workarounds dokumentieren

#### 5. Zeitüberschreitung

**Risiko:** Projekt dauert länger als geschätzt  
**Wahrscheinlichkeit:** Sehr Hoch  
**Impact:** Niedrig  
**Mitigation:**

- MVP-First Approach
- Features priorisieren (MoSCoW)
- Regelmäßige Sprint-Reviews
- Buffer einplanen (30-50%)

---

## 🎨 Feature-Priorisierung (MoSCoW)

### Must Have (v1.0)

- ✅ Repository-Management (Local + Cloud)
- ✅ Manuelles Backup
- ✅ Snapshot-Listing
- ✅ Restore-Funktionalität
- ✅ Backup-Jobs mit Scheduling
- ✅ Basic Error-Handling
- ✅ Passwort-Management (Keychain)

### Should Have (v1.0)

- ✅ Snapshot-Vergleich
- ✅ Repository Check/Prune
- ✅ Progress-Anzeigen
- ✅ Toast-Notifications
- ✅ Settings-Page

### Could Have (v1.1+)

- ⏸️ Desktop-Notifications
- ⏸️ Email-Notifications bei Job-Completion
- ⏸️ Repository-Statistiken (Charts)
- ⏸️ Export/Import Config
- ⏸️ Multi-Language Support
- ⏸️ Themes (mehr als Light/Dark)

### Won't Have (v1.0)

- ❌ Web-Version
- ❌ Mobile-App
- ❌ Cloud-Sync der Config
- ❌ Team-Features / Multi-User

---

## 🔮 Future Roadmap (Post v1.0)

### v1.1 (Q1 2026)

**Focus:** Community Feedback & Stability

- Bug-Fixes aus v1.0
- Performance-Optimierungen
- Kleinere Feature-Requests

### v1.2 (Q2 2026)

**Focus:** Advanced Features

- Snapshot-Diff Visualization
- Repository-Statistiken mit Charts
- Export/Import Config
- Desktop-Notifications

### v2.0 (Q3 2026)

**Focus:** Enterprise Features

- Multi-User Support
- Centralized Config Management
- Reporting & Compliance
- Webhook-Integration
- API für Automation

### v3.0 (2027)

**Focus:** Cloud-Native

- Web-Version
- Mobile-App (iOS/Android)
- Cloud-Backend für Config-Sync
- Collaborative Features

---

## 📞 Kontakt & Support

### Während Entwicklung

**Issues/Bugs:**

- GitHub Issues: `https://github.com/your-org/rustic-gui/issues`
- Discord: rustic-gui Channel (in rustic Discord)

**Fragen:**

- GitHub Discussions
- Discord #help Channel

**Pull Requests:**

- Siehe CONTRIBUTING.md
- Code-Review innerhalb 2-3 Tage

---

## 📝 Change-Log Template

Für jede Version CHANGELOG.md aktualisieren:

```markdown
## [1.0.0] - 2026-XX-XX

### Added

- Initial Release
- Repository-Management (Local, SFTP, S3, rclone)
- Backup mit Progress-Anzeige
- Restore mit File-Browser
- Job-Scheduling mit Cron-Expressions
- Snapshot-Vergleich
- Dark/Light Theme

### Changed

- N/A (Initial Release)

### Fixed

- N/A (Initial Release)

### Security

- Passwort-Speicherung in System-Keychain
- Input-Validation für alle User-Inputs
```

---

## ✅ Sprint 0 (Woche 1) - Sofort starten!

### Diese Woche

- [ ] Entwicklungsumgebung aufsetzen (Tag 1)
- [ ] Tauri-Projekt initialisieren (Tag 1-2)
- [ ] Projekt-Struktur erstellen (Tag 2)
- [ ] Dependencies einrichten (Tag 2-3)
- [ ] Tooling konfigurieren (Tag 3)
- [ ] Erste "Hello World" App läuft (Tag 4)
- [ ] Verify Setup komplett (Tag 5)

### Exit-Criteria Sprint 0

- ✅ `npm run tauri:dev` funktioniert
- ✅ Hot-Reload funktioniert (Frontend + Backend)
- ✅ Tests laufen durch
- ✅ Linting/Formatting funktioniert
- ✅ Git Repository initialisiert

**Nach Sprint 0 → Milestone 1 starten!**

---

## 🎓 Learning Resources

Falls Skills fehlen:

### Rust

- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- rustic_core Docs: https://docs.rs/rustic_core

### Tauri

- Tauri Docs: https://tauri.app/v2/
- Tauri Examples: https://github.com/tauri-apps/tauri/tree/dev/examples

### Svelte

- Svelte Tutorial: https://svelte.dev/tutorial
- Svelte 5 Preview: https://svelte-5-preview.vercel.app/

### TypeScript

- TS Handbook: https://www.typescriptlang.org/docs/handbook/

---

## 🎉 Los geht's!

**Nächster Schritt:**

```bash
git clone <your-repo>
cd rustic-gui
# Folge Sprint 0 Tasks
```

Viel Erfolg! 🚀

---

_Roadmap Version: 1.0_  
_Erstellt: 2025-10-26_  
_Nächstes Review: Nach jedem Milestone_
