# Frontend Instructions

> Svelte 5 + TypeScript Konventionen und Patterns für Rustic GUI

---

## 🎨 UI-Mockups sind PFLICHT!

### Vor jeder UI-Implementierung

**⚠️ KRITISCH: Prüfe IMMER zuerst ob ein Mockup existiert!**

1. ✅ Öffne `docs/mockups/` und suche relevantes Mockup
2. ✅ Analysiere Layout, Farben, Spacing, Interaktionen
3. ✅ Implementiere **EXAKT** nach Mockup-Vorgaben
4. ✅ Bei Unklarheiten: Frage nach bevor du abweichst
5. ✅ Bei fehlenden Mockups: Erstelle ERST Mockup, DANN Code

### Mockup-Übersicht

```
docs/mockups/
├── rustic_gui_mockup.html                # Hauptfenster, Dashboard, Listenansichten
├── rustic_backup_dialogs.html            # Backup-Wizard & Job-bezogene Dialoge
├── rustic_repo_security_dialogs.html     # Repository-Setup, Security-Flows
├── rustic_restore_dialogs.html           # Restore- und Snapshot-Vergleichsdialoge
├── rustic_advanced_ui_mockup.html        # Erweiterte Snapshot-Funktionen
├── rustic_location_picker.html           # Unified Location Picker
└── portable_notice.html                  # Portable-Build-Hinweise (nur Layout-Referenz)
```

> 💡 Die HTML-Mockups enthalten inline CSS/JS. Öffne sie im Browser, um das erwartete Layout, Verhalten und die Farbwerte pixelgenau zu sehen.

---

## 📁 Projekt-Struktur (Frontend)

```
src/
├── lib/
│   ├── components/                # UI-Komponenten
│   │   ├── shared/                # Wiederverwendbare Basis-Komponenten
│   │   │   ├── Button.svelte
│   │   │   ├── Modal.svelte
│   │   │   ├── Toast.svelte
│   │   │   ├── LoadingSpinner.svelte
│   │   │   ├── ProgressBar.svelte
│   │   │   ├── FileTree.svelte
│   │   │   ├── TagList.svelte
│   │   │   └── ContextMenu.svelte
│   │   ├── dialogs/               # Dialog-Komponenten
│   │   │   ├── AddRepositoryDialog.svelte
│   │   │   ├── JobDialog.svelte       # Unified Create/Edit (mode-basiert)
│   │   │   │   └── JobDialog/        # 4 Tab-Komponenten (General, Paths, Schedule, Retention)
│   │   │   ├── LocationPickerDialog.svelte  # Modularisiert mit 5 Sub-Komponenten
│   │   │   │   └── LocationPicker/  # LocalTab, NetworkTab, CloudTab, RecentTab, CredentialPromptModal
│   │   │   ├── RestoreDialog.svelte
│   │   │   └── ConfirmDialog.svelte
│   │   ├── layout/                # Layout-Komponenten
│   │   │   ├── Sidebar.svelte
│   │   │   └── MainLayout.svelte  # ⚠️ Header.svelte entfernt (Nov 2025)! Pages haben eigene Headers.
│   │   └── pages/                 # Seiten-Komponenten
│   │       ├── DashboardPage.svelte      # Mit eigenem Page-Header (🔄 Refresh)
│   │       ├── Repositories.svelte       # Mit eigenem Page-Header (➕ Add, 📂 Open)
│   │       ├── Snapshots.svelte          # Mit eigenem Page-Header (🔄 Refresh)
│   │       │   └── Snapshots/           # SnapshotTable, SnapshotDetailsModal, SnapshotContextMenu
│   │       ├── BackupJobs.svelte         # Mit eigenem Page-Header (➕ Create Job)
│   │       └── Settings.svelte           # Mit eigenem Page-Header (🔄 Reset)
│   ├── stores/                    # Svelte Stores (State Management)
│   │   ├── repositories.ts
│   │   ├── snapshots.ts
│   │   ├── backup-jobs.ts
│   │   ├── ui.ts
│   │   └── toasts.ts
│   ├── api/                       # Backend-API-Wrapper
│   │   ├── backup.ts
│   │   ├── restore.ts
│   │   ├── snapshots.ts
│   │   ├── repositories.ts
│   │   └── types.ts               # Tauri Command Types
│   ├── types/                     # TypeScript Types
│   │   ├── backup.types.ts
│   │   ├── snapshot.types.ts
│   │   ├── repository.types.ts
│   │   └── index.ts
│   └── utils/                     # Hilfsfunktionen
│       ├── format.ts              # Formatierungs-Helpers
│       ├── validation.ts          # Validierungs-Helpers
│       ├── date.ts                # Datums-Helpers
│       └── tauri.ts               # Tauri-Helpers
├── routes/                        # SvelteKit Routes (falls verwendet)
│   └── +page.svelte
├── app.html                       # HTML-Template
└── app.css                        # Globale Styles
```

---

## 🔧 Svelte 5 Patterns

### Component-Template

