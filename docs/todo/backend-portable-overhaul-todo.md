# Backend Portable Overhaul – ToDo (Stand 2025-11-01)

- [x] Repository-State und Cache in `src-tauri/src/commands/repository.rs` und `src-tauri/src/state.rs` korrigieren
- [x] Verschlüsselten Portable-Store neben der Binary implementieren (`src-tauri/src/config.rs`, `src-tauri/src/keychain/`, State-Initialisierung)
- [x] (Subtask) Schlüsselableitung und Salt-Management festlegen (AES-GCM oder ChaCha20-Poly1305 evaluieren)
- [x] (Subtask) Schreib-/Leseweg im PortableStore verschlüsseln und Roundtrip-Tests ergänzen (`src-tauri/src/storage/portable.rs`, `src-tauri/tests`)
- [ ] (Subtask) Keychain/Passwortfluss dokumentieren und Fehlerfälle behandeln (Passwortwechsel, fehlender Key)
- [ ] Fallback bei schreibgeschützter Umgebung inkl. UI-Hinweis einbauen
- [ ] (Subtask) Status-Events und Logs im Backend ergänzen (`src-tauri/src/commands/repository.rs`, `AppState::portable_status`)
- [ ] (Subtask) Frontend-Store/-Toast vorbereiten, kompatibel zu Svelte-5-State-APIs (`src/lib/stores/repositories.ts`)
- [ ] (Subtask) Mockup-konforme Hinweis-Komponente erstellen (Modal/Toast) und dokumentieren
- [ ] JSON-Import/Export-Assistent (Tauri-Commands + Frontend-Dialog) ergänzen
- [ ] Backup-Flow mit Passwort-Handling und Scheduler-Lock (`cron::Schedule`, Multi-Instanz) verknüpfen
- [ ] Restore-Flow sowie Dateibaum an `rustic_core` anbinden (Progress, Lazy Loading)
- [ ] Retention-Logik vervollständigen und Frontend-Filter anpassen
- [ ] Tests, Live-System-Doku und Portable-Mode-Anleitung aktualisieren (`tests/*`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `docs/todo/backend-audit-2025-11-01.md`)
- [ ] Svelte-5-State-Migration für Stores vorbereiten (Signals/Runes Roadmap, Übergangsschritte dokumentieren)
- [ ] Hinweis/Update-Flow für bestehende portable Installationen ergänzen (UI + Docs)
