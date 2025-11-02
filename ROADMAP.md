# Rustic GUI Roadmap

> Stand: 2025-11-02 &nbsp;·&nbsp; Verantwortlich: Rustic GUI Team

---

## Aktueller Projektstatus

- **Build & Laufzeit**
  - ✅ `cargo build` und `npm run build` kompilieren **ohne Errors** (0 TypeScript-Fehler, alle Rust-Tests passing, Stand 2025-11-02).
  - Portable-Konfiguration wird beim Start über `AppState::new()` geladen; verschlüsselte Speicherung funktioniert, fällt bei schreibgeschützten Medien korrekt auf ein Fallback-Verzeichnis zurück.
- **Backend (Tauri + Rust)**
  - Repository-, Backup-, Snapshot- und Restore-Commands sind strukturell vorhanden (`src-tauri/src/commands/*`).
  - ✅ **Repository-Wartung vollständig implementiert:** `check_repository`, `prune_repository`, `change_password` nutzen rustic_core API (2025-11-02).
  - ✅ `delete_snapshot` vollständig implementiert mit Backend-Integration.
  - `run_backup_command` sendet Progress-Events, aber Passwort-Handling und vollständige Error-Propagierung noch ausstehend.
  - Scheduler (`BackupScheduler`) initialisiert und stellt Jobs wieder her, führt aber keine echten Backups aus (Simulation via `tokio::sleep`).
- **Frontend (Svelte 5 + TypeScript)**
  - ✅ **Vollständige Svelte 5 Migration:** Alle Komponenten nutzen `$state()`, `$bindable()` und `bind:open` Pattern (2025-11-02).
  - Layout, Navigation und Kernseiten existieren; Stores laden reale Daten über die Tauri-API.
  - ✅ **Repository-Wartungs-Dialoge vollständig integriert:** Check, Prune, Change Password mit Backend-Anbindung (2025-11-02).
  - Snapshots-Seite: Liste/Filter funktionieren, ✅ Snapshot-Vergleich mit vollständigem Tree-Diff implementiert (2025-11-02), Restore-Dialoge warten auf Wiring.
  - Backup-Jobs-Seite: CRUD läuft über API, ✅ Scheduler-Integration mit Schedule/Unschedule Buttons funktionsfähig (2025-11-02).
- **Tests & Qualität**
  - ✅ 16/16 Rust Integration-Tests passing (2025-11-02).
  - Vitest-Konfiguration aktiv (`npm test` → `vitest run`), Store-Tests vorhanden und passing.
  - Keine automatisierten End-To-End-Tests.

---

## Feature-Breakdown (Status nach Funktionsbereich)

### Backend

- [x] AppState mit Repository-Cache, Scheduler und PortableStore (AES-256-GCM) (`src-tauri/src/state.rs`, `storage/portable.rs`)
- [x] Settings-Commands (`get_settings`, `save_settings`, `reset_settings`, `update_theme`)
- [x] Repository-Commands (✅ check_repository, prune_repository, change_password vollständig implementiert 2025-11-02)
- [~] Backup-Ausführung (Command-Struktur + Events vorhanden, Passwort-Handling/Progress ausständig)
- [x] Snapshot-Management (Listen/Tagging fertig, ✅ Batch-Löschen ok, ✅ Tree-basierter Snapshot-Vergleich implementiert, ✅ Einzel-Löschen implementiert)
- [~] Restore (`restore_files_v1` ruft rustic::restore, Fehler-/Progresswerte Platzhalter)
- [x] Scheduled Backups (✅ BackupScheduler mit tokio_cron_scheduler, schedule_backup/unschedule_backup Commands, Frontend-Integration in BackupJobs.svelte)

### Frontend

- [x] MainLayout, Sidebar, Router-Store (`src/lib/components/layout`, `src/lib/stores/router.ts`)
- [x] Settings-Seite inkl. Backend-Speicherung
- [x] Repositories-Seite (✅ Check/Prune/ChangePassword-Dialoge vollständig integriert 2025-11-02)
- [~] Snapshots-Seite (Liste & Filter ok, ✅ Compare mit Tree-Diff implementiert, Restore/Advanced Filter fehlen)
- [x] Backup-Jobs (CRUD und Dialoge vorhanden, ✅ Scheduler-Integration mit Schedule/Unschedule Buttons)
- [ ] Restore-Dialog-Flow (FileTree lädt Daten, aber Restore-Button nicht verdrahtet)
- [ ] Dashboard-Widgets (Platzhalterdaten, keine echten Statistiken)

### Packaging & Distribution

