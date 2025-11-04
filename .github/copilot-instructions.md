# Rustic GUI - AI Development Instructions

> **Zentrale Anweisungen für KI-gestützte Entwicklung des Rustic GUI Projekts**
>
> Version: 2.1 | Datum: 2025-10-26 | Sprache: Deutsch/Englisch (hybrid)

---

## 🎯 Wichtige Grundregeln

### ⚠️ KRITISCH: Instructions vor jedem Schritt einlesen!

**Du MUSST diese Instructions (inkl. aller verlinkten Dateien) vor JEDEM Entwicklungsschritt einlesen und befolgen!**

**Workflow für jeden Task:**

1. ✅ Lies diese Datei vollständig
2. ✅ Lies die relevanten verlinkten Instructions-Dateien
3. ✅ **Prüfe UI-Mockups in `docs/mockups/` (falls UI-Änderungen)**
4. ✅ Prüfe aktuelle Roadmap in `ROADMAP.md`
5. ✅ Implementiere gemäß den Vorgaben
6. ✅ Aktualisiere Dokumentation falls nötig
7. ✅ Aktualisiere Roadmap-Status falls relevant

### 📋 Dokumentations-Pflicht

Nach **jedem** Feature/Bugfix:

- ✅ Aktualisiere `ROADMAP.md` (Status, Fortschritt)
- ✅ Aktualisiere relevante `*.instructions.md` bei neuen Patterns
- ✅ Aktualisiere `CHANGELOG.md` bei User-relevanten Änderungen
- ✅ Prüfe ob `README.md` aktualisiert werden muss

### 🎨 UI-Mockups sind PFLICHT!

Im Ordner `docs/mockups/` befinden sich **4 HTML-Mockup-Dateien** mit dem kompletten UI-Design.

**Bei allen Svelte-Komponenten:**

- ✅ Prüfe ZUERST welches Mockup existiert
- ✅ Implementiere **exakt** nach Mockup-Vorgaben
- ✅ Bei Abweichungen: Begründe und dokumentiere
- ✅ Neue UI-Elemente: Erweitere Mockup, dann Code

---

## 📚 Instructions-Struktur

Diese Instructions sind modular aufgeteilt. Lies die relevanten Dateien je nach Task:

### Kern-Instructions (immer relevant)

- **[workflow.instructions.md](workflow.instructions.md)**
  - Entwicklungs-Workflow
  - Git-Konventionen (Conventional Commits)
  - Task-Abläufe
  - ROADMAP.md & CHANGELOG.md Management

- **[code-style.instructions.md](code-style.instructions.md)**
  - Sprachkonventionen (Deutsch/Englisch Hybrid)
  - Naming Conventions (TypeScript, Rust)
  - Dokumentations-Standards (TSDoc, Rustdoc)
  - Best Practices & Anti-Patterns

- **[architecture.instructions.md](architecture.instructions.md)**
  - System-Architektur & Datenflüsse
  - Komponentenstruktur
  - Frontend ↔ Backend Kommunikation
  - Performance & Security

### Technologie-spezifisch

- **[frontend.instructions.md](frontend.instructions.md)**
  - Svelte 5 + TypeScript
  - UI-Komponenten-Patterns
  - State Management (Stores)
  - **UI-Mockup-Integration** (WICHTIG!)

- **[backend.instructions.md](backend.instructions.md)**
  - Rust + Tauri 2.0
  - rustic_core Integration
  - Tauri Commands & AppState
  - Config-Management (TOML)
  - Passwort-Handling (Keychain)
  - Job-Scheduling (Cron)

- **[testing.instructions.md](testing.instructions.md)**
  - Test-Strategien & Test-Pyramide
  - Frontend-Tests (Vitest)
  - Backend-Tests (Cargo Test)
  - Coverage-Ziele & CI/CD

### Feature-spezifisch

- **[backup-restore-snapshots.instructions.md](instructions/backup-restore-snapshots.instructions.md)**
  - Backup-Implementation & Progress-Tracking
  - Restore-Logik & File-Browser
  - Snapshot-Management & Vergleich
  - Job-Scheduling & Retention-Policies

