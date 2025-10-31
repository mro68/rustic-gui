# Rustic GUI - Development Roadmap v2.0

> **Projekt-Roadmap mit Fokus auf vollständige rustic_core/rustic_backend Integration**
>
> Version: 2.0 | Status: Backend-Integration Phase | Aktualisiert: 2025-10-31

---

## 📊 Projekt-Übersicht

### Ziel

**Produktionsreife Desktop-Anwendung** für rustic Backup-Management mit vollständiger rustic_core/rustic_backend Integration, Cloud-Storage-Support (S3, Azure, GCS via OpenDAL + Rclone), Job-Scheduling und Multi-Repository-Management.

### Aktueller Status (2025-10-31)

- ✅ **UI-Layer:** ~95% komplett (alle Dialoge, Pages, Komponenten implementiert gemäß Mockups)
- ✅ **Backend-Integration:** ~75% komplett (M1 vollständig, rustic_core voll integriert)
- 🟢 **Cloud-Backends:** ~70% (M2 in Arbeit - Backend komplett, Frontend-Integration ⏳)
- ❌ **Job-Scheduler:** 0% (tokio-cron Dependency vorhanden, aber nicht implementiert)
- ❌ **Testing:** ~55% (54 Backend-Tests, Frontend-Tests fehlen noch)

### Geschätzte verbleibende Dauer

**~167 Stunden / 4 Wochen** (bei Vollzeit-Entwicklung)

**Kritischer Pfad zu v1.0:** M2 → M3 → M5 → M6 (127h / 3 Wochen)

### Technologie-Stack

**Frontend:**

- Svelte 5 (Runes API) + TypeScript 5.6+
- TailwindCSS (via CDN, kein Build-Step)
- Vite 6.0 + SvelteKit (File-Router)

**Backend:**

- Rust 1.75+ (Edition 2021) + Tauri 2.0
- **rustic_core 0.8.0** - Backup-Engine ([API Docs](docs/rustic/rustic_core_api.md))
- **rustic_backend 0.5.3** - Storage-Backends ([API Docs](docs/rustic/rustic_backend_api.md))
- **librclone 0.9.0** - 70+ Cloud-Provider via rclone
- **tokio-cron-scheduler 0.13** - Job-Scheduling
- **keyring 3.6** - Passwort-Verwaltung (OS Keychain)

**Build Targets:**

- Linux: AppImage (primär)
- Windows: EXE + MSI Installer

---

## 🎯 Milestone-Übersicht

| Milestone                                             | Beschreibung                    | Dauer    | Status   | Priorität  | Blocker?      |
| ----------------------------------------------------- | ------------------------------- | -------- | -------- | ---------- | ------------- |
| **M0**                                                | Projekt-Setup                   | 5 Tage   | ✅ 100%  | -          | -             |
| **[M1](docs/roadmaps/M1-rustic-core-integration.md)** | rustic_core Integration         | 60h      | ✅ 100%  | ✅ DONE    | ✅ RESOLVED   |
| **[M2](docs/roadmaps/M2-cloud-backends.md)**          | Cloud-Backends (OpenDAL/Rclone) | 30h      | 🟡 70%   | 🟠 HIGH    | 🟡 PARTIAL    |
| **[M3](docs/roadmaps/M3-job-scheduler.md)**           | Job-Scheduler (tokio-cron)      | 30h      | 🔴 0%    | 🟠 HIGH    | ✅ YES        |
| **[M4](docs/roadmaps/M4-advanced-features.md)**       | Erweiterte Features             | 40h      | 🟡 20%   | 🟡 MEDIUM  | ❌ NO         |
| **[M5](docs/roadmaps/M5-testing-qa.md)**              | Testing & QA                    | 54h      | 🟡 25%   | 🔴 HIGHEST | ✅ YES        |
| **[M6](docs/roadmaps/M6-documentation-release.md)**   | Dokumentation & Release         | 13h      | 🟡 50%   | 🟢 LOW     | ❌ NO         |
| **GESAMT**                                            |                                 | **227h** | **~40%** |            | **2 Blocker** |

---

## 📋 Detaillierte Milestones

### Milestone 0: Projekt-Setup ✅ ABGESCHLOSSEN

**Dauer:** 5 Tage | **Status:** 100% komplett | **Abgeschlossen:** Oktober 2025

