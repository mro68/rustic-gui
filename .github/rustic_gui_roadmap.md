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

## Milestone 1: Core Backend 🔵

**Status:** In Progress 🔵 (1.1 & 1.2 completed, 1.3-1.5 pending)  
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

### 1.3 App State & Lifecycle

#### Tasks
- [ ] **AppState Struct** (4h)
  - [ ] `src-tauri/src/state.rs`
  - [ ] `current_repo: Mutex<Option<Repository>>`
  - [ ] `cancellation_tokens: HashMap`
  - [ ] `scheduler: Mutex<BackupScheduler>`
- [ ] **Repository Switching** (6h)
  - [ ] `switch_repository()` Command
  - [ ] Altes Repo sauber schließen
  - [ ] Neues Repo öffnen
  - [ ] State aktualisieren
- [ ] **Graceful Shutdown** (4h)
  - [ ] Laufende Backups prüfen
  - [ ] Confirmation-Dialog
  - [ ] Cleanup vor Exit
- [ ] **Integration-Tests** (4h)

**Subtotal:** 18 Stunden

### 1.4 Config-Persistence

#### Tasks
- [ ] **Config-Strukturen** (4h)
  - [ ] `AppConfig` struct
  - [ ] `RepositoryConfig`
  - [ ] `BackupJobConfig`
  - [ ] TOML serde derives
- [ ] **Config laden/speichern** (6h)
  - [ ] `load_config()` Funktion
  - [ ] `save_config()` Funktion
  - [ ] Platform-spezifische Pfade
  - [ ] Default-Config erstellen
- [ ] **Config-Migration** (4h)
  - [ ] Versioning
  - [ ] Upgrade-Path von alten Configs
- [ ] **Tests** (3h)

**Subtotal:** 17 Stunden

### 1.5 Keychain-Integration

#### Tasks
- [ ] **Keychain-Wrapper** (6h)
  - [ ] `src-tauri/src/keychain/mod.rs`
  - [ ] `store_password()` Funktion
  - [ ] `get_password()` Funktion
  - [ ] `delete_password()` Funktion
  - [ ] Platform-Tests (Linux/Windows)
- [ ] **Password-Handling-Pattern** (4h)
  - [ ] Zeroize after use
  - [ ] Env-Var setzen für rustic
  - [ ] Security-Best-Practices
- [ ] **Tauri Commands** (3h)
  - [ ] `store_repository_password`
  - [ ] `get_repository_password`
- [ ] **Tests** (3h)

**Subtotal:** 16 Stunden

**Milestone 1 Total:** ~90 Stunden (~2.5 Wochen)

---

## Milestone 2: Basic UI 🟡

**Status:** Todo  
**Geschätzte Dauer:** 2-3 Wochen  
**Dependencies:** Milestone 1

### 2.1 Layout & Navigation

#### Tasks
- [ ] **MainLayout Component** (6h)
  - [ ] `src/lib/components/layout/MainLayout.svelte`
  - [ ] Sidebar + Content-Bereich
  - [ ] Responsive Grid
- [ ] **Sidebar Navigation** (8h)
  - [ ] Logo/Branding
  - [ ] Nav-Items (Dashboard, Repos, Snapshots, Jobs, Settings)
  - [ ] Active-State
  - [ ] Icons
  - [ ] Mobile Toggle
- [ ] **Header Component** (4h)
  - [ ] Page-Title
  - [ ] User-Avatar (optional)
  - [ ] Mobile Menu-Button
- [ ] **Router Setup** (6h)
  - [ ] Page-Switching Logic
  - [ ] URL-basiertes Routing (optional)
  - [ ] Back/Forward (optional)

**Subtotal:** 24 Stunden

### 2.2 Shared Components

#### Tasks
- [ ] **Button Component** (3h)
  - [ ] Variants (primary, secondary, danger)
  - [ ] Sizes (small, medium, large)
  - [ ] Loading-State
  - [ ] Icon-Support
- [ ] **Modal Component** (6h)
  - [ ] Overlay + Dialog
  - [ ] Close-Button
  - [ ] ESC-Key Handler
  - [ ] Click-Outside-to-Close
  - [ ] Accessibility (focus-trap)
