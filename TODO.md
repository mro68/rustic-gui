# TODO-Liste: Rustic GUI Integration (Svelte 5 + Tauri 2)

## ✅ IMPLEMENTIERUNGS-STATUS (Stand: 2025-10-30, Update: aktualisiert)

### 🟢 Phase 1: Rust-Backend - **KOMPLETT (mit Stubs)** 

**Alle Backend-Commands registriert in `src-tauri/src/lib.rs`:**

- ✅ Repository-Management (7 Commands): 
  - ✅ list_repositories (Zeile 410, implementiert in commands/repository.rs)
  - ✅ init_repository (Zeile 385, simuliert in rustic/repository.rs)
  - ✅ open_repository (Zeile 386, simuliert in rustic/repository.rs)
  - ✅ delete_repository (Zeile 411, implementiert in commands/repository.rs)
  - ✅ check_repository (Zeile 412, simuliert in commands/repository.rs)
  - ✅ prune_repository (Zeile 413, stub in commands/repository.rs:134)
  - ✅ change_password (Zeile 414, stub in commands/repository.rs:161)

- ✅ Backup-Jobs (5 Commands):
  - ✅ list_backup_jobs (Zeile 400, implementiert in commands/backup.rs)
  - ✅ create_backup_job (Zeile 396, implementiert in commands/backup.rs)
  - ✅ update_backup_job (Zeile 397, implementiert in commands/backup.rs)
  - ✅ delete_backup_job (Zeile 398, implementiert in commands/backup.rs)
  - ✅ get_backup_job (Zeile 399, implementiert in commands/backup.rs)

- ✅ Snapshots (4 Commands):
  - ✅ list_snapshots (Zeile 402, implementiert in rustic/snapshot.rs)
  - ✅ get_snapshot (Zeile 403, implementiert in rustic/snapshot.rs)
  - ✅ delete_snapshot (Zeile 404, simuliert in rustic/snapshot.rs)
  - ✅ forget_snapshots (Zeile 405, simuliert mit policy in rustic/snapshot.rs)

- ✅ Backup & Restore (4 Commands):
  - ✅ run_backup_command (Zeile 394, simuliert in lib.rs:121)
  - ✅ cancel_backup (Zeile 395, implementiert in lib.rs:37)
  - ✅ restore_files_v1 (Zeile 408, simuliert in lib.rs:324)
  - ✅ get_file_tree_command (Zeile 407, simuliert in rustic/restore.rs)

- ✅ System & Keychain (4 Commands):
  - ✅ prepare_shutdown (Zeile 383, implementiert in lib.rs:253)
  - ✅ store_repository_password (Zeile 390, implementiert in lib.rs:275)
  - ✅ get_repository_password (Zeile 391, implementiert in lib.rs:288)
  - ✅ delete_repository_password (Zeile 392, implementiert in lib.rs:300)

- ✅ Event-System mit einheitlichem Format (BackupEvent, RestoreEvent, lib.rs:16-33)
- ✅ Config-Management (TOML) in src-tauri/src/config.rs
- ✅ State-Management (AppState) in src-tauri/src/state.rs

**⚠️ Hinweis:** Viele Backend-Commands sind als **Stubs/Simulationen** implementiert und benötigen noch vollständige rustic_core Integration (siehe TODO-Kommentare im Code, insgesamt 26 TODOs in Rust-Dateien).

### 🟡 Phase 2: Frontend - **~75% FERTIG**

**Implementiert:**
- ✅ API-Wrapper vollständig (repositories, backup-jobs, backup, restore, snapshots) in src/lib/api/
- ✅ Stores mit Daten-Loading (repositories, backup-jobs, snapshots, settings, router, toast) in src/lib/stores/
- ✅ Type-System synchronisiert (BackupJobDto, RepositoryDto, SnapshotDto in src/lib/types/)
- ✅ 5 Dialog-Workflows implementiert:
  - ✅ AddRepositoryDialog.svelte
  - ✅ DeleteRepoDialog.svelte  
  - ✅ CreateJobDialog.svelte
  - ✅ EditJobDialog.svelte
  - ✅ DeleteJobDialog.svelte
