# Milestone 1: rustic_core Integration 🔴

> **Detaillierte Task-Liste für rustic_core/rustic_backend Integration**

**Dauer:** 60h (1.5 Wochen) | **Status:** 0% - BLOCKING v1.0  
**Priorität:** 🔴 HIGHEST  
**Dependencies:** M0 (abgeschlossen)

---

## Übersicht

Derzeit sind **fast alle Backend-Commands Stubs** die Mock-Daten zurückgeben. M1 ersetzt alle Stubs durch echte rustic_core/rustic_backend Integration.

**Problem:** UI funktioniert perfekt, aber Backend macht nichts "echtes".

**Betroffene Commands:** 44 TODOs in:

- `src-tauri/src/rustic/repository.rs` - init, open, check, prune, change_password
- `src-tauri/src/commands/backup.rs` - run_backup mit Progress
- `src-tauri/src/commands/restore.rs` - restore_files, list_snapshot_files
- `src-tauri/src/commands/snapshot.rs` - list, get, delete, forget, tags

**Ziel:** Alle rustic_core API-Calls implementiert, keine Stubs mehr.

---

## 1.1 Repository-Initialisierung & Öffnen

**Dauer:** 15h | **Priorität:** 🔴 HIGHEST

### Tasks

#### Task 1.1.1: `init_repository()` mit rustic_core (8h)

**Beschreibung:** Echte Repository-Initialisierung statt Mock.

**Schritte:**

1. [ ] `Repository::new()` mit `RepositoryOptions` aufrufen
2. [ ] Backend aus `RepositoryType` enum ermitteln:
   - `Local` → `rustic_backend::LocalBackend`
   - `S3` → siehe M2 (OpenDAL)
   - `Rclone` → siehe M2 (RcloneBackend)
3. [ ] Für Local-Backend:
   - `.rustic/` Verzeichnisstruktur erstellen
   - Config-Datei schreiben
   - Keys generieren
4. [ ] Password via `KeyringManager::set_password()` in Keychain speichern
5. [ ] Config-Eintrag für neues Repo in `AppConfig` erstellen
6. [ ] Error-Handling implementieren:
   - Path already exists
   - Invalid backend config
   - Keychain-Fehler
7. [ ] Unit-Test schreiben (temp directory)

**Datei:** `src-tauri/src/rustic/repository.rs:19-54`  
**Aktuell:** Mock-Implementierung (TODO auf Zeile 31)  
**Referenz:**

- `docs/rustic/rustic_core_api.md` - `Repository::new()`
- `docs/rustic/rustic_backend_api.md` - `BackendOptions::to_backends()`

**Erfolgs-Kriterium:**

```bash
# Integration-Test
cargo test test_init_local_repository -- --ignored
# Ergebnis: .rustic/ Ordner existiert, Repo kann geöffnet werden
```

---

#### Task 1.1.2: `open_repository()` mit rustic_core (7h)

**Beschreibung:** Repository mit Passwort öffnen, Index laden.

**Schritte:**

1. [ ] Password aus Keychain laden:
   ```rust
   let keyring = KeyringManager::new()?;
   let password = keyring.get_password(&repo_id)?;
   ```
2. [ ] `Repository::open_with_password()` aufrufen
3. [ ] Index-Dateien laden und validieren
4. [ ] Repository-Handle in `AppState.current_repo` speichern:
   ```rust
   let mut current_repo = state.current_repo.lock().unwrap();
   *current_repo = Some(repo);
   ```
5. [ ] Error-Handling:
   - Wrong password → benutzerfreundliche Meldung
   - Corrupted repository → Hinweis auf `check_repository()`
   - Missing files → Repo neu klonen/initialisieren
6. [ ] Unit-Test mit verschiedenen Error-Szenarien

**Datei:** `src-tauri/src/rustic/repository.rs:56-88`  
**Aktuell:** Mock mit simulated delay (TODO auf Zeile 68)  
**Referenz:** `docs/rustic/rustic_core_api.md` - `Repository::open_with_password()`

**Erfolgs-Kriterium:**

```bash
# Test mit falschem Passwort
cargo test test_open_repository_wrong_password
# Ergebnis: Error "Invalid password"

# Test mit korrektem Passwort
cargo test test_open_repository_success
# Ergebnis: Repository in AppState geladen
```

---

### Abhängigkeiten

