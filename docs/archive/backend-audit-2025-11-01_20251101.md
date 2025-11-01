# Backend-Audit (Stand 2025-11-01)

## Kritische Punkte

- `src-tauri/src/commands/repository.rs::switch_repository` setzt den aktiven Repository-State nicht über `AppState::set_current_repository` und cached das geöffnete Repository nicht. Nach einem Wechsel bleiben `get_current_repository_id()` sowie `has_current_repo()` leer, wodurch Snapshot-, Restore- und Scheduler-Kommandos unmittelbar fehlschlagen.
- `src-tauri/src/rustic/backup.rs::run_backup_logic` öffnet Repositories ohne Passwort (blanke `RepositoryOptions::default()`). Jeder reale Backup-Lauf endet dadurch mit `AuthenticationFailed`, weil das Passwort-Handling via AppState/Keychain fehlt.
- `src-tauri/src/commands/backup.rs::{create_backup_job, update_backup_job}` validieren Quellpfade ausschließlich mit `PathBuf::exists()`. Remote-Backends (SFTP/S3/rclone) lassen sich so nicht konfigurieren, obwohl Multi-Backend-Support laut Instructions Pflicht ist.
- `src-tauri/src/commands/backup.rs::schedule_backup` simuliert geplante Jobs nur per `tokio::sleep` und ruft kein reales `run_backup_command`. Geplante Backups werden somit nie ausgeführt.
- `src-tauri/src/commands/restore.rs` und `src-tauri/src/rustic/restore.rs` schreiben beim Restore Platzhalter-Dateien und ignorieren `rustic_core`. Metadaten, Progress und echte Datenwiederherstellung fehlen → Risiko von Datenverlust.
- Die Tests in `src-tauri/src/config.rs` setzen `RUSTIC_GUI_CONFIG_PATH`, aber `AppConfig::config_path()` ignoriert die Variable. Die Tests überschreiben dadurch reale Nutzerkonfigurationen unter `~/.config/rustic-gui/config.toml`, was gegen die Sicherheitsvorgaben verstößt.

## Hohe Priorität

- `src-tauri/src/commands/backup.rs::validate_cron_expression` prüft Cron-Ausdrücke nur auf die Feldanzahl. Gefordert ist `cron::Schedule::from_str`, sonst fallen ungültige Cron-Expressions erst zur Laufzeit auf.
- Kommentare und Docstrings in mehreren Dateien (z. B. `src-tauri/src/commands/snapshot.rs`, `src-tauri/src/commands/mod.rs`, Tests) sind englisch. Die Code-Style-Instructions verlangen durchgängig deutsche Kommentare/Dokumentation.

## Mittlere Priorität

- `src-tauri/src/rustic/restore.rs` rekonstruiert den Dateibaum aus `snapshot.paths` ohne Lazy-Loading und ohne echte Metadaten (Größe, Rechte). Vorgabe ist `node_from_snapshot` inkl. vollständiger Metadaten, um doppelte Knoten und unvollständige Infos zu vermeiden.
- `src-tauri/src/rustic/snapshot.rs::forget_snapshots` berücksichtigt ausschließlich `keep_last`. Die übrigen Retention-Policy-Felder bleiben ungenutzt, obwohl sie laut Instructions zwingend umzusetzen sind.

## Offene Fragen

- Wie sollen Tests zukünftig mit der Konfigurationsdatei umgehen? Brauchen wir einen konfigurierbaren Pfad (z. B. via ENV) oder ein vollständiges Mocking?
- Existiert bereits ein Konzept für die reale Einbindung von `restore_files`/`run_backup` in die `rustic_core`-Callbacks inklusive Progress und Passwort-Handling, oder muss dies noch erarbeitet werden?

## Empfohlene nächste Schritte

- State-Management in `switch_repository` korrigieren und Repository-Caching aktivieren.
- Passwort- und Backend-Handling im Backup-Flow gemäß Instructions implementieren.
- Pfadvalidierung backend-spezifisch ausgestalten, damit Remote-Backups funktionieren.
- Scheduler mit echtem Backup-Aufruf verknüpfen und Cron-Validierung über `cron::Schedule` umsetzen.
- Restore- und Retention-Logik vollständig anhand der `rustic_core`-API implementieren.
- Konfigurationspfad testbar machen und sämtliche Kommentare/Docs ins Deutsche übertragen.