- ✅ Cron-Schedule-Konvertierung (daily, weekly, monthly) in CreateJobDialog
- ✅ Seiten laden Daten: Dashboard, Repositories, BackupJobs, Snapshots, Settings

**Noch offen (~25%):**
- ⏳ 7 Dialog-Workflows fehlen noch:
  - ⏳ UnlockRepositoryDialog.svelte (Dialog existiert, aber kein API-Aufruf)
  - ⏳ CheckRepoDialog.svelte (Dialog existiert, aber kein API-Aufruf)
  - ⏳ PruneRepoDialog.svelte (Dialog existiert, aber kein API-Aufruf)
  - ⏳ ChangePasswordDialog.svelte (Dialog existiert, aber kein API-Aufruf)
  - ⏳ RestoreDialog.svelte (Dialog existiert, API teilweise integriert)
  - ⏳ CompareSnapshotsDialog.svelte (Dialog existiert, aber kein API-Aufruf)
  - ⏳ RunBackupDialog.svelte (Dialog existiert, API teilweise integriert)
- ⏳ Snapshots-Seite: Erweiterte Funktionen (Vergleich, Batch-Delete) fehlen noch
- ⏳ File-Browser: Lazy-Loading und Performance-Optimierung fehlen
- ⏳ Error-Handling: Strukturiertes ErrorDto wird noch nicht überall verwendet

### 📊 Code-Qualität

**Aktuell (Stand 2025-10-30, Update):**
- TODO-Kommentare: 69 (in .rs, .ts, .svelte Dateien)
  - Rust-Backend: 26 TODOs (hauptsächlich rustic_core Integration)
  - TypeScript/Svelte: 43 TODOs (hauptsächlich Dialog-Workflows und Features)
- Linter-Warnungen (Frontend): 52 Probleme (11 Errors, 41 Warnings)
  - Hauptsächlich: no-undef für Browser-Globals (window, document, setTimeout)
  - 2x console.log in Stores
- Rust-Build: Benötigt System-Dependencies (glib-2.0) für CI

**Ziel:**
- TODO-Kommentare: < 20 (nur echte TODOs für künftige Features)
- Linter-Warnungen: 0 Errors, < 10 Warnings
- Alle Backend-Stubs durch echte rustic_core Integration ersetzen

---

Hier ist die ausführlich ergänzte TODO-Liste für die Integration von Svelte 5 und Tauri 2, inklusive Best-Practice-Hinweisen und konkreten Ergänzungen:

---

## Phase 1: Rust-Backend (Tauri 2-Befehle & Events)

Der wichtigste Schritt ist die Implementierung der Rust-Seite, die die in `src/lib/api/` definierten Verträge erfüllt.

- [x] **Grund-Setup (in `src-tauri/src/main.rs`)** ✅ KOMPLETT
  - [x] `main`-Funktion mit `tauri::Builder` aufsetzen. ✅ (main.rs vorhanden, aber run() in lib.rs)
  - [x] Alle `#[tauri::command]`-Funktionen im `.invoke_handler()` registrieren. ✅ (lib.rs:380-420, 24 Commands)
  - [x] Einen `tauri::State` (z.B. `AppState`) einrichten, um langlebige Objekte wie Repository-Verbindungen oder einen `tokio::Mutex` für den Zugriff auf `rustic_core` zu verwalten. ✅ (state.rs:1-35, AppState mit current_repo, cancellation_tokens)
  - [x] **Best-Practice:** State thread-sicher gestalten, um parallele Operationen (z.B. mehrere Backups) zu ermöglichen. ✅ (Parking_lot::Mutex verwendet)

