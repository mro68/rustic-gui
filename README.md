# Rustic GUI 🦀

> **Moderne Desktop-Anwendung für rustic Backup-Management**  
> Intuitive GUI für das [rustic](https://rustic.cli.rs/) Backup-Tool mit Cloud-Storage, Scheduling und Multi-Repository-Support

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Svelte 5](https://img.shields.io/badge/Svelte-5.0-red.svg)](https://svelte.dev/)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Progress](https://img.shields.io/badge/progress-78%25-yellow.svg)](ROADMAP.md)

**Rustic GUI** ist eine **Cross-Platform Desktop-Anwendung** (Linux/Windows), die eine moderne, benutzerfreundliche Oberfläche für das mächtige [rustic](https://rustic.cli.rs/) Backup-Tool bietet.

Mit vollständiger Unterstützung für **Cloud-Storage** (S3, Azure, GCS, SFTP, rclone), **Backup-Scheduling**, **Snapshot-Vergleich** und **sicherer Credential-Verwaltung** im System-Keychain.

![Rustic GUI Screenshot](docs/screenshots/dashboard.png)

---

## 📋 Inhaltsverzeichnis

- [Features](#-features)
- [Installation](#-installation)
- [Erste Schritte](#-erste-schritte)
- [Cloud-Storage](#️-cloud-storage-konfiguration)
- [Sicherheit](#-sicherheit--credentials)
- [Entwicklung](#️-entwicklung)
- [Dokumentation](#-dokumentation)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [Lizenz](#-lizenz)

---

## ✨ Features

### 🎨 **Moderne Benutzeroberfläche**

- Intuitive, responsive UI basierend auf **Svelte 5** mit **Runes API**
- Dark/Light/System-Theme-Support
- Deutsche Lokalisierung (weitere Sprachen geplant)
- Echtzeit-Updates & Toast-Benachrichtigungen

### ☁️ **Umfassende Cloud-Storage-Unterstützung**

- **Native Support:** S3, Azure Blob Storage, Google Cloud Storage, Backblaze B2
- **Via Rclone:** 70+ weitere Provider (Google Drive, Dropbox, OneDrive, SFTP, FTP, WebDAV, pCloud, Mega, etc.)
- **OpenDAL-Integration** für performante Cloud-Operationen
- Connection-Test vor Repository-Erstellung

### 🔐 **Sichere Credential-Verwaltung**

- Passwörter & API-Keys verschlüsselt im **System-Keychain**
  - Linux: GNOME Keyring / KWallet / Secret Service
  - Windows: Windows Credential Manager
  - macOS: Keychain Access (geplant)
- Kein Plaintext-Passwort in Config-Dateien
- Optional: Passwort-Speicherung deaktivierbar

### ⭐ **Favoriten-System**

- Häufig genutzte Repository-Locations als Favoriten speichern
- Auto-Tracking der letzten Verwendung
- Relative Zeitangaben ("Vor 5 Min", "Gestern", "15. Okt")
- Smart-Input mit Auto-Type-Detection (Local/SFTP/S3/rclone)

### 📦 **Multi-Repository-Management**

- Mehrere Repositories gleichzeitig verwalten
- Schnelles Umschalten zwischen Repositories
- Repository-Statistiken (Snapshots, Größe, Last-Accessed)
- Batch-Operationen geplant (M4)

### 🔄 **Backup & Restore**

- Vollständige Backup-Funktionalität
- Fortschrittsanzeige mit Echtzeit-Updates
- Backup-Cancellation während Ausführung
- Exclude-Patterns (z.B. `*.tmp`, `node_modules/`)
- Tag-Support für Snapshot-Organisation

### 📸 **Snapshot-Verwaltung**

- Snapshots durchsuchen & filtern (Datum, Tags, Hostname)
- Snapshot-Vergleich (Side-by-Side Diff)
- Snapshot-Tags hinzufügen/entfernen
- Hierarchischer File-Browser für selektive Restores
- Retention-Policies (Keep-Last/Daily/Weekly/Monthly/Yearly)

### ⏰ **Backup-Scheduling** _(in Entwicklung)_

- Cron-basierte Job-Scheduler
- Visuelle Cron-Expression-Builder
- Job-Execution-History mit Logs
- Desktop-Benachrichtigungen bei Job-Completion/Fehler

### 🌐 **Cross-Platform**

- **Linux**: AppImage (alle Distributionen)
- **Windows**: MSI-Installer & Portable EXE
- **macOS**: _(geplant für M6)_

---

## 📥 Installation

### Linux (AppImage)

```bash
# Download der aktuellen Version
wget https://github.com/mro68/rustic-gui/releases/latest/download/rustic-gui.AppImage

# Ausführbar machen
chmod +x rustic-gui.AppImage

# Starten
./rustic-gui.AppImage
```

**Optional:** Integration ins Menü mit [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher)

### Windows

**Option 1: MSI-Installer** _(empfohlen)_

```powershell
# Download und Doppelklick
# Installiert in C:\Program Files\Rustic GUI\
```

**Option 2: Portable EXE**

```powershell
# Download rustic-gui.exe
# Direkt ausführen ohne Installation
```

### macOS _(coming soon)_

```bash
# DMG-Installer geplant für M6 (Q2 2025)
```

---

## 🚀 Erste Schritte

### 1️⃣ Repository erstellen

1. **Klicken Sie auf "Neues Repository"** im Dashboard
2. **Wählen Sie einen Speicherort:**
   - **📂 Local**: Lokales Verzeichnis oder externe Festplatte
   - **🌐 Network**: SFTP-Server
   - **☁️ Cloud**: S3, Azure, GCS oder andere Cloud-Provider
   - **🕒 Recent**: Kürzlich verwendete Locations
3. **Konfigurieren Sie den Speicherort:**
   - Bei Cloud: Access Keys, Region, Bucket etc. eingeben
   - **Connection-Test**: Klicken Sie auf "Verbindung testen"
4. **Sicheres Passwort eingeben** (min. 8 Zeichen empfohlen)
5. **Optional:** Als Favorit speichern für schnellen Zugriff
6. **Klicken Sie auf "Repository erstellen"**

✅ **Fertig!** Das Repository ist jetzt einsatzbereit.

### 2️⃣ Backup erstellen

1. **Wählen Sie Ihr Repository** aus der Liste
2. **Klicken Sie auf "Backup erstellen"**
3. **Quellpfade hinzufügen:**
   - `/home/user/Documents`
   - `/home/user/Pictures`
4. **Optional: Ausschlussmuster definieren:**
   - `*.tmp`
   - `.cache/`
   - `node_modules/`
5. **Optional: Tags hinzufügen** (z.B. "daily", "important")
6. **Optional: Zeitplan konfigurieren** (Cron: `0 2 * * *` = täglich 2 Uhr)
7. **Klicken Sie auf "Backup starten"**

📊 **Fortschritt verfolgen**: Die Fortschrittsanzeige zeigt Datei-Count, Upload-Bytes und Prozent.

### 3️⃣ Dateien wiederherstellen

1. **Wählen Sie einen Snapshot** aus der Snapshot-Liste
2. **Klicken Sie auf "Wiederherstellen"**
3. **Navigieren Sie durch den Dateibaum:**
   - Ordner erweitern mit Klick
   - Dateien/Ordner mit Checkbox auswählen
4. **Wählen Sie ein Zielverzeichnis** (z.B. `/home/user/restore`)
5. **Overwrite-Behavior wählen:**
   - Nicht überschreiben
   - Überschreiben
   - Nur neuere Dateien
6. **Klicken Sie auf "Wiederherstellen"**

✅ **Done!** Ihre Dateien sind wiederhergestellt.

---

## ☁️ Cloud-Storage-Konfiguration

Rustic GUI unterstützt verschiedene Cloud-Storage-Backends über **OpenDAL** und **Rclone**.

### Amazon S3

**Provider:** `s3`

**Erforderliche Credentials:**

```
Bucket: my-backup-bucket
Region: eu-central-1 (oder us-east-1, ap-southeast-1, etc.)
Access Key ID: AKIAIOSFODNN7EXAMPLE
Secret Access Key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

**Endpoint-Format:** `s3:my-backup-bucket/optional/prefix`

**Permissions benötigt:**

- `s3:PutObject`
- `s3:GetObject`
- `s3:DeleteObject`
- `s3:ListBucket`

**Test:** Klicken Sie auf "Verbindung testen" um die Konfiguration zu prüfen.

---

### Azure Blob Storage

**Provider:** `azure` (azblob)

**Erforderliche Credentials:**

```
Container: backups
Account Name: mystorageaccount
Account Key: <your-account-key>
```

**Endpoint-Format:** `azblob:backups/optional/prefix`

---

### Google Cloud Storage

**Provider:** `gcs`

**Erforderliche Credentials:**

```
Bucket: my-backup-bucket
Service Account JSON: <service-account-credentials.json>
```

**Endpoint-Format:** `gcs:my-backup-bucket/optional/prefix`

**Setup:**

1. Erstellen Sie ein Service Account in [GCP Console](https://console.cloud.google.com/iam-admin/serviceaccounts)
2. Laden Sie die JSON-Key-Datei herunter
3. Fügen Sie den Inhalt in das "Service Account JSON"-Feld ein

---

### Backblaze B2

**Provider:** `b2`

**Erforderliche Credentials:**

```
Bucket: my-backup-bucket
Application Key ID: <key-id>
Application Key: <application-key>
```

**Endpoint-Format:** `b2:my-backup-bucket/optional/prefix`

---

### Wasabi / MinIO (S3-kompatibel)

**Provider:** `s3` mit Custom Endpoint

**Wasabi:**

```
Provider: s3
Endpoint: s3.eu-central-1.wasabisys.com
Bucket: my-backup-bucket
Access Key: <access-key>
Secret Key: <secret-key>
```

**MinIO (Self-Hosted):**

```
Provider: s3
Endpoint: minio.myserver.com:9000
Bucket: backups
Access Key: <minio-access-key>
Secret Key: <minio-secret-key>
```

---

### SFTP (via Rclone)

**Provider:** `rclone` → `sftp`

**Erforderliche Credentials:**

```
Host: backup.example.com
Port: 22 (default)
Username: backup-user
Password: <password>
Path: /backup/rustic
```

**Endpoint-Format:** `sftp://backup-user@backup.example.com:22/backup/rustic`

**Voraussetzung:** [Rclone](https://rclone.org/downloads/) muss installiert sein (siehe unten).

---

### Weitere Cloud-Provider (via Rclone)

**Rustic GUI unterstützt über Rclone 70+ weitere Cloud-Provider:**

| Provider               | Typ                     | Endpoint-Format                         |
| ---------------------- | ----------------------- | --------------------------------------- |
| **Google Drive**       | Personal/Business Cloud | `gdrive:backup/`                        |
| **Dropbox**            | Personal/Business Cloud | `dropbox:/Backups`                      |
| **Microsoft OneDrive** | Personal/Business Cloud | `onedrive:/Backups`                     |
| **pCloud**             | Cloud Storage           | `pcloud:/Backups`                       |
| **Mega.nz**            | Cloud Storage           | `mega:/Backups`                         |
| **FTP/FTPS**           | Network Storage         | `ftp://ftp.example.com/backup`          |
| **WebDAV**             | Network Storage         | `webdav://dav.example.com/backup`       |
| **Box**                | Cloud Storage           | `box:/Backups`                          |
| **Yandex Disk**        | Cloud Storage           | `yandex:/Backups`                       |
| **...und 60+ weitere** |                         | siehe [rclone.org](https://rclone.org/) |

**Voraussetzung:** [Rclone](https://rclone.org/downloads/) muss installiert sein.

**Installation:**

```bash
# Linux
sudo apt install rclone

# macOS
brew install rclone

# Windows
winget install Rclone.Rclone

# oder manuell: https://rclone.org/downloads/
```

**Verifikation:**

```bash
rclone version
# Sollte "rclone v1.67.0" oder höher anzeigen
```

---

## 🔐 Sicherheit & Credentials

### Keychain-Integration

**Alle sensiblen Daten** (Passwörter, API-Keys, Access-Tokens) werden **verschlüsselt im System-Keychain** gespeichert:

| OS          | Keychain-System                          | Verschlüsselung |
| ----------- | ---------------------------------------- | --------------- |
| **Linux**   | GNOME Keyring / KWallet / Secret Service | ✅ AES-256      |
| **Windows** | Windows Credential Manager               | ✅ DPAPI        |
| **macOS**   | Keychain Access _(geplant)_              | ✅ Keychain     |

**Nicht gespeichert werden:**

- ❌ Konfigurationsdateien (`~/.config/rustic-gui/config.toml`) enthalten **keine** Passwörter oder API-Keys
- ✅ Nur Metadata (Provider-Typ, Endpoint, Bucket-Name, Repository-ID, etc.)

**Beispiel Config-Datei:**

```toml
[[repositories]]
id = "repo-abc123"
name = "S3 Backup"
path = "s3:my-bucket/backups"
backend_type = "s3"
password_stored = true  # ← Indikator, dass Passwort im Keychain ist

[repositories.backend_options]
region = "eu-central-1"
# KEINE Access Keys hier!
```

### Credential-Workflow

1. **Verbindung testen**: Klicken Sie auf "Verbindung testen" im LocationPickerDialog
2. **Erfolgs-Dialog**: Bei erfolgreicher Verbindung erscheint ein Prompt:

   ```
   ✅ Verbindung erfolgreich!

   [ ] Zugangsdaten im Keychain speichern
   [ ] Als Favorit speichern

   [Abbrechen]  [Speichern]
   ```

3. **Speichern**: Wählen Sie die gewünschten Optionen
4. **Fertig**: Credentials sind sicher gespeichert

### Favoriten-System

**Häufig genutzte Repository-Locations** können als Favoriten gespeichert werden:

**Features:**

- ✅ **Auto-Tracking**: Zuletzt verwendete Favoriten erscheinen zuerst
- ✅ **Relative Zeitangaben**: "Vor 5 Min", "Vor 2 Std", "Gestern", "15. Okt"
- ✅ **Sichere Speicherung**: Nur Metadata in Config, Credentials im Keychain
- ✅ **One-Click-Access**: Favorit auswählen → Automatisch verbinden

**Beispiel:**

```
📌 Favoriten
─────────────────────────────────────────
☁️  S3 Production Backup
    s3:prod-backups/
    Vor 5 Min

📂  Local NAS Backup
    /mnt/nas/backups
    Gestern

🌐  SFTP Office Server
    sftp://backup@office.example.com/backup
    15. Okt
```

---

## 🛠️ Entwicklung

### Voraussetzungen

- **Rust 1.75+** - [rustup.rs](https://rustup.rs/)
- **Node.js 20+** - [nodejs.org](https://nodejs.org/)
- **pnpm 8+** - `npm install -g pnpm` (oder npm/yarn)
- **Tauri CLI** - `cargo install tauri-cli`

**Linux-Zusätze:**

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch
sudo pacman -S webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  gtk3 \
  libappindicator-gtk3 \
  librsvg
```

### Development-Setup

```bash
# 1. Repository klonen
git clone https://github.com/mro68/rustic-gui.git
cd rustic-gui

# 2. Dependencies installieren
pnpm install

# 3. Rust-Dependencies kompilieren
cd src-tauri
cargo build
cd ..

# 4. Development-Server starten
pnpm tauri:dev
```

Der Development-Server startet:

- **Frontend**: http://localhost:1420 (Vite HMR)
- **Backend**: Tauri Window mit Hot-Reload

### Projekt-Struktur

```
rustic-gui/
├── src/                    # Frontend (Svelte 5 + TypeScript)
│   ├── lib/
│   │   ├── components/     # Svelte-Komponenten
│   │   │   ├── shared/     # Button, Modal, Toast, etc.
│   │   │   ├── dialogs/    # JobDialog, RestoreDialog, LocationPicker (modularisiert)
│   │   │   │   ├── JobDialog/          # Unified Create/Edit (4 Tabs)
│   │   │   │   └── LocationPicker/     # 5 Sub-Komponenten (Local/Network/Cloud/Recent/Credentials)
│   │   │   ├── layout/     # Sidebar, Header, MainLayout
│   │   │   └── pages/      # Dashboard, BackupJobs, Snapshots (mit Sub-Komponenten)
│   │   │       └── Snapshots/          # SnapshotTable, DetailsModal, ContextMenu
│   │   ├── stores/         # Svelte Stores (State)
│   │   ├── api/            # Backend-API-Wrapper
│   │   ├── types/          # TypeScript Types
│   │   └── utils/          # Helper-Funktionen
│   └── routes/             # SvelteKit Routes
│
├── src-tauri/              # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── main.rs         # Entry Point
│   │   ├── lib.rs          # Command-Registrierung
│   │   ├── state.rs        # AppState (Repository-Cache, Scheduler)
│   │   ├── config.rs       # Config-Management (TOML)
│   │   ├── error.rs        # Error-Typen
│   │   ├── types.rs        # DTOs (Data Transfer Objects)
│   │   ├── commands/       # Tauri Commands (47 Commands in 6 Modulen)
│   │   │   ├── repository.rs
│   │   │   ├── backup.rs
│   │   │   ├── snapshot.rs
│   │   │   ├── job.rs
│   │   │   ├── settings.rs
│   │   │   └── system.rs
│   │   ├── rustic/         # rustic_core Integration
│   │   ├── scheduler/      # Job-Scheduler (Cron)
│   │   └── keychain/       # Passwort-Management
│   └── Cargo.toml
│
├── docs/                   # Dokumentation
│   ├── mockups/            # UI-Mockups (HTML)
│   ├── features/           # Feature-Specs
│   └── reports/            # Entwicklungs-Reports
│
├── ROADMAP.md              # Entwicklungs-Roadmap (6 Milestones)
├── CHANGELOG.md            # Änderungshistorie
├── CONTRIBUTING.md         # Contribution-Guidelines
└── README.md               # Diese Datei
```

### Build

```bash
# Development-Build
pnpm tauri:dev

# Production-Build
pnpm tauri:build

# Artifacts werden erstellt in:
# Linux:   src-tauri/target/release/bundle/appimage/
# Windows: src-tauri/target/release/bundle/msi/
```

### Tests

```bash
# Frontend-Tests (Vitest)
pnpm test
pnpm test:ui      # Mit UI
pnpm test:coverage # Mit Coverage

# Backend-Tests (Cargo Test)
cd src-tauri
cargo test
cargo test -- --nocapture # Mit Logs
cargo clippy              # Linting

# Integration-Tests
cd src-tauri
cargo test --test integration_tests
```

### Code-Style

- **Rust**: `rustfmt` + `clippy` (automatisch via Pre-Commit Hooks)
- **TypeScript**: ESLint + Prettier (automatisch via Pre-Commit Hooks)
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/) (feat:, fix:, docs:, etc.)

**Pre-Commit Hooks:**

```bash
# Werden automatisch bei git commit ausgeführt:
# 1. cargo fmt --check
# 2. cargo clippy
# 3. cargo test
# 4. npm run lint
# 5. npm run check (TypeScript)
```

---

## 📚 Dokumentation

- **[ROADMAP.md](ROADMAP.md)** - Entwicklungs-Roadmap (6 Milestones, aktuell: **78% fertig**)
- **[CHANGELOG.md](CHANGELOG.md)** - Änderungshistorie & Release-Notes
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution-Guidelines
- **[docs/](docs/)** - Erweiterte Dokumentation
  - [AI Instructions](docs/copilot-instructions.md) - Für AI-gestützte Entwicklung
  - [UI-Mockups](docs/mockups/) - HTML-Prototypen der gesamten UI
  - [Feature-Specs](docs/features/) - Detaillierte Feature-Beschreibungen
  - [Development-Reports](docs/reports/) - Entwicklungs-Status-Reports

---

## 🔧 Troubleshooting

### Cloud-Verbindung fehlgeschlagen

**Symptom:** `Connection test failed: Invalid credentials`

**Lösungen:**

1. **Prüfen Sie Access Keys auf Tippfehler**
   - Access Key ID: Exakt 20 Zeichen (nur Großbuchstaben & Zahlen)
   - Secret Access Key: Exakt 40 Zeichen (Base64)

2. **Prüfen Sie Berechtigungen**
   - S3: `s3:PutObject`, `s3:GetObject`, `s3:DeleteObject`, `s3:ListBucket`
   - Azure: `Storage Blob Data Contributor` Rolle
   - GCS: Service Account mit `Storage Admin` Berechtigung

3. **Prüfen Sie die Region** (nur S3)
   - Muss mit Bucket-Region übereinstimmen
   - Beispiel: Bucket in `eu-central-1` → Region `eu-central-1`

4. **Prüfen Sie Custom Endpoints** (Wasabi/MinIO)
   - Format: `s3.eu-central-1.wasabisys.com` (ohne `https://`)
   - MinIO: `minio.myserver.com:9000` (mit Port wenn != 443/80)

---

### Rclone nicht gefunden

**Symptom:** `rclone not installed` oder `RcloneNotFound`

**Lösung:**

```bash
# Linux (Debian/Ubuntu)
sudo apt install rclone

# Linux (Fedora)
sudo dnf install rclone

# Linux (Arch)
sudo pacman -S rclone

# macOS
brew install rclone

# Windows
winget install Rclone.Rclone

# oder manuell: https://rclone.org/downloads/
```

**Verifikation:**

```bash
rclone version
# Sollte zeigen: rclone v1.67.0 oder höher
```

---

### Repository gesperrt

**Symptom:** `Repository locked by another process` oder `RepositoryLocked`

**Ursachen:**

- Anderer rustic-Prozess läuft gerade
- Vorheriger Prozess wurde abgebrochen ohne Lock freizugeben
- Lock-File ist veraltet (> 30 Minuten alt)

**Lösungen:**

1. **Prüfen Sie laufende Prozesse:**

   ```bash
   ps aux | grep rustic
   # Wenn Prozesse gefunden: Beenden mit kill <PID>
   ```

2. **Warten Sie einige Minuten:**
   - Lock läuft automatisch nach 30 Minuten ab

3. **Notfall: Force-Unlock** (nur wenn sicher dass kein anderer Prozess läuft!)
   - Klicken Sie auf Repository → "Erweitert" → "Repository entsperren (Force)"
   - ⚠️ **Warnung**: Kann zu Datenkorruption führen wenn wirklich ein Prozess läuft!

---

### Passwort vergessen

**Symptom:** `Wrong password or corrupted repository` oder `AuthenticationFailed`

**Lösung:**

- ❌ **Repository-Passwörter können NICHT wiederhergestellt werden**
- ❌ Ohne Passwort ist **kein Zugriff** auf Backups möglich
- ✅ **Prävention**: Bewahren Sie Passwörter sicher auf (z.B. KeePass, 1Password, Bitwarden)

**Best Practices:**

- Passwort in Passwort-Manager speichern
- Passwort-Hint als Repository-Name notieren
- Recovery-Key-File erstellen (geplant für M4)

---

### Build-Fehler

**Symptom:** `error: linker 'cc' not found` (Linux)

**Lösung:**

```bash
# Debian/Ubuntu
sudo apt install build-essential

# Fedora
sudo dnf install gcc gcc-c++

# Arch
sudo pacman -S base-devel
```

---

**Symptom:** `webkit2gtk not found` (Linux)

**Lösung:**

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel

# Arch
sudo pacman -S webkit2gtk-4.1
```

---

### Weitere Probleme?

**Nicht gefunden?** Erstellen Sie ein [GitHub Issue](https://github.com/mro68/rustic-gui/issues) mit:

- Fehlermeldung (vollständig)
- OS & Version (z.B. Ubuntu 22.04, Windows 11)
- Rustic GUI Version
- Schritte zur Reproduktion

---

## 🤝 Contributing

**Contributions sind willkommen!** 🎉

Bitte lesen Sie [CONTRIBUTING.md](CONTRIBUTING.md) für Details zu:

- Code-Style-Guidelines
- Pull-Request-Prozess
- Development-Workflow
- Conventional Commits

**Quick-Start:**

```bash
# 1. Fork erstellen
# 2. Feature-Branch erstellen
git checkout -b feature/awesome-feature

# 3. Änderungen committen (Conventional Commits!)
git commit -m "feat(snapshots): Add snapshot comparison view"

# 4. Push & Pull-Request erstellen
git push origin feature/awesome-feature
```

---

## 📄 Lizenz

Dieses Projekt ist unter der **MIT-Lizenz** lizenziert - siehe [LICENSE](LICENSE) Datei.

```
MIT License

Copyright (c) 2024 rustic-gui contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
...
```

---

## 🙏 Danksagungen

- **[rustic](https://rustic.cli.rs/)** - Das zugrundeliegende Backup-Tool
- **[Tauri](https://tauri.app/)** - Desktop-Framework für sichere, performante Apps
- **[Svelte](https://svelte.dev/)** - Reaktives UI-Framework
- **[OpenDAL](https://opendal.apache.org/)** - Unified Cloud-Storage-Abstraction
- **[Rclone](https://rclone.org/)** - Cloud-Storage-Sync & Mount
- **[rustic Community](https://discord.gg/WRUWENZnzQ)** - Support & Feedback

---

## 📞 Support

- **Bugs & Feature-Requests**: [GitHub Issues](https://github.com/mro68/rustic-gui/issues)
- **Diskussionen**: [GitHub Discussions](https://github.com/mro68/rustic-gui/discussions)
- **rustic Community**: [Discord](https://discord.gg/WRUWENZnzQ)

---

## 🗺️ Roadmap

Aktueller Stand: **78% fertig** ([Details](ROADMAP.md))

### ✅ Phase 0: Basis-Setup _(Q4 2024)_ - **DONE**

- [x] Projekt-Setup (Tauri + Svelte)
- [x] Basic UI-Struktur
- [x] State-Management

### ✅ Phase 1: Core-Features _(Q4 2024)_ - **DONE**

- [x] Repository-Management (Add, List, Switch, Delete)
- [x] Backup-Funktionalität mit Progress-Tracking
- [x] Snapshot-Liste & Details
- [x] Basic Restore
- [x] Keychain-Integration

### 🚧 Phase 2: Cloud-Storage _(Q1 2025)_ - **IN PROGRESS**

- [x] OpenDAL-Integration (S3, Azure, GCS)
- [x] Rclone-Integration (70+ Provider)
- [x] Location-Picker-Dialog
- [x] Favoriten-System
- [ ] Connection-Pooling _(in Arbeit)_

### ⏸️ Phase 3: Scheduling _(Q1 2025)_ - **PLANNED**

- [ ] Cron-based Job-Scheduler
- [ ] Job-Execution-History
- [ ] Desktop-Benachrichtigungen

### ⏸️ Phase 4: Advanced-Features _(Q2 2025)_ - **PLANNED**

- [ ] Snapshot-Vergleich (Side-by-Side Diff)
- [ ] Batch-Operations (Multi-Select)
- [ ] Advanced Filtering
- [ ] Repository-Repair

### ⏸️ Phase 5: Testing & QA _(Q2 2025)_ - **PLANNED**

- [ ] Unit-Tests (Backend: 80%, Frontend: 70%)
- [ ] Integration-Tests
- [ ] E2E-Tests
- [ ] Performance-Benchmarks

### ⏸️ Phase 6: Release _(Q2-Q3 2025)_ - **PLANNED**

- [ ] macOS-Support
- [ ] i18n (Mehrsprachigkeit)
- [ ] Auto-Update
- [ ] v1.0.0 Release

Siehe [ROADMAP.md](ROADMAP.md) für Details.

---

<div align="center">

**Made with ❤️ for the rustic community**

[![Star on GitHub](https://img.shields.io/github/stars/mro68/rustic-gui?style=social)](https://github.com/mro68/rustic-gui)
[![Follow on GitHub](https://img.shields.io/github/followers/mro68?style=social&label=Follow)](https://github.com/mro68)

</div>
