# Anpassungs-Empfehlungen für Rustic GUI Instructions

> **Konkrete Änderungsvorschläge basierend auf Projekt-Analyse vom 2025-11-01**
>
> Referenz: `PROJEKT_ANALYSE.md`

---

## 🎯 Übersicht

Dieser Dokument enthält **konkrete, sofort umsetzbare** Änderungsvorschläge für die GitHub Instructions, um sie an den aktuellen Projekt-Stand (01.11.2025) anzugleichen.

**Prioritäten:**

- ❌ **Kritisch (Priorität 1):** Sofort beheben (funktionale Fehler)
- 🟡 **Wichtig (Priorität 2):** Zeitnah beheben (Dokumentations-Lücken)
- 🟢 **Optional (Priorität 3):** Bei Gelegenheit (Verbesserungen)

---

## ❌ Priorität 1: Kritische Korrekturen

### 1.1 Dateiname korrigieren

**Problem:** Datei hat doppelten `.md`-Suffix

**Betroffene Dateien:**

- `.github/instructions/backup-restore-snapshots.instructions.md.md`
- `.github/copilot-instructions.md` (3 Vorkommen)

**Lösung:**

#### Schritt 1: Datei umbenennen

```bash
cd /home/mro/Share/repos/rustic-gui/.github/instructions/
git mv backup-restore-snapshots.instructions.md.md backup-restore-snapshots.instructions.md
```

#### Schritt 2: Links in `copilot-instructions.md` korrigieren

**Datei:** `.github/copilot-instructions.md`

**Zeile 95** (Link in Feature-spezifisch-Sektion):

```diff
- - **[backup-restore-snapshots.instructions.md](backup-restore-snapshots.instructions.md.md)**
+ - **[backup-restore-snapshots.instructions.md](instructions/backup-restore-snapshots.instructions.md)**
```

**Zeile 275** (In Dateibaum):

```diff
- │   │   ├── backup-restore-snapshots.instructions.md
+ │   │   ├── backup-restore-snapshots.instructions.md
```

_(Kein Änderung nötig, nur Referenz)_

**Zeile 408** (Im Workflow):

```diff
-   - Backup-Feature → `backup-restore-snapshots.instructions.md`
+   - Backup-Feature → `backup-restore-snapshots.instructions.md`
```

_(Kein Änderung nötig, nur Referenz)_

**Commit:**

```bash
git commit -m "fix(docs): Korrigiere Instructions-Dateiname (doppeltes .md entfernt)

- Rename: backup-restore-snapshots.instructions.md.md → .md
- Fix: Links in copilot-instructions.md aktualisiert

Schließt Issue aus PROJEKT_ANALYSE.md (Finding #1)"
```

---

## 🟡 Priorität 2: Wichtige Ergänzungen

### 2.1 Neue Mockups dokumentieren

**Problem:** 2 Mockups existieren, sind aber nicht in Copilot-Instructions erwähnt

**Betroffene Datei:** `.github/copilot-instructions.md`

**Neue Mockups:**

- `rustic_location_picker.html` (seit 2025-10-30)
- `rustic_advanced_functions.html`

**Lösung:**

**Datei:** `.github/copilot-instructions.md`  
**Nach Zeile 153** (im Mockups-Abschnitt) einfügen:

```markdown
├── rustic_location_picker.html # 📂 Unified Location Picker (seit 2025-10-30)
│ ├── 4 Tabs: Local, Network (SFTP), Cloud (S3/rclone), Recent
│ ├── Smart-Input mit Auto-Type-Detection
│ │ - Local: /path/to/repo
│ │ - SFTP: sftp://user@host:/path
│ │ - S3: s3://bucket/prefix
│ │ - rclone: rclone:remote:path
│ ├── Connection-Test-Button mit Validierung
│ ├── Favoriten-Management (Speichern/Laden)
│ └── Integration mit AddRepositoryDialog
│
└── rustic_advanced_functions.html # ⚡ Erweiterte Repository-Funktionen
├── Repository-Wartung (Check, Prune, Repair)
├── Diagnose & Statistiken
│ - Repository-Größe & Kompression
│ - Index-Statistiken
│ - Pack-File-Analyse
├── Prune-Dialog mit Dry-Run-Modus
└── Check-Dialog mit Progress-Reporting
```

**Zusätzlich in Zeile 184** (Beispiel-Mapping erweitern):

```markdown
rustic_location_picker.html → src/lib/components/dialogs/LocationPickerDialog.svelte
rustic_advanced_functions.html → src/lib/components/dialogs/CheckRepoDialog.svelte
→ src/lib/components/dialogs/PruneRepoDialog.svelte
```

**Commit:**

```bash
git commit -m "docs(mockups): Dokumentiere neue Mockup-Dateien in Copilot-Instructions

- Ergänze rustic_location_picker.html (seit 2025-10-30)
- Ergänze rustic_advanced_functions.html
- Update Component-Mapping

Schließt Issue aus PROJEKT_ANALYSE.md (Finding #2)"
```

