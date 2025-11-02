# Workflow Instructions

> Entwicklungs-Workflow, Git-Konventionen und Task-Abläufe

---

## 🔄 Entwicklungs-Workflow

### Standard-Task-Ablauf

```bash
# 1. ⭐ WICHTIG: Instructions einlesen
# Lies copilot-instructions.md + relevante *.instructions.md

# 2. Feature-Branch erstellen
git checkout -b feature/snapshot-tags

# 3. Entwickeln
# → Backend implementieren (Rust)
# → Frontend implementieren (Svelte)
# → Tests schreiben

# 4. Dokumentation aktualisieren
# → ROADMAP.md: Status auf [x] oder [~]
# → CHANGELOG.md: User-Änderung eintragen
# → *.instructions.md: Neue Patterns?

# 5. Code-Qualität prüfen
npm run lint
npm run type-check
cargo fmt --check
cargo clippy -- -D warnings

# 6. Tests ausführen
npm test
cargo test

# 7. Commit erstellen (siehe Git-Konventionen unten)
git add .
git commit -m "feat(snapshots): Tag-Verwaltung implementiert"

# 8. Push und Pull Request
git push origin feature/snapshot-tags
```

---

## 🌿 Git Workflow

### Branch-Strategie: GitHub Flow

```
main (geschützt, immer deploybar)
  ├── feature/snapshot-compare
  ├── feature/job-scheduling
  ├── fix/restore-permissions
  └── docs/update-readme
```

### Branch-Naming

```bash
# Feature-Branches
feature/kurze-beschreibung
feature/snapshot-tags
feature/backup-scheduler

# Bugfix-Branches
fix/kurze-beschreibung
fix/restore-permissions
fix/memory-leak

# Dokumentation
docs/kurze-beschreibung
docs/update-api-docs

# Refactoring
refactor/kurze-beschreibung
refactor/extract-backup-logic

# Chores (Build, Dependencies, etc.)
chore/kurze-beschreibung
chore/update-dependencies
```

---

## 📝 Commit-Konventionen: Conventional Commits (Deutsch)

### Format

```
<typ>(<scope>): <kurzbeschreibung>

<längere beschreibung>

<footer>
```

### Typen

| Typ        | Beschreibung  | Wann verwenden                               |
| ---------- | ------------- | -------------------------------------------- |
| `feat`     | Neues Feature | Neue Funktionalität für User                 |
| `fix`      | Bugfix        | Fehler behoben                               |
| `docs`     | Dokumentation | README, Instructions, etc.                   |
| `style`    | Formatierung  | Keine Code-Änderung (Prettier, etc.)         |
| `refactor` | Refactoring   | Code-Umstrukturierung ohne Funktionsänderung |
| `test`     | Tests         | Tests hinzugefügt/geändert                   |
| `chore`    | Build/Tools   | Dependencies, CI, Build-Config               |
| `perf`     | Performance   | Performance-Optimierung                      |

### Scopes (optional)

| Scope       | Bereich                |
| ----------- | ---------------------- |
| `backup`    | Backup-Funktionalität  |
| `restore`   | Restore-Funktionalität |
| `snapshots` | Snapshot-Management    |
| `scheduler` | Job-Scheduling         |
| `ui`        | User Interface         |
| `config`    | Konfiguration          |
| `repo`      | Repository-Verwaltung  |

### Beispiele

#### Feature-Commit

```bash
git commit -m "feat(backup): Job-Scheduling implementiert

Backup-Jobs können jetzt mit Cron-Expressions zeitgesteuert werden.
Verwendet tokio-cron-scheduler für asynchrone Ausführung.

- Cron-Parser integriert
- UI-Dialog für Schedule-Eingabe
- Backend-Command schedule_backup_job
- Tests für Scheduler-Logik

Closes #42"
```

#### Bugfix-Commit

```bash
git commit -m "fix(restore): Berechtigungen werden korrekt wiederhergestellt

Vorher wurden Datei-Permissions ignoriert.
Jetzt wird chmod() korrekt nach Restore aufgerufen.

Fixes #123"
```

#### Dokumentations-Commit

```bash
git commit -m "docs: README mit Installation-Anleitung ergänzt

- Linux-AppImage-Setup
- Windows-Portable-EXE-Setup
- Troubleshooting-Sektion hinzugefügt"
```