- [x] **Befehle: Repository-Management (Rust)** ✅ REGISTRIERT (teilweise Stubs)
  - [x] `#[tauri::command] async fn list_repositories() -> Result<Vec<RepositoryDto>, ErrorDto>` ✅ (commands/repository.rs:7, implementiert)
  - [x] `#[tauri::command] async fn init_repository(path: String, password: String, ...) -> Result<(), ErrorDto>` ✅ (lib.rs:180, simuliert in rustic/repository.rs:32)
  - [x] `#[tauri::command] async fn open_repository(path: String, password: String) -> Result<RepositoryDto, ErrorDto>` ✅ (lib.rs:191, simuliert in rustic/repository.rs:78)
  - [x] `#[tauri::command] async fn delete_repository(id: String, delete_data: bool) -> Result<(), ErrorDto>` ✅ (commands/repository.rs:41, implementiert)
  - [x] `#[tauri::command] async fn check_repository(id: String, read_data: bool, ...) -> Result<CheckResultDto, ErrorDto>` ✅ (commands/repository.rs:84, stub TODO:131)
  - [x] `#[tauri::command] async fn prune_repository(id: String, ...) -> Result<PruneResultDto, ErrorDto>` ✅ (commands/repository.rs:124, stub TODO:134)
  - [x] `#[tauri::command] async fn change_password(id: String, old_pass: String, new_pass: String) -> Result<(), ErrorDto>` ✅ (commands/repository.rs:151, stub TODO:161)
  - [x] **Ergänzung:** Fehler als strukturierte Objekte (`ErrorDto`) zurückgeben, nicht nur als String. ✅ (types.rs:45-51, ErrorDto definiert)
  - [x] **Hinweis:** Alle Repository-Commands sind nun in lib.rs registriert ✅ (lib.rs:410-414)

- [x] **Befehle: Backup-Job-Management (Rust)** ✅ IMPLEMENTIERT
  - [x] `#[tauri::command] async fn list_jobs() -> Result<Vec<BackupJob>, ErrorDto>` ✅ (commands/backup.rs:235, implementiert)
  - [x] `#[tauri::command] async fn create_job(job: BackupJob) -> Result<(), ErrorDto>` ✅ (commands/backup.rs:152, implementiert)
  - [x] `#[tauri::command] async fn update_job(job: BackupJob) -> Result<(), ErrorDto>` ✅ (commands/backup.rs:196, implementiert)
  - [x] `#[tauri::command] async fn delete_job(id: String) -> Result<(), ErrorDto>` ✅ (commands/backup.rs:220, implementiert)
  - [x] `#[tauri::command] async fn get_backup_job(id: String) -> Result<BackupJob, ErrorDto>` ✅ (commands/backup.rs:255, implementiert mit TODO für last_run/next_run)

- [x] **Befehle: Snapshot-Management (Rust)** ✅ REGISTRIERT (teilweise Stubs)
  - [x] `#[tauri::command] async fn list_snapshots(repository_id: String) -> Result<Vec<SnapshotDto>, ErrorDto>` ✅ (lib.rs:96, rustic/snapshot.rs implementiert)
  - [x] `#[tauri::command] async fn get_snapshot_info(id: String) -> Result<SnapshotDto, ErrorDto>` ✅ (lib.rs:84, rustic/snapshot.rs implementiert)
  - [x] `#[tauri::command] async fn delete_snapshot(id: String) -> Result<(), ErrorDto>` ✅ (lib.rs:73, stub in rustic/snapshot.rs)
  - [x] `#[tauri::command] async fn compare_snapshots(id_a: String, id_b: String) -> Result<DiffResultDto, ErrorDto>` ⏳ (auskommentiert in lib.rs:416, stub in commands/snapshot.rs:41)
  - [x] `#[tauri::command] async fn forget_snapshots(policy: RetentionPolicy) -> Result<Vec<String>, ErrorDto>` ✅ (lib.rs:62, stub in rustic/snapshot.rs)