- **[patterns.instructions.md](patterns.instructions.md)**
  - Repository-Switching
  - Batch-Operations mit Progress
  - FileTree mit Lazy-Loading
  - Toast-Notifications
  - Form-Validation
  - Theme-Switching
  - Debounced Search
  - Weitere wiederverwendbare Patterns

### Hilfsdokumente

- **[troubleshooting.instructions.md](troubleshooting.instructions.md)**
  - 8+ häufige Probleme & Lösungen
  - Debug-Workflows
  - Performance-Debugging
  - Issue-Template

---

## 🎨 UI-Mockups (WICHTIG!)

### Mockup-Dateien

Die **komplette UI** ist in 5 HTML-Mockup-Dateien definiert:

```
docs/mockups/
├── rustic_gui_mockup.html                # 🏠 Hauptfenster & Navigation
│   ├── Sidebar mit Navigation
│   ├── Dashboard (Repository-Cards, Storage-Charts)
│   ├── Backup-Jobs-Liste
│   ├── Snapshots-Tabelle
│   ├── Repository-Details
│   └── Settings
│
├── rustic_backup_dialogs.html            # 💼 Backup-Job-Dialogs
│   ├── Create Backup Job (4-Tab-Wizard)
│   │   ├── General (Name, Repository, Tags)
│   │   ├── Paths & Exclusions
│   │   ├── Schedule (Cron-Builder)
│   │   └── Retention Policy
│   ├── Edit Backup Job
│   ├── Run Backup (Progress-Dialog)
│   └── Delete Job (Confirmation)
│
├── rustic_repo_security_dialogs.html     # 🔐 Repository & Security
│   ├── Add Repository (Type-Selector: Local/SFTP/S3/rclone)
│   ├── Unlock Repository (Password-Input)
│   ├── Check Repository (Progress mit Log)
│   ├── Prune Repository (Stats & Confirmation)
│   └── Change Password (mit Strength-Indicator)
│
├── rustic_restore_dialogs.html           # 🔄 Restore & Vergleich
│   ├── Restore Files (File-Browser mit Checkboxes)
│   ├── Restore Options (Target, Overwrite-Behavior)
│   ├── Compare Snapshots (Side-by-Side Diff)
│   │   ├── Filter-Bar (Added/Removed/Modified)
│   │   └── Diff-Statistics
│   └── Snapshot Info (Details-Ansicht)
│
├── rustic_advanced_ui_mockup.html        # ⭐ Erweiterte UI-Features für Snapshots
│   ├── Advanced Filter-Bar (Tags, Hostname, Zeitraum, Größe)
│   ├── Kontextmenü für Snapshots (Rechtsklick, Bulk)
│   ├── Pagination-Controls
│   ├── Snapshot-Vergleichs-UI (Side-by-Side)
│   └── Responsive Layouts (Desktop/Tablet/Mobile)
│
├── rustic_location_picker.html           # 📂 Unified Location Picker (seit 2025-10-30)
│   ├── 4 Tabs: Local, Network (SFTP), Cloud (S3/rclone), Recent
│   ├── Smart-Input mit Auto-Type-Detection
│   │   - Local: /path/to/repo
│   │   - SFTP: sftp://user@host:/path
│   │   - S3: s3://bucket/prefix
│   │   - rclone: rclone:remote:path
│   ├── Connection-Test-Button mit Validierung
│   ├── Favoriten-Management (Speichern/Laden)
│   └── Integration mit AddRepositoryDialog
│
└── rustic_advanced_functions.html        # ⚡ Erweiterte Repository-Funktionen
    ├── Repository-Wartung (Check, Prune, Repair)
    ├── Diagnose & Statistiken
    │   - Repository-Größe & Kompression
    │   - Index-Statistiken
    │   - Pack-File-Analyse
    ├── Prune-Dialog mit Dry-Run-Modus
    └── Check-Dialog mit Progress-Reporting
```

### Mockup-Nutzung

**Vor dem Erstellen/Ändern einer Svelte-Komponente:**

1. ✅ **Öffne relevantes Mockup** (HTML-Datei im Browser)
2. ✅ **Analysiere genau:**
   - Layout & Struktur
   - Farben & Spacing
   - Interaktionselemente
   - States (Active, Hover, Disabled)
   - Responsive-Verhalten