- ✅ Keychain-System (bereits implementiert in M0)
- ✅ Config-System (bereits implementiert in M0)
- ✅ Error-Types (bereits definiert in M0)

### Risiken

| Risiko                                       | Mitigation                                                            |
| -------------------------------------------- | --------------------------------------------------------------------- |
| Keychain auf Linux unterschiedlich je Distro | Fallback auf manuelle Eingabe, Unit-Tests für jeden Keychain-Provider |
| Repository-Init schlägt fehl (Permissions)   | Bessere Error-Messages mit Lösungsvorschlägen                         |

---

## 1.2 Backup-Execution mit Progress

**Dauer:** 12h | **Priorität:** 🔴 HIGHEST

### Tasks

#### Task 1.2.1: `run_backup()` mit rustic_core (10h)

**Beschreibung:** Echtes Backup mit Progress-Callbacks.

**Schritte:**

1. [ ] `BackupOptions` aus `BackupJobConfig` bauen:
   ```rust
   let opts = BackupOptions::default()
       .with_sources(config.source_paths.clone())
       .with_excludes(config.exclude_patterns.clone())
       .with_tags(config.tags.clone());
   ```
2. [ ] **Progress-Callback implementieren:**

   ```rust
   let app_handle_clone = app_handle.clone();
   let job_id_clone = job_id.clone();

   let progress_fn = move |info: BackupProgress| {
       app_handle_clone.emit_all(
           &format!("backup-progress-{}", job_id_clone),
           json!({
               "files_done": info.files_done,
               "files_total": info.files_total,
               "bytes_done": info.bytes_done,
               "bytes_total": info.bytes_total,
               "current_file": info.current_file,
           })
       ).ok();
   };
   ```

3. [ ] `Repository.backup()` mit Options und Callback aufrufen
4. [ ] `CancellationToken` aus `AppState` nutzen:

   ```rust
   let token = CancellationToken::new();
   state.cancellation_tokens.lock().unwrap()
       .insert(job_id.clone(), token.clone());

   tokio::select! {
       result = repo.backup(&opts, Some(Box::new(progress_fn))) => result,
       _ = token.cancelled() => Err("Backup abgebrochen".into())
   }
   ```

5. [ ] Snapshot-ID bei Erfolg zurückgeben
6. [ ] Error-Recovery implementieren:
   - Netzwerk-Timeout → Retry-Logic
   - Disk voll → Cleanup + Error
   - Access denied → Permissions-Check
7. [ ] Integration-Test mit echten Dateien

**Datei:** `src-tauri/src/commands/backup.rs:14-81`  
**Aktuell:** Simulierter Progress (TODO auf Zeile 30)  
**Referenz:** `.github/instructions/backup-restore-snapshots.instructions.md` - Backup-Implementation

**Erfolgs-Kriterium:**

```bash
# Integration-Test mit 100 Dateien
cargo test test_backup_with_progress -- --ignored
# Ergebnis:
# - Backup läuft
# - Progress-Events kommen an (0% → 100%)
# - Snapshot wurde erstellt
# - Snapshot in Repository sichtbar
```

---

#### Task 1.2.2: `cancel_backup()` implementieren (2h)

**Beschreibung:** Laufendes Backup gracefully abbrechen.

**Schritte:**

1. [ ] `CancellationToken` aus `AppState.cancellation_tokens` holen
2. [ ] Token canceln → rustic_core stoppt Backup
3. [ ] Cleanup:
   - Temporäre Dateien löschen
   - Inkomplette Snapshots entfernen
4. [ ] Event senden: `backup-cancelled-{job_id}`
5. [ ] Token aus HashMap entfernen

**Datei:** `src-tauri/src/commands/backup.rs:83-98`  
**Aktuell:** Stub (keine Cancellation)

**Erfolgs-Kriterium:**

```bash
# Test: Backup starten, nach 2s abbrechen
cargo test test_cancel_backup
# Ergebnis: Backup stoppt, keine korrupten Daten
```

---

### Test-Szenarien

- [ ] Backup von 100 Dateien (~10 MB)
- [ ] Backup von 1000 Dateien (~100 MB)
- [ ] Backup während laufendem anderen Backup (sollte queued werden oder fehlschlagen)
- [ ] Backup abbrechen nach 50% Progress
- [ ] Backup mit ungültigen Source-Paths
- [ ] Backup auf read-only Repository