---

### 2.2 Neue Dialogs in Frontend-Instructions

**Problem:** 3 neue Dialogs seit letztem Instructions-Update (26.10.2025) nicht dokumentiert

**Betroffene Datei:** `.github/instructions/frontend.instructions.md`

**Neue Komponenten:**

- `LocationPickerDialog.svelte` (30.10.2025)
- `PruneRepoDialog.svelte` (31.10.2025)
- `SnapshotInfoDialog.svelte`

**Lösung:**

**Datei:** `.github/instructions/frontend.instructions.md`  
**Nach Zeile 500** (im Dialogs-Abschnitt, nach RestoreDialog) einfügen:

````markdown
#### LocationPickerDialog.svelte (seit 2025-10-30)

Vereinheitlichter Location-Picker für alle Repository-Backend-Typen.

**Verwendung:**

- In `AddRepositoryDialog.svelte` als Haupt-Input
- Ersetzt separate Inputs für Local/SFTP/S3/rclone

**Features:**

- **4 Tabs:**
  - Local: Filesystem-Browser (mit OS-native Dialoge)
  - Network: SFTP-Konfiguration (Host, Port, User, Path)
  - Cloud: S3-kompatible Backends (Bucket, Region, Prefix)
  - Recent: Zuletzt verwendete Locations (gespeichert in Settings)
- **Smart-Input mit Auto-Detection:**
  - Erkennt Location-Typ automatisch (z.B. `sftp://...` → Network-Tab)
  - Validiert Format in Echtzeit
  - Zeigt Typ-spezifische Felder
- **Connection-Test:**
  - Button "Test Connection"
  - Backend-Command: `test_repository_connection()`
  - Zeigt Erfolg/Fehler mit Details
- **Favoriten:**
  - Speichern-Button für häufig genutzte Locations
  - Gespeichert in `settings.toml`
  - Dropdown zur schnellen Auswahl

**Props:**

```svelte
export let open = false; // Dialog-Sichtbarkeit export let initialLocation = ''; // Vorausgefüllte
Location export let allowedTypes = ['local', 'sftp', 's3', 'rclone']; // Erlaubte Typen
```
````

**Events:**

```svelte
on:select={(e) => {
  location = e.detail.location;
}}
on:cancel
```

**Backend-Integration:**

- Backend-Command: `test_repository_connection(location: String)`
- Settings-API: `save_favorite_location()`, `list_favorite_locations()`

**Mockup:** `docs/mockups/rustic_location_picker.html`

**Beispiel:**

```svelte
<script lang="ts">
  import LocationPickerDialog from '$lib/components/dialogs/LocationPickerDialog.svelte';

  let showLocationPicker = false;
  let selectedLocation = '';

  function handleLocationSelect(event: CustomEvent<{ location: string }>) {
    selectedLocation = event.detail.location;
    showLocationPicker = false;
  }
</script>

<Button on:click={() => (showLocationPicker = true)}>Repository-Location wählen</Button>

<LocationPickerDialog
  bind:open={showLocationPicker}
  initialLocation={selectedLocation}
  on:select={handleLocationSelect}
/>
```

---

#### PruneRepoDialog.svelte (seit 2025-10-31)

Prune-Dialog für Repository-Bereinigung (Löschen ungenutzter Daten).

**Features:**

- **Dry-Run-Modus:**
  - Checkbox "Nur Vorschau (kein Löschen)"
  - Backend-Command mit `dry_run: bool`-Flag
  - Zeigt was gelöscht würde ohne zu löschen
- **Statistik-Anzeige:**
  - Vorher/Nachher-Größe
  - Freed Space (in GB)
  - Anzahl gelöschter Pack-Files
- **Confirmation-Workflow:**
  - Warnung: "Diese Aktion kann nicht rückgängig gemacht werden"
  - Zwei-Schritt-Bestätigung bei Dry-Run deaktiviert
  - Progress-Bar während Prune-Operation
- **Prune-Optionen:**
  - Max. unused (z.B. "10% ungenutzter Space erlaubt")
  - Keep snapshots (Retention-Policy)

**Props:**

```svelte
export let open = false; export let repositoryId: string;
```

**Backend-Integration:**

- Backend-Command: `prune_repository(repo_id: String, dry_run: bool, options: PruneOptions)`
- Event: `prune-progress` (für Live-Updates)

**Mockup:** `docs/mockups/rustic_advanced_functions.html` (Prune-Section)

**Beispiel:**

```svelte
<PruneRepoDialog
  bind:open={showPruneDialog}
  repositoryId={currentRepo.id}
  on:complete={() => {
    toast.success('Prune erfolgreich abgeschlossen');
    refreshRepoStats();
  }}
/>
```

---

#### SnapshotInfoDialog.svelte

Detail-Ansicht für einzelnen Snapshot mit vollständigen Metadaten.

**Features:**

- **Metadaten-Anzeige:**
  - Snapshot-ID (mit Copy-Button)
  - Timestamp (formatiert)
  - Hostname, Username
  - Tags (mit Edit-Button)
  - Parent-Snapshot (Link zum Öffnen)
