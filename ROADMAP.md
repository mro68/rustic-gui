# Rustic GUI - Development Roadmap v3.0

> **REALISTISCHE Projekt-Roadmap basierend auf tatsächlichem Implementierungsstand**
>
> Version: 3.0 | Status: 🔴 BUILD BROKEN - Phase 0 erforderlich | Aktualisiert: 2025-10-31

---

## � KRITISCHE WARNUNG

**Das Projekt kompiliert aktuell NICHT!**

```bash
error[E0433]: failed to resolve: could not find `PackFile` in `repofile`
   --> src-tauri/src/rustic/repository.rs:593:43
```

**Phase 0 (Notfall-Reparatur) MUSS zuerst durchgeführt werden!**

---

## �📊 Projekt-Übersicht

### Ziel

**Produktionsreife Desktop-Anwendung** für rustic Backup-Management mit vollständiger rustic_core/rustic_backend Integration, Cloud-Storage-Support (S3, Azure, GCS via OpenDAL + Rclone), Job-Scheduling und Multi-Repository-Management.

### Aktueller Status (2025-10-31) - KORRIGIERT

- ✅ **UI-Layer:** ~90% vorhanden (Komponenten existieren, Backend-Integration fehlt)
- 🔴 **Backend-Integration:** ~30% funktional (Code existiert, aber nicht getestet/vollständig)
- 🔴 **Cloud-Backends:** 0% funktional (nur Code-Skelett vorhanden)
- 🟡 **Job-Scheduler:** ~40% (Struktur da, nicht integriert/getestet)
- 🔴 **Testing:** ~5% (Build broken = Tests laufen nicht)

### Tatsächlicher Gesamtfortschritt: ~20-30%

**NICHT** die vorher genannten ~53%! Die vorherige Roadmap war zu optimistisch.

### Geschätzte verbleibende Dauer bis v1.0

**~108 Stunden / 3 Wochen** (bei Vollzeit-Entwicklung)

- Phase 0 (Notfall): 8h
- Phase 1 (MVP): 25h
- Phase 2 (Erweitert): 30h
- Phase 3 (Advanced): 25h
- Phase 4 (Testing+Docs): 20h

**Kritischer Pfad:** Phase 0 → Phase 1 → MVP (33h / 1 Woche)

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

## 🎯 Phasen-Übersicht (NEU - Realistisch)

| Phase                                     | Beschreibung            | Dauer    | Status   | Priorität  | Blocker?         |
| ----------------------------------------- | ----------------------- | -------- | -------- | ---------- | ---------------- |
| **[Phase 0](#phase-0-notfall-reparatur)** | Build Fix + Basis-Demo  | 8h       | 🔴 0%    | 🔴 HIGHEST | ✅ YES (BUILD)   |
| **[Phase 1](#phase-1-mvp-core)**          | MVP Core Features       | 25h      | 🔴 20%   | 🔴 HIGHEST | ✅ YES (Phase 0) |
| **[Phase 2](#phase-2-erweiterte-basis)**  | Cloud + Scheduler       | 30h      | 🔴 5%    | 🟠 HIGH    | ❌ NO            |
| **[Phase 3](#phase-3-advanced-features)** | Erweiterte Features     | 25h      | � 0%     | 🟡 MEDIUM  | ❌ NO            |
| **[Phase 4](#phase-4-testing--release)**  | Testing, Docs & Release | 20h      | � 10%    | � HIGH     | ❌ NO            |
| **GESAMT**                                |                         | **108h** | **~20%** |            | **2 Blocker**    |

### Alte Milestones (M0-M6) archiviert

Die vorherigen Milestones M1-M6 waren zu optimistisch dokumentiert. Status siehe `docs/reports/archive/`.

## 📋 Detaillierte Phasen

### Phase 0: NOTFALL-REPARATUR 🚨 KRITISCH

**Dauer:** 8h (1 Tag) | **Status:** 🔴 0% - NICHT GESTARTET  
**Priorität:** 🔴 HÖCHSTE | **Blocker:** Ja (Projekt kompiliert nicht)

**Ziel:** Projekt kompilierbar machen und minimalen Funktionsumfang demonstrieren können.

#### Tasks

**Task 0.1: Build-Fehler beheben** (2h)

- ❌ PackFile-Error in `src-tauri/src/rustic/repository.rs:593,602` fixen
- ❌ rustic_core 0.8.0 API-Änderungen nachvollziehen
- ❌ Alternative API für Repository-Statistiken finden ODER
- ❌ Feature temporär deaktivieren (get_repository_stats)

**Task 0.2: Warnings bereinigen** (1h)

- ❌ Unused imports entfernen (3 Stellen)
- ❌ Unused variables fixen (5+ Stellen)

---

### Phase 1: MVP CORE FEATURES 🎯

**Dauer:** 25h (3-4 Tage) | **Status:** 🔴 20% - IN ARBEIT  
**Priorität:** 🔴 HÖCHSTE | **Abhängigkeit:** Phase 0 MUSS abgeschlossen sein

**Ziel:** Minimaler funktionsfähiger Prototyp - Backup/Restore für lokale Repositories.

```

**Task 0.4: Minimaler Funktionstest** (3h)
- ❌ Repository erstellen (lokal, OHNE Passwort als Zwischenlösung)
- ❌ Basic UI zeigt etwas an
- ❌ Ein Command funktioniert (z.B. list_repositories)
- ❌ App startet ohne Crash

**Deliverables:**
- ✅ `cargo build` erfolgreich
- ✅ App startet und zeigt UI
- ✅ Mindestens 1 funktionierender Workflow demonstrierbar


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
```