- [ ] **Toast/Notification** (6h)
  - [ ] Toast-Container
  - [ ] Success/Error/Warning Variants
  - [ ] Auto-Dismiss
  - [ ] Queue-System
  - [ ] Svelte Store für Toast-State
- [ ] **LoadingSpinner** (2h)
- [ ] **ProgressBar** (3h)
  - [ ] Determinate (%)
  - [ ] Indeterminate
  - [ ] Smooth Animations

**Subtotal:** 20 Stunden

### 2.3 Stores & State Management

#### Tasks
- [ ] **repositories Store** (4h)
  - [ ] `src/lib/stores/repositories.ts`
  - [ ] Writable store für Liste
  - [ ] Active repository
  - [ ] Load/Add/Remove Funktionen
- [ ] **snapshots Store** (4h)
  - [ ] Snapshot-Liste
  - [ ] Filter/Sort State
  - [ ] Load-Funktionen
- [ ] **backupJobs Store** (4h)
  - [ ] Job-Liste
  - [ ] Running-State
  - [ ] Load/Update Funktionen
- [ ] **toast Store** (2h)
  - [ ] Toast-Queue
  - [ ] Add/Remove Functions
- [ ] **settings Store** (3h)
  - [ ] Theme (dark/light)
  - [ ] Language
  - [ ] Load/Save

**Subtotal:** 17 Stunden

### 2.4 API-Wrapper (Frontend)

#### Tasks
- [ ] **Repository API** (6h)
  - [ ] `src/lib/api/repositories.ts`
  - [ ] initRepository()
  - [ ] openRepository()
  - [ ] checkConnection()
  - [ ] listRepositories()
- [ ] **Snapshot API** (4h)
  - [ ] listSnapshots()
  - [ ] getSnapshotInfo()
  - [ ] deleteSnapshot()
- [ ] **Backup API** (4h)
  - [ ] runBackup()
  - [ ] cancelBackup()
- [ ] **Event-Listener Setup** (4h)
  - [ ] backup-progress Events
  - [ ] backup-completed Events
  - [ ] backup-failed Events

**Subtotal:** 18 Stunden

### 2.5 Dashboard Page

#### Tasks
- [ ] **Dashboard Layout** (6h)
  - [ ] Grid für Repository-Cards
  - [ ] Recent Activity Log
  - [ ] Storage Usage Charts (optional)
- [ ] **Repository Card Component** (6h)
  - [ ] Repo Name + Path
  - [ ] Status Badge (Healthy/Warning)
  - [ ] Quick-Actions (Backup, Browse)
  - [ ] Context Menu (3-dot)
- [ ] **Activity Log Component** (4h)
  - [ ] Scrollable Log
  - [ ] Timestamp + Message
  - [ ] Icons für Actions
- [ ] **Dashboard Store Integration** (3h)
  - [ ] Load Repos on Mount
  - [ ] Refresh-Logic

**Subtotal:** 19 Stunden

### 2.6 CSS/Styling Setup

#### Tasks
- [ ] **CSS aus Mockups übernehmen** (8h)
  - [ ] `src/app.css`
  - [ ] Dark Theme Variables
  - [ ] Component-Styles
  - [ ] Responsive Breakpoints
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

**Status:** Todo  
**Geschätzte Dauer:** 2-3 Wochen  
**Dependencies:** Milestone 2

### 3.1 Backup-Funktionalität (Backend)

#### Tasks
- [ ] **run_backup Command** (8h)
  - [ ] `src-tauri/src/rustic/backup.rs`
  - [ ] rustic_core Integration
  - [ ] Progress-Callbacks
  - [ ] Event-Emission an Frontend
  - [ ] Cancellation-Support
- [ ] **Backup-Options** (4h)
  - [ ] Exclude-Patterns
  - [ ] Tags
  - [ ] Compression-Level
- [ ] **Progress-Reporting** (6h)
  - [ ] Files-Processed
  - [ ] Bytes-Uploaded
  - [ ] Current-File
  - [ ] ETA (optional)
- [ ] **Tests** (6h)
  - [ ] Mock-Repository
  - [ ] Success-Case
  - [ ] Error-Cases
  - [ ] Cancellation

**Subtotal:** 24 Stunden