#### Refactoring-Commit

```bash
git commit -m "refactor(ui): Dialog-Components in separate Dateien ausgelagert

Verbessert Wartbarkeit und Wiederverwendbarkeit.
Keine funktionalen Änderungen.

- Snapshots.svelte modularisiert (SnapshotTable, SnapshotDetailsModal, SnapshotContextMenu)
- JobDialog konsolidiert (CreateJobDialog + EditJobDialog → unified Dialog)
- LocationPickerDialog modularisiert (5 Sub-Komponenten: Local, Network, Cloud, Recent, Credentials)"
```

#### Chore-Commit

```bash
git commit -m "chore: Dependencies aktualisiert

- Tauri 2.0.5 → 2.0.6
- Svelte 5.0.0 → 5.0.1
- rustic_core 0.3.0 → 0.3.1"
```

---

## 🔀 Pull Request Workflow

### PR erstellen

1. **Branch pushen**

   ```bash
   git push origin feature/snapshot-tags
   ```

2. **PR auf GitHub erstellen**
   - Titel: `feat(snapshots): Tag-Verwaltung` (wie Commit)
   - Beschreibung:

     ```markdown
     ## Änderungen

     - Tag-Editor-Komponente (gemäß Mockup 08)
     - Backend-Command add_snapshot_tag
     - Tests für Tag-CRUD-Operationen

     ## Screenshots

     ![Tag-Editor](screenshots/tag-editor.png)

     ## Checklist

     - [x] Tests hinzugefügt und erfolgreich
     - [x] Dokumentation aktualisiert
     - [x] UI folgt Mockups
     - [x] Keine Linter-Warnungen

     Closes #42
     ```

3. **Review anfordern**
   - Assigniere Reviewer
   - Warte auf Feedback

### PR-Review-Checkliste

#### Als Reviewer prüfe:

**Code-Qualität**

- [ ] Folgt Code-Style (`code-style.instructions.md`)
- [ ] Keine Warnungen (Clippy, ESLint)
- [ ] Error-Handling vollständig
- [ ] Keine unnötigen `unwrap()` (Rust)
- [ ] Keine `any` (TypeScript)

**Funktionalität**

- [ ] Tests vorhanden und erfolgreich
- [ ] Feature funktioniert wie beschrieben
- [ ] Keine Regressionen
- [ ] Performance akzeptabel

**UI (falls relevant)**

- [ ] Folgt Mockups
- [ ] Responsive Design
- [ ] Keyboard-Navigation
- [ ] Loading-States vorhanden

**Dokumentation**

- [ ] ROADMAP.md aktualisiert
- [ ] CHANGELOG.md aktualisiert (bei User-Änderungen)
- [ ] Code-Kommentare vorhanden
- [ ] README aktualisiert (bei neuen Features)

**Git**

- [ ] Commit-Messages folgen Konventionen
- [ ] Branch ist aktuell mit main
- [ ] Keine Merge-Konflikte

### Nach Review

**Bei Änderungswünschen:**

```bash
# Änderungen implementieren
git add .
git commit -m "fix(review): Feedback umgesetzt

- Error-Handling verbessert
- Tests ergänzt
- Typo in Docs korrigiert"

git push origin feature/snapshot-tags
```

**Nach Approval:**

```bash
# Merge via GitHub UI (Squash & Merge bevorzugt)
# Oder lokal:
git checkout main
git merge --no-ff feature/snapshot-tags
git push origin main

# Branch löschen
git branch -d feature/snapshot-tags
git push origin --delete feature/snapshot-tags
```

---

## 🔄 ROADMAP.md aktualisieren

### Nach jedem Task

1. Öffne `ROADMAP.md`
2. Finde relevantes Feature/Task
3. Aktualisiere Status:
   - `[ ]` → ⏰ In Planung
   - `[~]` → 🚧 In Arbeit
   - `[x]` → ✅ Erledigt

4. Füge Notizen hinzu:

   ```markdown
   - [x] Backup-Job-Erstellung (2025-10-26: Fertig)
     - [x] Job-Konfiguration-Dialog
     - [x] Backend-Integration
     - [x] Validation & Error-Handling
   ```