**Durchgeführte Arbeiten:**

- ✅ Tauri 2.0 + Svelte 5 Projekt initialisiert
- ✅ Projekt-Struktur erstellt (`src/`, `src-tauri/`, `docs/`)
- ✅ Dependencies konfiguriert (Cargo.toml, package.json)
- ✅ Build-Pipeline eingerichtet (Vite, Cargo, Tauri CLI)
- ✅ Grundlegende Typen definiert (DTOs, Error-Types)
- ✅ AppState-Struktur erstellt
- ✅ Config-System aufgesetzt (TOML-basiert)
- ✅ Keychain-Integration für Passwörter
- ✅ Development-Workflow etabliert

**Resultat:** Funktionsfähiges Entwicklungs-Environment mit allen Dependencies.

---

### [Milestone 1: rustic_core Integration](docs/roadmaps/M1-rustic-core-integration.md) ✅ ABGESCHLOSSEN

**Dauer:** 60h (1.5 Wochen) | **Status:** 100% - KOMPLETT  
**Priorität:** 🔴 HIGHEST | **Abgeschlossen:** 2025-10-31

**Ziel:** Alle Backend-Stubs durch echte rustic_core/rustic_backend Calls ersetzen.

**Implementierte Features:**

- ✅ **Repository-Management** (src-tauri/src/rustic/repository.rs)
  - init_repository() - Voll funktional mit rustic_core
  - open_repository() - Voll funktional
  - check_repository() - Mit Status-Ermittlung (Healthy/Locked/Unavailable)
  - get_repository_info() - Mit echten Snapshot-Counts
  - prune_repository() - Mit dry-run Support
  - change_password() - Vollständig implementiert

- ✅ **Backup-Execution** (src-tauri/src/rustic/backup.rs)
  - run_backup_logic() - Mit rustic_core::Repository::backup()
  - Progress-Callbacks über Tauri Events
  - CancellationToken-Integration
  - Tags und Exclude-Patterns Support
  - Error-Handling mit RusticGuiError

- ✅ **Restore-Funktionalität** (src-tauri/src/rustic/restore.rs)
  - restore_files() - Echte Datei-Wiederherstellung
  - get_file_tree() - File-Browser für Navigation
  - Overwrite-Policy Support
  - Progress-Events (Start, Completed, Failed)

- ✅ **Snapshot-Management** (src-tauri/src/rustic/snapshot.rs)
  - list_snapshots() - Mit Sortierung nach Datum
  - get_snapshot() - Vollständige Metadaten
  - delete_snapshot() - Voll funktional
  - forget_snapshots() - Retention-Policy (keep_last)

**Qualitäts-Metriken:**
- ✅ 54 Tests erfolgreich (40 unit + 10 integration + 4 snapshot)
- ✅ 26 Tauri Commands voll funktional
- ✅ ~65% Test-Coverage (geschätzt)
- ✅ Strukturiertes Error-Handling
- ✅ Durchgängiges Logging mit tracing
- ✅ Vollständige Rustdoc-Dokumentation

**Resultat:** Alle Kern-Features nutzen jetzt rustic_core statt Mock-Daten. Backend ist produktionsreif für Haupt-Workflows (Init → Backup → Restore → Snapshot-Management).

---

### [Milestone 2: Cloud-Backend-Integration](docs/roadmaps/M2-cloud-backends.md) 🟡 70% KOMPLETT

**Dauer:** 30h (1 Woche) | **Status:** ~70% - Haupt-Features implementiert  
**Priorität:** 🟠 HIGH

**Ziel:** OpenDAL + Rclone Backend-Support für Cloud-Storage.

**Bereits implementiert (2025-10-31):**

- ✅ OpenDAL-Modul (backends/opendal.rs) - S3, Azure, GCS, B2 Support
- ✅ Rclone-Modul (backends/rclone.rs) - SFTP + 70+ Provider Support
- ✅ Backend-Integration in repository.rs (init_repository erweitert)
- ✅ Connection-Test Command (test_repository_connection)
- ✅ Cloud-Credential-Management (Keychain-Integration)
- ✅ LocationPickerDialog: Connection-Test UI
- ✅ Favoriten-Management (Commands + UI komplett)
- ✅ Recent-Tab mit Backend-Integration

**Noch offen:**

