# ToDo-Liste: 1:1-Umsetzung aller Mockups (Rustic GUI)

## 1. Seiten (Pages)

- [x] **DashboardPage.svelte**
  - [x] Toolbar exakt nach Mockup (Sticky, Buttons, Icons)
  - [x] „+ Repository öffnen“-Button mit Dialog (UI-Dummy, dann Funktion)
  - [x] Grid-Layout für RepositoryCards (Spacing, Responsive)
  - [x] Section-Titel, Abstände, Farben exakt wie im Mockup
  - [x] ActivityLog: Styling, Icons, Farben, States (info/warning/error)
  - [x] Storage Usage Charts (wie Mockup)
  - [x] Responsive-Design (Tablet/Mobile)

- [x] **Snapshots.svelte**
  - [x] Tabelle mit Snapshots (Spalten, Filter, Pagination)
  - [x] FilterBar: Tags, Hostname, Zeitraum, Größe
  - [x] Kontextmenü für Snapshots (Bulk, Rechtsklick)
  - [x] Snapshot-Vergleichs-UI (Side-by-Side, Diff)
  - [x] Snapshot-Info-Dialog
  - [x] Responsive-Design

- [x] **BackupJobs.svelte** (neu anlegen)
  - [x] Liste aller Backup-Jobs (Job-Item-Layout, Status, Aktionen)
  - [x] „+ New Job“-Button mit Dialog (UI-Dummy, dann Funktion)
  - [x] Job-Kontextmenü: Edit, Enable/Disable, Delete
  - [x] Scheduling-Info, Retention-Policy-Anzeige
  - [x] Responsive-Design

- [x] **Repositories.svelte** (neu anlegen)
  - [x] Toolbar mit „+ Add Repository"-Button
  - [x] Liste aller Repositories (Job-Item-Layout, Status, Aktionen)
  - [x] Repository-Kontextmenü: Edit, Unlock, Delete
  - [x] Repository-Details-Dialog
  - [x] Responsive-Design

- [x] **Settings.svelte** (neu anlegen)
  - [x] Layout und Felder gemäß Mockup
  - [x] Responsive-Design

## 2. Dialoge & Popups

- [x] **AddRepositoryDialog.svelte**
  - [x] Typ-Selector, Felder, States
- [x] **UnlockRepositoryDialog.svelte** (neu anlegen)
  - [x] Passwortfeld, States
- [x] **CreateJobDialog.svelte**
  - [x] Wizard, Tabs, Felder, Validierung
- [x] **EditJobDialog.svelte** (neu anlegen)
  - [x] Wizard, Tabs, Felder, Validierung
- [x] **RunBackupDialog.svelte** (neu anlegen)
  - [x] Progress, Log
- [x] **DeleteJobDialog.svelte** (neu anlegen)
  - [x] Confirmation, States
- [x] **DeleteRepoDialog.svelte** (neu anlegen)
  - [x] Confirmation, States
- [x] **RestoreDialog.svelte**
  - [x] File-Browser, Restore-Optionen
- [x] **CompareSnapshotsDialog.svelte**
  - [x] Side-by-Side, FilterBar
- [x] **SnapshotInfoDialog.svelte** (neu anlegen)
  - [x] Details-Ansicht
- [x] **ChangePasswordDialog.svelte** (neu anlegen)
  - [x] Felder, Strength-Indicator
- [x] **CheckRepoDialog.svelte** (neu anlegen)
  - [x] Progress, Log
- [x] **PruneRepoDialog.svelte** (neu anlegen)
  - [x] Stats, Confirmation

## 3. Shared/Advanced UI-Komponenten

- [x] **FilterBar.svelte**
  - [x] Tags, Hostname, Zeitraum, Größe (wie Mockup)
- [x] **ContextMenu.svelte**
  - [x] Bulk, Rechtsklick, Actions (wie Mockup)
- [x] **Pagination.svelte**
  - [x] Controls, States (wie Mockup)
- [x] **Input.svelte, Select.svelte, Checkbox.svelte**
  - [x] Reusable Components für Formulare
- [x] **Modal.svelte, Button.svelte, Badge.svelte, ProgressBar.svelte**
  - [x] Auf Mockup-Details prüfen und ggf. anpassen

## 4. Responsive Design & Accessibility

- [ ] Alle Seiten und Dialoge auf Responsive-Design prüfen und anpassen (Breakpoints, Mobile-Ansicht)
- [ ] Sidebar als Overlay auf Mobile/Tablet
- [ ] Grid/Tabelle einspaltig auf kleinen Screens
- [ ] Buttons und Touch-Flächen vergrößern
- [ ] ARIA-Labels, Keyboard-Navigation, Fokus-Management
- [ ] Tooltips für alle wichtigen Aktionen
- [ ] Visuelle Rückmeldungen für alle Interaktionen

## 5. UI-Details & States

- [ ] Alle Buttons: Icons, Tooltips, ARIA-Labels, Disabled/Loading-States
- [ ] Badges, Tags, Status-Anzeigen
- [ ] Hover-, Active-, Disabled-, Empty-, Success-States
- [ ] Animationen (Transitions, Progress, Dialoge)
- [ ] User-Avatar im Header (Platzhalter, später Funktion)

## 6. Testing & Review

- [ ] UI-Tests für alle Hauptzustände (Vitest, Playwright o.ä.)
- [ ] Visuelle Regressionstests (Pixel-Perfect)
- [ ] Review mit Mockup-Vergleich (Screenshot-Review)

---

**Hinweis:**
Diese Liste ist direkt aus der Feature-Matrix abgeleitet. Jeder Punkt kann in weitere Subtasks aufgeteilt werden, sobald die Detailplanung für die jeweilige Komponente/Seite erfolgt.