- [x] **Befehle: Prozess-Steuerung (Rust)** ✅ IMPLEMENTIERT (teilweise simuliert)
  - [x] `#[tauri::command] async fn run_backup(job_id: String, app_handle: tauri::AppHandle)` ✅ (lib.rs:121, simuliert mit Events)
    - [x] Diese Funktion _muss_ Events für Fortschritt (`backup-progress`), Erfolg (`backup-completed`) und Fehler (`backup-failed`) an das Frontend senden (siehe `src/lib/api/events.ts`). ✅ (lib.rs:139, 156, 165)
    - [x] **Best-Practice:** Einheitliches Event-Format verwenden: `{ type, progress, message, jobId, ... }` ✅ (BackupEvent struct, lib.rs:111-117)
  - [x] `#[tauri::command] async fn cancel_backup(job_id: String)` ✅ (lib.rs:37, implementiert mit CancellationToken)
    - [x] Implementieren Sie eine Logik (z.B. über `tokio::sync::watch`), um laufende Backups abzubrechen. ✅ (CancellationToken in AppState verwendet)
  - [x] `#[tauri::command] async fn restore_files_command(..., app_handle: tauri::AppHandle)` ✅ (lib.rs:324, simuliert mit Events)
    - [x] Diese Funktion _muss_ Events für den Wiederherstellungs-Fortschritt senden (siehe `src/lib/api/restore.ts`, Event: `restore-progress`). ✅ (lib.rs:349, 362)
    - [x] **Best-Practice:** Auch hier einheitliches Event-Format. ✅ (RestoreEvent struct, lib.rs:16-24)
  - [x] `#[tauri::command] async fn get_file_tree_command(...) -> Result<FileTreeNode, ErrorDto>` ✅ (lib.rs:312, stub in rustic/restore.rs:181)

- [x] **Typen & DTOs (Rust)** ✅ TEILWEISE KOMPLETT
  - [x] Sicherstellen, dass alle `...Dto`-Typen (wie `RepositoryDto`, `SnapshotDto`, `FileTreeNode`, `ErrorDto`) in Rust definiert sind, `serde::Serialize` implementieren und den TypeScript-Typen in `src/lib/types/` entsprechen. ✅ (types.rs:1-180, alle DTOs vorhanden)
  - [ ] **Best-Practice:** Automatisierte Synchronisation der DTOs mit TypeScript-Typen (z.B. via `ts-rs` oder `typeshare`), Build-Workflow anpassen. ⏳ (TODO für künftige Implementierung)

---

## Phase 2: Svelte 5-Frontend (API-Anbindung & Logik)

- [x] **Fehlende API-Wrapper (TypeScript)** ✅ KOMPLETT
  - [x] `src/lib/api/backup-jobs.ts` erstellt für `list_jobs`, `create_job`, `update_job`, `delete_job`, `get_backup_job`. ✅ (backup-jobs.ts:17-46)
  - [x] `src/lib/api/repositories.ts` ergänzt um `delete_repository`, `check_repository`, `prune_repository`, `change_password`. ✅ (repositories.ts:42-54)
  - [x] `src/lib/api/snapshots.ts` implementiert mit `listSnapshots`, `getSnapshot`, `deleteSnapshot`, `forgetSnapshots`. ✅ (snapshots.ts:1-40)
  - [ ] `src/lib/api/snapshots.ts` ergänzen um `compare_snapshots`. ⏳ (Backend-Command auskommentiert, noch nicht registriert)
  - [x] **Ergänzung:** Alle API-Wrapper müssen strukturierte Fehlerobjekte (`ErrorDto`) korrekt behandeln. ⏳ (Teilweise implementiert, Error-Handling kann verbessert werden)

