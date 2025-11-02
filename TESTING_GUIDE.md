# 🧪 Schnell-Test-Anleitung: Backup & Restore

> **Ready to Test:** Die App läuft im Dev-Modus (`npm run tauri dev`)  
> **Test-Repository:** `/tmp/rustic-test/repo`  
> **Passwort:** `test123`

---

## ✅ Test 1: Repository hinzufügen (2 min)

1. **App öffnen** (sollte automatisch gestartet sein)

2. **Repository hinzufügen:**
   - Klicke "+" Button oder "Repository hinzufügen"
   - Name: `Test Repository`
   - Typ: **Lokal**
   - Pfad: `/tmp/rustic-test/repo`
   - Passwort: `test123`
   - Speichern

3. **Verifizieren:**
   - Repository erscheint in Liste
   - Status: "Entsperrt" oder ähnlich
   - Repository kann ausgewählt werden

---

## ✅ Test 2: Backup-Job erstellen mit .gitignore (5 min)

1. **Backup-Job erstellen:**
   - Navigation → "Backup-Jobs"
   - Klicke "+ Neuer Job"
2. **Job-Konfiguration:**

   **Tab 1: General**
   - Name: `Test Daily Backup`
   - Repository: `Test Repository` auswählen
   - Tags: `test, daily` (komma-getrennt)

   **Tab 2: Paths & Exclusions**
   - Source Path: `/tmp/rustic-test/backup-source`
   - Klicke "Pfad hinzufügen"
   - ✅ WICHTIG: .gitignore-Support ist **automatisch aktiv**
   - Custom Excludes: (leer lassen - nur .gitignore testen)

   **Tab 3: Schedule**
   - Leer lassen (Manueller Job)

   **Tab 4: Retention**
   - Keep Last: `7`
   - Rest leer
   - Klicke "Job erstellen"

3. **Verifizieren:**
   - Job erscheint in Liste
   - Status: "Manuell"
   - "Ausführen" Button sichtbar

---

## ✅ Test 3: Backup ausführen & Progress (3 min)

1. **Backup starten:**
   - Klicke "Ausführen" Button beim Test-Job
   - Button wird zu "⏳ Läuft..."
2. **Progress beobachten:**
   - Progress-Bar erscheint
   - Zähler steigen:
     - Dateien: sollte ~4 Dateien sein
     - Bytes: ~200-300 Bytes
     - Prozent: 0% → 100%
   - Toast-Notification: "Starte Backup-Job..."
3. **Nach Completion:**
   - Toast: "Backup-Job erfolgreich abgeschlossen"
   - Button zurück zu "Ausführen"
   - Progress verschwindet

4. **Browser-Console öffnen (F12):**
   - Schaue nach Console-Errors
   - Expected: Keine Errors, nur Info-Logs

---

## ✅ Test 4: Snapshot verifizieren (2 min)

1. **Snapshots-Seite:**
   - Navigation → "Snapshots"
   - Snapshot sollte erscheinen
   - Tags: `test`, `daily`
   - Hostname: dein Hostname
   - Datum: jetzt

2. **Snapshot-Details anschauen:**
   - Klick auf Snapshot-Zeile
   - Details-Panel öffnet
   - Dateien sollten sichtbar sein

3. **Im Terminal verifizieren:**

   ```bash
   cd /tmp/rustic-test
   export RUSTIC_REPOSITORY=/tmp/rustic-test/repo
   export RUSTIC_PASSWORD=test123
   rustic snapshots
   ```

   **Erwartung:**
   - 1 Snapshot vorhanden
   - Tags: test, daily

   ```bash
   rustic ls latest
   ```

   **Erwartung - Dateien GESICHERT:**
   - ✅ `important.txt`
   - ✅ `config.yaml`
   - ✅ `docs/README.md`
   - ✅ `docs/images/logo.png`

   **Erwartung - Dateien IGNORIERT (.gitignore):**
   - ❌ `debug.log` (\*.log)
   - ❌ `data.tmp` (\*.tmp)
   - ❌ `node_modules/` (node_modules/)
   - ❌ `temp/` (temp/)

   **✅ ERFOLG:** Nur 4-5 Dateien gesichert (nicht 8!)

---

## ✅ Test 5: Restore-Dialog (5 min)

1. **Restore öffnen:**
   - Snapshots-Seite
   - Klicke "Wiederherstellen" Button beim Snapshot
   - RestoreDialog öffnet

2. **File-Browser testen:**
   - Root-Verzeichnis zeigt:
     - `important.txt`
     - `config.yaml`
     - `docs/` (Ordner-Icon)
   - ❌ NICHT sichtbar: debug.log, node_modules, temp
   - Klicke auf `docs/` Ordner
   - Unterordner öffnet:
     - `README.md`
     - `images/` (Ordner)

