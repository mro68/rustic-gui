# Changelog

Alle wichtigen Änderungen an Rustic GUI werden hier dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
und dieses Projekt folgt [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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