### 3.2 Snapshot-Management (Backend)

#### Tasks
- [ ] **list_snapshots Command** (4h)
  - [ ] rustic_core Integration
  - [ ] DTO-Konvertierung
  - [ ] Sorting/Filtering (Backend)
- [ ] **get_snapshot_info Command** (3h)
  - [ ] Detaillierte Snapshot-Infos
  - [ ] Summary-Stats
- [ ] **delete_snapshot Command** (3h)
  - [ ] Einzelnes Snapshot löschen
  - [ ] Validation
- [ ] **forget_snapshots Command** (6h)
  - [ ] Retention-Policy anwenden
  - [ ] Batch-Delete
  - [ ] Progress
- [ ] **Tests** (4h)

**Subtotal:** 20 Stunden

### 3.3 Restore-Funktionalität (Backend)

#### Tasks
- [ ] **get_file_tree Command** (8h)
  - [ ] `src-tauri/src/rustic/restore.rs`
  - [ ] Tree aus Snapshot laden
  - [ ] Hierarchische Struktur
  - [ ] Lazy-Loading für große Trees
- [ ] **restore_files Command** (8h)
  - [ ] rustic_core Integration
  - [ ] File-Selection
  - [ ] Target-Path
  - [ ] Progress-Reporting
  - [ ] Cancellation
- [ ] **Restore-Options** (4h)
  - [ ] Overwrite-Policy
  - [ ] Restore-Permissions
  - [ ] Restore-Timestamps
- [ ] **Tests** (5h)

**Subtotal:** 25 Stunden

### 3.4 Snapshots Page (Frontend)

#### Tasks
- [ ] **Snapshots Page Layout** (6h)
  - [ ] `src/lib/pages/Snapshots.svelte`
  - [ ] Toolbar (Search, Filter, Refresh)
  - [ ] Snapshot-Table
- [ ] **Snapshot Table Component** (8h)
  - [ ] Sortable Columns
  - [ ] Tag-Display
  - [ ] Action-Buttons
  - [ ] Context-Menu
  - [ ] Selection (Multi-Select)
- [ ] **Filter/Search** (6h)
  - [ ] Search-Box
  - [ ] Tag-Filter
  - [ ] Date-Range-Filter
  - [ ] Hostname-Filter
- [ ] **Snapshot Details Dialog** (6h)
  - [ ] Modal mit Snapshot-Info
  - [ ] Stats anzeigen
  - [ ] Actions (Browse, Restore, Delete)

**Subtotal:** 26 Stunden

### 3.5 Restore Dialog (Frontend)

#### Tasks
- [ ] **Restore Dialog Component** (10h)
  - [ ] `src/lib/components/dialogs/RestoreDialog.svelte`
  - [ ] Snapshot-Selection
  - [ ] File-Tree Component
  - [ ] Restore-Options Form
- [ ] **File-Tree Component** (12h)
  - [ ] Hierarchische Darstellung
  - [ ] Checkboxen für Selection
  - [ ] Lazy-Loading
  - [ ] Breadcrumb-Navigation
  - [ ] Icons (Folder/File)
- [ ] **Restore-Progress Dialog** (6h)
  - [ ] Progress-Bar
  - [ ] File-Count
  - [ ] Current-File Display
  - [ ] Cancel-Button
- [ ] **Integration Tests** (4h)

**Subtotal:** 32 Stunden

### 3.6 Backup-Job Execution (Frontend)

#### Tasks
- [ ] **Run-Backup UI** (6h)
  - [ ] Backup-Button mit Loading
  - [ ] Progress-Modal
  - [ ] Real-Time Updates
  - [ ] Cancel-Button
- [ ] **Progress-Updates via Events** (4h)
  - [ ] Event-Listener
  - [ ] Store-Updates
  - [ ] UI-Refresh
- [ ] **Error-Handling** (4h)
  - [ ] Toast bei Fehler
  - [ ] Error-Details anzeigen
  - [ ] Retry-Option
- [ ] **Success-Notification** (2h)
  - [ ] Toast mit Summary
  - [ ] Link zu Snapshot

**Subtotal:** 16 Stunden

**Milestone 3 Total:** ~143 Stunden (~3.5 Wochen)