---

## 1.3 Restore-Funktionalität

**Dauer:** 10h | **Priorität:** 🔴 HIGHEST

### Tasks

#### Task 1.3.1: `restore_files()` mit rustic_core (6h)

**Beschreibung:** Dateien aus Snapshot wiederherstellen.

**Schritte:**

1. [ ] `RestoreOptions` aus Request-Params erstellen:

   ```rust
   let mut opts = RestoreOptions::default();

   if !files.is_empty() {
       opts = opts.with_files(files); // Selektive Restore
   }
   ```

2. [ ] `LocalDestination` für Ziel-Pfad einrichten:
   ```rust
   let dest = LocalDestination::new(target_path)?;
   ```
3. [ ] **Progress-Callback:**
   ```rust
   let progress_fn = move |info: RestoreProgress| {
       app_handle.emit_all("restore-progress", json!({
           "files_done": info.files_done,
           "files_total": info.files_total,
           "bytes_done": info.bytes_done,
           "bytes_total": info.bytes_total,
       })).ok();
   };
   ```
4. [ ] `Repository.restore()` aufrufen
5. [ ] Overwrite-Behavior konfigurierbar:
   - `skip` - Existierende überspringen
   - `overwrite` - Existierende ersetzen
   - `rename` - Mit Suffix versehen (.restored.1)
6. [ ] Permissions/Timestamps wiederherstellen
7. [ ] Integration-Test

**Datei:** `src-tauri/src/commands/restore.rs:14-65`  
**Aktuell:** Simuliert (TODO auf Zeile 31)  
**Referenz:** `.github/instructions/backup-restore-snapshots.instructions.md` - Restore-Implementation

**Erfolgs-Kriterium:**

```bash
# Test: 10 Dateien wiederherstellen
cargo test test_restore_selective_files
# Ergebnis: Dateien korrekt wiederhergestellt mit original Permissions
```

---

#### Task 1.3.2: `list_snapshot_files()` File-Tree laden (4h)

**Beschreibung:** File-Browser mit echten Snapshot-Inhalten.

**Schritte:**

1. [ ] `Repository.node_from_snapshot()` für Snapshot aufrufen:
   ```rust
   let tree = repo.node_from_snapshot(&snapshot)?;
   ```
2. [ ] Tree-Navigation (Pfad-basiert für Lazy-Loading):
   ```rust
   let nodes = if let Some(p) = path {
       repo.get_node_at_path(&tree, &p)?.nodes
   } else {
       tree.nodes // Root-Level
   };
   ```
3. [ ] `FileTreeNode` DTOs erstellen:
   ```rust
   FileTreeNode {
       name: node.name().to_string(),
       path: node.path().to_string(),
       is_dir: matches!(node.node_type(), NodeType::Dir),
       size: node.meta().size,
       modified: node.meta().mtime.to_rfc3339(),
       permissions: format!("{:o}", node.meta().mode),
   }
   ```
4. [ ] Sortierung (Ordner zuerst, dann alphabetisch)
5. [ ] Lazy-Loading für große Verzeichnisse (>500 Dateien)

**Datei:** `src-tauri/src/commands/restore.rs:67-97`  
**Aktuell:** Mock-Daten (TODO auf Zeile 78)  
**Referenz:** `.github/instructions/backup-restore-snapshots.instructions.md` - File-Tree

**Erfolgs-Kriterium:**

```bash
# Test: File-Tree für Snapshot laden
cargo test test_list_snapshot_files
# Ergebnis: Echte Dateinamen/Pfade, korrekte is_dir Flags
```

---

### Test-Szenarien

- [ ] Einzelne Datei wiederherstellen
- [ ] Gesamten Ordner wiederherstellen (100+ Dateien)
- [ ] 100 Dateien selektiv wiederherstellen
- [ ] Restore in existierendes Verzeichnis (Overwrite-Test)
- [ ] Restore mit Permissions-Check (Symlinks, Executable-Bit)
- [ ] File-Tree für großes Verzeichnis (>1000 Dateien)

---

## 1.4 Snapshot-Management

**Dauer:** 12h | **Priorität:** 🟠 HIGH

### Tasks

#### Task 1.4.1: `list_snapshots()` mit rustic_core (3h)

**Schritte:**

