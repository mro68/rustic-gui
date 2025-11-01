<!--
  RunBackupDialog.svelte
  ---------------------
  TODO.md: Phase 2 - Dialog-Workflow: Backup & Restore (Zeile 258)
  Status: ✅ KOMPLETT - API-Integration vollständig
  
  Backend-Command: src-tauri/src/lib.rs:121 (run_backup_command)
  API-Wrapper: src/lib/api/backup.ts:runBackup
  
  Implementierung:
  - ✅ API-Integration mit runBackup
  - ✅ Event-Listener für Progress, Completed, Failed
  - ✅ Error-Handling mit Toasts
  - ✅ Retry-Funktionalität
  - ✅ Log-Anzeige
-->
<script lang="ts">
  /**
   * Progress-Dialog für laufende Backup-Jobs.
   *
   * Zeigt Echtzeit-Progress via Tauri Events mit:
   * - Fortschrittsbalken (Dateien/Bytes)
   * - Error-Handling & Retry
   * - Log-Anzeige
   *
   * @component
   *
   * @example
   * ```svelte
   * <RunBackupDialog
   *   bind:open={showProgress}
   *   jobId={activeJobId}
   *   jobName={activeJobName}
   *   on:completed={handleBackupComplete}
   * />
   * ```
   */
  import { runBackup } from '$lib/api/backup';
  import { onBackupCompleted, onBackupFailed, onBackupProgress } from '$lib/api/events';
  import { setError, setLoading, setRunningJobId } from '$lib/stores/backup-jobs';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupProgress } from '$lib/types';
  import { formatBytes } from '$lib/utils/format';
  import { onDestroy, onMount } from 'svelte';
  import Modal from '../shared/Modal.svelte';
  import ProgressBar from '../shared/ProgressBar.svelte';

  interface RunBackupDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Job-Name für Anzeige */
    jobName?: string;
    /** Job-ID für Backend-Commands */
    jobId?: string;
    /** Callback bei Cancel */
    onCancel?: (() => void) | undefined;
  }

  let {
    open = $bindable(false),
    jobName = '',
    jobId = '',
    onCancel = undefined,
  }: RunBackupDialogProps = $props();

  // Retry-Handler
  async function handleRetry() {
    error = null;
    completed = false;
    progress = null;
    logLines = [];
    await setupListeners();
    try {
      await runBackup(jobId);
      // Wenn kein Fehler geworfen wurde und completed true ist, Toast anzeigen
      if (completed) {
        toastStore.success(`Backup für "${jobName}" erfolgreich wiederholt!`);
      }
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      toastStore.error(`Backup-Retry fehlgeschlagen: ${error}`);
    }
  }

  let progress: BackupProgress | null = $state(null);
  let error: string | null = $state(null);
  let completed = $state(false);
  let logLines: string[] = $state([]);
  let unlistenProgress: (() => void) | null = null;
  let unlistenCompleted: (() => void) | null = null;
  let unlistenFailed: (() => void) | null = null;

  function close() {
    // Modal darf nur geschlossen werden, wenn abgeschlossen oder Fehler
    if (completed || error) {
      open = false;
      progress = null;
      error = null;
      completed = false;
      logLines = [];
      setRunningJobId(null);
      setLoading(false);
      setError(null);
    }
  }

  function addLogLine(line: string) {
    logLines = [...logLines.slice(-49), line]; // max 50 Zeilen
  }

  async function setupListeners() {
    setRunningJobId(jobId);
    setLoading(true);
    setError(null);
    unlistenProgress = await onBackupProgress((data) => {
      progress = data;
      setRunningJobId(jobId);
      setLoading(true);
      setError(null);
      if (data.message) addLogLine(`[${new Date().toLocaleTimeString()}] ${data.message}`);
    });
    unlistenCompleted = await onBackupCompleted((data) => {
      if (data.jobId === jobId && data.success) {
        completed = true;
        setRunningJobId(null);
        setLoading(false);
        setError(null);
        addLogLine(`[${new Date().toLocaleTimeString()}] Backup abgeschlossen!`);
        toastStore.success(`Backup für "${jobName}" abgeschlossen!`);
      }
    });
    unlistenFailed = await onBackupFailed((data) => {
      if (data.jobId === jobId) {
        error = data.error;
        setRunningJobId(null);
        setLoading(false);
        setError(data.error);
        addLogLine(`[${new Date().toLocaleTimeString()}] Fehler: ${data.error}`);
        toastStore.error(`Backup fehlgeschlagen: ${data.error}`);
      }
    });
  }

  onMount(() => {
    if (open) setupListeners();
  });
  onDestroy(() => {
    unlistenProgress?.();
    unlistenCompleted?.();
    unlistenFailed?.();
  });