3. **Dateien auswählen:**
   - Checkbox bei `important.txt`
   - Checkbox bei `docs/` (alle Dateien im Ordner)

4. **Ziel-Pfad:**
   - Target: `/tmp/rustic-test/restore-target`
   - Optionen:
     - ✅ Berechtigungen wiederherstellen
     - ❌ Vorhandene Dateien überschreiben (erste Restore)

5. **Restore starten:**
   - Klicke "Wiederherstellen"
   - Toast: "Wiederherstellung läuft..."
   - Progress-Events (simulated)
   - Nach ~2 Sekunden:
     - Toast: "Wiederherstellung erfolgreich"
     - Dialog schließt automatisch (1.5s delay)

6. **Verifizieren:**

   ```bash
   ls -R /tmp/rustic-test/restore-target
   ```

   **Erwartung:**
   - ✅ `important.txt`
   - ✅ `docs/README.md`
   - ✅ `docs/images/logo.png`
   - Inhalt identisch mit Originalen

---

## ✅ Test 6: Custom Excludes (Bonus - 3 min)

1. **Job bearbeiten:**
   - BackupJobs → "Bearbeiten" beim Test-Job
2. **Tab 2: Paths & Exclusions:**
   - Custom Excludes hinzufügen:
     - `*.yaml` (YAML-Dateien ausschließen)
   - Speichern

3. **Zweites Backup:**
   - Klicke "Ausführen"
   - Warte auf Completion

4. **Verifizieren (Terminal):**

   ```bash
   rustic ls latest
   ```

   **Erwartung:**
   - ✅ `important.txt` (noch da)
   - ❌ `config.yaml` (jetzt ausgeschlossen!)
   - ✅ `docs/` (noch da)

5. **Kombiniert:**
   - .gitignore schließt aus: node_modules/, _.log, temp/, _.tmp
   - Custom Exclude schließt aus: \*.yaml
   - **Insgesamt:** 5 Patterns aktiv!

---

## 🐛 Troubleshooting

### Problem: "Repository-Passwort nicht verfügbar"

**Lösung:**

- Keychain-Integration funktioniert noch nicht perfekt
- Passwort muss beim Job gespeichert werden (Backend-TODO)
- Workaround: Repository vorher "entsperren" (wenn UI vorhanden)

### Problem: Progress bleibt bei 0%

**Ursache:**

- Progress ist **simuliert** (10 steps @ 200ms)
- Echte Progress-Tracking kommt in Task 6 (ProgressBars trait)
- Funktioniert trotzdem, nur nicht präzise

### Problem: File-Browser zeigt alle Dateien 1024 bytes

**Ursache:**

- Placeholder-Größen (echte Größen brauchen Index-Parsing)
- TODO: Index-Integration

### Problem: Event-Listener Memory Leak?

**Check:**

- F12 → Performance Tab
- Schaue nach wachsenden Event-Listeners
- **Fix:** Sollte jetzt mit finally-Block gecleant sein

---

## 📊 Checkliste

Nach Tests:

- [ ] ✅ Repository hinzugefügt
- [ ] ✅ Backup-Job erstellt
- [ ] ✅ .gitignore automatisch erkannt
- [ ] ✅ Custom Excludes funktionieren
- [ ] ✅ Backup läuft erfolgreich
- [ ] ✅ Progress wird angezeigt
- [ ] ✅ Snapshot erstellt (nur 4-5 Dateien, nicht 8!)
- [ ] ✅ Restore-Dialog öffnet
- [ ] ✅ File-Browser navigierbar
- [ ] ✅ Restore erfolgreich
- [ ] ✅ Wiederhergestellte Dateien korrekt
- [ ] ✅ Keine Console-Errors
- [ ] ✅ Event-Listener gecleant

---

## 🎉 Success Criteria

**Alle Tests bestanden wenn:**

1. **Backup:**
   - .gitignore-Regeln werden befolgt ✅
   - Nur 4-5 Dateien gesichert (nicht 8) ✅
   - Custom Excludes funktionieren ✅
   - Progress sichtbar (auch wenn simuliert) ✅

2. **Restore:**
   - File-Browser zeigt nur gesicherte Dateien ✅
   - Navigation funktioniert ✅
   - Restore erfolgreich ✅
   - Dateien identisch ✅

3. **Code Quality:**
   - Keine Console-Errors ✅
   - Event-Listener cleanup funktioniert ✅
   - UI reagiert flüssig ✅

---

**Viel Erfolg beim Testen!** 🚀

**Nächste Schritte:**

- Bugs dokumentieren → GitHub Issues
- Task 4: Scheduler Testing
- Task 5: Job-History UI
- Task 6: Progress-Tracking ("das Monster" 😅)