1. [ ] `Repository.get_all_snapshots()` aufrufen
2. [ ] **Filtering implementieren:**
   ```rust
   snapshots.into_iter().filter(|snap| {
       // Hostname-Filter
       if let Some(ref hostname) = filter.hostname {
           if &snap.hostname != hostname { return false; }
       }

       // Tag-Filter (OR-Logic)
       if let Some(ref tags) = filter.tags {
           if !tags.iter().any(|t| snap.tags.contains(t)) { return false; }
       }

       // Datums-Range
       if let Some(ref from) = filter.date_from {
           if snap.time < from.parse().unwrap() { return false; }
       }

       true
   }).collect()
   ```
3. [ ] Nach Datum sortieren (neueste zuerst)
4. [ ] In `SnapshotDto` konvertieren

**Datei:** `src-tauri/src/commands/snapshot.rs:14-51`  
**Aktuell:** Mock-Daten (TODO auf Zeile 23)  
**Referenz:** `docs/rustic/rustic_core_api.md` - `Repository::get_all_snapshots()`

---

#### Task 1.4.2: `get_snapshot()` Details laden (2h)

**Schritte:**

1. [ ] `Repository.get_snapshots()` mit spezifischer ID
2. [ ] Snapshot-Summary laden (Dateien, Größe, Dauer)
3. [ ] Error-Handling (Snapshot nicht gefunden)

**Datei:** `src-tauri/src/commands/snapshot.rs:53-68`

---

#### Task 1.4.3: `delete_snapshot()` implementieren (3h)

**Schritte:**

1. [ ] `Repository.forget_snapshot()` aufrufen
2. [ ] **Optional:** `Repository.prune()` danach (User-Wahl im Frontend)
3. [ ] Confirmation im Frontend (bereits vorhanden)
4. [ ] Event senden nach Löschung

**Datei:** `src-tauri/src/commands/snapshot.rs:70-91`

---

#### Task 1.4.4: `forget_snapshots()` Batch-Delete (2h)

**Schritte:**

1. [ ] Mehrere Snapshots auf einmal löschen
2. [ ] Progress-Reporting für jede Löschung
3. [ ] Prune nur einmal am Ende (nicht pro Snapshot)

**Datei:** `src-tauri/src/commands/snapshot.rs:93-120`

---

#### Task 1.4.5: Snapshot-Tag-Management (2h)

**Schritte:**

1. [ ] `add_snapshot_tags()`:
   ```rust
   let mut snapshot = repo.get_snapshot(&id)?;
   snapshot.add_tags(vec![tag.into()]);
   repo.save_snapshot(&snapshot)?;
   ```
2. [ ] `remove_snapshot_tags()`:
   ```rust
   snapshot.remove_tags(&[tag.as_str()]);
   ```
3. [ ] Tags in Config speichern für Persistence

**Neue Funktionen** (noch nicht im Code vorhanden)  
**Referenz:** `docs/rustic/rustic_core_api.md` - `SnapshotFile::add_tags()`

---

### Test-Szenarien

- [ ] Liste mit 100 Snapshots filtern (Hostname)
- [ ] Liste mit 100 Snapshots filtern (Tags)
- [ ] Snapshot löschen (mit Prune)
- [ ] 10 Snapshots batch-löschen
- [ ] Tags zu Snapshot hinzufügen/entfernen

---

## 1.5 Repository-Wartung

**Dauer:** 11h | **Priorität:** 🟡 MEDIUM

### Tasks

#### Task 1.5.1: `check_repository()` implementieren (6h)

**Schritte:**

1. [ ] `Repository.check()` mit Validierungs-Optionen aufrufen
2. [ ] **Progress-Events senden:**
   ```rust
   app_handle.emit_all("check-progress", json!({
       "pack_files_checked": info.packs_done,
       "pack_files_total": info.packs_total,
       "errors_found": info.errors.len(),
   })).ok();
   ```
3. [ ] Check-Optionen konfigurierbar:
   - `quick` - Nur Index prüfen
   - `full` - Pack-Files validieren
4. [ ] Error-Report erstellen (strukturiert):
   ```rust
   CheckResult {
       errors: vec!["Pack XYZ corrupted", ...],
       warnings: vec!["Orphaned data found", ...],
       statistics: CheckStatistics { ... },
   }
   ```

**Datei:** `src-tauri/src/commands/repository.rs:84-122`  
**Aktuell:** Mock (TODO auf Zeile 92)