````svelte
<script lang="ts">
  /**
   * Wiederverwendbarer Button mit verschiedenen Varianten.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Button variant="primary" on:click={handleSave}>
   *   Speichern
   * </Button>
   * ```
   */

  // 1. Imports
  import { createEventDispatcher } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  // 2. Props (Svelte 5 mit $props())
  interface Props extends HTMLButtonAttributes {
    /** Button-Text */
    label?: string;

    /** Button-Variante */
    variant?: 'primary' | 'secondary' | 'danger';

    /** Größe des Buttons */
    size?: 'small' | 'medium' | 'large';

    /** Zeigt Loading-Spinner */
    loading?: boolean;

    /** Icon links vom Text (optional) */
    icon?: string;
  }

  let {
    label,
    variant = 'primary',
    size = 'medium',
    loading = false,
    icon,
    disabled = false,
    ...restProps
  }: Props = $props();

  // 3. State (Svelte 5 mit $state())
  let isHovered = $state(false);

  // 4. Derived State (Svelte 5 mit $derived())
  let isDisabled = $derived(disabled || loading);

  // 5. Event Dispatcher
  const dispatch = createEventDispatcher<{
    click: MouseEvent;
  }>();

  // 6. Functions
  function handleClick(event: MouseEvent) {
    if (!isDisabled) {
      dispatch('click', event);
    }
  }
</script>

<!-- 7. Template -->
<button
  class="btn btn-{variant} btn-{size}"
  class:disabled={isDisabled}
  class:loading
  disabled={isDisabled}
  on:click={handleClick}
  on:mouseenter={() => (isHovered = true)}
  on:mouseleave={() => (isHovered = false)}
  {...restProps}
