# Rustic GUI

> **Moderne Desktop-Anwendung für rustic Backup-Management mit Cloud-Storage-Support**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Svelte 5](https://img.shields.io/badge/Svelte-5.0-red.svg)](https://svelte.dev/)
[![Tauri 2.0](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)

Rustic GUI ist eine benutzerfreundliche Desktop-Anwendung für das [rustic](https://rustic.cli.rs/) Backup-Tool mit vollständiger Unterstützung für Cloud-Storage-Backends (S3, Azure, GCS, SFTP uvm.).

## ✨ Features

- 🎨 **Moderne UI** - Intuitive Benutzeroberfläche basierend auf Svelte 5
- ☁️ **Cloud-Storage** - S3, Azure Blob, Google Cloud Storage, Backblaze B2, SFTP und 70+ weitere Provider
- 🔐 **Sichere Credentials** - Passwörter und API-Keys werden verschlüsselt im System-Keychain gespeichert
- ⭐ **Favoriten-System** - Häufig genutzte Repository-Locations schnell wiederverwenden
- 📦 **Multi-Repository** - Verwaltung mehrerer Backup-Repositories
- 🔄 **Backup & Restore** - Vollständige Backup- und Wiederherstellungs-Funktionen
- 📸 **Snapshot-Verwaltung** - Snapshots durchsuchen, vergleichen und verwalten
- 🔍 **File-Browser** - Hierarchischer Dateibaum für selektive Restores
- 🌐 **Cross-Platform** - Linux und Windows Support

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

### Windows

```powershell
# Download und Installation des MSI-Installers
# Oder portable EXE direkt ausführen
```

## 🚀 Erste Schritte

### 1. Repository erstellen

1. Klicken Sie auf "Neues Repository"
2. Wählen Sie einen Speicherort:
   - **Local**: Lokales Verzeichnis oder externe Festplatte
   - **Network**: SFTP-Server
   - **Cloud**: S3, Azure, GCS oder andere Cloud-Provider
   - **Recent**: Kürzlich verwendete Locations
3. Geben Sie ein sicheres Passwort ein
4. Klicken Sie auf "Repository erstellen"

### 2. Backup erstellen

1. Wählen Sie Ihr Repository aus
2. Klicken Sie auf "Backup erstellen"
3. Wählen Sie die zu sichernden Ordner
4. Optional: Tags, Ausschlussmuster, Zeitplan konfigurieren
5. Klicken Sie auf "Backup starten"

### 3. Dateien wiederherstellen

1. Wählen Sie einen Snapshot aus
2. Klicken Sie auf "Wiederherstellen"
3. Navigieren Sie durch den Dateibaum
4. Wählen Sie die gewünschten Dateien/Ordner
5. Wählen Sie ein Zielverzeichnis
6. Klicken Sie auf "Wiederherstellen"

## ☁️ Cloud-Storage-Konfiguration

Rustic GUI unterstützt verschiedene Cloud-Storage-Backends über **OpenDAL** und **Rclone**.

### Amazon S3

**Provider:** `s3`

```
Bucket: my-backup-bucket
Region: eu-central-1
Access Key ID: AKIAIOSFODNN7EXAMPLE
Secret Access Key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

**Endpoint-Format:** `s3:my-backup-bucket`

**Test:** Klicken Sie auf "Verbindung testen" um die Konfiguration zu prüfen.

### Azure Blob Storage

**Provider:** `azure`

```
Container: backups
Account Name: mystorageaccount
Account Key: <your-account-key>
```

**Endpoint-Format:** `azblob:backups`

### Google Cloud Storage

**Provider:** `gcs`

```
Bucket: my-backup-bucket
Service Account JSON: <service-account-credentials.json>
```

**Endpoint-Format:** `gcs:my-backup-bucket`

### Backblaze B2

**Provider:** `b2`

```
Bucket: my-backup-bucket
Application Key ID: <key-id>
Application Key: <application-key>
```

**Endpoint-Format:** `b2:my-backup-bucket`

### Wasabi / MinIO (S3-kompatibel)

**Provider:** `s3` mit Custom Endpoint

```
Provider: s3
Endpoint: s3.eu-central-1.wasabisys.com  (Wasabi)
         oder
         minio.myserver.com:9000          (MinIO)
Bucket: my-backup-bucket
Access Key: <access-key>
Secret Key: <secret-key>
```

### SFTP (via Rclone)

**Provider:** `rclone` → `sftp`

```
Host: backup.example.com
Port: 22
Username: backup-user
Password: <password>
Path: /backup
```

**Endpoint-Format:** `sftp://backup-user@backup.example.com/backup`

### Weitere Cloud-Provider (via Rclone)

Rustic GUI unterstützt über Rclone 70+ weitere Cloud-Provider:

- **Google Drive** - persönlicher Cloud-Speicher
- **Dropbox** - Dropbox Business & Personal
- **Microsoft OneDrive** - OneDrive Personal & Business
- **pCloud** - pCloud
- **Mega** - Mega.nz
- **FTP/FTPS** - FTP-Server
- **WebDAV** - WebDAV-Server

**Voraussetzung:** [Rclone](https://rclone.org/downloads/) muss installiert sein.

```bash
# Linux
sudo apt install rclone

# macOS
brew install rclone

# Windows
winget install Rclone.Rclone
```

## 🔐 Sicherheit & Credentials

### Keychain-Integration

Alle sensiblen Daten (Passwörter, API-Keys, Access-Tokens) werden verschlüsselt im System-Keychain gespeichert:

- **Linux**: GNOME Keyring / KWallet / Secret Service
- **Windows**: Windows Credential Manager
- **macOS**: Keychain Access

**Nicht gespeichert werden:**
- Konfigurationsdateien enthalten **keine** Passwörter oder API-Keys
- Nur Metadata (Provider-Typ, Endpoint, Bucket-Name, etc.)

### Credential-Workflow

1. **Verbindung testen**: Klicken Sie auf "Verbindung testen" im LocationPickerDialog
2. **Erfolgs-Dialog**: Bei erfolgreicher Verbindung erscheint ein Prompt
3. **Speichern**: Wählen Sie "Zugangsdaten im Keychain speichern"
4. **Optional**: "Als Favorit speichern" für schnellen Zugriff

### Favoriten-System

Häufig genutzte Repository-Locations können als Favoriten gespeichert werden:

- **Auto-Tracking**: Zuletzt verwendete Favoriten erscheinen zuerst
- **Relative Zeitangaben**: "Vor 5 Min", "Gestern", "15. Okt"
- **Sichere Speicherung**: Nur Metadata in Config, Credentials im Keychain

## 🛠️ Entwicklung

### Voraussetzungen

- **Rust 1.75+** - [rustup.rs](https://rustup.rs/)
- **Node.js 20+** - [nodejs.org](https://nodejs.org/)
- **Tauri CLI** - `cargo install tauri-cli`

### Development-Setup

```bash
# Repository klonen
git clone https://github.com/mro68/rustic-gui.git
cd rustic-gui

# Dependencies installieren
npm install
cd src-tauri && cargo build && cd ..

# Development-Server starten
npm run tauri:dev
```

### Build

```bash
# Production-Build erstellen
npm run tauri:build

# Artifacts in:
# - src-tauri/target/release/bundle/appimage/ (Linux)
# - src-tauri/target/release/bundle/msi/ (Windows)
```

### Tests

```bash
# Frontend-Tests
npm test

# Backend-Tests
cd src-tauri
cargo test
```

## 📚 Dokumentation

- **[ROADMAP.md](ROADMAP.md)** - Entwicklungs-Roadmap und Projekt-Status
- **[CHANGELOG.md](CHANGELOG.md)** - Änderungshistorie
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution-Guidelines
- **[docs/](docs/)** - Erweiterte Dokumentation

## 🔧 Troubleshooting

### Cloud-Verbindung fehlgeschlagen

**Symptom:** "Connection test failed: Invalid credentials"

**Lösung:**
1. Prüfen Sie Access Key und Secret Key auf Tippfehler
2. Prüfen Sie ob die Credentials die erforderlichen Berechtigungen haben
3. Bei S3: Prüfen Sie die Region (muss mit Bucket-Region übereinstimmen)
4. Bei Custom Endpoints: Prüfen Sie das Endpoint-Format

### Rclone nicht gefunden

**Symptom:** "rclone not installed"

**Lösung:**
```bash
# Linux
sudo apt install rclone

# macOS
brew install rclone

# Windows
winget install Rclone.Rclone

# oder manuell: https://rclone.org/downloads/
```

### Repository gesperrt

**Symptom:** "Repository locked by another process"

**Lösung:**
1. Prüfen Sie ob ein anderer rustic-Prozess läuft: `ps aux | grep rustic`
2. Warten Sie einige Minuten (Lock läuft automatisch ab)
3. Notfall: "Repository entsperren (Force)" im Repository-Menü

### Passwort vergessen

**Symptom:** "Wrong password or corrupted repository"

**Lösung:**
- Repository-Passwörter können **nicht wiederhergestellt** werden
- Ohne Passwort ist kein Zugriff auf Backups möglich
- Bewahren Sie Passwörter sicher auf (z.B. Passwort-Manager)

## 🤝 Contributing

Contributions sind willkommen! Bitte lesen Sie [CONTRIBUTING.md](CONTRIBUTING.md) für Details.

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) Datei.

## 🙏 Danksagungen

- [rustic](https://rustic.cli.rs/) - Das zugrundeliegende Backup-Tool
- [Tauri](https://tauri.app/) - Desktop-Framework
- [Svelte](https://svelte.dev/) - UI-Framework
- [OpenDAL](https://opendal.apache.org/) - Cloud-Storage-Abstraction
- [Rclone](https://rclone.org/) - Cloud-Storage-Sync

## 📞 Support

- **Bugs & Features**: [GitHub Issues](https://github.com/mro68/rustic-gui/issues)
- **Diskussionen**: [GitHub Discussions](https://github.com/mro68/rustic-gui/discussions)
- **rustic Community**: [Discord](https://discord.gg/WRUWENZnzQ)

---

**Made with ❤️ for the rustic community**
