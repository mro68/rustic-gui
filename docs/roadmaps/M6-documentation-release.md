# Milestone 6: Dokumentation & Release 📚

> **Release-Readiness für v1.0**

**Dauer:** 13h (2 Tage) | **Status:** ~50%  
**Priorität:** 🟢 LOW (erst nach M5)  
**Dependencies:** M1-M5 (alle Features fertig, Tests grün)

---

## Übersicht

**Ziel:** Projekt ist bereit für öffentliches Release v1.0.

**Status:** README ist noch Template, CHANGELOG komplett, Release-Prozess fehlt.

---

## 6.1 README.md Komplettrewrite

**Dauer:** 5h | **Priorität:** 🟠 HIGH

### Struktur

````markdown
# Rustic GUI

> Modern Desktop GUI for rustic Backup Tool

[Badges: Build Status, Version, License, Downloads]
[Screenshots]

## Features

- 🎨 Modern UI (Svelte 5 + TailwindCSS)
- 📦 Multi-Repository Support
- ⏰ Scheduled Backups (Cron)
- ☁️ Cloud Storage (S3, Azure, GCS, SFTP via Rclone)
- 🔄 Restore with File Browser
- 📊 Statistics & Reporting
- 🔐 Secure Password Storage (OS Keychain)

## Installation

### Linux (AppImage)

\```bash
wget https://github.com/mro68/rustic-gui/releases/latest/download/rustic-gui-x86_64.AppImage
chmod +x rustic-gui-x86_64.AppImage
./rustic-gui-x86_64.AppImage
\```

### Windows (MSI Installer)

Download from [Releases](https://github.com/mro68/rustic-gui/releases)

## Quick Start

1. **Add Repository:** Click "+ Add Repository" → Select location
2. **Create Backup Job:** Define sources, schedule, retention
3. **Run Backup:** Manual or scheduled
4. **Restore Files:** Browse snapshots, select files

## Screenshots

[4-6 Screenshots der Hauptfunktionen]

## Documentation

- [User Guide](docs/user-guide.md)
- [Developer Guide](CONTRIBUTING.md)
- [API Documentation](docs/rustic/)

## Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Tauri CLI

### Build from Source

\```bash
git clone https://github.com/mro68/rustic-gui.git
cd rustic-gui
npm install
cargo build
npm run tauri dev
\```

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Technology Stack

- **Frontend:** Svelte 5 + TypeScript + TailwindCSS
- **Backend:** Rust + Tauri 2.0
- **Backup Engine:** rustic_core 0.8.0
- **Storage Backends:** rustic_backend 0.5.3 (Local, S3, Azure, GCS, Rclone)
- **Scheduler:** tokio-cron 0.13

## Roadmap

See [ROADMAP.md](ROADMAP.md)

## License

MIT License - see [LICENSE](LICENSE)

## Credits

- [rustic](https://rustic.cli.rs/) - Backup engine
- [Tauri](https://tauri.app/) - Desktop framework
- [Svelte](https://svelte.dev/) - UI framework
  \```

### Tasks

- [ ] **Badges erstellen** (1h)
  - Build-Status (GitHub Actions)
  - Version (shields.io)
  - License Badge
  - Download-Counter
- [ ] **Screenshots** (2h)
  - Dashboard
  - Add Repository Dialog
  - Create Job Dialog
  - Snapshots Page
  - Restore Dialog
  - Settings
- [ ] **Installation-Section** (1h)
  - AppImage-Anleitung
  - MSI-Anleitung
  - Build-from-Source
- [ ] **Quick-Start-Guide** (1h)
  - 4 Schritte mit Screenshots

---

## 6.2 CHANGELOG.md finalisieren

**Dauer:** 2h | **Priorität:** 🟡 MEDIUM

### Tasks

- [ ] **v1.0.0 Eintrag erstellen** (1h)

  ```markdown
  ## [1.0.0] - 2025-11-XX

  ### Added

  - 🎉 Initiales Release
  - ✨ Multi-Repository-Management
  - ⏰ Job-Scheduling mit Cron
  - ☁️ Cloud-Backend-Support (S3, Azure, GCS, SFTP)
  - 🔄 Restore mit File-Browser
  - 📊 Repository-Statistiken
  - 🔐 Keychain-Integration

  ### Changed

  - (keine, erstes Release)

  ### Fixed

  - (keine, erstes Release)

  ### Security

  - Credentials werden in OS Keychain gespeichert
  ```
````

- [ ] **Release-Notes schreiben** (1h)
  - Highlights
  - Known Issues
  - Migration-Guide (falls nötig)

---

## 6.3 Build-Artefakte erstellen

**Dauer:** 4h | **Priorität:** 🔴 HIGHEST

### Tasks

#### 6.3.1 AppImage (Linux) (2h)

**Datei:** `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - uses: actions/setup-node@v3

      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev

      - name: Build
        run: |
          npm ci
          npm run tauri build -- --target x86_64-unknown-linux-gnu

      - name: Create AppImage
        run: |
          # Tauri erstellt automatisch AppImage
          mv src-tauri/target/release/bundle/appimage/*.AppImage rustic-gui-x86_64.AppImage

      - uses: softprops/action-gh-release@v1
        with:
          files: rustic-gui-x86_64.AppImage
```

---

#### 6.3.2 MSI Installer (Windows) (2h)

**Cross-Compilation oder Windows-Runner:**

```yaml
build-windows:
  runs-on: windows-latest
  steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
    - uses: actions/setup-node@v3

    - name: Build
      run: |
        npm ci
        npm run tauri build -- --target x86_64-pc-windows-msvc

    - name: Upload MSI
      uses: softprops/action-gh-release@v1
      with:
        files: src-tauri/target/release/bundle/msi/*.msi
```

---

## 6.4 Release-Prozess

**Dauer:** 2h | **Priorität:** 🟡 MEDIUM

### Tasks

- [ ] **GitHub Release erstellen** (1h)
  - Tag: `v1.0.0`
  - Title: "Rustic GUI v1.0.0 - Initial Release"
  - Description: CHANGELOG.md kopieren
  - Artefakte anhängen (AppImage, MSI)
- [ ] **Announcement** (1h)
  - rustic Discord
  - Reddit r/rust
  - Hacker News (Show HN)
  - Twitter/X

---

## Zusammenfassung

**Gesamt-Dauer:** 13h

**Deliverables:**

- [ ] README.md komplett (User + Developer Guide)
- [ ] CHANGELOG.md v1.0.0
- [ ] Build-Artefakte (AppImage + MSI)
- [ ] GitHub Release v1.0.0
- [ ] Announcements

**Akzeptanz-Kriterien:**

- [ ] README enthält alle Installations-Optionen
- [ ] Screenshots zeigen alle Haupt-Features
- [ ] AppImage funktioniert auf Ubuntu 22.04
- [ ] MSI installiert auf Windows 10/11
- [ ] GitHub Release ist live

---

**🎉 Nach M6: v1.0.0 Release erfolgreich!**

**Nächste Schritte (v1.1):**

- Native SFTP-Backend (ohne Rclone)
- Performance-Optimierungen
- Mobile App (React Native + Tauri Mobile)
- Internationalisierung (i18n)

---

**[← Zurück zu M5](M5-testing-qa.md)** | **[Zurück zur Roadmap](../../ROADMAP.md)**