- [x] `npm run tauri:build` erzeugt AppImage (Linux) und portable Windows-EXE (`src-tauri/tauri.conf.json` Targets)
- [x] Portable-Deployments speichern `config.toml` verschlüsselt neben dem Binary; Fallback-Verzeichnis (Temp) wird protokolliert
- [ ] Release-Automatisierung / Signierung (keine Pipelines dokumentiert)
- [ ] Installer-Pfade testen (Windows MSI, Linux Desktopeinträge)

### Qualität & Infrastruktur

- [x] ESLint, Prettier, svelte-check und Clippy konfiguriert
- [~] Unit-Tests (Vitest + Rust) vorhanden, aber geringe Abdeckung
- [ ] Integrations-/E2E-Tests
- [ ] CI/CD-Pipeline für Build, Test, Packaging

Legende: `[x]` fertig · `[~]` teilweise · `[ ]` offen

---

## Kurzfristige Prioritäten (November 2025)

1. **✅ Repository-Wartung vervollständigen** (Abgeschlossen 2025-11-02)
   - [x] `check_repository`, `prune_repository`, `change_password` mit echter rustic_core-Logik implementiert
   - [x] Dialoge auf der Repositories-Seite aktiviert und mit Status-/Fehlermeldungen versehen
   - [ ] Browser-Tests mit echtem Repository durchführen
2. **🚧 Backup/Restore Ende-zu-Ende**
   - [ ] Passwortdurchleitung & Progress-Callbacks in `run_backup_command` und `restore_files_v1` vervollständigen
   - [ ] Frontend-Buttons („Backup jetzt", „Restore") an die Commands anbinden; Events abonnieren
3. **🚧 Scheduler nutzbar machen**
   - [ ] Geplante Jobs sollen reale Backups starten (`schedule_backup`, `restore_scheduled_jobs`)
   - [ ] Job-Historie und Statusfelder (`last_run`, `next_run`) in UI/Config pflegen
4. **✅ Snapshot-Lifecycle abrunden** (Kern-Funktionen abgeschlossen 2025-11-02)
   - [x] Einzelnes Löschen (`delete_snapshot`) fertiggestellt
   - [x] Tag-Verwaltung vollständig implementiert (Backend + UI)
   - [x] Retention-Policy vollständig implementiert (Backend + Frontend + UI Dialog)
   - [ ] Restore-Dialog wire-up ausstehend
   - [ ] Diff-UI verbessern
5. **🚧 Test- & Dokumentationslücken schließen**
   - [x] 16/16 Rust Integration-Tests passing (2025-11-02)
   - [ ] Vitest- und Rust-Tests für neue Funktionen ergänzen
   - [ ] CHANGELOG und README aktualisieren, sobald Features greifen
   - CHANGELOG und README aktualisieren, sobald Features greifen.

---

## Milestones

### Milestone A – MVP „Lokale Backups“ (Ziel: KW 46)

- Repository- und Passwort-Fluss stabil
- Manuelles Backup/Restore inkl. Progress & Fehlerhandling
- Snapshot-Liste mit Löschen, Restore und Tagging
- Settings persistiert, PortableStore-Status im UI sichtbar

### Milestone B – Geplante Backups & Cloud (Ziel: KW 48)

- Scheduler führt echte Backups aus und protokolliert Ergebnisse
- Connection-Test & Location-Picker für S3/rclone vollständig
- Repository-Health (check/prune) funktionsfähig
- UI-Feedback für portable Fallbacks und Keychain-Status

### Milestone C – Beta-Qualität (Ziel: KW 50)

- Testabdeckung: ≥60 % Kernlogik (Vitest + Rust)
- Release-Pipeline (AppImage, Windows portable + Signierung)
- Benutzer-Dokumentation & Onboarding
- Performance-/Stabilitätsprüfung mit großen Repositories

---

## Referenzen & nächste Schritte

| Thema                            | Relevante Dateien                                                                                                                |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Portable Store & Verschlüsselung | `src-tauri/src/state.rs`, `src-tauri/src/storage/portable.rs`                                                                    |
| Repository-Commands              | `src-tauri/src/commands/repository.rs`                                                                                           |
| Backup & Scheduler               | `src-tauri/src/commands/backup.rs`, `src-tauri/src/rustic/backup.rs`, `src-tauri/src/scheduler/mod.rs`                           |
| Snapshot/Restore                 | `src-tauri/src/commands/snapshot.rs`, `src-tauri/src/commands/restore.rs`, `src/lib/components/pages/Snapshots/` (modularisiert) |
| Frontend Stores                  | `src/lib/stores/*.ts`                                                                                                            |
| Test-Setup                       | `src/test-setup.ts`, `package.json (npm test)`, Rust Tests in `src-tauri/src/*`                                                  |

Bitte ROADMAP nach Abschluss jedes Tasks aktualisieren und mit Commit `docs: roadmap aktualisiert (<kurzbeschreibung>)` einchecken.