- ⏳ Credential-Prompt nach Connection-Test (2h)
- ⏳ Integration-Tests mit Cloud-Backends (5h)
- ⏳ Dokumentation (Cloud-Setup-Anleitung) (2h)

**Geschätzter Restaufwand:** ~9h von 30h

**UI bereits vorhanden:** LocationPickerDialog mit Local/Network/Cloud/Recent Tabs

➡️ **[Detaillierte Tasks anzeigen](docs/roadmaps/M2-cloud-backends.md)**

---

### [Milestone 3: Job-Scheduler](docs/roadmaps/M3-job-scheduler.md) ⏰ KRITISCH

**Dauer:** 30h (1 Woche) | **Status:** 0% - BLOCKING Automation  
**Priorität:** 🟠 HIGH

**Ziel:** Automatisierte Backup-Jobs mit Cron-Scheduling.

**Umfang:**

- tokio-cron-scheduler Integration (12h)
- Job-State-Persistence & History (10h)
- Retry-Logic & Error-Handling (8h)

**UI bereits vorhanden:** Backup Jobs Page, CreateJobDialog mit Cron-Builder

➡️ **[Detaillierte Tasks anzeigen](docs/roadmaps/M3-job-scheduler.md)**

---

### [Milestone 4: Erweiterte Features](docs/roadmaps/M4-advanced-features.md) ⭐ ENHANCEMENT

**Dauer:** 40h (1 Woche) | **Status:** ~20% (partial UI)  
**Priorität:** 🟡 MEDIUM

**Ziel:** Erweiterte rustic_core Features nutzen.

**Umfang:**

- Snapshot-Tag-Management (8h)
- Snapshot-Diff/Vergleich (12h)
- Repository-Statistiken (10h)
- Settings-Backend-Integration (6h)
- Batch-Operations (4h)

**UI Status:** FilterBar (Tags), CompareSnapshotsDialog vorhanden, Backend fehlt

➡️ **[Detaillierte Tasks anzeigen](docs/roadmaps/M4-advanced-features.md)**

---

### [Milestone 5: Testing & QA](docs/roadmaps/M5-testing-qa.md) 🧪 KRITISCH

**Dauer:** 54h (1.5 Wochen) | **Status:** 0% - BLOCKING Release  
**Priorität:** 🔴 HIGHEST

**Ziel:** Produktionsreife durch Testing absichern.

**Coverage-Ziele:**

- 60% Backend (Unit-Tests für rustic_core Integration)
- 40% Frontend (Stores, API-Wrapper, Dialoge)
- 100% kritische Pfade (Backup, Restore, Repository Init/Open)
- E2E Happy-Path (Add Repo → Create Job → Run Backup → Restore)

➡️ **[Detaillierte Tasks anzeigen](docs/roadmaps/M5-testing-qa.md)**

---

### [Milestone 6: Dokumentation & Release](docs/roadmaps/M6-documentation-release.md) 📚 FINAL

**Dauer:** 13h (2 Tage) | **Status:** ~50%  
**Priorität:** 🟢 LOW

**Ziel:** Release-Readiness herstellen.

**Umfang:**

- README.md Komplettrewrite (User + Developer Guide)
- CHANGELOG.md finalisieren
- Release-Notes & Screenshots
- Build-Artefakte (AppImage, MSI)

➡️ **[Detaillierte Tasks anzeigen](docs/roadmaps/M6-documentation-release.md)**

---

## 📈 Fortschritts-Tracking

### Aktuelle Sprint-Prioritäten (KW 44-45 2025)

1. 🔴 **M1.1-1.2:** Repository Init/Open + Backup-Execution (27h)
2. 🔴 **M1.3:** Restore-Funktionalität (10h)
3. 🟠 **M2.1:** OpenDAL S3-Backend (erste Cloud-Integration, 6h)

**Nächste 2 Wochen Ziel:** M1 komplett abgeschlossen, M2 zu 50%

### Risiko-Matrix