5. Committe Änderung:
   ```bash
   git add ROADMAP.md
   git commit -m "docs: Roadmap aktualisiert (Backup-Jobs abgeschlossen)"
   ```

### Roadmap-Status-Symbole

```markdown
[ ] Nicht gestartet
[~] In Arbeit (mit Datum/Details)
[x] Abgeschlossen
[!] Blockiert (mit Grund)
[?] Unklar/Zu klären
```

---

## 📋 CHANGELOG.md aktualisieren

### Wann aktualisieren?

**JA, aktualisieren bei:**

- ✅ Neues Feature für User
- ✅ Bugfix der User betrifft
- ✅ Breaking Changes
- ✅ Neue Dependencies mit User-Impact
- ✅ Performance-Verbesserungen

**NEIN, nicht aktualisieren bei:**

- ❌ Interne Refactorings
- ❌ Test-Änderungen
- ❌ Code-Style-Fixes
- ❌ Docs-Updates (außer User-Docs)

### Format (Keep a Changelog)

```markdown
# Changelog

Alle wichtigen Änderungen an diesem Projekt werden in dieser Datei dokumentiert.

Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/).

## [Unreleased]

### Added

- Job-Scheduling mit Cron-Expressions (#42)
- Tag-Verwaltung für Snapshots (#55)
- Snapshot-Vergleich mit Diff-Anzeige (#67)

### Changed

- Verbesserte Performance bei großen Repositories
- UI-Verbesserungen im Restore-Dialog

### Fixed

- Restore-Berechtigungen werden korrekt wiederhergestellt (#123)
- Memory-Leak bei langen Backup-Sessions (#145)

### Security

- Passwörter werden nicht mehr in Logs ausgegeben

## [0.2.0] - 2025-10-15

### Added

- Multi-Repository-Support
- Cloud-Storage via rclone

### Fixed

- Windows-Pfade mit Backslashes funktionieren jetzt

## [0.1.0] - 2025-10-01

### Added

- Initiales Release
- Basis-Backup-Funktionalität
- Snapshot-Verwaltung
```

### Changelog aktualisieren

```bash
# 1. Öffne CHANGELOG.md
# 2. Füge unter [Unreleased] hinzu
# 3. Committe mit:
git add CHANGELOG.md
git commit -m "docs: Changelog aktualisiert"
```

---

## 🚀 Release-Workflow

### Version bumpen

```bash
# 1. Entscheide Version (Semantic Versioning)
# MAJOR.MINOR.PATCH
# - MAJOR: Breaking Changes
# - MINOR: Neue Features (rückwärtskompatibel)
# - PATCH: Bugfixes

# 2. Update Version in allen Dateien
# package.json
"version": "0.2.0"

# src-tauri/Cargo.toml
version = "0.2.0"

# src-tauri/tauri.conf.json
"version": "0.2.0"

# 3. CHANGELOG.md aktualisieren
## [0.2.0] - 2025-10-26

### Added
...

## 4. Commit und Tag
git add .
git commit -m "chore: Release v0.2.0"
git tag v0.2.0
git push origin main --tags
```

### Build erstellen

```bash
# Lokaler Build für Test
npm run tauri:build

# Artifacts in:
# src-tauri/target/release/bundle/
#   - appimage/  (Linux)
#   - msi/       (Windows)
#   - nsis/      (Windows Installer)

# CI/CD Build (automatisch via GitHub Actions bei Tag)
git push origin v0.2.0
# → GitHub Actions erstellt Release mit Artifacts
```

---

## ⚙️ Pre-Commit Hooks (empfohlen)

### Setup mit Husky

```bash
# Installation
npm install --save-dev husky
npx husky init

# Pre-Commit Hook
# .husky/pre-commit
#!/bin/sh

echo "🔍 Running pre-commit checks..."

# Frontend
npm run lint || exit 1
npm run type-check || exit 1

# Backend
cd src-tauri
cargo fmt --check || exit 1
cargo clippy -- -D warnings || exit 1
cd ..

echo "✅ All checks passed!"
```

### Alternative: Manueller Check

```bash
# Vor Commit manuell ausführen
npm run lint && npm run type-check
cd src-tauri && cargo fmt --check && cargo clippy
```

---

