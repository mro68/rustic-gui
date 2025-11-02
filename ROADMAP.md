# Rustic GUI Roadmap

> Stand: 2025-11-01 &nbsp;·&nbsp; Verantwortlich: Rustic GUI Team

---

## Aktueller Projektstatus

- **Build & Laufzeit**
  - `cargo build` und `npm run build` kompilieren ohne Blocker; Runtime-Features sind jedoch nur teilweise funktionsfähig, weil mehrere Tauri-Commands noch Platzhalter verwenden.
  - Portable-Konfiguration wird beim Start über `AppState::new()` geladen; verschlüsselte Speicherung funktioniert, fällt bei schreibgeschützten Medien korrekt auf ein Fallback-Verzeichnis zurück.
- **Backend (Tauri + Rust)**
  - Repository-, Backup-, Snapshot- und Restore-Commands sind strukturell vorhanden (`src-tauri/src/commands/*`).
  - Wichtige Operationen wie `check_repository`, `prune_repository`, `change_password` und `delete_snapshot` liefern noch Platzhalter- oder Fehlermeldungen.
  - `run_backup_command`/-Optionen öffnen Repositories und senden Events, aber Passwort-Weitergabe, echte Progress-Kopplung und Fehlerbehandlung sind nicht abgeschlossen.
  - Scheduler (`BackupScheduler`) initialisiert und stellt Jobs wieder her, führt aber keine echten Backups aus (Simulation via `tokio::sleep`).
- **Frontend (Svelte 5 + TypeScript)**
  - Layout, Navigation und Kernseiten existieren; Stores laden reale Daten über die Tauri-API.
  - Zahlreiche Dialoge (Restore, Snapshot-Vergleich, Repository-Wartung) sind UI-seitig angelegt, aber ohne vollständige Backend-Verknüpfung.
  - Snapshots-Seite: Liste/Filter funktionieren, ✅ Snapshot-Vergleich mit vollständigem Tree-Diff implementiert (2025-11-02), Restore-Dialoge warten auf Wiring.
  - Backup-Jobs-Seite: CRUD läuft über API, „Job ausführen“ und Scheduler-Status sind Platzhalter.
- **Tests & Qualität**
  - Vitest-Konfiguration aktiv (`npm test` → `vitest run`), aber nur wenige Komponenten-/Store-Tests vorhanden.
  - Rust-Tests existieren (PortableStore, Scheduler), manche Commands enthalten deaktivierte Assertions.
  - Keine automatisierten End-To-End-Tests.

---

## Feature-Breakdown (Status nach Funktionsbereich)

### Backend

- [x] AppState mit Repository-Cache, Scheduler und PortableStore (AES-256-GCM) (`src-tauri/src/state.rs`, `storage/portable.rs`)
- [x] Settings-Commands (`get_settings`, `save_settings`, `reset_settings`, `update_theme`)
- [~] Repository-Commands (listen/switchen funktionieren, Health/Prune/Passwort sind Stubs)
- [~] Backup-Ausführung (Command-Struktur + Events vorhanden, Passwort-Handling/Progress ausständig)
- [x] Snapshot-Management (Listen/Tagging fertig, Batch-Löschen ok, ✅ Tree-basierter Snapshot-Vergleich implementiert, Einzel-Löschen TODO)
- [~] Restore (`restore_files_v1` ruft rustic::restore, Fehler-/Progresswerte Platzhalter)
- [ ] Scheduled Backups (Callbacks simulieren nur Erfolg; echte Job-Ausführung muss integriert werden)

### Frontend

- [x] MainLayout, Sidebar, Router-Store (`src/lib/components/layout`, `src/lib/stores/router.ts`)
- [x] Settings-Seite inkl. Backend-Speicherung
- [~] Repositories-Seite (Listen/Löschen funktionieren, Unlock/Check/Prune-Dialoge ohne Backend-Funktionen)
- [~] Snapshots-Seite (Liste & Filter ok, ✅ Compare mit Tree-Diff implementiert, Restore/Advanced Filter fehlen)
- [~] Backup-Jobs (CRUD und Dialoge vorhanden, Run/Scheduler-Infos fehlen)
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

1. **Repository-Wartung vervollständigen**
   - `check_repository`, `prune_repository`, `change_password` mit echter rustic_core-Logik implementieren.
   - Dialoge auf der Repositories-Seite aktivieren und mit vernünftigen Status-/Fehlermeldungen versehen.
2. **Backup/Restore Ende-zu-Ende**
   - Passwortdurchleitung & Progress-Callbacks in `run_backup_command` und `restore_files_v1` vervollständigen.
   - Frontend-Buttons („Backup jetzt“, „Restore“) an die Commands anbinden; Events abonnieren.
3. **Scheduler nutzbar machen**
   - Geplante Jobs sollen reale Backups starten (`schedule_backup`, `restore_scheduled_jobs`).
   - Job-Historie und Statusfelder (`last_run`, `next_run`) in UI/Config pflegen.
4. **Snapshot-Lifecycle abrunden**
   - Einzelnes Löschen (`delete_snapshot`) fertigstellen, Restore-Dialog wire-up, Diff-UI verbessern.
5. **Test- & Dokumentationslücken schließen**
   - Vitest- und Rust-Tests für die oben genannten neuen Funktionen ergänzen.
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

| Thema                            | Relevante Dateien                                                                                                      |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Portable Store & Verschlüsselung | `src-tauri/src/state.rs`, `src-tauri/src/storage/portable.rs`                                                          |
| Repository-Commands              | `src-tauri/src/commands/repository.rs`                                                                                 |
| Backup & Scheduler               | `src-tauri/src/commands/backup.rs`, `src-tauri/src/rustic/backup.rs`, `src-tauri/src/scheduler/mod.rs`                 |
| Snapshot/Restore                 | `src-tauri/src/commands/snapshot.rs`, `src-tauri/src/commands/restore.rs`, `src/lib/components/pages/Snapshots.svelte` |
| Frontend Stores                  | `src/lib/stores/*.ts`                                                                                                  |
| Test-Setup                       | `src/test-setup.ts`, `package.json (npm test)`, Rust Tests in `src-tauri/src/*`                                        |

Bitte ROADMAP nach Abschluss jedes Tasks aktualisieren und mit Commit `docs: roadmap aktualisiert (<kurzbeschreibung>)` einchecken.