- **Statistiken:**
  - Files: `12,345 Dateien`
  - Directories: `1,234 Ordner`
  - Total Size: `42.5 GB`
  - Added Data: `+2.1 GB` (seit Parent)
- **Aktionen:**
  - Button "Restore" → Öffnet `RestoreDialog` mit diesem Snapshot
  - Button "Compare" → Öffnet `CompareSnapshotsDialog`
  - Button "Delete Snapshot" (mit Bestätigung)
  - Button "Add/Remove Tags"
- **Erweiterte Infos (Collapsible):**
  - Backup-Duration
  - Command (wie Backup gestartet wurde)
  - Original Paths

**Props:**

```svelte
export let open = false; export let snapshotId: string;
```

**Backend-Integration:**

- Backend-Command: `get_snapshot_info(snapshot_id: String)`
- Returns: `SnapshotInfo` (vollständiges Objekt)

**Mockup:** `docs/mockups/rustic_restore_dialogs.html` (Snapshot Info-Section)

**Beispiel:**

```svelte
<SnapshotInfoDialog
  bind:open={showSnapshotInfo}
  snapshotId={selectedSnapshot.id}
  on:restore={(e) => openRestoreDialog(e.detail.snapshotId)}
  on:delete={handleSnapshotDelete}
/>
```

---

**Commit:**

```bash
git commit -m "docs(frontend): Dokumentiere neue Dialog-Komponenten seit 2025-10-30

- LocationPickerDialog.svelte (Unified Location Picker)
- PruneRepoDialog.svelte (Repository-Bereinigung)
- SnapshotInfoDialog.svelte (Snapshot-Details)

Inkl. vollständiger Dokumentation:
- Features, Props, Events
- Backend-Integration
- Mockup-Referenzen
- Code-Beispiele

Schließt Issue aus PROJEKT_ANALYSE.md (Finding #6)"
```

---

### 2.3 README.md erweitern

**Problem:** README.md ist noch Minimal-Template von Tauri-Scaffold

**Betroffene Datei:** `README.md`

**Lösung:**

**Datei:** `README.md` (komplett ersetzen):

````markdown
# Rustic GUI

> **Moderne Desktop-Anwendung für rustic Backup-Management**
>
> Tauri 2 + Svelte 5 | Cross-Platform (Linux, Windows, macOS)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://v2.tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-5-red)](https://svelte-5-preview.vercel.app/)

---

## ✨ Features

- 🗂️ **Multi-Repository-Support**
  - Verwalte mehrere rustic-Repositories gleichzeitig
  - Automatische Passwort-Speicherung im System-Keychain
  - Schnelles Switching zwischen Repositories

- ⏰ **Backup-Scheduling**
  - Automatisierte Backups mit Cron-Expressions
  - Job-Management mit Start/Stop/Edit
  - Retention-Policies für automatisches Aufräumen

- 📸 **Snapshot-Verwaltung**
  - Liste aller Snapshots mit Filter & Suche
  - Tag-Management
  - Snapshot-Vergleich (Side-by-Side Diff)
  - Detaillierte Snapshot-Informationen

- 🔄 **Restore-Funktionalität**
  - File-Browser für selektive Wiederherstellung
  - Restore zu Original-Location oder Custom-Path
  - Dry-Run-Modus (Vorschau)

- 🌍 **Cloud-Backend-Support**
  - Local (Filesystem)
  - SFTP (SSH File Transfer)
  - S3 (AWS S3, MinIO, Wasabi, etc.)
  - rclone (100+ Cloud-Provider)

- 🎨 **Modernes UI**
  - Dark-Mode-Design
  - Responsive Layout (Desktop/Tablet/Mobile)
  - Accessibility (ARIA-Labels, Keyboard-Navigation)
  - Mockup-basierte Entwicklung für konsistentes Design

---

## 📦 Installation

### 🐧 Linux (AppImage)

```bash
# Download latest release
wget https://github.com/rustic-rs/rustic-gui/releases/latest/download/rustic-gui.AppImage

# Make executable
chmod +x rustic-gui.AppImage

# Run
./rustic-gui.AppImage
```
````

**Optional: Desktop-Integration**

```bash
./rustic-gui.AppImage --appimage-extract
sudo mv squashfs-root /opt/rustic-gui
sudo ln -s /opt/rustic-gui/rustic-gui /usr/local/bin/rustic-gui
```

### 🪟 Windows (MSI Installer)

