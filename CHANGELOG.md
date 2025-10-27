# Changelog

Alle wichtigen Änderungen an Rustic GUI werden hier dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
und dieses Projekt folgt [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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
