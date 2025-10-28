<!--
  RunBackupDialog.svelte
  ---------------------
  Fortschritts-Dialog für laufende Backups (gemäß Mockup)
-->
<script lang="ts">
  import { onBackupCompleted, onBackupFailed, onBackupProgress } from '$lib/api/events';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupProgress } from '$lib/types';
  import { onDestroy, onMount } from 'svelte';
  import Modal from '../shared/Modal.svelte';
  import ProgressBar from '../shared/ProgressBar.svelte';

  export let open: boolean = false;
  export let jobName: string = '';
  export let jobId: string = '';
  export let onCancel: (() => void) | undefined = undefined;

  let progress: BackupProgress | null = null;
  let error: string | null = null;
  let completed = false;
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
    }
  }

  async function setupListeners() {
    unlistenProgress = await onBackupProgress((data) => {
      progress = data;
    });
    unlistenCompleted = await onBackupCompleted((data) => {
      if (data.jobId === jobId && data.success) {
        completed = true;
        toastStore.success(`Backup für "${jobName}" abgeschlossen!`);
      }
    });
    unlistenFailed = await onBackupFailed((data) => {
      if (data.jobId === jobId) {
        error = data.error;
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
  <div slot="header">
    <h2 class="modal-title">Backup läuft</h2>
  </div>
  <div>
    <div
      class="info-box"
      style="background: rgba(34, 197, 94, 0.1); border-color: rgba(34, 197, 94, 0.3); color: #4ade80; margin-bottom: 18px;"
    >
      🔄 Backup-Job "{jobName}" läuft...
    </div>
    {#if progress}
      <ProgressBar value={progress.percentage} label={`Fortschritt: ${progress.percentage}%`} />
      <div class="progress-details" style="margin-top: 12px; font-size: 14px;">
        <div>Dateien verarbeitet: {progress.files_processed}</div>
        <div>Bytes verarbeitet: {progress.bytes_processed}</div>
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
  <div slot="footer">
    {#if !completed && !error}
      <button class="btn btn-secondary" on:click={onCancel}>Backup abbrechen</button>
    {:else}
      <button class="btn btn-primary" on:click={close}>Schließen</button>
    {/if}
  </div>
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
