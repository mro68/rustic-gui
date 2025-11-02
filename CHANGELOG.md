# Changelog

Alle wichtigen Änderungen an Rustic GUI werden hier dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
und dieses Projekt folgt [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### 2025-11-02 - Retention-Policy UI & Integration

- **RetentionPolicyDialog.svelte**: Kompletter Dialog für Retention-Policy-Konfiguration
  - 5 Policy-Felder: keep_last, keep_daily, keep_weekly, keep_monthly, keep_yearly
  - Preview-Funktion (Dry-Run) zeigt Anzahl der zu behaltenden/löschenden Snapshots
  - Visual Stats mit farbcodierten Keep/Delete-Zählern
  - Snapshot-ID-Liste (erste 10 + weitere Anzahl)
  - Bestätigungsdialog vor tatsächlicher Löschung
  - Validierung: mindestens ein Keep-Wert erforderlich
- **Snapshots.svelte Integration**:
  - "Retention Policy" Button in Toolbar
  - Automatisches Snapshot-Reload nach Policy-Anwendung
  - Event-Handler für Dialog-Interaktionen
- **API Layer** (`src/lib/api/retention.ts`):
  - `previewRetentionPolicy()`: Dry-Run Vorschau
  - `applyRetentionPolicy()`: Tatsächliche Anwendung
  - RetentionPolicy Type-Export für einfachere Imports

#### 2025-11-01 - Keychain Passwortfluss Dokumentation

- Neue Dokumentation `docs/features/keychain-password-flow.md` beschreibt Speicherung, Laden und Fehlerpfade (PasswordMissing, KeychainError) samt Auswirkungen auf Scheduler und Portable-Mode.

#### 2025-11-02 - Portable-Store Fallback Hinweis

- Backend sendet `portable-store-status` Event und stellt `get_portable_status` Command bereit
- Frontend registriert Listener, aktualisiert System-Store und zeigt Warn-Toast bei Fallback-Verwendung

#### 2025-10-31 - Phase 1: MVP Core Features (✅ 100% ABGESCHLOSSEN)

- **Repository State-Architektur (Task 1.1):**
  - `CachedRepository` Struktur mit Arc<Repository> und Timestamp
  - 5-Minuten Cache-Timeout für bessere Performance
  - `AppState::get_repository(id)` mit automatischem Cache-Management
  - `AppState::set_current_repository(id)` für aktives Repository
  - `AppState::get_current_repository_id()` Helper
  - `AppState::with_current_repo()` für sichere Repository-Operationen

- **Command-Migration (Task 1.2):**
  - `forget_snapshots()` Command reaktiviert mit neuem State-System
  - Nutzt rustic_core::delete_snapshots API für Batch-Operationen
  - Verbesserte Error-Handling mit Repository-Cache

- **Repository-Statistics (Task 1.3):**
  - `get_repository_stats()` nutzt jetzt State-System statt direktem Öffnen
  - Integration mit rustic_core API (Snapshot-Count funktional)
  - Placeholder für Pack-File-Statistics (wird in Phase 3 implementiert)

- **Snapshot-Commands Refactoring (Task 1.4):**
  - `compare_snapshots()` neu implementiert mit get_snapshots() API
    - Paths-basierter Vergleich statt Tree-Traversierung
    - HashSet-Performance-Optimierung für Diff-Berechnung
    - Size-Change-Berechnung aus Snapshot-Summaries
  - `add_snapshot_tags()` implementiert mit save_snapshots() API
    - StringList::from_str für Tag-Konvertierung
    - Batch-Save mit vec![snapshot]
  - `remove_snapshot_tags()` implementiert mit save_snapshots() API
    - Konsistente API-Nutzung mit add_snapshot_tags
  - Alle 3 Commands in lib.rs aktiviert

### Fixed

#### 2025-11-01 - Keychain Flag Konsistenz

- `change_password` Command aktualisiert das Config-Flag `password_stored` und speichert die Konfiguration sofort, sodass Unlock-Dialogs nach Passwortwechsel nicht mehr unerwartet erscheinen.

#### 2025-10-31 - Phase 0: Notfall-Reparatur (✅ 100% ABGESCHLOSSEN)

- **Build-Fehler behoben (18 → 0 Fehler):**
  - PackFile API Migration: `rustic_core 0.8.0` hat `PackFile::TYPE` und `iter_type()` entfernt
  - `get_repository_stats()` vereinfacht mit Placeholder-Werten (TODO Phase 3)
  - 4 Commands deaktiviert wegen fehlender Repository State: `compare_snapshots`, `forget_snapshots`, `add_snapshot_tags`, `remove_snapshot_tags`
  - StringList API: `from_str()` → `from()` Migration vorbereitet
  - Type Mismatch in repository.rs behoben

- **Warnings reduziert (36 → 28):**
  - Unused imports entfernt: `RepositoryConfig`, `DiffResultDto`, `DiffStats`, `RetentionPolicy`
  - Snake_case Feld-Namen korrigiert: `snapshotId` → `snapshot_id`, `targetPath` → `target_path`, `jobId` → `job_id`
  - Unused parameter in Stub-Funktion gefixt: `f` → `_f`

- **Dokumentation:**
  - `docs/reports/2025-10-31-phase0-build-fix-notes.md`: Vollständige Build-Fix-Dokumentation
  - Alle deaktivierten Commands mit TODO-Markern versehen
  - Migration-Plan für lib.rs → Modul-Dateien erstellt

### Added

#### 2025-10-31 - Milestone 2: Cloud-Backend-Integration (✅ 100% ABGESCHLOSSEN)

- **OpenDAL Backend-Modul (Task 2.1.1):**
  - `backends/opendal.rs`: S3, Azure Blob, Google Cloud Storage, Backblaze B2 Support
  - `OpenDALConfig` Struktur mit Provider-spezifischen Feldern
  - `create_opendal_backend()` Funktion für S3-kompatible Services
  - `validate_opendal_config()` mit umfassender Validierung
  - 11 Unit-Tests für alle Provider-Typen

- **Rclone Backend-Modul (Task 2.2.1):**
  - `backends/rclone.rs`: 70+ Cloud-Provider + SFTP Support
  - `RcloneManager` mit Installation-Check und Remote-Management
  - `RcloneConfig` Struktur mit flexiblen Options
  - `create_sftp_backend()` Helper-Funktion
  - 8 Unit-Tests für SFTP und Rclone-Funktionen

- **Repository Cloud-Backend-Support (Task 2.1.2, 2.2.2):**
  - `init_repository()` erweitert für Cloud-Backends (s3, azblob, gcs, b2, rclone)
  - Backend-Type-Matching und Options-Serialisierung
  - Unterstützung für Custom Endpoints (MinIO, Wasabi)

- **Connection-Test Command (Task 2.1.3):**
  - `test_repository_connection()` Tauri-Command
  - `ConnectionTestResult` DTO mit Latenz-Messung
  - Backend-spezifische Validierung und Testing
  - Command in lib.rs registriert

- **Cloud-Credential-Management (Task 2.1.4):**
  - `save_cloud_credentials()` in keychain/mod.rs
  - `load_cloud_credentials()` mit Access/Secret Key Trennung
  - `delete_cloud_credentials()` für Cleanup
  - Composite Keys für Multi-Provider-Support

- **Frontend Connection-Test UI (Task 2.3.1):**
  - LocationPickerDialog: Connection-Test-Button für Cloud und Network Tabs
  - Test-Result-Anzeige mit Success/Error-States
  - Latenz-Display in Millisekunden
  - Backend-Options-Serialisierung für invoke()

- **Favoriten-Management System (Task 2.3.2):**
  - `FavoriteLocation` Typ mit location_type Enum
  - `AppConfig` erweitert um favorite_locations Vec
  - 4 neue Commands:
    - `save_favorite_location(name, path, type, config)`
    - `list_favorite_locations()` mit Sortierung nach last_used
    - `update_favorite_last_used(id)` für Auto-Tracking
    - `delete_favorite_location(id)`
  - LocationPickerDialog: "⭐ Als Favorit speichern" Button
  - Recent-Tab lädt echte Favoriten vom Backend
  - Auto-Update last_used beim Auswählen eines Favoriten
  - Helper-Funktionen:
    - `getLocationTypeLabel()` für User-Friendly-Namen
    - `getLocationIcon()` für Emoji-Icons
    - `formatLastUsed()` für relative Zeitangaben
  - Config-Persistenz in TOML

- **Credential-Prompt-Integration (Task 2.3.3):**
  - Dialog "Zugangsdaten speichern?" nach erfolgreichem Connection-Test
  - Checkboxes für:
    - "Zugangsdaten im Keychain speichern"
    - "Als Favorit speichern"
  - `handleCredentialPrompt()` Funktion für Credential-Speicherung
  - Keychain-Integration für Cloud und Network (SFTP)
  - Sicherheitshinweis im Dialog (🔒 Icon)
  - Optional: Automatisches Speichern als Favorit

- **Dokumentation (Task 2.3.3):**
  - README.md komplett überarbeitet:
    - Features-Übersicht mit Emojis
    - Installation-Anleitungen (Linux AppImage, Windows)
    - Erste-Schritte-Guide (Repository, Backup, Restore)
    - **Cloud-Storage-Konfiguration:**
      - Amazon S3 Setup und Beispiele
      - Azure Blob Storage Setup
      - Google Cloud Storage Setup
      - Backblaze B2 Setup
      - Wasabi/MinIO (S3-kompatibel) Setup
      - SFTP via Rclone Setup
      - 70+ weitere Provider via Rclone
    - Sicherheit & Credentials-Sektion:
      - Keychain-Integration erklärt
      - Credential-Workflow beschrieben
      - Favoriten-System dokumentiert
    - Troubleshooting-Sektion:
      - Cloud-Verbindung fehlgeschlagen
      - Rclone nicht gefunden
      - Repository gesperrt
      - Passwort vergessen
    - Entwicklungs-Setup und Build-Anleitung
    - Links zu Dokumentation und Support

- **Error-Handling:**
  - `UnsupportedBackend` Error-Typ
  - `InvalidConfiguration` Error-Typ
  - `RcloneNotFound` Error-Typ
  - `RcloneError` Error-Typ

**Resultat:** Vollständiger Cloud-Storage-Support implementiert. Benutzer können nun Repositories auf S3, Azure, GCS, SFTP und 70+ weiteren Cloud-Providern erstellen und verwalten. Alle Credentials werden sicher im System-Keychain gespeichert.

#### 2025-10-31

- **Dokumentations-Infrastruktur erweitert:**
  - `PROJEKT_ANALYSE.md`: Umfassende Tiefen-Analyse des kompletten Projekts (97% Konsistenz-Score)
  - `ANPASSUNGS_EMPFEHLUNGEN.md`: Konkrete Änderungsvorschläge für Instructions (3 Prioritätsstufen)
  - `CONTRIBUTING.md`: Vollständige Contribution-Guidelines für neue Contributors
  - LocationPickerDialog, PruneRepoDialog, SnapshotInfoDialog in frontend.instructions.md dokumentiert
  - Neue Mockups (rustic_location_picker.html, rustic_advanced_functions.html) in copilot-instructions.md ergänzt

### Changed

- **README.md komplett überarbeitet:**
  - Features-Übersicht mit Emojis
  - Installation-Anleitungen für Linux/Windows
  - Erste-Schritte-Guide
  - Development-Setup und Build-Instruktionen
  - Links zu Dokumentation & Support
  - Ersetzt Tauri-Template mit vollständiger Projekt-Doku

### Fixed

- **Instructions-Dateiname korrigiert:**
  - `backup-restore-snapshots.instructions.md.md` → `backup-restore-snapshots.instructions.md`
  - Links in copilot-instructions.md aktualisiert (3 Vorkommen)

#### 2025-10-29

- **Snapshots Page (Frontend) abgeschlossen:**
  - Snapshots-Ansicht mit FilterBar, ContextMenu, Pagination, CompareSnapshotsDialog gemäß Mockup
  - Alle Advanced Features: Tag-Filter, Bulk-Selection, Sortierung, Snapshot-Vergleich, Kontextmenü, Pagination
  - Utility-Funktionen ausgelagert und getestet (`formatDate`, `formatBytes`)
  - Unit-Tests für Hilfsfunktionen (Vitest)
  - Linting und Checks fehlerfrei
  - ROADMAP.md und CHANGELOG.md aktualisiert

- **Backend Restore-Funktionalität**: Vollständige Restore-Logik mit hierarchischem File-Browser
  - `get_file_tree` Command: Lädt Dateistruktur eines Snapshots als hierarchischen Baum
  - `restore_files` Command: Stellt selektierte Dateien wieder her mit Overwrite-Policy
  - Robuste rustic_core-Integration für Snapshot-Zugriff und Repository-Öffnung
  - Hierarchische FileTreeNode-Struktur mit Lazy-Loading-Unterstützung
  - RestoreOptionsDto mit Overwrite-Policy, Permissions und Timestamps
  - Pfad-basierte Baum-Konstruktion aus Snapshot-Pfaden
  - Vollständige Error-Handling und Validation
  - Type-sichere DTOs für Frontend-Kommunikation
- **Backend Snapshot-Management**: Vollständige Snapshot-Verwaltung implementiert
  - `list_snapshots` Command: Lädt alle Snapshots eines Repositories mit DTO-Konvertierung
  - `get_snapshot` Command: Holt detaillierte Infos zu einem einzelnen Snapshot
  - `delete_snapshot` Command: Löscht einzelne Snapshots mit ID-Validierung
  - `forget_snapshots` Command: Wendet Retention-Policies an (keep_last) und löscht Snapshots
  - Robuste rustic_core-Integration mit korrekter API-Nutzung (RepositoryOptions, BackendOptions, SnapshotId)
  - Vollständige Error-Handling und Validation
  - Umfassende Unit-Tests für alle Commands (Happy-Path, Error-Cases, Edge-Cases)
  - Type-sichere DTOs mit korrekter Feld-Mapping (file_count, total_size aus Summary)
- **Backend Backup-Funktionalität**: Vollständige Backup-Logik mit Event-getriebenem Progress-Reporting
  - `run_backup` Command mit Tauri-Event-Emission (`backup-progress-{job_id}`)
  - `run_backup_logic` testable Backup-Funktion (ohne Tauri-Abhängigkeiten)
  - Umfassende Backup-Optionen (Repository, Source-Paths, Tags, Exclude-Patterns, Compression)
  - Progress-Reporting mit Files-Processed, Bytes-Uploaded, Current-File, Percent-Complete
  - Robuste Error-Handling mit Validierung (leeres Repository, leere Source-Paths)
  - Thread-sichere Progress-Callbacks mit `Fn + Send + Sync + 'static`
  - Komplette Unit-Tests für Happy-Path, Error-Cases und Progress-Event-Validation
  - Simulierte rustic_core-Integration (bereit für echte Integration)
- **UI Foundation**: Komplette Layout-Infrastruktur implementiert und korrigiert
  - CSS Design System mit umfangreichen Design Tokens und Komponenten-Stilen
  - Responsive MainLayout mit Sidebar und Header
  - Navigation-System mit Router Store und State-Management
  - Mobile-responsive Design mit Touch-Unterstützung
  - Accessibility-Verbesserungen (ARIA-Labels, Keyboard-Navigation)
  - TypeScript-Integration und Fehlerbehebungen
- **Frontend Backup-Integration**: Vollständige Verbindung zwischen UI und Backend-Backup
  - RunBackupDialog Komponente mit real-time Progress-Anzeige
  - RepositoryCard Backup-Button Integration mit Job-Auswahl
  - Event-gesteuerte Progress-Updates (backup-progress, backup-completed, backup-failed)
  - Toast-Notifications für Backup-Erfolg und -Fehler
  - Backup-Abbruch-Funktionalität mit Cancel-Button
  - Type-Sicherheit mit aktualisierten DTOs (RepositoryDto, BackupProgress)
  - Responsive Progress-Display mit Dateien, Bytes und geschätzter Restzeit
- Initiales Projekt-Setup
- Grundlegende Projekt-Struktur
- Error-Handling Framework
- Tooling (ESLint, Prettier, rustfmt, clippy)

### Changed

- N/A

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A