| Risiko                                        | Wahrscheinlichkeit | Impact     | Mitigation                                    |
| --------------------------------------------- | ------------------ | ---------- | --------------------------------------------- |
| rustic_core API-Breaking-Changes              | 🟢 Niedrig         | 🔴 Hoch    | v0.8.0 ist stabil, keine Änderungen erwartet  |
| Performance bei großen Repos (>10k Snapshots) | 🟡 Mittel          | 🟡 Mittel  | Lazy-Loading, Pagination, Optimierung in M4   |
| Cloud-Backend Authentifizierung komplex       | 🟠 Mittel          | 🟠 Mittel  | OpenDAL abstrahiert viel, gute Docs vorhanden |
| Testing verzögert Release                     | 🟡 Mittel          | 🟠 Mittel  | Parallel zu M1-M4 entwickeln, nicht am Ende   |
| Keychain-Probleme (Linux Distros)             | 🟡 Mittel          | 🟢 Niedrig | Fallback auf manuelle Passwort-Eingabe        |

### Abhängigkeits-Graph

```
M0 (Setup) ✅
  ↓
M1 (rustic_core) 🔴 ← BLOCKER
  ↓
  ├→ M2 (Cloud-Backends) 🟠
  ├→ M3 (Job-Scheduler) 🟠
  └→ M4 (Advanced Features) 🟡
       ↓
     M5 (Testing & QA) 🔴 ← BLOCKER
       ↓
     M6 (Release) 🟢
```

**Kritischer Pfad:** M1 → M2 → M3 → M5 → M6 = 187h

---

## 🔗 Ressourcen & Referenzen

### Interne Dokumentation

- **[AI Instructions](.github/copilot-instructions.md)** - Entwicklungs-Guidelines für KI
- **[Backend Instructions](.github/instructions/backend.instructions.md)** - Rust/Tauri Best Practices
- **[Frontend Instructions](.github/instructions/frontend.instructions.md)** - Svelte/TypeScript Patterns
- **[Backup/Restore Instructions](.github/instructions/backup-restore-snapshots.instructions.md)** - Feature-Implementation
- **[Testing Instructions](.github/instructions/testing.instructions.md)** - Test-Strategie

### API-Dokumentation

- **[rustic_core API](docs/rustic/rustic_core_api.md)** - Backup-Engine Referenz
- **[rustic_backend API](docs/rustic/rustic_backend_api.md)** - Storage-Backends Referenz

### UI-Mockups

- **[Haupt-UI](docs/mockups/rustic_gui_mockup.html)** - Dashboard, Navigation, Settings
- **[Backup-Dialoge](docs/mockups/rustic_backup_dialogs.html)** - Job-Creation, Run-Dialog
- **[Repository & Security](docs/mockups/rustic_repo_security_dialogs.html)** - Add Repo, Unlock, Check, Prune
- **[Restore-Dialoge](docs/mockups/rustic_restore_dialogs.html)** - File-Browser, Restore-Options
- **[Location Picker](docs/mockups/rustic_location_picker.html)** - Unified Location-Auswahl
- **[Advanced UI](docs/mockups/rustic_advanced_ui_mockup.html)** - Snapshot-Filter, Vergleich, Pagination

### Externe Referenzen

- **rustic_core:** https://docs.rs/rustic_core
- **rustic_backend:** https://docs.rs/rustic_backend
- **Tauri 2.0:** https://v2.tauri.app/
- **Svelte 5:** https://svelte-5-preview.vercel.app/
- **OpenDAL:** https://opendal.apache.org/
- **Rclone:** https://rclone.org/

---

## 📝 Change Log

### v2.0 - 2025-10-31

- **🔄 Roadmap-Restrukturierung:** Fokus-Shift von "UI-First" zu "Backend-Integration-First"
- **📂 Modulare Struktur:** Milestones in separate Dateien ausgelagert (`docs/roadmaps/`)
- **🎯 Neue Priorisierung:** M1 rustic_core Integration als höchste Priorität
- **☁️ Cloud-Focus:** M2 dedicated zu Cloud-Backends (OpenDAL + Rclone)
- **🧪 Testing-Strategie:** M5 mit 60/40 Coverage-Zielen definiert
- **📊 Zeitschätzungen:** Realistische 227h basierend auf aktueller Analyse

### v1.0 - 2025-10-26

- Initiale Roadmap mit M0-M6 Struktur
- UI-First-Ansatz (abgelöst durch v2.0)

---

**Version:** 2.0  
**Letzte Aktualisierung:** 2025-10-31  
**Maintainer:** Rustic GUI Team

🚀 **Los geht's mit [M1: rustic_core Integration](docs/roadmaps/M1-rustic-core-integration.md)!**
