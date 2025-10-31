<script lang="ts">
  /**
   * Bestätigungsdialog für Repository-Löschung.
   *
   * Erfordert Eingabe des Repository-Namens zur Bestätigung.
   * Optional: Löschung der Backup-Daten.
   *
   * @component
   *
   * @example
   * ```svelte
   * <DeleteRepoDialog
   *   {repository}
   *   on:delete-repo={handleRepoDeleted}
   * />
   * ```
   */
  import { toastStore } from '$lib/stores/toast';
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Modal from '../shared/Modal.svelte';

  const dispatch = createEventDispatcher();

  import type { RepositoryDto } from '$lib/types';

  interface DeleteRepoDialogProps {
    /** Zu löschendes Repository */
    repository?: RepositoryDto | null;
  }

  let { repository = null }: DeleteRepoDialogProps = $props();

  let confirmName = $state('');
  let deleteData = false;
  let isDeleting = false;

  const isValid = $derived(confirmName === (repository?.name || ''));

  async function handleDelete() {
    if (!isValid || !repository) return;

    isDeleting = true;
    try {
      const { deleteRepository } = await import('$lib/api/repositories');
      await deleteRepository(repository.id, deleteData);

      toastStore.success('Repository erfolgreich gelöscht');

      dispatch('delete-repo', {
        repositoryId: repository.id,
        deleteData,
      });
    } catch (error: any) {
      // ✅ Error-Toast hinzugefügt (TODO.md Zeile 247)
      const errorMessage = error?.message || 'Unbekannter Fehler';
      toastStore.error('Repository löschen fehlgeschlagen: ' + errorMessage);
      console.error('Failed to delete repository:', error);
    } finally {
      isDeleting = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }

  // eslint-disable-next-line no-unused-vars
  function resetForm() {
    // TODO: Call when dialog closes or add reset button
    confirmName = '';
    deleteData = false;
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository löschen</div>
  <div class="delete-repo-dialog">
    <div class="warning-section">
      <div class="warning-icon">⚠️</div>
      <div class="warning-content">
        <h3>Gefahr: Unwiderrufliche Löschung</h3>
        <p>
          Sie sind dabei, das Repository <strong>{repository?.name || 'Unbekannt'}</strong> zu
          löschen. Diese Aktion kann <strong>nicht rückgängig</strong> gemacht werden.
        </p>
      </div>
    </div>

    {#if repository}
      <div class="repo-info">
        <h4>Repository-Details</h4>
        <div class="info-grid">
          <div class="info-row">
            <span class="info-label">Name:</span>
            <span>{repository.name}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Pfad:</span>
            <span class="mono">{repository.path}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Typ:</span>
            <span>{repository.type || 'Lokal'}</span>
          </div>
          <div class="info-row">
            <span class="info-label">Snapshots:</span>
            <span>{repository.snapshotCount || 0}</span>
          </div>
        </div>
      </div>
    {/if}

    <div class="confirmation-section">
      <h4>Bestätigung erforderlich</h4>
      <p>Geben Sie den Namen des Repositorys ein, um die Löschung zu bestätigen:</p>
      <input
        type="text"
        placeholder="Repository-Name eingeben"
        bind:value={confirmName}
        class="confirm-input"
      />
      {#if confirmName && !isValid}
        <div class="error-message">
          Der eingegebene Name stimmt nicht mit dem Repository-Namen überein.
        </div>
      {/if}
    </div>

    <div class="options-section">
      <Checkbox label="Repository-Daten vom Datenträger löschen" bind:checked={deleteData} />
      <p class="option-description">
        Wenn aktiviert, werden auch die gespeicherten Backup-Daten vom Datenträger entfernt.
        Andernfalls bleiben die Daten erhalten, aber das Repository wird aus der Verwaltung
        entfernt.
      </p>
    </div>

    <div class="danger-notice">
      <div class="danger-icon">🚨</div>
      <div class="danger-text">
        <strong>Letzte Warnung:</strong> Stellen Sie sicher, dass Sie alle wichtigen Daten wiederhergestellt
        haben. Nach dem Löschen sind die Daten nicht mehr zugänglich.
      </div>
    </div>
  </div>

  <div slot="footer">
    <Button variant="secondary" onclick={handleClose} disabled={isDeleting}>Abbrechen</Button>
    <Button variant="danger" onclick={handleDelete} disabled={!isValid || isDeleting}>
      {#if isDeleting}
        Lösche Repository...
      {:else}
        Repository löschen
      {/if}
    </Button>
  </div>
</Modal>

<style>
  .delete-repo-dialog {
    max-width: 600px;
  }

  .warning-section {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    padding: 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
    margin-bottom: 24px;
  }

  .warning-icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .warning-content h3 {
    font-size: 16px;
    font-weight: 600;
    color: #dc2626;
    margin-bottom: 8px;
  }

  .warning-content p {
    color: #dc2626;
    line-height: 1.5;
    margin: 0;
  }

  .repo-info {
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 24px;
  }

  .repo-info h4 {
    font-size: 14px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .info-row .info-label {
    font-size: 12px;
    font-weight: 500;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .info-row span {
    font-size: 14px;
    color: #e4e4e7;
  }

  .mono {
    font-family: 'Courier New', monospace;
    font-size: 13px;
  }

  .confirmation-section {
    margin-bottom: 24px;
  }

  .confirmation-section h4 {
    font-size: 14px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 8px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .confirmation-section p {
    color: #71717a;
    font-size: 14px;
    line-height: 1.5;
    margin-bottom: 12px;
  }

  .confirm-input {
    width: 100%;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    padding: 10px 12px;
    color: #e4e4e7;
    font-size: 14px;
  }

  .confirm-input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .confirm-input::placeholder {
    color: #71717a;
  }

  .error-message {
    margin-top: 8px;
    color: #ef4444;
    font-size: 12px;
    font-weight: 500;
  }

  .options-section {
    margin-bottom: 24px;
  }

  .option-description {
    margin-top: 8px;
    font-size: 13px;
    color: #71717a;
    line-height: 1.4;
  }

  .danger-notice {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    padding: 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .danger-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .danger-text {
    font-size: 14px;
    color: #dc2626;
    line-height: 1.5;
  }

  .danger-text strong {
    color: #b91c1c;
  }

  @media (max-width: 768px) {
    .info-grid {
      grid-template-columns: 1fr;
    }

    .info-row {
      justify-content: flex-start;
      gap: 8px;
    }

    .warning-section,
    .danger-notice {
      flex-direction: column;
      text-align: center;
    }
  }
</style>