---

#### Task 1.5.2: `prune_repository()` implementieren (3h)

**Schritte:**

1. [ ] `Repository.prune()` aufrufen
2. [ ] **Dry-Run-Modus** (zeige was gelöscht würde):
   ```rust
   let opts = PruneOptions::default().dry_run(true);
   let stats = repo.prune(&opts)?;
   ```
3. [ ] Statistics zurückgeben:
   ```rust
   PruneResult {
       freed_space_bytes: stats.data_removed,
       removed_pack_files: stats.packs_removed,
       duration_secs: stats.duration,
   }
   ```
4. [ ] Progress-Events

**Datei:** `src-tauri/src/commands/repository.rs:124-149`

---

#### Task 1.5.3: `change_password()` implementieren (2h)

**Schritte:**

1. [ ] Altes Passwort validieren (Repository öffnen)
2. [ ] Neues Passwort setzen (rustic_core API)
3. [ ] Keychain aktualisieren:
   ```rust
   keyring.delete_password(&repo_id)?;
   keyring.set_password(&repo_id, &new_password)?;
   ```
4. [ ] Repository neu öffnen mit neuem Passwort

**Datei:** `src-tauri/src/commands/repository.rs:151-172`

---

### Test-Szenarien

- [ ] Repository-Check findet korrupte Pack-Files
- [ ] Prune Dry-Run zeigt korrekte Statistics
- [ ] Prune entfernt unused data (Größe vorher/nachher vergleichen)
- [ ] Passwort ändern, dann Repository öffnen

---

## Zusammenfassung & Deliverables

### Gesamt-Dauer

**60 Stunden (1.5 Wochen Vollzeit)**

### Kritischer Pfad

✅ Ja - M1 blockt alle anderen Milestones

### Deliverables Checklist

- [ ] **44 Backend-TODOs abgeschlossen** (aus `TODO.md`)
- [ ] **Alle rustic_core API-Calls implementiert** (keine Stubs mehr)
- [ ] **Progress-Callbacks funktionieren** (Backup, Restore, Check, Prune)
- [ ] **Error-Handling robust** (min. 10 verschiedene Error-Szenarien getestet)
- [ ] **Unit-Tests geschrieben** (min. 60% Coverage für `src-tauri/src/commands/` und `src-tauri/src/rustic/`)

### Akzeptanz-Kriterien

| Kriterium                                 | Test                              | Status |
| ----------------------------------------- | --------------------------------- | ------ |
| Repository kann initialisiert werden      | `cargo test test_init_repository` | [ ]    |
| Repository kann geöffnet werden           | `cargo test test_open_repository` | [ ]    |
| Backup läuft mit echten Dateien           | Integration-Test mit 100 Dateien  | [ ]    |
| Restore stellt Dateien korrekt wieder her | Permissions + Timestamps prüfen   | [ ]    |
| Snapshots werden korrekt aufgelistet      | Liste mit 100 Snapshots           | [ ]    |
| Check findet Fehler                       | Korruptes Repo testen             | [ ]    |
| Prune entfernt Daten                      | Größe vorher/nachher              | [ ]    |

### Risiken & Mitigation

| Risiko                      | Wahrscheinlichkeit | Impact     | Mitigation                                     |
| --------------------------- | ------------------ | ---------- | ---------------------------------------------- |
| rustic_core API komplex     | 🟡 Mittel          | 🟠 Mittel  | API-Docs studieren, Community fragen (Discord) |
| Performance-Probleme        | 🟡 Mittel          | 🟡 Mittel  | Profiling, Optimierung in M4                   |
| Fehlerhafte Error-Recovery  | 🟠 Mittel          | 🟠 Mittel  | Umfangreiche Tests, Edge-Cases dokumentieren   |
| Keychain-Integration fragil | 🟡 Mittel          | 🟢 Niedrig | Fallback vorhanden (manuelle Eingabe)          |

### Nächste Schritte nach M1

1. **M2: Cloud-Backends** - OpenDAL/Rclone Integration
2. **M3: Job-Scheduler** - Automatisierte Backups
3. **M5: Testing** - Parallel zu M2-M4 entwickeln

---

**[← Zurück zur Roadmap](../../ROADMAP.md)** | **[Weiter zu M2: Cloud-Backends →](M2-cloud-backends.md)**
