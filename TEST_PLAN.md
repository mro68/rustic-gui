# Test-Plan: Backup & Restore Features

> Erstellt: 2025-11-02  
> Status: Ready for testing ✅

## 🎯 Zu testende Features

### ✅ 1. .gitignore & Exclude Patterns

**Backend:** `src-tauri/src/rustic/backup.rs` (Zeilen 133-148)

**Was testen:**

1. Erstelle Test-Repository mit `.gitignore`
2. Erstelle Backup-Job mit Quellpfad der `.gitignore` enthält
3. Führe Backup aus
4. Verifiziere: Dateien in `.gitignore` wurden NICHT gesichert
5. Teste Custom Excludes (z.B. `*.log`, `temp/*`)

**Test-Schritte:**

```bash
# 1. Test-Verzeichnis erstellen
mkdir -p /tmp/test-backup-source
cd /tmp/test-backup-source

# 2. .gitignore erstellen
cat > .gitignore <<EOF
node_modules/
*.log
.DS_Store
temp/
EOF

# 3. Test-Dateien erstellen
mkdir -p node_modules temp
echo "test" > file.txt
echo "keep" > important.txt
echo "ignore" > debug.log
echo "temp" > temp/data.txt
echo "module" > node_modules/package.json

# 4. Backup-Job über UI erstellen
# - Quellpfad: /tmp/test-backup-source
# - Keine Custom Excludes (nur .gitignore)
# - Backup ausführen

# 5. Snapshot prüfen
# Erwartung: file.txt, important.txt GESICHERT
# Erwartung: debug.log, temp/, node_modules/ NICHT gesichert
```

**Erfolgskriterien:**

- ✅ `.gitignore`-Regeln werden automatisch angewendet
- ✅ Custom Excludes funktionieren (Test mit `*.tmp` Pattern)
- ✅ Funktioniert auch OHNE Git-Repository (`no_require_git = true`)

---

### ✅ 2. Restore-Dialog

**Backend:** `src-tauri/src/commands/restore.rs`, `src-tauri/src/rustic/restore.rs`  
**Frontend:** `src/lib/components/dialogs/RestoreDialog.svelte`

**Was testen:**

1. Snapshot-Auswahl funktioniert
2. File-Browser lädt Verzeichnisstruktur
3. Datei-Auswahl mit Checkboxes
4. Restore mit Progress-Anzeige
5. Error-Handling (falsches Passwort, ungültiger Pfad)

**Test-Schritte:**

```bash
# 1. Voraussetzung: Repository mit mindestens 1 Snapshot
# 2. In UI: Snapshots-Seite öffnen
# 3. Snapshot auswählen → "Wiederherstellen" Button
# 4. RestoreDialog öffnet sich

# 5. File-Browser testen:
# - Root-Verzeichnis zeigt alle Top-Level-Ordner
# - Klick auf Ordner → Unterordner laden
# - Checkboxes funktionieren

# 6. Ziel-Pfad wählen (z.B. /tmp/restore-test)
# 7. Restore starten

# 8. Progress beobachten:
# - Toast: "Wiederherstellung läuft..."
# - Progress-Events werden empfangen
# - Bei Success: Toast + Dialog schließt nach 1.5s
# - Bei Fehler: Error-Toast

# 9. Dateien verifizieren:
ls -la /tmp/restore-test
# Erwartung: Ausgewählte Dateien vorhanden
```

**Erfolgskriterien:**

- ✅ File-Browser zeigt korrekte Hierarchie
- ✅ Progress wird angezeigt (simulated 10 steps)
- ✅ Event-Listener werden gecleant (keine Memory Leaks)
- ✅ Error-Handling funktioniert
- ✅ Wiederhergestellte Dateien sind identisch

---

### ✅ 3. Backup-Button Integration

**Backend:** `src-tauri/src/commands/backup.rs:616`  
**Frontend:** `src/lib/components/pages/BackupJobs.svelte`

**Was testen:**

1. "Ausführen" Button in BackupJobs-Liste
2. Passwort von Keychain wird geholt
3. Progress-Tracking (Files, Bytes, Prozent)
4. Completion/Error Events
5. Button-States (Running, Disabled)

**Test-Schritte:**

