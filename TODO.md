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

- [ ] **Snapshots.svelte**
  - [ ] Tabelle mit Snapshots (Spalten, Filter, Pagination)
  - [ ] FilterBar: Tags, Hostname, Zeitraum, Größe
  - [ ] Kontextmenü für Snapshots (Bulk, Rechtsklick)
  - [ ] Snapshot-Vergleichs-UI (Side-by-Side, Diff)
  - [ ] Snapshot-Info-Dialog
  - [ ] Responsive-Design

- [ ] **BackupJobs.svelte** (neu anlegen)
  - [ ] Liste aller Backup-Jobs (Job-Item-Layout, Status, Aktionen)
  - [ ] „+ New Job“-Button mit Dialog (UI-Dummy, dann Funktion)
  - [ ] Job-Kontextmenü: Edit, Enable/Disable, Delete
  - [ ] Scheduling-Info, Retention-Policy-Anzeige
  - [ ] Responsive-Design

- [ ] **Repositories.svelte** (neu anlegen)
  - [ ] Toolbar mit „+ Add Repository“-Button
  - [ ] Liste aller Repositories (Job-Item-Layout, Status, Aktionen)
  - [ ] Repository-Kontextmenü: Edit, Unlock, Delete
  - [ ] Repository-Details-Dialog
  - [ ] Responsive-Design

- [ ] **Settings.svelte** (neu anlegen)
  - [ ] Layout und Felder gemäß Mockup
  - [ ] Responsive-Design

## 2. Dialoge & Popups

- [ ] **AddRepositoryDialog.svelte**
  - [ ] Typ-Selector, Felder, States
- [ ] **UnlockRepositoryDialog.svelte** (neu anlegen)
  - [ ] Passwortfeld, States
- [ ] **CreateJobDialog.svelte**
  - [ ] Wizard, Tabs, Felder, Validierung
- [ ] **EditJobDialog.svelte** (neu anlegen)
  - [ ] Wizard, Tabs, Felder, Validierung
- [ ] **RunBackupDialog.svelte** (neu anlegen)
  - [ ] Progress, Log
- [ ] **DeleteJobDialog.svelte** (neu anlegen)
  - [ ] Confirmation, States
- [ ] **DeleteRepoDialog.svelte** (neu anlegen)
  - [ ] Confirmation, States
- [ ] **RestoreDialog.svelte**
  - [ ] File-Browser, Restore-Optionen
- [ ] **CompareSnapshotsDialog.svelte**
  - [ ] Side-by-Side, FilterBar
- [ ] **SnapshotInfoDialog.svelte** (neu anlegen)
  - [ ] Details-Ansicht
- [ ] **ChangePasswordDialog.svelte** (neu anlegen)
  - [ ] Felder, Strength-Indicator
- [ ] **CheckRepoDialog.svelte** (neu anlegen)
  - [ ] Progress, Log
- [ ] **PruneRepoDialog.svelte** (neu anlegen)
  - [ ] Stats, Confirmation

## 3. Shared/Advanced UI-Komponenten

- [ ] **FilterBar.svelte**
  - [ ] Tags, Hostname, Zeitraum, Größe (wie Mockup)
- [ ] **ContextMenu.svelte**
  - [ ] Bulk, Rechtsklick, Actions (wie Mockup)
- [ ] **Pagination.svelte**
  - [ ] Controls, States (wie Mockup)
- [ ] **Modal.svelte, Button.svelte, Badge.svelte, ProgressBar.svelte**
  - [ ] Auf Mockup-Details prüfen und ggf. anpassen

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