</script>

<Modal
  bind:open
  ariaLabel="Backup läuft"
  closeOnEsc={!!(completed || error)}
  closeOnBackdrop={!!(completed || error)}
  on:close={close}
>
  {#snippet header()}
    <h2 class="modal-title">Backup läuft</h2>
  {/snippet}
  <div>
    <div
      class="info-box"
      style="background: rgba(34, 197, 94, 0.1); border-color: rgba(34, 197, 94, 0.3); color: #4ade80; margin-bottom: 18px;"
    >
      🔄 Backup-Job "{jobName}" läuft...
    </div>
    {#if progress}
      <!-- Dateien-Fortschritt -->
      <div class="form-group">
        <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
          <span style="font-size: 13px; color: #e4e4e7;">Dateien</span>
          <span style="font-size: 13px; color: #71717a;"
            >{progress.files_processed?.toLocaleString()} / {progress.total?.toLocaleString()} ({progress.percentage}%)</span
          >
        </div>
        <ProgressBar value={progress.percentage} label={undefined} />
      </div>
      <!-- Upload-Fortschritt -->
      <div class="form-group">
        <div style="display: flex; justify-content: space-between; margin-bottom: 8px;">
          <span style="font-size: 13px; color: #e4e4e7;">Upload</span>
          <span style="font-size: 13px; color: #71717a;"
            >{formatBytes(progress.bytes_processed)} / {formatBytes(
              progress.total_bytes ?? 0
            )}</span
          >
        </div>
        <ProgressBar
          value={progress.total_bytes
            ? Math.round((progress.bytes_processed / progress.total_bytes) * 100)
            : 0}
          label={undefined}
        />
      </div>
      <div
        style="background: #1a1d2e; border: 1px solid #2d3348; border-radius: 8px; padding: 16px; font-family: 'Courier New', monospace; font-size: 12px; max-height: 200px; overflow-y: auto; margin-bottom: 8px;"
      >
        {#each logLines as line}
          <div
            style="margin-bottom: 6px; color: {line.includes('Fehler')
              ? '#ef4444'
              : line.includes('abgeschlossen')
                ? '#22c55e'
                : '#4ade80'}"
          >
            {line}
          </div>
        {/each}
      </div>
      <div class="progress-details" style="margin-top: 12px; font-size: 14px;">
        {#if progress.current_file}
          <div>Aktuelle Datei: {progress.current_file}</div>
        {/if}
        {#if progress.estimated_time_remaining}
          <div>
            Geschätzte Restzeit: {Math.round(progress.estimated_time_remaining / 60)} Minuten
          </div>
        {/if}
      </div>
    {:else}
      <ProgressBar value={undefined} label="Backup wird initialisiert..." />
    {/if}
    {#if error}
      <div class="error-message" style="margin-top: 18px; color: #ef4444;">Fehler: {error}</div>
    {/if}
    {#if completed}
      <div class="success-message" style="margin-top: 18px; color: #22c55e;">
        Backup abgeschlossen!
      </div>
    {/if}
  </div>
  {#snippet footer()}
    {#if !completed && !error}
      <button class="btn btn-secondary" onclick={onCancel}>Backup abbrechen</button>
    {:else if error}
      <button class="btn btn-primary" onclick={handleRetry}>Erneut versuchen</button>
      <button class="btn btn-secondary" onclick={close}>Schließen</button>
    {:else}
      <button class="btn btn-primary" onclick={close}>Schließen</button>
    {/if}
  {/snippet}
</Modal>

<style>
  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    padding: 12px 16px;
    font-size: 13px;
    color: #93c5fd;
    margin-bottom: 20px;
  }
  .progress-details {
    background: var(--bg-tertiary);
    border-radius: 8px;
    border: 1px solid var(--border-color);
    padding: 12px;
    font-family: 'Courier New', monospace;
    font-size: 12px;
  }
  .progress-details > div {
    margin-bottom: 6px;
  }
  .progress-details > div:last-child {
    margin-bottom: 0;
  }
  .error-message {
    color: var(--color-error);
    font-size: 14px;
    margin-top: 12px;
  }
  .success-message {
    color: var(--color-success);
    font-size: 14px;
    margin-top: 12px;
  }
</style>