```bash
# 1. Backup-Job erstellen (falls noch keiner existiert)
# - Name: "Test Daily Backup"
# - Repository: Existierendes Repository auswählen
# - Quellpfade: /tmp/test-backup-source
# - Tags: test, daily
# - Schedule: (leer lassen für manuell)

# 2. In BackupJobs-Seite:
# - Job sollte in Liste erscheinen
# - Status: "Manuell" (kein Schedule)

# 3. "Ausführen" Button klicken
# - Passwort-Prompt falls Repository locked (oder Keychain)
# - Button wird zu "⏳ Läuft..."
# - Progress-Bar erscheint

# 4. Progress beobachten:
# - filesProcessed Counter steigt
# - bytesUploaded Counter steigt
# - Prozent-Anzeige (0-100%)

# 5. Nach Completion:
# - Toast: "Backup-Job erfolgreich abgeschlossen"
# - Progress verschwindet
# - Button zurück zu "Ausführen"
# - Jobs-Liste lädt neu (last_run aktualisiert)

# 6. Snapshot verifizieren:
# - Snapshots-Seite öffnen
# - Neuester Snapshot sollte da sein
# - Tags sollten korrekt sein (test, daily)
```

**Erfolgskriterien:**

- ✅ Backup startet ohne Errors
- ✅ Progress-Events werden empfangen und angezeigt
- ✅ Event-Listener werden nach Completion gecleant
- ✅ Snapshot wird erfolgreich erstellt
- ✅ UI-States korrekt (Button disabled während Backup)

---

## 🧪 Erweiterte Tests

### Test 4: Parallel-Backups (Stress-Test)

```bash
# Teste ob mehrere Jobs gleichzeitig laufen können
# 1. Erstelle 2-3 Backup-Jobs
# 2. Starte alle gleichzeitig
# 3. Beobachte Progress für alle Jobs
# Erwartung: Jeder Job hat eigenen Progress-State
```

### Test 5: Cancellation (wenn implementiert)

```bash
# 1. Backup starten
# 2. Cancel-Button während Backup (TODO: noch nicht implementiert)
# Erwartung: Backup bricht ab, cleanup funktioniert
```

### Test 6: Error-Scenarios

```bash
# Teste folgende Error-Fälle:
# - Repository locked (falsches Passwort)
# - Quellpfad existiert nicht mehr
# - Disk voll
# - Network-Repository offline
# Erwartung: Sinnvolle Error-Messages, kein Crash
```

---

## 📊 Checkliste

### Vor dem Testen

- [ ] App kompiliert ohne Errors
- [ ] Dev-Build läuft: `npm run tauri dev`
- [ ] Test-Repository erstellt
- [ ] Test-Daten vorhanden

### Feature-Tests

- [ ] .gitignore automatisch erkannt
- [ ] Custom Excludes funktionieren
- [ ] Restore-Dialog öffnet
- [ ] File-Browser navigierbar
- [ ] Restore erfolgreich
- [ ] Backup-Button startet Job
- [ ] Progress wird angezeigt
- [ ] Snapshot erstellt

### Code-Quality

- [ ] Keine Console-Errors
- [ ] Event-Listener cleaned up
- [ ] Memory Leaks geprüft (DevTools)
- [ ] Toast-Notifications sinnvoll

### Performance

- [ ] File-Browser lädt schnell (<500ms)
- [ ] Progress-Updates flüssig (keine Lags)
- [ ] Große Backups (>1000 Dateien) funktionieren

---

## 🐛 Bekannte Issues

### Backend

- ⚠️ Progress ist **simuliert** (10 steps @ 200ms)
  - Grund: rustic_core bietet keinen Progress-Callback
  - TODO: Task 6 - ProgressBars trait implementieren

### Frontend

- ⚠️ File-Browser zeigt Placeholder-Größen (1024 bytes)
  - Grund: Echte Größen brauchen Index-Daten
  - TODO: Index-Parser implementieren

---

## 🚀 Nächste Schritte nach Tests

1. **Bugs dokumentieren** (GitHub Issues)
2. **Task 4:** Scheduler Testing
3. **Task 5:** Job-History UI
4. **Task 6:** Progress-Tracking ("das Monster")

---

**Happy Testing!** 🎉
