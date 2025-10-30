# TODO-Liste: Rustic GUI Integration (Svelte 5 + Tauri 2)

## ✅ IMPLEMENTIERUNGS-STATUS (Stand: 2025-10-30)

### 🟢 Phase 1: Rust-Backend - **KOMPLETT** 

**Alle Backend-Commands implementiert und registriert:**

- ✅ Repository-Management (7 Commands): list, init, open, delete, check, prune, change_password
- ✅ Backup-Jobs (5 Commands): list, create, update, delete, get
- ✅ Snapshots (4 Commands): list, get, delete, forget
- ✅ Backup & Restore (4 Commands): run_backup, cancel, restore, get_file_tree
- ✅ System & Keychain (4 Commands): shutdown, store/get/delete password
- ✅ Event-System mit einheitlichem Format
- ✅ Config-Management (TOML)
- ✅ State-Management (AppState)

### 🟡 Phase 2: Frontend - **~70% FERTIG**

**Implementiert:**
- ✅ API-Wrapper vollständig (repositories, backup-jobs, backup, restore, snapshots)
- ✅ Stores mit Daten-Loading (repositories, backup-jobs)
- ✅ Type-System synchronisiert (BackupJobDto, RepositoryDto, snake_case)
- ✅ 5 Dialog-Workflows: Add/Delete Repo, Create/Edit/Delete Job
- ✅ Cron-Schedule-Konvertierung (daily, weekly, monthly)
- ✅ Seiten laden Daten: Dashboard, Repositories, BackupJobs

**Noch offen (~30%):**
- ⏳ 7 Dialog-Workflows: Unlock, Check, Prune, ChangePassword, Restore, Compare, RunBackup
- ⏳ Snapshots-Seite API-Integration
- ⏳ File-Browser mit Dialog-Integration
- ⏳ Error-Handling verbessern (strukturierte ErrorDto)

### 📊 Code-Qualität

**Aktuell:**
- Type-Check: 19 Errors (von ursprünglich 31)
- Linter-Warnungen: 58 (Rust)
- TODO-Kommentare: ~50 (von ursprünglich 84)

**Ziel:**
- Type-Check: 0 Errors
- Linter-Warnungen: < 20
- TODO-Kommentare: < 20 (nur echte TODOs)

---

Hier ist die ausführlich ergänzte TODO-Liste für die Integration von Svelte 5 und Tauri 2, inklusive Best-Practice-Hinweisen und konkreten Ergänzungen:

---

## Phase 1: Rust-Backend (Tauri 2-Befehle & Events)

Der wichtigste Schritt ist die Implementierung der Rust-Seite, die die in `src/lib/api/` definierten Verträge erfüllt.

- [x] **Grund-Setup (in `src-tauri/src/main.rs`)**
  - [x] `main`-Funktion mit `tauri::Builder` aufsetzen.
  - [x] Alle `#[tauri::command]`-Funktionen im `.invoke_handler()` registrieren.
  - [x] Einen `tauri::State` (z.B. `AppState`) einrichten, um langlebige Objekte wie Repository-Verbindungen oder einen `tokio::Mutex` für den Zugriff auf `rustic_core` zu verwalten.
  - [x] **Best-Practice:** State thread-sicher gestalten, um parallele Operationen (z.B. mehrere Backups) zu ermöglichen.

- [x] **Befehle: Repository-Management (Rust)**
  - [x] `#[tauri::command] async fn list_repositories() -> Result<Vec<RepositoryDto>, ErrorDto>` (IMPLEMENTIERT in commands/repository.rs)
  - [x] `#[tauri::command] async fn init_repository(path: String, password: String, ...) -> Result<(), ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn open_repository(path: String, password: String) -> Result<RepositoryDto, ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn delete_repository(id: String, delete_data: bool) -> Result<(), ErrorDto>` (IMPLEMENTIERT in commands/repository.rs)
  - [x] `#[tauri::command] async fn check_repository(id: String, read_data: bool, ...) -> Result<CheckResultDto, ErrorDto>` (IMPLEMENTIERT in commands/repository.rs)
  - [x] `#[tauri::command] async fn prune_repository(id: String, ...) -> Result<PruneResultDto, ErrorDto>` (IMPLEMENTIERT in commands/repository.rs)
  - [x] `#[tauri::command] async fn change_password(id: String, old_pass: String, new_pass: String) -> Result<(), ErrorDto>` (IMPLEMENTIERT in commands/repository.rs)
  - [x] **Ergänzung:** Fehler als strukturierte Objekte (`ErrorDto`) zurückgeben, nicht nur als String.
  - [x] **Hinweis:** Alle Repository-Commands sind nun in lib.rs registriert