- [x] **Daten-Initialisierung (Stores & Pages)** ✅ GRÖSSTENTEILS KOMPLETT
  - [x] `DashboardPage.svelte`: `refreshRepos` (in `onMount`) implementiert, ruft `api.listRepositories` auf und füllt den `$repositories`-Store. ✅ (DashboardPage.svelte)
  - [x] `Repositories.svelte`: `loadRepositories` (in `onMount`) implementiert, ruft `api.listRepositories` auf. ✅ (Repositories.svelte)
  - [x] `Snapshots.svelte`: `refreshSnapshots` (in `onMount`) implementiert, ruft `api.listSnapshots` für Repository auf. ✅ (Snapshots.svelte)
  - [x] `BackupJobs.svelte`: `loadJobs` (in `onMount`) implementiert, ruft `api.listBackupJobs` auf. ✅ (BackupJobs.svelte)
  - [x] **Best-Practice:** Lade- und Fehlerzustände in den jeweiligen Stores abbilden. ✅ (Stores haben loading/error States)

- [ ] **Fehlerbehandlung (Global)**
  - [ ] Alle `invoke`-Aufrufe in `src/lib/api/` und in den Komponenten mit `try...catch`-Blöcken versehen.
  - [ ] Fehler einheitlich über `toastStore.error(error.message)` dem Benutzer anzeigen.
  - [ ] **Ergänzung:** Fehlerobjekte auswerten und ggf. spezifische UI-Reaktionen (z.B. Passwort falsch, Netzwerkfehler) ermöglichen.

- [x] **Dialog-Workflow: Repository**
  - [x] `AddRepositoryDialog.svelte`: `handleSubmit` an `api.initRepository` angebunden.
  - [ ] `AddRepositoryDialog.svelte`: "Durchsuchen"-Button mit `@tauri-apps/api/dialog` (`open({ directory: true })`) implementieren.
  - [x] `DeleteRepoDialog.svelte`: `handleDelete` an `api.deleteRepository` angebunden.
  - [ ] `UnlockRepositoryDialog.svelte`: `handleUnlock` an `api.openRepository` anbinden.
  - [ ] `CheckRepoDialog.svelte`: `startCheck` an `api.checkRepository` anbinden (Fortschritts-Events verarbeiten).
  - [ ] `PruneRepoDialog.svelte`: `startPruning` an `api.pruneRepository` anbinden (Fortschritts-Events verarbeiten).
  - [ ] `ChangePasswordDialog.svelte`: `handleSubmit` an `api.changePassword` anbinden.
  - [ ] **Best-Practice:** Fortschritts- und Ergebnis-Events einheitlich und wiederverwendbar im UI behandeln.

- [x] **Dialog-Workflow: Backup & Restore**
  - [x] `CreateJobDialog.svelte`: `createJob` an `api.createBackupJob` angebunden.
  - [x] `EditJobDialog.svelte`: `handleSubmit` an `api.updateBackupJob` angebunden.
  - [x] `DeleteJobDialog.svelte`: `handleDelete` an `api.deleteBackupJob` angebunden.
  - [ ] `RunBackupDialog.svelte`: Sicherstellen, dass das Starten des Backups (z.B. von `RepositoryCard.svelte`) korrekt funktioniert.
  - [ ] `RestoreDialog.svelte`: `loadFileTree` an `api.getFileTreeCommand` anbinden.
  - [ ] `RestoreDialog.svelte`: `handleRestore` an `api.restoreFilesCommand` anbinden und die `onRestoreProgress`-Events verarbeiten.
  - [ ] `CompareSnapshotsDialog.svelte`: Logik implementieren, um `api.compareSnapshots` aufzurufen und die `diff`-Daten anzuzeigen.
  - [ ] **Best-Practice:** Dialoge auf Fokusmanagement und Accessibility prüfen.

- [ ] **State-Management & Parallelität**
  - [ ] Globales Loading/Error-Handling in den Stores (`backup-jobs.ts`, `repositories.ts`) konsistent nutzen.
  - [ ] Parallele Prozesse (z.B. mehrere Backups) mit eindeutigen Job-IDs und thread-sicherem State verwalten.
  - [ ] **Ergänzung:** UI muss mehrere gleichzeitige Prozesse klar visualisieren.