## 🔄 Sync mit Upstream (bei Fork)

```bash
# Upstream hinzufügen (einmalig)
git remote add upstream https://github.com/original/rustic-gui.git

# Regelmäßig synchronisieren
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

---

## 🎯 Task-Priorisierung

### ROADMAP.md als Single Source of Truth

```markdown
## Aktuelle Prioritäten (Q4 2025)

### 🔥 Kritisch (diese Woche)

- [ ] Bugfix: Backup bricht bei großen Dateien ab (#234)
- [~] Feature: Retry-Logik für Netzwerk-Fehler (#245)

### ⚡ Wichtig (dieser Monat)

- [ ] Feature: Differential Backup (#156)
- [ ] UI: Dark Mode (#178)

### 💡 Nice-to-Have (nächstes Quartal)

- [ ] Feature: Backup-Statistiken (#189)
- [ ] Docs: Video-Tutorial erstellen
```

### Bei mehreren offenen Tasks

1. ✅ Kritische Bugs zuerst
2. ✅ Dann Features gemäß Roadmap
3. ✅ Dokumentation parallel
4. ✅ Refactorings als separate Tasks

---

## 📊 Fortschritts-Tracking

### GitHub Issues nutzen

**Issue-Labels:**

```
Typ:
- bug
- feature
- docs
- refactor
- question

Priorität:
- priority-critical
- priority-high
- priority-medium
- priority-low

Status:
- status-blocked
- status-in-progress
- status-review
- status-ready

Bereich:
- area-backup
- area-restore
- area-ui
- area-scheduler
```

**Issue-Template:**

```markdown
## Beschreibung

[Was soll implementiert/gefixt werden?]

## Kontext

[Warum ist das wichtig?]

## Mockups

[Falls UI-Änderung: Link zu Mockup]

## Akzeptanzkriterien

- [ ] Feature X funktioniert
- [ ] Tests vorhanden
- [ ] Docs aktualisiert

## Technische Details

[Implementierungs-Hinweise]

## Related Issues

Closes #42
Related to #55
```

---

## 🛠️ Entwickler-Produktivität

### Häufige Kommandos als Aliases

```bash
# .bashrc / .zshrc
alias rustic-dev='npm run tauri:dev'
alias rustic-build='npm run tauri:build'
alias rustic-test='npm test && cd src-tauri && cargo test && cd ..'
alias rustic-lint='npm run lint && cd src-tauri && cargo clippy && cd ..'
alias rustic-fmt='npm run format && cd src-tauri && cargo fmt && cd ..'
```

### VSCode-Tasks

```json
// .vscode/tasks.json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Rustic: Dev",
      "type": "shell",
      "command": "npm run tauri:dev",
      "problemMatcher": []
    },
    {
      "label": "Rustic: Test All",
      "type": "shell",
      "command": "npm test && cd src-tauri && cargo test",
      "problemMatcher": []
    },
    {
      "label": "Rustic: Lint All",
      "type": "shell",
      "command": "npm run lint && cd src-tauri && cargo clippy",
      "problemMatcher": []
    }
  ]
}
```

---

## ✅ Workflow-Checkliste

### Vor Arbeitsbeginn

- [ ] `git pull origin main` (aktueller Stand)
- [ ] Instructions erneut gelesen
- [ ] ROADMAP.md gecheckt
- [ ] Issue verstanden

### Während Entwicklung

- [ ] Tests laufen durch
- [ ] Linter zeigt keine Fehler
- [ ] Code folgt Style-Guide
- [ ] UI folgt Mockups (falls relevant)

### Vor Commit

- [ ] `npm run lint && npm run type-check`
- [ ] `cargo fmt && cargo clippy`
- [ ] Alle Tests erfolgreich
- [ ] Dokumentation aktualisiert

### Vor Push

- [ ] ROADMAP.md aktualisiert
- [ ] CHANGELOG.md aktualisiert (bei User-Änderung)
- [ ] Commit-Message folgt Konventionen
- [ ] Branch ist aktuell mit main

### Nach Merge

- [ ] Branch lokal gelöscht
- [ ] Branch remote gelöscht
- [ ] Nächster Task aus ROADMAP.md gewählt

---

**Version**: 1.0  
**Letzte Aktualisierung**: 2025-10-26