---

## Milestone 4: Job Management 🟣

**Status:** Todo  
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
- [ ] **BackupJobConfig erweitern** (4h)
  - [ ] Schedule-Field
  - [ ] Last-Run Timestamp
  - [ ] Next-Run Timestamp
  - [ ] Enabled/Disabled
- [ ] **Job CRUD Commands** (8h)
  - [ ] create_backup_job
  - [ ] update_backup_job
  - [ ] delete_backup_job
  - [ ] get_backup_job
- [ ] **Job-Persistence** (4h)
  - [ ] Config speichern
  - [ ] Scheduler bei Start wiederherstellen
- [ ] **Tests** (4h)

**Subtotal:** 20 Stunden

### 4.3 Backup Jobs Page (Frontend)

#### Tasks
- [ ] **Jobs Page Layout** (6h)
  - [ ] `src/lib/pages/BackupJobs.svelte`
  - [ ] Job-Liste
  - [ ] "+ New Job" Button
- [ ] **Job-Card Component** (8h)
  - [ ] Job-Name + Details
  - [ ] Status-Badge (Idle/Running)
  - [ ] Last-Run Info
  - [ ] Next-Run Info
  - [ ] Actions (Run, Edit, Delete)
- [ ] **Job-List Filtering** (4h)
  - [ ] Active/Inactive
  - [ ] By Repository
  - [ ] By Schedule
- [ ] **Integration** (4h)

**Subtotal:** 22 Stunden

### 4.4 Create/Edit Job Dialog (Frontend)

#### Tasks
- [ ] **Job-Dialog Basis** (6h)
  - [ ] `src/lib/components/dialogs/CreateJobDialog.svelte`
  - [ ] Tab-System (4 Tabs)
  - [ ] Form-State Management
- [ ] **Tab 1: General** (4h)
  - [ ] Job-Name Input
  - [ ] Repository-Selection
  - [ ] Tags Input
- [ ] **Tab 2: Paths & Exclusions** (6h)
  - [ ] Source-Paths List
  - [ ] Add/Remove Paths
  - [ ] File-Browser Integration
  - [ ] Exclude-Patterns Textarea
- [ ] **Tab 3: Schedule** (8h)
  - [ ] Schedule-Type (Manual/Daily/Weekly/Monthly/Custom)
  - [ ] Time-Picker
  - [ ] Weekday-Selection (Weekly)
  - [ ] Cron-Expression (Custom)
  - [ ] Next-Run Preview
- [ ] **Tab 4: Retention** (6h)
  - [ ] Policy-Selection
  - [ ] Keep-Last Input
  - [ ] Keep-Daily/Weekly/Monthly/Yearly
  - [ ] Auto-Prune Checkbox
  - [ ] Policy-Summary
- [ ] **Form-Validation** (4h)
  - [ ] Required Fields
  - [ ] Path Validation
  - [ ] Cron Validation
- [ ] **Save/Update** (4h)

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

| Milestone | Beschreibung | Geschätzte Zeit | Wochen |
|-----------|-------------|-----------------|--------|
| M0 | Project Setup | 16h | 0.4 |
| M1 | Core Backend | 90h | 2.3 |
| M2 | Basic UI | 114h | 2.9 |
| M3 | Backup & Restore | 143h | 3.6 |
| M4 | Job Management | 124h | 3.1 |
| M5 | Polish & Testing | 200h | 5.0 |
| M6 | Release | 55h | 1.4 |
| **Total** | | **742h** | **~18.5 Wochen** |

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

| Sprint | Fokus | Deliverables |
|--------|-------|--------------|
| Sprint 0 | Setup | Projekt läuft lokal |
| Sprint 1-2 | Backend Core | Repository CRUD funktioniert |
| Sprint 3-4 | UI Foundation | Dashboard + Navigation |
| Sprint 5-7 | Backup/Restore | Vollständiger Flow |
| Sprint 8-9 | Job Management | Scheduling funktioniert |
| Sprint 10-11 | Testing | Stabile Alpha |
| Sprint 12 | Release | v1.0 Production-Ready |

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

*Roadmap Version: 1.0*  
*Erstellt: 2025-10-26*  
*Nächstes Review: Nach jedem Milestone*