3. ✅ **Implementiere exakt nach Vorgabe:**
   - CSS-Klassen aus Mockup übernehmen
   - Gleiche Farben (`#3b82f6`, `#22273a`, etc.)
   - Gleiche Border-Radius, Padding, Gaps
   - Gleiche Font-Sizes
4. ✅ **Bei Unklarheiten:** Frage nach bevor du abweichst
5. ✅ **Dokumentiere Abweichungen** mit Begründung

**Beispiel-Mapping:**

```
Mockup-Datei                              → Svelte-Komponente
───────────────────────────────────────────────────────────────────────────────
⚠️ WICHTIG: Seit Nov 2025 hat jede Page EIGENEN Header (kein globales Header.svelte mehr!)
───────────────────────────────────────────────────────────────────────────────
rustic_gui_mockup.html (Sidebar)         → src/lib/components/layout/Sidebar.svelte
rustic_gui_mockup.html (Dashboard)       → src/lib/components/pages/DashboardPage.svelte (mit eigenem Page-Header!)
rustic_backup_dialogs.html (Create)      → src/lib/components/dialogs/JobDialog.svelte (mode='create')
rustic_backup_dialogs.html (Edit)        → src/lib/components/dialogs/JobDialog.svelte (mode='edit')
                                         → dialogs/JobDialog/GeneralTab.svelte
                                         → dialogs/JobDialog/PathsTab.svelte
                                         → dialogs/JobDialog/ScheduleTab.svelte
                                         → dialogs/JobDialog/RetentionTab.svelte
rustic_repo_security_dialogs.html        → src/lib/components/dialogs/AddRepositoryDialog.svelte
rustic_restore_dialogs.html              → src/lib/components/dialogs/RestoreDialog.svelte
rustic_advanced_ui_mockup.html (Snapshots Advanced)
                                         → src/lib/components/pages/Snapshots/ (modularisiert)
                                         → pages/Snapshots/SnapshotTable.svelte
                                         → pages/Snapshots/SnapshotDetailsModal.svelte
                                         → pages/Snapshots/SnapshotContextMenu.svelte
                                         → src/lib/components/dialogs/CompareSnapshotsDialog.svelte
                                         → src/lib/components/shared/Pagination.svelte
                                         → src/lib/components/shared/FilterBar.svelte
rustic_location_picker.html              → src/lib/components/dialogs/LocationPickerDialog.svelte (modularisiert)
                                         → dialogs/LocationPicker/LocalTab.svelte
                                         → dialogs/LocationPicker/NetworkTab.svelte
                                         → dialogs/LocationPicker/CloudTab.svelte
                                         → dialogs/LocationPicker/RecentTab.svelte
                                         → dialogs/LocationPicker/CredentialPromptModal.svelte
rustic_advanced_functions.html           → src/lib/components/dialogs/CheckRepoDialog.svelte
                                         → src/lib/components/dialogs/PruneRepoDialog.svelte
```

**CSS-Variablen aus Mockups:**

```css
/* Farben */
--bg-primary: #1a1d2e;
--bg-secondary: #22273a;
--bg-tertiary: #2d3348;
--border-color: #2d3348;

--color-primary: #3b82f6;
--color-primary-dark: #2563eb;
--color-success: #22c55e;
--color-error: #ef4444;
--color-warning: #fbbf24;

--text-primary: #e4e4e7;
--text-secondary: #71717a;

/* Spacing */
--spacing-sm: 8px;
--spacing-md: 12px;
--spacing-lg: 16px;
--spacing-xl: 24px;

/* Border-Radius */
--radius-sm: 6px;
--radius-md: 8px;
--radius-lg: 12px;
--radius-xl: 16px;
```

---

## 🚀 Projekt-Übersicht

### Was ist Rustic GUI?