>
  {#if loading}
    <span class="spinner"></span>
  {:else if icon}
    <span class="icon">{icon}</span>
  {/if}

  {#if label}
    <span>{label}</span>
  {:else}
    <slot />
  {/if}
</button>

<!-- 8. Styles -->
<style>
  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 500;
    border: none;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s;
  }

  .btn-primary {
    background: var(--color-primary);
    color: white;
  }

  .btn-primary:hover:not(.disabled) {
    background: var(--color-primary-dark);
  }

  .btn-secondary {
    background: var(--color-secondary);
    color: var(--color-text);
  }

  .btn-danger {
    background: var(--color-danger);
    color: white;
  }

  .btn-small {
    padding: 0.25rem 0.5rem;
    font-size: 0.875rem;
  }

  .btn-large {
    padding: 0.75rem 1.5rem;
    font-size: 1.125rem;
  }

  .btn.disabled,
  .btn.loading {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 1rem;
    height: 1rem;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
````

### Dialog-Pattern

```svelte
<script lang="ts">
  /**
   * Dialog zum Erstellen eines neuen Backup-Jobs.
   *
   * Mehrstufiger Wizard mit folgenden Schritten:
   * 1. Allgemeine Einstellungen (Name, Repository)
   * 2. Quellpfade und Exclusions
   * 3. Zeitplan (optional)
   * 4. Retention-Policy
   */

  import { createEventDispatcher } from 'svelte';
  import type { BackupJob, BackupJobConfig } from '$lib/types';
  import Button from '$lib/components/shared/Button.svelte';
  import TabContainer from '$lib/components/layout/TabContainer.svelte';

  // Props
  let {
    isOpen = $bindable(false),
    initialData,
  }: {
    isOpen?: boolean;
    initialData?: Partial<BackupJobConfig>;
  } = $props();

  // State
  let activeTab = $state(0);
  let jobName = $state(initialData?.name ?? '');
  let repositoryId = $state(initialData?.repositoryId ?? '');
  let sourcePaths = $state<string[]>(initialData?.sourcePaths ?? []);
  let excludePatterns = $state<string[]>(initialData?.excludePatterns ?? []);
  let schedule = $state<string | null>(initialData?.schedule ?? null);

  // Validation
  let isStep1Valid = $derived(jobName.trim().length > 0 && repositoryId.length > 0);

  let isStep2Valid = $derived(sourcePaths.length > 0);

  let canSave = $derived(isStep1Valid && isStep2Valid);

  // Events
  const dispatch = createEventDispatcher<{
    save: BackupJobConfig;
    cancel: void;
  }>();

  // Functions
  function nextStep() {
    if (activeTab < 3) {
      activeTab++;
    }
  }

  function prevStep() {
    if (activeTab > 0) {
      activeTab--;
    }
  }

  function handleSave() {
    if (!canSave) return;

    const config: BackupJobConfig = {
      name: jobName,
      repositoryId,
      sourcePaths,
      excludePatterns,
      schedule,
      retention: {
        keepLast: 10,
        keepDaily: 7,
        keepWeekly: 4,
        keepMonthly: 6,
      },
    };

    dispatch('save', config);
    handleClose();
  }

  function handleClose() {
    isOpen = false;
    // Reset state
    activeTab = 0;
    jobName = '';
    repositoryId = '';
    sourcePaths = [];
    excludePatterns = [];
    schedule = null;
  }
</script>

{#if isOpen}
  <div class="modal-overlay" on:click={handleClose}>
    <dialog open on:click|stopPropagation on:keydown={(e) => e.key === 'Escape' && handleClose()}>
      <header>
        <h2>Neuen Backup-Job erstellen</h2>
        <button class="close" on:click={handleClose}>×</button>
      </header>

      <TabContainer tabs={['Allgemein', 'Pfade', 'Zeitplan', 'Retention']} bind:activeTab>
        {#snippet tab0()}
          <!-- Schritt 1: Allgemein -->
          <div class="form-group">
            <label for="job-name">Job-Name</label>
            <input
              id="job-name"
              type="text"
              bind:value={jobName}
              placeholder="z.B. Dokumente täglich"
            />
          </div>

          <div class="form-group">
            <label for="repository">Repository</label>
            <select id="repository" bind:value={repositoryId}>
              <option value="">Wähle Repository...</option>
              <!-- TODO: Load from store -->
            </select>
          </div>
        {/snippet}

        {#snippet tab1()}
          <!-- Schritt 2: Pfade -->
          <!-- TODO: Path selector -->
        {/snippet}

        {#snippet tab2()}
          <!-- Schritt 3: Zeitplan -->
          <!-- TODO: Cron editor -->
        {/snippet}

        {#snippet tab3()}
          <!-- Schritt 4: Retention -->
          <!-- TODO: Retention config -->
        {/snippet}
      </TabContainer>

      <footer>
        <Button variant="secondary" on:click={handleClose}>Abbrechen</Button>

        {#if activeTab > 0}
          <Button variant="secondary" on:click={prevStep}>Zurück</Button>
        {/if}

        {#if activeTab < 3}
          <Button
            variant="primary"
            on:click={nextStep}
            disabled={activeTab === 0 ? !isStep1Valid : !isStep2Valid}
          >
            Weiter
          </Button>
        {:else}
          <Button variant="primary" on:click={handleSave} disabled={!canSave}>Erstellen</Button>
        {/if}
      </footer>
    </dialog>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  dialog {
    background: var(--bg-primary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
    max-width: 600px;
    width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .close {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: var(--text-secondary);
  }

  .close:hover {
    color: var(--text-primary);
  }

  footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  .form-group {
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.25rem;
    font-weight: 500;
  }

  input,
  select {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    background: var(--bg-secondary);
  }
</style>
```

---

## 📦 State Management (Svelte Stores)

### Store-Pattern (Legacy + Runes Hybrid)

Rustic GUI nutzt bereits **Svelte 5**, bleibt aber aus Kompatibilitätsgründen bei den klassischen `writable`/`derived` Stores für globale Zustände. Innerhalb neuer Komponenten dürfen Svelte-5-Runes (`$state`, `$derived`, `$effect`) genutzt werden, solange sie nicht mit den bestehenden Stores kollidieren.

**Richtlinien:**

- Behalte bestehende Stores (`src/lib/stores/*.ts`) im klassischen Pattern bei.
- Wenn zusätzlicher lokaler Zustand benötigt wird, verwende in Komponenten `let foo = $state(initial)`.
- Greife in Runes-Komponenten auf Stores über `$store` zu (Svelte kompiliert dies weiterhin korrekt).
- Verwende `setContext`/`getContext` nur, wenn das Mockup oder bestehende Patterns es vorgeben.

```typescript
// src/lib/stores/repositories.ts

import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Repository } from '$lib/types';

/**
 * Store für Repository-Verwaltung.
 *
 * Enthält alle konfigurierten Repositories und das aktuell aktive.
 */

// Private Stores
const _repositories = writable<Repository[]>([]);
const _activeRepositoryId = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

// Public Stores (read-only)
export const repositories = { subscribe: _repositories.subscribe };
export const activeRepositoryId = { subscribe: _activeRepositoryId.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

// Derived Store: Aktives Repository
export const activeRepository = derived(
  [_repositories, _activeRepositoryId],
  ([$repos, $activeId]) => {
    if (!$activeId) return null;
    return $repos.find((r) => r.id === $activeId) ?? null;
  }
);

/**
 * Lädt alle Repositories vom Backend.
 */
export async function loadRepositories(): Promise<void> {
  _loading.set(true);
  _error.set(null);

  try {
    const repos = await invoke<Repository[]>('get_all_repositories');
    _repositories.set(repos);
  } catch (err) {
    _error.set(err instanceof Error ? err.message : 'Unbekannter Fehler');
    throw err;
  } finally {
    _loading.set(false);
  }
}

/**
 * Fügt ein neues Repository hinzu.
 */
export async function addRepository(repo: Repository): Promise<void> {
  await invoke('add_repository', { repo });

  // Update Store
  _repositories.update((repos) => [...repos, repo]);
}

/**
 * Entfernt ein Repository.
 */
export async function removeRepository(id: string): Promise<void> {
  await invoke('remove_repository', { id });

  // Update Store
  _repositories.update((repos) => repos.filter((r) => r.id !== id));

  // Deselect wenn aktiv
  _activeRepositoryId.update((activeId) => (activeId === id ? null : activeId));
}

/**
 * Wechselt das aktive Repository.
 */
export async function switchRepository(id: string, password: string): Promise<void> {
  await invoke('switch_repository', { repoId: id, password });
  _activeRepositoryId.set(id);
}

/**
 * Setzt den Store zurück (z.B. bei Logout).
 */
export function reset(): void {
  _repositories.set([]);
  _activeRepositoryId.set(null);
  _loading.set(false);
  _error.set(null);
}
```

### Store Usage in Components

```svelte
<script lang="ts">
  import { repositories, activeRepository, loadRepositories } from '$lib/stores/repositories';
  import { onMount } from 'svelte';

  // Load on mount
  onMount(() => {
    loadRepositories();
  });
</script>

<div>
  <h2>Repositories</h2>

  {#if $repositories.length === 0}
    <p>Keine Repositories konfiguriert</p>
  {:else}
    <ul>
      {#each $repositories as repo}
        <li class:active={repo.id === $activeRepository?.id}>
          {repo.name}
        </li>
      {/each}
    </ul>
  {/if}
</div>
```

---

## 🔌 Tauri API Integration

### API-Wrapper-Pattern

```typescript
// src/lib/api/backup.ts

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { BackupJob, BackupResult, BackupProgress } from '$lib/types';

/**
 * API-Wrapper für Backup-Operationen.
 *
 * Kapselt alle Tauri-Commands und Events für Backups.
 */

/**
 * Startet einen Backup-Job.
 *
 * @param jobId - ID des auszuführenden Jobs
 * @param password - Repository-Passwort
 * @returns Promise mit Backup-Ergebnis
 */
export async function runBackup(jobId: string, password: string): Promise<BackupResult> {
  return await invoke<BackupResult>('run_backup', {
    jobId,
    password,
  });
}

/**
 * Bricht einen laufenden Backup ab.
 *
 * @param jobId - ID des abzubrechenden Jobs
 */
export async function cancelBackup(jobId: string): Promise<void> {
  await invoke('cancel_backup', { jobId });
}

/**
 * Hört auf Backup-Progress-Events.
 *
 * @param jobId - Job-ID für die Progress-Events
 * @param callback - Callback für Progress-Updates
 * @returns Unlisten-Funktion zum Cleanup
 */
export async function onBackupProgress(
  jobId: string,
  callback: (progress: BackupProgress) => void
): Promise<UnlistenFn> {
  return await listen<BackupProgress>(`backup-progress-${jobId}`, (event) => {
    callback(event.payload);
  });
}

/**
 * Lädt alle konfigurierten Backup-Jobs.
 */
export async function listBackupJobs(): Promise<BackupJob[]> {
  return await invoke<BackupJob[]>('list_backup_jobs');
}

/**
 * Erstellt einen neuen Backup-Job.
 */
export async function createBackupJob(job: BackupJob): Promise<string> {
  return await invoke<string>('create_backup_job', { job });
}

/**
 * Aktualisiert einen existierenden Backup-Job.
 */
export async function updateBackupJob(job: BackupJob): Promise<void> {
  await invoke('update_backup_job', { job });
}

/**
 * Löscht einen Backup-Job.
 */
export async function deleteBackupJob(jobId: string): Promise<void> {
  await invoke('delete_backup_job', { jobId });
}
```

### API Usage with Progress

```svelte
<script lang="ts">
  import { runBackup, onBackupProgress, cancelBackup } from '$lib/api/backup';
  import ProgressBar from '$lib/components/shared/ProgressBar.svelte';
  import { onDestroy } from 'svelte';

  let isRunning = $state(false);
  let progress = $state({
    filesProcessed: 0,
    filesTotal: 0,
    bytesProcessed: 0,
    bytesTotal: 0,
  });
  let unlistenProgress: (() => void) | null = null;

  async function handleStartBackup(jobId: string, password: string) {
    try {
      isRunning = true;

      // Listen for progress
      unlistenProgress = await onBackupProgress(jobId, (p) => {
        progress = p;
      });

      // Start backup
      const result = await runBackup(jobId, password);

      console.log('Backup erfolgreich:', result);
      showToast('success', 'Backup abgeschlossen');
    } catch (error) {
      console.error('Backup fehlgeschlagen:', error);
      showToast('error', 'Backup fehlgeschlagen', error.message);
    } finally {
      isRunning = false;
      unlistenProgress?.();
      unlistenProgress = null;
    }
  }

  async function handleCancelBackup(jobId: string) {
    await cancelBackup(jobId);
    showToast('info', 'Backup wird abgebrochen...');
  }

  // Cleanup on destroy
  onDestroy(() => {
    unlistenProgress?.();
  });
</script>

{#if isRunning}
  <div class="backup-running">
    <ProgressBar
      value={progress.bytesProcessed}
      max={progress.bytesTotal}
      label="{progress.filesProcessed} / {progress.filesTotal} Dateien"
    />

    <button on:click={() => handleCancelBackup(currentJobId)}> Abbrechen </button>
  </div>
{/if}
```

---

## 🎨 UI-Komponenten-Bibliothek

### Shared Components

#### Toast Notifications

```svelte
<!-- src/lib/components/shared/Toast.svelte -->
<script lang="ts">
  /**
   * Toast-Notification-Komponente.
   *
   * Zeigt temporäre Benachrichtigungen an.
   */

  export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
  export let title: string;
  export let message: string = '';
  export let duration: number = 3000;
  export let onClose: (() => void) | undefined = undefined;

  let visible = $state(true);

  // Auto-dismiss after duration
  $effect(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        visible = false;
        onClose?.();
      }, duration);

      return () => clearTimeout(timer);
    }
  });

  function handleClose() {
    visible = false;
    onClose?.();
  }
</script>

{#if visible}
  <div class="toast toast-{type}" transition:fly={{ y: -20 }}>
    <div class="toast-icon">
      {#if type === 'success'}✓{/if}
      {#if type === 'error'}✗{/if}
      {#if type === 'warning'}⚠{/if}
      {#if type === 'info'}ℹ{/if}
    </div>

    <div class="toast-content">
      <div class="toast-title">{title}</div>
      {#if message}
        <div class="toast-message">{message}</div>
      {/if}
    </div>

    <button class="toast-close" on:click={handleClose}>×</button>
  </div>
{/if}

<style>
  .toast {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 0.5rem;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    min-width: 300px;
    max-width: 500px;
  }

  .toast-success {
    background: var(--color-success-bg);
    border-left: 4px solid var(--color-success);
  }

  .toast-error {
    background: var(--color-error-bg);
    border-left: 4px solid var(--color-error);
  }

  .toast-warning {
    background: var(--color-warning-bg);
    border-left: 4px solid var(--color-warning);
  }

  .toast-info {
    background: var(--color-info-bg);
    border-left: 4px solid var(--color-info);
  }

  .toast-icon {
    font-size: 1.5rem;
    font-weight: bold;
  }

  .toast-content {
    flex: 1;
  }

  .toast-title {
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .toast-message {
    font-size: 0.875rem;
    opacity: 0.9;
  }

  .toast-close {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    opacity: 0.6;
  }

  .toast-close:hover {
    opacity: 1;
  }
</style>
```

#### Toast Store & Helper

```typescript
// src/lib/stores/toasts.ts

import { writable } from 'svelte/store';

interface Toast {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message?: string;
  duration?: number;
}

const _toasts = writable<Toast[]>([]);

export const toasts = { subscribe: _toasts.subscribe };

export function showToast(
  type: Toast['type'],
  title: string,
  message?: string,
  duration = 3000
): void {
  const id = crypto.randomUUID();

  const toast: Toast = {
    id,
    type,
    title,
    message,
    duration,
  };

  _toasts.update((t) => [...t, toast]);

  // Auto-remove after duration
  if (duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, duration);
  }
}

export function removeToast(id: string): void {
  _toasts.update((t) => t.filter((toast) => toast.id !== id));
}
```

#### FileTree Component

```svelte
<!-- src/lib/components/shared/FileTree.svelte -->
<script lang="ts">
  /**
   * Hierarchische Dateibaum-Komponente.
   *
   * Verwendet für Restore-Browser und Pfad-Auswahl.
   */

  import { createEventDispatcher } from 'svelte';

  interface TreeNode {
    path: string;
    name: string;
    isDir: boolean;
    size?: number;
    children?: TreeNode[];
  }

  interface Props {
    nodes: TreeNode[];
    selectedPaths?: Set<string>;
    onSelect?: (path: string) => void;
    onExpand?: (path: string) => Promise<TreeNode[]>;
  }

  let { nodes, selectedPaths = new Set(), onSelect, onExpand }: Props = $props();

  let expandedPaths = $state(new Set<string>());
  let loadingPaths = $state(new Set<string>());

  const dispatch = createEventDispatcher<{
    select: string;
  }>();

  async function toggleExpand(node: TreeNode) {
    if (!node.isDir) return;

    const isExpanded = expandedPaths.has(node.path);

    if (isExpanded) {
      expandedPaths.delete(node.path);
      expandedPaths = expandedPaths;
    } else {
      // Lazy-load children if needed
      if (!node.children && onExpand) {
        loadingPaths.add(node.path);
        loadingPaths = loadingPaths;

        try {
          node.children = await onExpand(node.path);
        } finally {
          loadingPaths.delete(node.path);
          loadingPaths = loadingPaths;
        }
      }

      expandedPaths.add(node.path);
      expandedPaths = expandedPaths;
    }
  }

  function handleSelect(node: TreeNode) {
    onSelect?.(node.path);
    dispatch('select', node.path);
  }
</script>

<ul class="file-tree">
  {#each nodes as node}
    <li>
      <div
        class="tree-node"
        class:selected={selectedPaths.has(node.path)}
        class:directory={node.isDir}
      >
        {#if node.isDir}
          <button class="expand-btn" on:click={() => toggleExpand(node)}>
            {#if loadingPaths.has(node.path)}
              ⟳
            {:else if expandedPaths.has(node.path)}
              ▼
            {:else}
              ▶
            {/if}
          </button>
        {:else}
          <span class="expand-placeholder"></span>
        {/if}

        <button class="node-label" on:click={() => handleSelect(node)}>
          <span class="icon">
            {node.isDir ? '📁' : '📄'}
          </span>
          <span class="name">{node.name}</span>
          {#if node.size !== undefined}
            <span class="size">{formatBytes(node.size)}</span>
          {/if}
        </button>
      </div>

      {#if node.isDir && expandedPaths.has(node.path) && node.children}
        <svelte:self nodes={node.children} {selectedPaths} {onSelect} {onExpand} />
      {/if}
    </li>
  {/each}
</ul>

<style>
  .file-tree {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .tree-node {
    display: flex;
    align-items: center;
    padding: 0.25rem;
    border-radius: 0.25rem;
  }

  .tree-node:hover {
    background: var(--bg-hover);
  }

  .tree-node.selected {
    background: var(--bg-selected);
  }

  .expand-btn {
    background: none;
    border: none;
    cursor: pointer;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .expand-placeholder {
    width: 1.5rem;
  }

  .node-label {
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    text-align: left;
    padding: 0.25rem;
  }

  .size {
    margin-left: auto;
    font-size: 0.875rem;
    opacity: 0.7;
  }

  ul ul {
    padding-left: 1.5rem;
  }
</style>
```

---

## 🎨 Styling & Theming

### CSS Variables (Design Tokens)

```css
/* src/app.css */

:root {
  /* Colors */
  --color-primary: #3b82f6;
  --color-primary-dark: #2563eb;
  --color-primary-light: #60a5fa;

  --color-secondary: #6b7280;
  --color-secondary-dark: #4b5563;

  --color-success: #10b981;
  --color-success-bg: #d1fae5;

  --color-error: #ef4444;
  --color-error-bg: #fee2e2;

  --color-warning: #f59e0b;
  --color-warning-bg: #fef3c7;

  --color-info: #3b82f6;
  --color-info-bg: #dbeafe;

  /* Backgrounds */
  --bg-primary: #ffffff;
  --bg-secondary: #f3f4f6;
  --bg-tertiary: #e5e7eb;
  --bg-hover: #f9fafb;
  --bg-selected: #e0f2fe;

  /* Text */
  --text-primary: #111827;
  --text-secondary: #6b7280;
  --text-tertiary: #9ca3af;

  /* Borders */
  --border-color: #e5e7eb;
  --border-radius: 0.375rem;

  /* Spacing */
  --spacing-xs: 0.25rem;
  --spacing-sm: 0.5rem;
  --spacing-md: 1rem;
  --spacing-lg: 1.5rem;
  --spacing-xl: 2rem;

  /* Typography */
  --font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'Fira Code', 'Courier New', monospace;

  /* Transitions */
  --transition-fast: 150ms ease;
  --transition-normal: 200ms ease;
  --transition-slow: 300ms ease;
}

/* Dark Mode */
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #111827;
    --bg-secondary: #1f2937;
    --bg-tertiary: #374151;
    --bg-hover: #1f2937;
    --bg-selected: #1e3a8a;

    --text-primary: #f9fafb;
    --text-secondary: #d1d5db;
    --text-tertiary: #9ca3af;

    --border-color: #374151;
  }
}

/* Global Styles */
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  font-family: var(--font-sans);
  background: var(--bg-primary);
  color: var(--text-primary);
}

/* Utility Classes */
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border-width: 0;
}
```

---

## 🧪 Testing Frontend

### Component Tests

```typescript
// src/lib/components/shared/Button.test.ts

import { render, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button Component', () => {
  it('rendert Button mit Label', () => {
    const { getByText } = render(Button, {
      props: { label: 'Klick mich' },
    });

    expect(getByText('Klick mich')).toBeInTheDocument();
  });

  it('feuert click-Event beim Klicken', async () => {
    const handleClick = vi.fn();
    const { getByRole } = render(Button, {
      props: { label: 'Test' },
    });

    const button = getByRole('button');
    button.addEventListener('click', handleClick);

    await fireEvent.click(button);

    expect(handleClick).toHaveBeenCalledOnce();
  });

  it('zeigt Loading-Spinner wenn loading=true', () => {
    const { container } = render(Button, {
      props: { label: 'Test', loading: true },
    });

    const spinner = container.querySelector('.spinner');
    expect(spinner).toBeInTheDocument();
  });

  it('ist disabled wenn disabled=true', () => {
    const { getByRole } = render(Button, {
      props: { label: 'Test', disabled: true },
    });

    const button = getByRole('button') as HTMLButtonElement;
    expect(button.disabled).toBe(true);
  });
});
```

### Store Tests

```typescript
// src/lib/stores/repositories.test.ts

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';
import {
  repositories,
  loadRepositories,
  addRepository,
  removeRepository,
  reset,
} from './repositories';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('repositories store', () => {
  beforeEach(() => {
    reset();
    vi.clearAllMocks();
  });

  it('startet mit leerem Array', () => {
    const repos = get(repositories);
    expect(repos).toEqual([]);
  });

  it('lädt Repositories von Backend', async () => {
    const mockRepos = [
      { id: '1', name: 'Repo 1', path: '/path/1' },
      { id: '2', name: 'Repo 2', path: '/path/2' },
    ];

    vi.mocked(invoke).mockResolvedValue(mockRepos);

    await loadRepositories();

    const repos = get(repositories);
    expect(repos).toEqual(mockRepos);
  });

  it('fügt Repository hinzu', async () => {
    const newRepo = { id: '3', name: 'New Repo', path: '/path/3' };

    vi.mocked(invoke).mockResolvedValue(undefined);

    await addRepository(newRepo);

    const repos = get(repositories);
    expect(repos).toContainEqual(newRepo);
  });
});
```

---

## 🆕 Neue Dialog-Komponenten (seit 2025-10-30)

### LocationPickerDialog.svelte

Vereinheitlichter Location-Picker für alle Repository-Backend-Typen.

**Verwendung:**

- In `AddRepositoryDialog.svelte` als Haupt-Input
- Ersetzt separate Inputs für Local/SFTP/S3/rclone

**Features:**

- **4 Tabs:**
  - Local: Filesystem-Browser (mit OS-native Dialoge)
  - Network: SFTP-Konfiguration (Host, Port, User, Path)
  - Cloud: S3-kompatible Backends (Bucket, Region, Prefix)
  - Recent: Zuletzt verwendete Locations (gespeichert in Settings)
- **Smart-Input mit Auto-Detection:**
  - Erkennt Location-Typ automatisch (z.B. `sftp://...` → Network-Tab)
  - Validiert Format in Echtzeit
  - Zeigt Typ-spezifische Felder
- **Connection-Test:**
  - Button "Test Connection"
  - Backend-Command: `test_repository_connection()`
  - Zeigt Erfolg/Fehler mit Details
- **Favoriten:**
  - Speichern-Button für häufig genutzte Locations
  - Gespeichert in `settings.toml`
  - Dropdown zur schnellen Auswahl

**Props:**

```svelte
export let open = false; // Dialog-Sichtbarkeit export let initialLocation = ''; // Vorausgefüllte
Location export let allowedTypes = ['local', 'sftp', 's3', 'rclone']; // Erlaubte Typen
```

**Events:**

```svelte
on:select={(e) => {
  location = e.detail.location;
}}
on:cancel
```

**Backend-Integration:**

- Backend-Command: `test_repository_connection(location: String)`
- Settings-API: `save_favorite_location()`, `list_favorite_locations()`

**Mockup:** `docs/mockups/rustic_location_picker.html`

**Beispiel:**

```svelte
<script lang="ts">
  import LocationPickerDialog from '$lib/components/dialogs/LocationPickerDialog.svelte';

  let showLocationPicker = false;
  let selectedLocation = '';

  function handleLocationSelect(event: CustomEvent<{ location: string }>) {
    selectedLocation = event.detail.location;
    showLocationPicker = false;
  }
</script>

<Button on:click={() => (showLocationPicker = true)}>Repository-Location wählen</Button>

<LocationPickerDialog
  bind:open={showLocationPicker}
  initialLocation={selectedLocation}
  on:select={handleLocationSelect}
/>
```

---

### PruneRepoDialog.svelte

Prune-Dialog für Repository-Bereinigung (Löschen ungenutzter Daten).

**Features:**

- **Dry-Run-Modus:**
  - Checkbox "Nur Vorschau (kein Löschen)"
  - Backend-Command mit `dry_run: bool`-Flag
  - Zeigt was gelöscht würde ohne zu löschen
- **Statistik-Anzeige:**
  - Vorher/Nachher-Größe
  - Freed Space (in GB)
  - Anzahl gelöschter Pack-Files
- **Confirmation-Workflow:**
  - Warnung: "Diese Aktion kann nicht rückgängig gemacht werden"
  - Zwei-Schritt-Bestätigung bei Dry-Run deaktiviert
  - Progress-Bar während Prune-Operation
- **Prune-Optionen:**
  - Max. unused (z.B. "10% ungenutzter Space erlaubt")
  - Keep snapshots (Retention-Policy)

**Props:**

```svelte
export let open = false; export let repositoryId: string;
```

**Backend-Integration:**

- Backend-Command: `prune_repository(repo_id: String, dry_run: bool, options: PruneOptions)`
- Event: `prune-progress` (für Live-Updates)

**Mockup:** `docs/mockups/rustic_advanced_functions.html` (Prune-Section)

**Beispiel:**

```svelte
<PruneRepoDialog
  bind:open={showPruneDialog}
  repositoryId={currentRepo.id}
  on:complete={() => {
    toast.success('Prune erfolgreich abgeschlossen');
    refreshRepoStats();
  }}
/>
```

---

### SnapshotInfoDialog.svelte

Detail-Ansicht für einzelnen Snapshot mit vollständigen Metadaten.

**Features:**

- **Metadaten-Anzeige:**
  - Snapshot-ID (mit Copy-Button)
  - Timestamp (formatiert)
  - Hostname, Username
  - Tags (mit Edit-Button)
  - Parent-Snapshot (Link zum Öffnen)
- **Statistiken:**
  - Files: `12,345 Dateien`
  - Directories: `1,234 Ordner`
  - Total Size: `42.5 GB`
  - Added Data: `+2.1 GB` (seit Parent)
- **Aktionen:**
  - Button "Restore" → Öffnet `RestoreDialog` mit diesem Snapshot
  - Button "Compare" → Öffnet `CompareSnapshotsDialog`
  - Button "Delete Snapshot" (mit Bestätigung)
  - Button "Add/Remove Tags"
- **Erweiterte Infos (Collapsible):**
  - Backup-Duration
  - Command (wie Backup gestartet wurde)
  - Original Paths

**Props:**

```svelte
export let open = false; export let snapshotId: string;
```

**Backend-Integration:**

- Backend-Command: `get_snapshot_info(snapshot_id: String)`
- Returns: `SnapshotInfo` (vollständiges Objekt)

**Mockup:** `docs/mockups/rustic_restore_dialogs.html` (Snapshot Info-Section)

**Beispiel:**

```svelte
<SnapshotInfoDialog
  bind:open={showSnapshotInfo}
  snapshotId={selectedSnapshot.id}
  on:restore={(e) => openRestoreDialog(e.detail.snapshotId)}
  on:delete={handleSnapshotDelete}
/>
```

---

## 📐 Layout-Patterns

### Per-Page Header Pattern (seit Nov 2025)

**⚠️ WICHTIG:** Seit November 2025 hat **jede Page ihren eigenen Header** mit spezifischen Action-Buttons. Der globale `Header.svelte` wurde entfernt!

#### Warum der Wechsel?

**Früher:**

- Globaler `Header.svelte` in `MainLayout.svelte`
- Komplexe Prop/Snippet-Übergabe für Page-spezifische Buttons
- Schwer wartbar, unflexibel

**Jetzt:**

- Jede Page verwaltet eigenen Header
- Einfacher Code, volle Kontrolle
- Lokale Änderungen ohne globale Anpassungen

#### Standard-Implementation

**Jede Page sollte diesem Pattern folgen:**

```svelte
<script lang="ts">
  import Button from '$lib/components/shared/Button.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';

  // Page-spezifische Logik
  function handleAction() {
    // ...
  }
</script>

<div class="page-wrapper">
  <!-- Page Header mit Actions -->
  <div class="page-header">
    <h1 class="page-title">Seitenname</h1>
    <div class="header-actions">
      <Tooltip text="Beschreibung">
        <Button variant="primary" size="sm" onclick={handleAction}>➕ Add</Button>
      </Tooltip>

      <!-- Weitere Actions nach Bedarf -->
      <Tooltip text="Weitere Aktion">
        <Button variant="secondary" size="sm" onclick={handleRefresh}>🔄 Refresh</Button>
      </Tooltip>
    </div>
  </div>

  <!-- Page Content -->
  <div class="page-content">
    <!-- Dein Inhalt -->
  </div>
</div>
```

#### CSS für Page-Header

**Standard-Styles (in jeder Page):**

```css
.page-wrapper {
  width: 100%;
  min-width: 320px;
  max-width: 1600px; /* Oder je nach Page: 1200px für Settings, etc. */
  margin: 0 auto;
  padding: 0 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 0;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 24px;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  margin-left: auto; /* Rechtsbündig */
}
```

#### Button-Emoji-Konventionen

**Nutze diese Emojis konsistent über alle Pages:**

| Emoji | Bedeutung          | Beispiel                             |
| ----- | ------------------ | ------------------------------------ |
| ➕    | Add/Create         | "➕ Add Repository", "➕ Create Job" |
| 📂    | Open/Browse        | "📂 Open Repository"                 |
| 🔄    | Refresh/Reload     | "🔄 Refresh", "🔄 Reload Data"       |
| 🗑️    | Delete/Remove      | "🗑️ Delete", "🗑️ Remove"             |
| ⚙️    | Configure/Settings | "⚙️ Configure", "⚙️ Settings"        |

**Beispiel:**

```svelte
<Button variant="primary" size="sm" onclick={handleAdd}>➕ Add</Button>
```

#### Implementierte Pages

**Stand 2025-11-04:**

| Page         | Header-Actions  | Datei                  | Zeilen  |
| ------------ | --------------- | ---------------------- | ------- |
| Dashboard    | 🔄 Refresh      | `DashboardPage.svelte` | 93-111  |
| Repositories | ➕ Add, 📂 Open | `Repositories.svelte`  | 226-240 |
| Snapshots    | 🔄 Refresh      | `Snapshots.svelte`     | -       |
| Backup Jobs  | ➕ Create Job   | `BackupJobs.svelte`    | 225-236 |
| Settings     | 🔄 Reset        | `Settings.svelte`      | -       |

#### Best Practices

✅ **DO:**

- Header in jeder Page separat implementieren
- Button-Emojis konsistent verwenden
- `size="sm"` für Header-Buttons
- Tooltips für bessere UX
- `margin-left: auto` für rechtsbündige Actions

❌ **DON'T:**

- Versuche **nicht**, einen globalen Header zu verwenden
- Keine Props an MainLayout für Header-Content
- Keine Event-Bubbling über mehrere Ebenen für Page-Actions

#### Migration-Beispiel (falls du alte Pages findest)

**Vorher (ALT - nicht mehr verwenden):**

```svelte
<!-- MainLayout.svelte -->
<Header>
  {#snippet actions()}
    <!-- Props/Snippets für alle Pages -->
  {/snippet}
</Header>
```

**Nachher (NEU - aktuelles Pattern):**

```svelte
<!-- InDerPage.svelte -->
<div class="page-header">
  <h1 class="page-title">Meine Page</h1>
  <div class="header-actions">
    <Button size="sm">➕ Add</Button>
  </div>
</div>
```

---

## ✅ Frontend-Checkliste

### Vor Implementierung

- [ ] Mockup existiert und geprüft
- [ ] Komponenten-Struktur geplant
- [ ] Types definiert
- [ ] API-Integration geplant

### Während Implementierung

- [ ] UI folgt Mockup
- [ ] Responsive Design
- [ ] Keyboard-Navigation funktioniert
- [ ] Loading-States vorhanden
- [ ] Error-Handling implementiert
- [ ] Types korrekt
- [ ] Kommentare auf Deutsch

### Nach Implementierung

- [ ] Component Tests geschrieben
- [ ] Manuell getestet
- [ ] Dark Mode geprüft
- [ ] Performance akzeptabel
- [ ] Accessibility (A11y) beachtet

---

**Version**: 1.2  
**Letzte Aktualisierung**: 2025-11-04  
**Wichtige Änderungen:** Per-Page Header Pattern dokumentiert