1. Download: [rustic-gui-setup.msi](https://github.com/rustic-rs/rustic-gui/releases/latest)
2. Doppelklick auf MSI-Datei
3. Folge dem Installations-Assistenten

**Alternative: Portable EXE**

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/rustic-rs/rustic-gui/releases/latest/download/rustic-gui.exe" -OutFile "rustic-gui.exe"

# Run
.\rustic-gui.exe
```

### 🍎 macOS (DMG)

_Coming Soon_

---

## 🚀 Erste Schritte

1. **Repository hinzufügen:**
   - Klicke auf "➕ Repository öffnen"
   - Wähle Backend-Typ (Local, SFTP, S3, rclone)
   - Gib Repository-Location ein
   - Passwort eingeben oder erstellen

2. **Snapshots anzeigen:**
   - Wähle Repository in der Sidebar
   - Navigiere zu "Snapshots"
   - Nutze Filter für Suche (Tags, Hostname, Datum)

3. **Backup-Job erstellen:**
   - Klicke auf "Backup Jobs" → "➕ Neuer Job"
   - Konfiguriere Backup-Pfade
   - Setze Schedule (z.B. "0 2 \* \* \*" für täglich 2:00 Uhr)
   - Definiere Retention-Policy (z.B. "Keep 7 daily, 4 weekly")

4. **Dateien wiederherstellen:**
   - Öffne Snapshot → Klicke "Restore"
   - Navigiere im File-Browser
   - Wähle Dateien/Ordner aus
   - Bestätige Restore-Location

---

## 🛠️ Entwicklung

### Voraussetzungen

- **Node.js** 20+ ([Download](https://nodejs.org/))
- **Rust** 1.75+ ([Install](https://rustup.rs/))
- **Tauri Prerequisites** (siehe [Tauri Setup](https://v2.tauri.app/start/prerequisites/))

### Setup

```bash
# Repository klonen
git clone https://github.com/rustic-rs/rustic-gui.git
cd rustic-gui

# Dependencies installieren
npm install

# Development-Server starten
npm run tauri dev
```

### Build

```bash
# Production-Build (für aktuelles OS)
npm run tauri build

# Output in: src-tauri/target/release/bundle/
```

### Tests

```bash
# Frontend-Tests (Vitest)
npm test

# Backend-Tests (Cargo)
cd src-tauri
cargo test

# Coverage
npm run test:coverage
```

---

## 📚 Dokumentation

- **[ROADMAP.md](ROADMAP.md)** – Entwicklungs-Roadmap mit Milestones
- **[CHANGELOG.md](CHANGELOG.md)** – Änderungshistorie (Keep a Changelog)
- **[TODO.md](TODO.md)** – Detaillierte Implementierungs-TODOs
- **[.github/copilot-instructions.md](.github/copilot-instructions.md)** – AI-Entwicklungs-Richtlinien
- **[docs/mockups/](docs/mockups/)** – UI-Mockups (HTML)

---

## 🤝 Beitragen

Contributions sind willkommen! Bitte beachte:

1. Lies die [Contribution Guidelines](.github/copilot-instructions.md)
2. Folge dem [Code-Style](.github/instructions/code-style.instructions.md)
3. Erstelle Tests für neue Features
4. Halte Dokumentation aktuell

**Workflow:**

```bash
# Branch erstellen
git checkout -b feature/mein-feature

# Änderungen committen (Conventional Commits)
git commit -m "feat(snapshots): Tag-Filterung implementiert"

# Pull Request erstellen
```

---

## 📄 Lizenz

MIT License – siehe [LICENSE](LICENSE)

---

## 🙏 Credits

- **[rustic](https://rustic.cli.rs/)** – Schnelles, sicheres Backup-Tool
- **[Tauri](https://tauri.app/)** – Cross-Platform Desktop-Framework
- **[Svelte](https://svelte.dev/)** – Modernes Frontend-Framework

---

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/rustic-rs/rustic-gui/issues)
- **Discussions:** [GitHub Discussions](https://github.com/rustic-rs/rustic-gui/discussions)
- **rustic Discord:** [discord.gg/WRUWENZnzQ](https://discord.gg/WRUWENZnzQ)

---

**Made with ❤️ by the rustic community**

````

**Commit:**
```bash
git commit -m "docs(readme): Umfassende README.md mit Features, Installation und Dokumentation

Ersetzt Tauri-Template mit vollständiger Projekt-Dokumentation:
- Feature-Übersicht mit Emojis
- Installation für Linux/Windows/macOS
- Erste-Schritte-Anleitung
- Development-Setup & Build-Instruktionen
- Links zu Dokumentation & Support

Schließt Issue aus PROJEKT_ANALYSE.md (Finding #3)"
````

---

## 🟢 Priorität 3: Optionale Verbesserungen

### 3.1 Instructions-Versioning einführen

**Problem:** Instructions haben kein Versions-/Datums-Tracking

**Betroffene Dateien:** Alle 8 `.instructions.md`-Dateien

**Lösung:**

Füge in **allen** Instructions-Dateien am Anfang (Zeile 1-2) ein:

```markdown
# [Titel der Instructions]

> **Version:** 1.1 | **Stand:** 2025-11-01 | **Letzte Prüfung:** 2025-11-01 | **Autor:** Rustic GUI Team
```

**Beispiel für `frontend.instructions.md`:**

```diff
  # Rustic GUI – Frontend Instructions (Svelte 5 + TypeScript)

+ > **Version:** 1.1 | **Stand:** 2025-11-01 | **Letzte Prüfung:** 2025-11-01
+
  > **Umfassende Richtlinien für die Frontend-Entwicklung mit Svelte 5**
```

**Versions-Schema:**

- `1.0` – Initial (26.10.2025)
- `1.1` – Erste Aktualisierung (neue Komponenten dokumentiert)
- `2.0` – Breaking Changes in Instructions (z.B. neue Struktur)

**Commit:**

```bash
git commit -m "docs(instructions): Führe Versioning für alle Instructions ein

- Versions-Header in allen 8 Instructions-Dateien
- Version 1.1 (Stand 01.11.2025)
- Vereinfacht Tracking von Änderungen"
```

---

### 3.2 Automatisches Konsistenz-Check-Script

**Problem:** Keine automatische Prüfung auf Dokumentations-Inkonsistenzen

**Neue Datei:** `scripts/check-docs-consistency.sh`

**Inhalt:**

```bash
#!/bin/bash
# Prüft Dokumentations-Konsistenz für Rustic GUI
# Wird in CI/CD verwendet um Inkonsistenzen frühzeitig zu erkennen

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "🔍 Prüfe Dokumentations-Konsistenz..."
echo ""

ERRORS=0

# 1. Prüfe auf doppelte .md-Endungen
echo "1️⃣ Prüfe Instructions-Dateinamen..."
if find .github/instructions/ -name "*.md.md" | grep -q .; then
  echo "❌ Fehler: Dateien mit doppeltem .md gefunden:"
  find .github/instructions/ -name "*.md.md"
  ERRORS=$((ERRORS + 1))
else
  echo "✅ Alle Instructions-Dateinamen korrekt"
fi
echo ""

# 2. Prüfe ob alle Mockups in copilot-instructions.md erwähnt sind
echo "2️⃣ Prüfe Mockup-Dokumentation..."
MISSING_MOCKUPS=0
for mockup in docs/mockups/*.html; do
  mockup_name=$(basename "$mockup")
  if ! grep -q "$mockup_name" .github/copilot-instructions.md; then
    echo "⚠️  Mockup nicht in copilot-instructions.md: $mockup_name"
    MISSING_MOCKUPS=$((MISSING_MOCKUPS + 1))
  fi
done

if [ $MISSING_MOCKUPS -eq 0 ]; then
  echo "✅ Alle Mockups dokumentiert"
else
  echo "❌ $MISSING_MOCKUPS Mockups fehlen in Dokumentation"
  ERRORS=$((ERRORS + 1))
fi
echo ""

# 3. Prüfe auf Broken Links in Instructions
echo "3️⃣ Prüfe Broken Links in Instructions..."
BROKEN_LINKS=$(grep -r "\.instructions\.md\.md" .github/ || true)
if [ -n "$BROKEN_LINKS" ]; then
  echo "❌ Broken Links gefunden:"
  echo "$BROKEN_LINKS"
  ERRORS=$((ERRORS + 1))
else
  echo "✅ Keine Broken Links gefunden"
fi
echo ""

# 4. Prüfe ob alle Dialogs in frontend.instructions.md erwähnt sind
echo "4️⃣ Prüfe Dialog-Dokumentation..."
MISSING_DIALOGS=0
for dialog in src/lib/components/dialogs/*.svelte; do
  dialog_name=$(basename "$dialog" .svelte)
  if ! grep -q "$dialog_name" .github/instructions/frontend.instructions.md; then
    echo "⚠️  Dialog nicht in frontend.instructions.md: $dialog_name"
    MISSING_DIALOGS=$((MISSING_DIALOGS + 1))
  fi
done

if [ $MISSING_DIALOGS -eq 0 ]; then
  echo "✅ Alle Dialogs dokumentiert"
else
  echo "⚠️  $MISSING_DIALOGS Dialogs fehlen (könnte intentional sein)"
  # Nicht als Fehler werten, nur Warnung
fi
echo ""

# 5. Prüfe ob README.md mehr als 50 Zeilen hat (nicht mehr nur Template)
echo "5️⃣ Prüfe README.md Vollständigkeit..."
README_LINES=$(wc -l < README.md)
if [ "$README_LINES" -lt 50 ]; then
  echo "⚠️  README.md sehr kurz ($README_LINES Zeilen), eventuell noch Template?"
else
  echo "✅ README.md vollständig ($README_LINES Zeilen)"
fi
echo ""

# Zusammenfassung
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $ERRORS -eq 0 ]; then
  echo "✅ Alle Checks erfolgreich! Dokumentation konsistent."
  exit 0
else
  echo "❌ $ERRORS Fehler gefunden. Bitte beheben vor Commit."
  exit 1
fi
```

**Executable machen:**

```bash
chmod +x scripts/check-docs-consistency.sh
```

**Integration in GitHub Actions (`.github/workflows/docs-check.yml`):**

```yaml
name: Documentation Check

on:
  pull_request:
    paths:
      - '.github/**'
      - 'docs/**'
      - 'README.md'
      - 'ROADMAP.md'
      - 'CHANGELOG.md'
      - 'TODO.md'
  push:
    branches: [main, develop]

jobs:
  check-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run Documentation Consistency Check
        run: ./scripts/check-docs-consistency.sh
```

**Commit:**

```bash
git add scripts/check-docs-consistency.sh .github/workflows/docs-check.yml
git commit -m "ci: Automatisches Dokumentations-Konsistenz-Check-Script

- Script prüft Dateinamen, Mockups, Links, Dialogs, README
- Integration in GitHub Actions (PRs + Push)
- Verhindert Merge bei Dokumentations-Fehlern

Schließt Empfehlung aus PROJEKT_ANALYSE.md (#6)"
```

---

### 3.3 CONTRIBUTING.md erstellen

**Problem:** Keine explizite Contribution-Guidelines-Datei

**Neue Datei:** `CONTRIBUTING.md`

**Inhalt:**

````markdown
# Contribution Guidelines – Rustic GUI

> **Willkommen! Wir freuen uns über jeden Beitrag.**
>
> Bitte lies diese Guidelines bevor du einen Pull Request erstellst.

---

## 📋 Inhaltsverzeichnis

- [Code of Conduct](#code-of-conduct)
- [Entwicklungs-Workflow](#entwicklungs-workflow)
- [Code-Style](#code-style)
- [Testing](#testing)
- [Dokumentation](#dokumentation)
- [Pull Requests](#pull-requests)

---

## 📜 Code of Conduct

Wir folgen dem [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Sei freundlich, respektvoll und konstruktiv.

---

## 🔄 Entwicklungs-Workflow

### 1. Issue erstellen (optional, aber empfohlen)

```bash
# Für neue Features
https://github.com/rustic-rs/rustic-gui/issues/new?template=feature_request.md

# Für Bugs
https://github.com/rustic-rs/rustic-gui/issues/new?template=bug_report.md
```
````

### 2. Fork & Branch erstellen

```bash
# Fork erstellen auf GitHub UI
git clone https://github.com/DEIN-USERNAME/rustic-gui.git
cd rustic-gui

# Upstream hinzufügen
git remote add upstream https://github.com/rustic-rs/rustic-gui.git

# Feature-Branch erstellen
git checkout -b feature/mein-feature
```

### 3. Entwickeln

```bash
# Dependencies installieren
npm install

# Dev-Server starten
npm run tauri dev

# Tests laufen lassen (während Entwicklung)
npm test
```

### 4. Committen (Conventional Commits)

Wir nutzen [Conventional Commits](https://www.conventionalcommits.org/) **in Deutsch**:

**Format:**

```
<type>(<scope>): <Beschreibung>

[optionaler Body]

[optionaler Footer: "Closes #123"]
```

**Types:**

- `feat` – Neues Feature
- `fix` – Bugfix
- `docs` – Nur Dokumentation
- `style` – Code-Formatierung (keine Logik-Änderung)
- `refactor` – Code-Refactoring
- `test` – Tests hinzufügen/ändern
- `chore` – Build-System, Dependencies

**Scopes (Beispiele):**

- `snapshots` – Snapshot-Feature
- `backup` – Backup-Logik
- `ui` – UI-Komponenten
- `backend` – Rust-Backend
- `frontend` – Svelte-Frontend

**Beispiele:**

```bash
git commit -m "feat(snapshots): Tag-Filterung implementiert"
git commit -m "fix(backup): Progress-Event-Handling korrigiert"
git commit -m "docs(readme): Installation-Anleitung ergänzt"
```

### 5. Tests & Checks

```bash
# Alle Tests
npm test
cd src-tauri && cargo test

# Linting
npm run lint
cd src-tauri && cargo clippy

# Format-Check
npm run format:check
cd src-tauri && cargo fmt -- --check
```

### 6. Push & Pull Request

```bash
git push origin feature/mein-feature

# Dann auf GitHub: "Compare & pull request"
```

---

## 🎨 Code-Style

### Generell

- **Deutsch:** Kommentare, Docstrings, UI-Texte, Commit-Messages
- **Englisch:** Code-Elemente (Variablen, Funktionen, Typen)
- **DRY:** Don't Repeat Yourself – Wiederverwendbare Komponenten/Funktionen
- **KISS:** Keep It Simple, Stupid – Einfache Lösungen bevorzugen

Siehe: [.github/instructions/code-style.instructions.md](.github/instructions/code-style.instructions.md)

### Rust (Backend)

```rust
// ✅ Gut
/// Führt einen Backup durch.
///
/// # Arguments
/// * `repo_id` - Repository-ID
/// * `paths` - Zu sichernde Pfade
///
/// # Returns
/// Result mit Backup-ID bei Erfolg
#[tauri::command]
async fn run_backup(
    repo_id: String,
    paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Implementation
}

// ❌ Schlecht
#[tauri::command]
async fn backup(r: String, p: Vec<String>) -> Result<String, String> {
    // Keine Dokumentation, unklar
}
```

**Conventions:**

- Snake_case für Funktionen/Variablen
- PascalCase für Typen/Structs
- Clippy-Warnings beheben (`cargo clippy`)
- Rustfmt verwenden (`cargo fmt`)

### TypeScript (Frontend)

```typescript
// ✅ Gut
/**
 * Lädt alle Snapshots für ein Repository.
 *
 * @param repoId - Repository-ID
 * @returns Promise mit Snapshot-Array
 * @throws Bei Backend-Fehlern
 */
export async function loadSnapshots(repoId: string): Promise<Snapshot[]> {
  return invoke('list_snapshots', { repoId });
}

// ❌ Schlecht
export async function load(id: string) {
  return invoke('list_snapshots', { id }); // Keine Typen, keine Doku
}
```

**Conventions:**

- CamelCase für Funktionen/Variablen
- PascalCase für Typen/Interfaces
- ESLint-Rules beachten (`npm run lint`)
- Prettier verwenden (`npm run format`)

### Svelte (UI)

````svelte
<!-- ✅ Gut -->
<script lang="ts">
  /**
   * Button-Komponente mit verschiedenen Varianten.
   *
   * @component
   * @example
   * ```svelte
   * <Button variant="primary" on:click={handleClick}>
   *   Klick mich
   * </Button>
   * ```
   */

  interface ButtonProps {
    variant?: 'primary' | 'secondary' | 'danger';
    disabled?: boolean;
  }

  let { variant = 'primary', disabled = false, children }: ButtonProps & { children: any } = $props();
</script>

<button class="btn btn-{variant}" {disabled} onclick={() => /* ... */}>
  {@render children()}
</button>

<!-- ❌ Schlecht -->
<script lang="ts">
  let { v, d, children } = $props(); // Unklar
</script>
<button class={v}>{@render children()}</button>
````

**Conventions:**

- Immer TSDoc-Kommentare für Komponenten
- Props mit Interfaces definieren
- **UI muss Mockups in `docs/mockups/` entsprechen**

---

## 🧪 Testing

### Test-Pyramide (70/25/5)

- **70% Unit-Tests:** Einzelne Funktionen isoliert testen
- **25% Integration-Tests:** Zusammenspiel mehrerer Module
- **5% E2E-Tests:** Vollständige User-Workflows

### Frontend (Vitest)

```bash
# Alle Tests
npm test

# Watch-Modus
npm run test:watch

# Coverage
npm run test:coverage
```

**Beispiel:**

```typescript
// src/lib/components/shared/Button.test.ts
import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('should render with text', () => {
    const { getByText } = render(Button, { children: 'Klick mich' });
    expect(getByText('Klick mich')).toBeInTheDocument();
  });

  it('should call onClick handler', async () => {
    let clicked = false;
    const { getByRole } = render(Button, {
      onclick: () => {
        clicked = true;
      },
    });

    await fireEvent.click(getByRole('button'));
    expect(clicked).toBe(true);
  });
});
```

### Backend (Cargo Test)

```bash
cd src-tauri
cargo test
```

**Beispiel:**

```rust
// src-tauri/src/commands/snapshot.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_id_parsing() {
        let id = "abc123def456";
        let result = parse_snapshot_id(id);
        assert!(result.is_ok());
    }
}
```

### Tests sind Pflicht für:

- ✅ Neue Backend-Commands
- ✅ Neue API-Wrapper (Frontend)
- ✅ Shared Components mit Logik
- ✅ Stores mit komplexer State-Management

---

## 📚 Dokumentation

### Code-Dokumentation

- ✅ **TSDoc** für TypeScript-Funktionen
- ✅ **Rustdoc** für Rust-Funktionen/Structs
- ✅ **Svelte-Kommentare** für Komponenten (mit `@component`, `@example`)

### Instructions aktualisieren

Bei neuen Patterns/Features:

1. Prüfe ob Pattern in [.github/instructions/patterns.instructions.md](.github/instructions/patterns.instructions.md) existiert
2. Falls nicht: Füge neues Pattern mit Code-Beispiel hinzu
3. Bei UI-Komponenten: Ergänze [.github/instructions/frontend.instructions.md](.github/instructions/frontend.instructions.md)
4. Bei Backend-Features: Ergänze [.github/instructions/backend.instructions.md](.github/instructions/backend.instructions.md)

### ROADMAP.md & TODO.md

- Nach Feature-Completion: Status in ROADMAP.md aktualisieren (`[ ]` → `[x]`)
- Bei Bugfixes: Ergänze CHANGELOG.md

---

## 🔀 Pull Requests

### Vor dem PR

- [ ] Alle Tests erfolgreich (`npm test` + `cargo test`)
- [ ] Linting ohne Fehler (`npm run lint` + `cargo clippy`)
- [ ] Formatierung korrekt (`npm run format` + `cargo fmt`)
- [ ] Dokumentation aktualisiert (Instructions, ROADMAP, CHANGELOG)
- [ ] Commits folgen Conventional Commits

### PR-Template

Nutze unsere PR-Vorlage oder schreibe:

```markdown
## Beschreibung

Kurze Zusammenfassung der Änderungen.

## Änderungen

- [ ] Backend-Commands hinzugefügt
- [ ] Frontend-UI implementiert
- [ ] Tests geschrieben
- [ ] Dokumentation aktualisiert

## Screenshots (bei UI-Änderungen)

[Screenshot einfügen]

## Related Issues

Closes #123

## Checklist

- [ ] Tests erfolgreich
- [ ] Linting sauber
- [ ] Dokumentation aktualisiert
- [ ] ROADMAP.md Status aktualisiert
```

### Review-Prozess

1. **Automatische Checks:** CI/CD muss grün sein
2. **Code-Review:** Mindestens 1 Approval von Maintainer
3. **Merge:** Squash-Merge bevorzugt

---

## 🙏 Danke!

Vielen Dank für deinen Beitrag! Jeder PR hilft das Projekt besser zu machen.

Bei Fragen:

- **Issues:** [GitHub Issues](https://github.com/rustic-rs/rustic-gui/issues)
- **Discord:** [discord.gg/WRUWENZnzQ](https://discord.gg/WRUWENZnzQ)

**Happy Coding! 🚀**

````

**Commit:**
```bash
git add CONTRIBUTING.md
git commit -m "docs: Erstelle umfassende Contribution-Guidelines

- Entwicklungs-Workflow (Fork, Branch, PR)
- Code-Style für Rust/TypeScript/Svelte
- Testing-Anforderungen (Vitest + Cargo)
- Dokumentations-Pflichten
- PR-Template & Review-Prozess

Erleichtert Onboarding für neue Contributors."
````

---

## 📊 Zusammenfassung

### Priorität 1 (Kritisch) – 1 Fix

| #   | Beschreibung                           | Dateien   | Aufwand |
| --- | -------------------------------------- | --------- | ------- |
| 1.1 | Dateiname `.md.md` → `.md` korrigieren | 2 Dateien | 5 Min   |

**Gesamt:** ~5 Minuten

---

### Priorität 2 (Wichtig) – 3 Ergänzungen

| #   | Beschreibung               | Dateien                    | Aufwand |
| --- | -------------------------- | -------------------------- | ------- |
| 2.1 | Neue Mockups dokumentieren | `copilot-instructions.md`  | 10 Min  |
| 2.2 | Neue Dialogs dokumentieren | `frontend.instructions.md` | 30 Min  |
| 2.3 | README.md erweitern        | `README.md`                | 20 Min  |

**Gesamt:** ~60 Minuten

---

### Priorität 3 (Optional) – 3 Verbesserungen

| #   | Beschreibung              | Dateien                          | Aufwand |
| --- | ------------------------- | -------------------------------- | ------- |
| 3.1 | Instructions-Versioning   | 8 Instructions-Dateien           | 15 Min  |
| 3.2 | Konsistenz-Check-Script   | `scripts/`, `.github/workflows/` | 45 Min  |
| 3.3 | CONTRIBUTING.md erstellen | `CONTRIBUTING.md`                | 30 Min  |

**Gesamt:** ~90 Minuten

---

### Gesamtaufwand

- **Priorität 1:** 5 Minuten
- **Priorität 2:** 60 Minuten
- **Priorität 3:** 90 Minuten

**Total:** ~2.5 Stunden für vollständige Aktualisierung

---

## ✅ Umsetzungsplan

### Phase 1: Sofort (heute)

```bash
# 1.1 Dateiname korrigieren (5 Min)
cd .github/instructions/
git mv backup-restore-snapshots.instructions.md.md backup-restore-snapshots.instructions.md

# Links in copilot-instructions.md korrigieren
# (Manuell oder mit sed)

git commit -m "fix(docs): Korrigiere Instructions-Dateiname (doppeltes .md entfernt)"
```

### Phase 2: Diese Woche (3 Tasks, 60 Min)

```bash
# 2.1 Mockups dokumentieren (10 Min)
# → copilot-instructions.md editieren

# 2.2 Neue Dialogs dokumentieren (30 Min)
# → frontend.instructions.md editieren

# 2.3 README.md erweitern (20 Min)
# → README.md ersetzen

git commit -m "docs: Aktualisiere Instructions mit neuesten Komponenten"
```

### Phase 3: Nächste 2 Wochen (3 Tasks, 90 Min)

```bash
# 3.1 Versioning (15 Min)
# → Alle 8 Instructions-Dateien Header hinzufügen

# 3.2 Check-Script (45 Min)
# → scripts/check-docs-consistency.sh erstellen
# → .github/workflows/docs-check.yml erstellen

# 3.3 CONTRIBUTING.md (30 Min)
# → CONTRIBUTING.md erstellen

git commit -m "ci: Automatische Dokumentations-Checks + Contribution-Guidelines"
```

---

**Ende der Anpassungs-Empfehlungen**
