# Milestone 4: Erweiterte Features ⭐

> **Erweiterte rustic_core Features & UI-Polish**

**Dauer:** 40h (1 Woche) | **Status:** ~20% (partial UI)  
**Priorität:** 🟡 MEDIUM  
**Dependencies:** M1 (rustic_core funktioniert)

---

## Übersicht

**Ziel:** Erweiterte rustic_core Features nutzen, die bereits in der API vorhanden sind aber noch nicht im UI exponiert.

**UI Status:** Teilweise vorhanden (FilterBar, CompareSnapshotsDialog), Backend fehlt.

---

## 4.1 Snapshot-Tag-Management

**Dauer:** 8h | **Priorität:** 🟡 MEDIUM

### Tasks

- [ ] **`add_snapshot_tags()` Command** (2h)
  - `SnapshotFile.add_tags()` nutzen
  - Config aktualisieren
- [ ] **`remove_snapshot_tags()` Command** (2h)
  - `SnapshotFile.remove_tags()` nutzen
- [ ] **Tag-Filter im `list_snapshots()`** (2h)
  - OR-Logic (ANY tag matches)
  - AND-Logic (ALL tags match)
- [ ] **UI-Integration: FilterBar** (2h)
  - Tag-Checkboxes funktionieren
  - Filter wird an Backend gesendet

**UI:** `Snapshots.svelte:87` - Tag-Filter TODO

---

## 4.2 Snapshot-Diff/Vergleich

**Dauer:** 12h | **Priorität:** 🟠 HIGH

### Tasks

- [ ] **`compare_snapshots()` Command** (6h)
  - `Repository.diff_snapshots()` nutzen
  - Added/Removed/Modified Files ermitteln
  - Statistics berechnen (Größenänderungen)
- [ ] **Diff-UI Backend-Integration** (4h)
  - CompareSnapshotsDialog mit echten Daten
  - Filter (Added/Removed/Modified)
- [ ] **Side-by-Side-View** (2h)
  - Datei-Inhalte vergleichen (optional)

**UI:** `Snapshots.svelte:237` - Comparison TODO  
**Mockup:** `docs/mockups/rustic_advanced_ui_mockup.html` - Snapshot-Vergleichs-UI

---

## 4.3 Repository-Statistiken

**Dauer:** 10h | **Priorität:** 🟡 MEDIUM

### Tasks

- [ ] **`get_repository_stats()` Command** (4h)
  - `Repository.infos_files()` nutzen
  - Total size, Compression ratio
  - Snapshot count, Pack file count
- [ ] **Dashboard-Charts** (4h)
  - Storage-Usage-Chart (bereits im Mockup)
  - Backup-Frequency-Chart
  - Deduplication-Stats
- [ ] **Repository-Info-Dialog** (2h)
  - Detaillierte Statistics anzeigen

**UI:** `Dashboard.svelte` - Charts bereits vorhanden (mit Mock-Daten)  
**Mockup:** `docs/mockups/rustic_gui_mockup.html` - Dashboard mit Charts

---

## 4.4 Settings-Backend-Integration

**Dauer:** 6h | **Priorität:** 🟡 MEDIUM

### Tasks

- [ ] **Theme-Persistence** (2h)
  - `Settings.svelte:21,27` TODOs
  - Theme in Config speichern
  - Bei App-Start laden
- [ ] **Language-Settings** (2h)
  - `Settings.svelte:47,62` TODOs
  - i18n-Framework (optional für v1.1)
- [ ] **Notification-Preferences** (2h)
  - Toast-Dauer konfigurierbar
  - Sound-Notifications (optional)

**UI:** `Settings.svelte` - Backend-TODOs

---

## 4.5 Batch-Operations

**Dauer:** 4h | **Priorität:** 🟢 LOW

### Tasks

- [ ] **Batch-Restore** (2h)
  - Mehrere Snapshots auf einmal restoren
  - Progress für alle
- [ ] **Batch-Delete** (bereits in M1.4.4)
  - Verify implementation

**UI:** `Snapshots.svelte:405,576` - Batch-Restore TODOs

---

## Zusammenfassung

**Gesamt-Dauer:** 40h  
**Deliverables:**

- ✅ Tag-Management funktioniert
- ✅ Snapshot-Vergleich mit Diff
- ✅ Repository-Statistiken im Dashboard
- ✅ Settings werden persistiert
- ✅ Batch-Operations funktionieren

**Akzeptanz-Kriterien:**

- [ ] Snapshots können nach Tags gefiltert werden
- [ ] Diff zwischen 2 Snapshots wird angezeigt
- [ ] Dashboard zeigt echte Statistiken
- [ ] Theme bleibt nach Neustart erhalten

---

**[← Zurück zu M3](M3-job-scheduler.md)** | **[Zurück zur Roadmap](../../ROADMAP.md)** | **[Weiter zu M5 →](M5-testing-qa.md)**