- [ ] **UI-Konsistenz**
  - [ ] Alle Svelte-Komponenten exakt nach den HTML-Mockups in `docs/mockups/` umsetzen.
  - [ ] Abweichungen dokumentieren und begründen (im Code und PR-Text).

---

## Phase 3: Teststrategie (Vitest & E2E)

- [ ] **Unit-Tests (Vitest)**
  - [ ] Tests für alle Shared-Komponenten und zentrale Store-Logik schreiben.
  - [ ] **Best-Practice:** Möglichst viele kleine, isolierte Tests.

- [ ] **Integrations-Tests (Vitest + Mocked API)**
  - [ ] `test-setup.ts`: Die Mock-Implementierung von `invoke` erweitern, um verschiedene Szenarien (Erfolg, Fehler, Events) für die neuen Befehle zu simulieren.
  - [ ] Typische Workflows (z.B. Snapshots laden, Dialog-Interaktion, Event-Handling) als Integrationstests abdecken.
  - [ ] **Ergänzung:** Auch Fehlerfälle (z.B. Netzwerkfehler, falsches Passwort) abdecken.

- [ ] **End-to-End-Tests (Tauri Driver)**
  - [ ] E2E-Framework (z.B. `tauri-driver` + WebdriverIO) aufsetzen.
  - [ ] "Happy Path" und Fehlerfälle mit echten temporären Repositories/Snapshots testen.
  - [ ] **Best-Practice:** Echte Daten für E2E, Mock-Daten für Unit/Integration.

---

## Phase 4: Refinement & Polishing

- [ ] **Globales State Management**
  - [ ] Sicherstellen, dass alle Aktionen, die Ladezeiten verursachen (Prüfen, Prunen, Backup), den globalen `$loading`-Store in `backup-jobs.ts` oder `repositories.ts` konsistent nutzen.
  - [ ] Sicherstellen, dass alle Fehler in den globalen `$error`-Store geschrieben werden.

- [ ] **Barrierefreiheit (a11y)**
  - [ ] Alle interaktiven Elemente (Buttons, Tabs, Dialoge) auf vollständige Tastatur-Bedienbarkeit prüfen.
  - [ ] Sicherstellen, dass alle Dialoge (`Modal.svelte`) den Fokus korrekt verwalten (Fokusfalle).

- [ ] **Responsive Design**
  - [ ] Die in `rustic_advanced_ui_mockup.html` gezeigten mobilen und Tablet-Ansichten (z.B. für Snapshots, Responsive Grid) im Svelte-Code final implementieren und testen.

- [ ] **Code-Qualität & Aufräumen**
  - [ ] Alle `// TODO:`-Kommentare im Code entfernen oder abarbeiten.
  - [ ] `npm run lint:fix` und `npm run format` ausführen, um Konsistenz sicherzustellen.
  - [ ] **Best-Practice:** Security-Audits (`npm audit`, `cargo audit`) regelmäßig durchführen.
  - [ ] Alle Abhängigkeiten auf die finalen Versionen aktualisieren.

- [ ] **Dokumentation**
  - [ ] README, ROADMAP, CHANGELOG und ggf. Instructions nach jedem Feature/Bugfix aktualisieren.
  - [ ] **Ergänzung:** Neue Patterns in `patterns.instructions.md` dokumentieren.

---

**Hinweis:**

- Automatisierte DTO-Synchronisation, einheitliche Events, strukturierte Fehler, thread-sicheres State-Handling, realistische E2E-Tests und Mockup-Treue sind verbindliche Best-Practices.
- Offene Punkte (z.B. Toolauswahl für DTO-Sync, Detaillierungsgrad Fehlerobjekte, parallele Prozesse) vor Umsetzung final klären!
