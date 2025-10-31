<!-- DeleteJobDialog.svelte: Bestätigungsdialog für Job-Löschung -->
<!--
  TODO.md: Phase 2 - Dialog-Workflow: Backup & Restore ✅ IMPLEMENTIERT
  Referenz: TODO.md Zeile 237-244, 332-340
  
  Status:
  - handleDelete an api.deleteBackupJob angebunden ✅ (TODO.md Zeile 240)
  
  Backend-Command: delete_backup_job
  - src-tauri/src/commands/backup.rs:217
  
  API-Wrapper: src/lib/api/backup-jobs.ts:deleteBackupJob
  
  Verwendung:
  - src/lib/components/pages/BackupJobs.svelte
-->
<script lang="ts">
  /**
   * Bestätigungsdialog für Backup-Job-Löschung.
   *
   * Zeigt Job-Details und erfordert Bestätigung vor Löschung.
   *
   * @component
   *
   * @example
   * ```svelte
   * <DeleteJobDialog
   *   bind:open={showDelete}
   *   {job}
   *   on:deleted={handleJobDeleted}
   * />
   * ```
   */
  import Button from '$lib/components/shared/Button.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupJobDto } from '$lib/types';
  import { createEventDispatcher } from 'svelte';

  interface DeleteJobDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Zu löschender Job */
    job?: BackupJobDto | null;
  }

  let { open = $bindable(false), job = null }: DeleteJobDialogProps = $props();

  const dispatch = createEventDispatcher();

  let isDeleting = false;
  let confirmText = '';

  $: isConfirmValid = confirmText === 'LÖSCHEN';

  async function handleDelete() {
    if (!job || !isConfirmValid) return;

    isDeleting = true;
    try {
      const { deleteBackupJob } = await import('$lib/api/backup-jobs');
      await deleteBackupJob(job.id);

      console.log('Job deleted:', job.id);
      dispatch('deleted', job);
      open = false;
      confirmText = '';
      toastStore.success(`Backup-Job "${job.name}" wurde gelöscht`);
    } catch (error) {
      console.error('Failed to delete job:', error);
      toastStore.error('Fehler beim Löschen des Backup-Jobs');
    } finally {
      isDeleting = false;
    }
  }

  function handleCancel() {
    open = false;
    confirmText = '';
  }
</script>

<Modal bind:open ariaLabel="Backup-Job löschen">
  <div class="delete-job-dialog">
    {#if job}
      <div class="warning-content">
        <div class="warning-icon">⚠️</div>

        <div class="warning-text">
          <h3 class="warning-title">Job wirklich löschen?</h3>
          <p class="warning-description">
            Der Backup-Job "<strong>{job.name}</strong>" wird unwiderruflich gelöscht. Alle
            zugehörigen Snapshots bleiben erhalten, aber der Job wird nicht mehr automatisch
            ausgeführt.
          </p>
        </div>

        <div class="confirmation-input">
          <label for="confirm-input" class="confirm-label">
            Geben Sie <strong>LÖSCHEN</strong> ein, um zu bestätigen:
          </label>
          <input
            id="confirm-input"
            type="text"
            class="confirm-text-input"
            bind:value={confirmText}
            placeholder="LÖSCHEN"
            disabled={isDeleting}
          />
        </div>
      </div>
    {/if}

    <div class="dialog-actions">
      <Button variant="secondary" onclick={handleCancel} disabled={isDeleting}>Abbrechen</Button>
      <Button variant="danger" onclick={handleDelete} disabled={!isConfirmValid || isDeleting}>
        {#if isDeleting}
          Lösche...
        {:else}
          Endgültig löschen
        {/if}
      </Button>
    </div>
  </div>
</Modal>

<style>
  .delete-job-dialog {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .warning-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1.5rem;
  }

  .warning-icon {
    font-size: 3rem;
    opacity: 0.8;
  }

  .warning-text {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .warning-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .warning-description {
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .confirmation-input {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .confirm-label {
    font-size: 0.875rem;
    color: var(--text-primary);
    text-align: left;
  }

  .confirm-text-input {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    text-align: center;
    font-weight: 500;
    text-transform: uppercase;
  }

  .confirm-text-input:focus {
    outline: none;
    border-color: var(--color-error);
    box-shadow: 0 0 0 2px rgba(239, 68, 68, 0.1);
  }

  .confirm-text-input::placeholder {
    color: var(--text-secondary);
    opacity: 0.6;
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color);
  }

  @media (max-width: 768px) {
    .warning-content {
      gap: 1rem;
    }

    .warning-icon {
      font-size: 2.5rem;
    }

    .warning-title {
      font-size: 1.125rem;
    }

    .dialog-actions {
      flex-direction: column;
    }
  }
</style>