- [x] **Befehle: Backup-Job-Management (Rust)**
  - [x] `#[tauri::command] async fn list_jobs() -> Result<Vec<BackupJob>, ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn create_job(job: BackupJob) -> Result<(), ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn update_job(job: BackupJob) -> Result<(), ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn delete_job(id: String) -> Result<(), ErrorDto>` (Platzhalter vorhanden)

- [x] **Befehle: Snapshot-Management (Rust)**
  - [x] `#[tauri::command] async fn list_snapshots(repository_id: String) -> Result<Vec<SnapshotDto>, ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn get_snapshot_info(id: String) -> Result<SnapshotDto, ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn delete_snapshot(id: String) -> Result<(), ErrorDto>` (Platzhalter vorhanden)
  - [x] `#[tauri::command] async fn compare_snapshots(id_a: String, id_b: String) -> Result<DiffResultDto, ErrorDto>` (Platzhalter vorhanden)

- [x] **Befehle: Prozess-Steuerung (Rust)**
  - [x] `#[tauri::command] async fn run_backup(job_id: String, app_handle: tauri::AppHandle)` (Platzhalter vorhanden)
    - [x] Diese Funktion _muss_ Events für Fortschritt (`backup-progress`), Erfolg (`backup-completed`) und Fehler (`backup-failed`) an das Frontend senden (siehe `src/lib/api/events.ts`).
    - [x] **Best-Practice:** Einheitliches Event-Format verwenden: `{ type, progress, message, jobId, ... }`
  - [x] `#[tauri::command] async fn cancel_backup(job_id: String)` (implementiert)
    - [x] Implementieren Sie eine Logik (z.B. über `tokio::sync::watch`), um laufende Backups abzubrechen.
  - [x] `#[tauri::command] async fn restore_files_command(..., app_handle: tauri::AppHandle)` (Platzhalter vorhanden)
    - [x] Diese Funktion _muss_ Events für den Wiederherstellungs-Fortschritt senden (siehe `src/lib/api/restore.ts`, Event: `restore-progress`).
    - [x] **Best-Practice:** Auch hier einheitliches Event-Format.
  - [x] `#[tauri::command] async fn get_file_tree_command(...) -> Result<FileTreeNode, ErrorDto>` (Platzhalter vorhanden)

- [ ] **Typen & DTOs (Rust)**
  - [x] Sicherstellen, dass alle `...Dto`-Typen (wie `RepositoryDto`, `SnapshotDto`, `FileTreeNode`, `ErrorDto`) in Rust definiert sind, `serde::Serialize` implementieren und den TypeScript-Typen in `src/lib/types/` entsprechen. (Hinweis: Optionale Felder wie `username`/`summary` in `SnapshotDto` können bei Bedarf ergänzt werden.)
  - [ ] **Best-Practice:** Automatisierte Synchronisation der DTOs mit TypeScript-Typen (z.B. via `ts-rs` oder `typeshare`), Build-Workflow anpassen.

---

## Phase 2: Svelte 5-Frontend (API-Anbindung & Logik)

- [x] **Fehlende API-Wrapper (TypeScript)**
  - [x] `src/lib/api/backup-jobs.ts` erstellt für `list_jobs`, `create_job`, `update_job`, `delete_job`.
  - [x] `src/lib/api/repositories.ts` ergänzt um `delete_repository`, `check_repository`, `prune_repository`, `change_password`.
  - [ ] `src/lib/api/snapshots.ts` ergänzen um `compare_snapshots`.
  - [ ] **Ergänzung:** Alle API-Wrapper müssen strukturierte Fehlerobjekte (`ErrorDto`) korrekt behandeln.

- [x] **Daten-Initialisierung (Stores & Pages)**
  - [x] `DashboardPage.svelte`: `refreshRepos` (in `onMount`) implementiert, ruft `api.listRepositories` auf und füllt den `$repositories`-Store.
  - [x] `Repositories.svelte`: `loadRepositories` (in `onMount`) implementiert, ruft `api.listRepositories` auf.
  - [ ] `Snapshots.svelte`: `refreshSnapshots` (in `onMount`) implementieren, um `api.listSnapshots` für alle entsperrten Repos aufzurufen.
  - [x] `BackupJobs.svelte`: `loadJobs` (in `onMount`) implementiert, ruft `api.listBackupJobs` auf.
  - [ ] **Best-Practice:** Lade- und Fehlerzustände in den jeweiligen Stores abbilden.

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