Rustic GUI ist eine Desktop-Anwendung für das Backup-Tool [rustic](https://rustic.cli.rs/), entwickelt mit:

- **Frontend**: Svelte 5 + TypeScript + Tauri
- **Backend**: Rust (Tauri 2.0) + rustic_core
- **Build Targets**: Linux (AppImage), Windows (EXE/MSI)
- **Zielgruppe**: Technisch versierte Nutzer

### Projekt-Ziele

- ✨ Intuitive grafische Oberfläche für rustic
- ⏰ Backup-Job-Scheduling mit Cron
- 📸 Snapshot-Verwaltung und -Vergleich
- 🔄 Restore-Funktionalität mit File-Browser
- 📦 Multi-Repository-Support
- 🌍 Unterstützung für Cloud-Storage (via rclone/OpenDAL)

---

## 📁 Projekt-Struktur

```
rustic-gui/
├── docs/
│   ├── copilot-instructions.md          # ⭐ Diese Datei
│   ├── instructions/                    # ⭐ Modulare AI-Instructions
│   │   ├── workflow.instructions.md
│   │   ├── code-style.instructions.md
│   │   ├── architecture.instructions.md
│   │   ├── frontend.instructions.md
│   │   ├── backend.instructions.md
│   │   ├── testing.instructions.md
│   │   ├── backup-restore-snapshots.instructions.md
│   │   ├── patterns.instructions.md
│   │   └── troubleshooting.instructions.md
│   └── mockups/                         # ⭐ UI-Mockups (HTML)
│       ├── rustic_gui_mockup.html
│       ├── rustic_backup_dialogs.html
│       ├── rustic_repo_security_dialogs.html
│       └── rustic_restore_dialogs.html
├── src/                                 # Frontend (Svelte + TS)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── shared/                  # Button, Modal, Toast, etc.
│   │   │   ├── dialogs/                 # JobDialog, RestoreDialog, LocationPicker (modularisiert)
│   │   │   │   ├── JobDialog/          # Unified Create/Edit (4 Tabs: General, Paths, Schedule, Retention)
│   │   │   │   ├── LocationPicker/     # 5 Sub-Komponenten (Local, Network, Cloud, Recent, Credentials)
│   │   │   │   └── *.svelte            # Andere Dialoge (AddRepository, Restore, etc.)
│   │   │   ├── layout/                  # Sidebar, Header, MainLayout
│   │   │   └── pages/                   # Dashboard, BackupJobs, Repositories (mit Sub-Komponenten)
│   │   │       └── Snapshots/          # Modularisiert: Table, DetailsModal, ContextMenu
│   │   ├── stores/                      # Svelte Stores (State)
│   │   ├── api/                         # Backend-API-Wrapper
│   │   ├── types/                       # TypeScript Types
│   │   └── utils/                       # Helper-Funktionen
│   └── routes/                          # SvelteKit Routes
├── src-tauri/                           # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs                      # Entry Point + Command-Registrierung
│   │   ├── state.rs                     # AppState
│   │   ├── config.rs                    # Config-Management (TOML)
│   │   ├── commands/                    # Tauri Commands
│   │   ├── rustic/                      # rustic_core Integration
│   │   ├── scheduler/                   # Job-Scheduler (Cron)
│   │   └── keychain/                    # Passwort-Management
│   ├── Cargo.toml
│   └── tauri.conf.json
├── ROADMAP.md                           # ⭐ Entwicklungs-Roadmap
├── CHANGELOG.md                         # Änderungshistorie
└── README.md                            # Projekt-Readme
```

---

## 📊 Roadmap-Management

### ROADMAP.md pflegen

Die Datei `ROADMAP.md` ist das **zentrale Planungsdokument**.

**Nach jedem Task:**

1. ✅ Öffne `ROADMAP.md`
2. ✅ Finde relevantes Feature/Task
3. ✅ Aktualisiere Status:
   - `[ ]` → ⏰ In Planung
   - `[~]` → 🚧 In Arbeit (mit Datum/Details)
   - `[x]` → ✅ Erledigt
   - `[!]` → ⛔ Blockiert (mit Grund)
4. ✅ Füge Notizen hinzu falls relevant
5. ✅ Committe: `docs: Roadmap aktualisiert (Feature XY abgeschlossen)`

**Beispiel:**

```markdown
### Phase 1: Basis-Setup ✅ (Q4 2024)

- [x] Projekt-Setup (Tauri + Svelte)
- [x] Repository-Verwaltung (Add, List, Switch)
- [~] Backup-Job-Erstellung (2025-10-26: UI fertig gemäß Mockup, Backend in Arbeit)
  - [x] Job-Konfiguration-Dialog (basiert auf rustic_backup_dialogs.html)
  - [~] Backend-Integration
  - [ ] Validation & Error-Handling
```

---

## 🔄 Entwicklungs-Workflow

### Typischer Task-Ablauf

```bash
# 1. Instructions einlesen
# → copilot-instructions.md + relevante *.instructions.md

# 2. Mockups prüfen (bei UI-Tasks)
# → docs/mockups/*.html im Browser öffnen

# 3. Roadmap checken
# → ROADMAP.md: Status aktuell?

# 4. Branch erstellen
git checkout -b feature/snapshot-tags

# 5. Entwickeln
# → Backend (Rust): siehe backend.instructions.md
# → Frontend (Svelte): siehe frontend.instructions.md + Mockups
# → Tests: siehe testing.instructions.md

# 6. Dokumentation aktualisieren
# → ROADMAP.md: Status auf [x]
# → *.instructions.md: Neue Patterns?
# → CHANGELOG.md: User-relevante Änderung?

# 7. Commit (Conventional Commits, siehe workflow.instructions.md)
git add .
git commit -m "feat(snapshots): Tag-Verwaltung implementiert

- Tag-Editor-Komponente erstellt (gemäß rustic_gui_mockup.html)
- Backend-Command add_snapshot_tag
- Tests hinzugefügt

Closes #42"

# 8. Push & PR
git push origin feature/snapshot-tags
```

---

## 📖 Wie nutze ich diese Instructions?

### Als AI/Copilot

**Du bist eine AI die an diesem Projekt arbeitet. Befolge diese Schritte:**

#### 1. Initial-Setup (einmalig)

- [ ] Lies `copilot-instructions.md` (diese Datei) komplett
- [ ] Lies alle `instructions/*.instructions.md`
- [ ] Öffne alle 4 Mockup-Dateien und analysiere das UI-Design
- [ ] Lies `ROADMAP.md` für Projekt-Status
- [ ] Lies `CHANGELOG.md` für Änderungshistorie

#### 2. Vor jedem Task

- [ ] Lies `copilot-instructions.md` erneut (könnte Updates haben)
- [ ] Lies relevante `*.instructions.md` für den Task:
  - UI-Task → `frontend.instructions.md` + **Mockup prüfen!**
  - Backup-Feature → `backup-restore-snapshots.instructions.md`
  - Backend-Änderung → `backend.instructions.md`
  - Bugfix → `troubleshooting.instructions.md`
- [ ] Checke `ROADMAP.md` für Kontext
- [ ] Checke ob Tests existieren

#### 3. Während Implementierung

- [ ] Befolge Code-Style aus `code-style.instructions.md`
- [ ] Nutze Patterns aus `patterns.instructions.md`
- [ ] **Bei UI: Implementiere exakt nach Mockup**
- [ ] Bei Problemen: Siehe `troubleshooting.instructions.md`
- [ ] Schreibe Tests (siehe `testing.instructions.md`)

#### 4. Nach Implementierung

- [ ] Aktualisiere `ROADMAP.md` (Status, Fortschritt)
- [ ] Aktualisiere `CHANGELOG.md` bei User-Änderungen
- [ ] Ergänze `*.instructions.md` bei neuen Patterns
- [ ] Erstelle Git-Commit nach Konventionen

#### 5. Code-Review-Checkliste

- [ ] Code folgt `code-style.instructions.md`
- [ ] **UI entspricht Mockups** (kritisch!)
- [ ] Tests vorhanden und erfolgreich
- [ ] Dokumentation aktualisiert
- [ ] Keine Warnungen (Clippy, ESLint)
- [ ] Error-Handling vollständig

---

## 🎓 Best Practices Zusammenfassung

### Code

- ✅ **Deutsch**: Kommentare, Docstrings, UI-Texte
- ✅ **Englisch**: Code-Elemente (Variablen, Funktionen, Typen)
- ✅ **DRY**: Wiederverwendbare Komponenten/Funktionen
- ✅ **KISS**: Einfache Lösungen bevorzugen
- ✅ **Type-Safety**: TypeScript strict, Rust ohne `unwrap()`

### UI

- ✅ **Mockups first**: Immer Mockup prüfen vor Implementierung
- ✅ **Pixel-perfect**: CSS aus Mockups übernehmen
- ✅ **Responsive**: Funktioniert bei verschiedenen Größen
- ✅ **Accessible**: Keyboard-Navigation, ARIA-Labels
- ✅ **Consistent**: Einheitliches Design-System

### Dokumentation

- ✅ **Aktuell halten**: Nach jedem Feature aktualisieren
- ✅ **Beispiele geben**: Code-Snippets in Docs
- ✅ **Kontext liefern**: Warum, nicht nur Was/Wie
- ✅ **Roadmap pflegen**: Status immer korrekt

### Testing

- ✅ **Test-First**: Bei komplexer Logik erst Tests
- ✅ **Unit > Integration**: Viele kleine Tests
- ✅ **Real-World**: Realistische Test-Daten
- ✅ **CI-Integration**: Tests in GitHub Actions

---

## 🆘 Hilfe & Support

### Bei Problemen

1. ✅ Prüfe `troubleshooting.instructions.md`
2. ✅ Suche in `patterns.instructions.md`
3. ✅ Schaue in `CHANGELOG.md` ob bekanntes Problem
4. ✅ Prüfe GitHub Issues
5. ✅ Frage menschlichen Entwickler

### Unklarheiten in Instructions

Wenn diese Instructions unklar sind:

1. ✅ Dokumentiere das Problem (Issue erstellen)
2. ✅ Verbessere die Instructions (PR erstellen)
3. ✅ Informiere Team

### Neue Patterns entdeckt

Wenn du ein neues wiederverwendbares Pattern entwickelst:

1. ✅ Dokumentiere es in `patterns.instructions.md`
2. ✅ Füge Code-Beispiel hinzu
3. ✅ Committe: `docs(patterns): Neues Pattern XY hinzugefügt`

---

## ✅ Abschluss-Checkliste für AI

Bevor du mit einem Task startest:

### Setup-Phase (einmalig)

- [ ] Alle `*.instructions.md` gelesen
- [ ] **Alle 4 Mockup-Dateien angeschaut und UI verstanden**
- [ ] Projekt-Struktur verstanden
- [ ] Code-Conventions verinnerlicht

### Task-Start

- [ ] Instructions erneut gelesen
- [ ] Relevante Instructions-Dateien identifiziert
- [ ] **Mockups geprüft (falls UI-Task) - KRITISCH!**
- [ ] Roadmap-Status gecheckt

### Während Task

- [ ] Code folgt Style-Guide
- [ ] **UI entspricht Mockup (Farben, Spacing, Layout)**
- [ ] Error-Handling implementiert
- [ ] Tests geschrieben

### Task-Ende

- [ ] Roadmap aktualisiert
- [ ] Dokumentation ergänzt
- [ ] Tests erfolgreich
- [ ] Git-Commit nach Konventionen

---

## 📚 Weiterführende Ressourcen

### Externe Dokumentation

#### 📄 Projektspezifische API-Referenz

- **rustic_core API (lokal, Stand 0.8.0):** [docs/rustic/rustic_core_api.md](../docs/rustic/rustic_core_api.md)
- **rustic_backend API (lokal, Stand 0.5.3):** [docs/rustic/rustic_backend_api.md](../docs/rustic/rustic_backend_api.md)

#### Externe Dokumentation

- **rustic_core**: https://docs.rs/rustic_core
- **rustic_backend**: https://docs.rs/rustic_backend
- **Tauri 2.0**: https://v2.tauri.app/
- **Svelte 5**: https://svelte-5-preview.vercel.app/
- **TypeScript**: https://www.typescriptlang.org/docs/

### Community

- **rustic Discord**: https://discord.gg/WRUWENZnzQ
- **Tauri Discord**: https://discord.com/invite/tauri

---

**Version**: 2.1  
**Letzte Aktualisierung**: 2025-10-26  
**Maintainer**: Rustic GUI Team

🚀 **Happy Coding!**
