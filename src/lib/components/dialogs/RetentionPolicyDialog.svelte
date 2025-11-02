<!-- RetentionPolicyDialog.svelte: Apply Retention Policy to Snapshots -->
<script lang="ts">
  /**
   * Retention-Policy Dialog
   * Task 4.3: Retention-Policy Apply
   *
   * Ermöglicht Konfiguration und Anwendung von Retention-Policies auf Snapshots.
   * - Preview-Modus: Zeigt welche Snapshots gelöscht würden (Dry-Run)
   * - Apply-Modus: Löscht Snapshots tatsächlich gemäß Policy
   *
   * @component
   *
   * @example
   * ```svelte
   * <RetentionPolicyDialog
   *   bind:open={showRetentionDialog}
   *   on:applied={handlePolicyApplied}
   * />
   * ```
   */
  import { createEventDispatcher } from 'svelte';
  import { applyRetentionPolicy, previewRetentionPolicy } from '$lib/api/retention';
  import type { RetentionPolicy, RetentionPolicyPreview } from '$lib/api/retention';
  import { toastStore } from '$lib/stores/toast';
  import Button from '$lib/components/shared/Button.svelte';
  import Input from '$lib/components/shared/Input.svelte';

  interface RetentionPolicyDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
  }

  let { open = $bindable(false) }: RetentionPolicyDialogProps = $props();

  const dispatch = createEventDispatcher<{
    applied: { deleted_count: number };
    cancel: void;
  }>();

  // Policy-Konfiguration (Default-Werte)
  let policy = $state<RetentionPolicy>({
    keep_last: 10,
    keep_daily: 7,
    keep_weekly: 4,
    keep_monthly: 12,
    keep_yearly: 2,
  });

  // Preview-State
  let preview = $state<RetentionPolicyPreview | null>(null);
  let isLoadingPreview = $state(false);
  let isApplying = $state(false);
  let showPreview = $state(false);

  // Reset wenn Dialog geschlossen wird
  $effect(() => {
    if (!open) {
      resetState();
    }
  });

  function resetState() {
    preview = null;
    showPreview = false;
    isLoadingPreview = false;
    isApplying = false;
    // Policy-Werte zurücksetzen auf Defaults
    policy = {
      keep_last: 10,
      keep_daily: 7,
      keep_weekly: 4,
      keep_monthly: 12,
      keep_yearly: 2,
    };
  }

  async function handlePreview() {
    isLoadingPreview = true;
    try {
      preview = await previewRetentionPolicy(policy);
      showPreview = true;
      toastStore.info(`Preview: ${preview.snapshots_to_delete.length} Snapshots würden gelöscht`);
    } catch (error: any) {
      console.error('Failed to preview retention policy:', error);
      toastStore.error(
        `Fehler beim Preview: ${error.message || 'Unbekannter Fehler'}`
      );
    } finally {
      isLoadingPreview = false;
    }
  }

  async function handleApply() {
    if (!preview) {
      toastStore.warning('Bitte zuerst Preview anzeigen');
      return;
    }

    if (preview.snapshots_to_delete.length === 0) {
      toastStore.info('Keine Snapshots zum Löschen gemäß Policy');
      handleClose();
      return;
    }

    const confirmMessage = `Sind Sie sicher, dass Sie ${preview.snapshots_to_delete.length} Snapshot(s) löschen möchten?\n\nDieser Vorgang kann nicht rückgängig gemacht werden!`;
    
    if (!confirm(confirmMessage)) {
      return;
    }

    isApplying = true;
    try {
      const deleted_ids = await applyRetentionPolicy(policy);
      toastStore.success(`${deleted_ids.length} Snapshots erfolgreich gelöscht`);
      dispatch('applied', { deleted_count: deleted_ids.length });
      handleClose();
    } catch (error: any) {
      console.error('Failed to apply retention policy:', error);
      toastStore.error(
        `Fehler beim Anwenden der Policy: ${error.message || 'Unbekannter Fehler'}`
      );
    } finally {
      isApplying = false;
    }
  }

  function handleClose() {
    open = false;
    dispatch('cancel');
  }

  // Validierung: Mindestens ein Keep-Wert muss gesetzt sein
  function isValidPolicy(): boolean {
    return !!(
      policy.keep_last ||
      policy.keep_daily ||
      policy.keep_weekly ||
      policy.keep_monthly ||
      policy.keep_yearly
    );
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={handleClose} role="presentation">
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()}>
      <header class="modal-header">
        <h2>Retention-Policy anwenden</h2>
        <button class="modal-close" aria-label="Schließen" onclick={handleClose}> &times; </button>
      </header>

      <div class="modal-content">
        <div class="policy-config">
          <p class="description">
            Konfigurieren Sie wie viele Snapshots behalten werden sollen. Alle anderen werden
            gelöscht.
          </p>

          <div class="policy-fields">
            <div class="field">
              <label for="keep-last">Letzte Snapshots behalten</label>
              <Input
                id="keep-last"
                type="number"
                bind:value={policy.keep_last}
                placeholder="z.B. 10"
                min="0"
                disabled={isApplying}
              />
              <span class="field-hint">Behalte die letzten N Snapshots</span>
            </div>

            <div class="field">
              <label for="keep-daily">Tägliche Snapshots</label>
              <Input
                id="keep-daily"
                type="number"
                bind:value={policy.keep_daily}
                placeholder="z.B. 7"
                min="0"
                disabled={isApplying}
              />
              <span class="field-hint">Behalte tägliche Snapshots für N Tage</span>
            </div>

            <div class="field">
              <label for="keep-weekly">Wöchentliche Snapshots</label>
              <Input
                id="keep-weekly"
                type="number"
                bind:value={policy.keep_weekly}
                placeholder="z.B. 4"
                min="0"
                disabled={isApplying}
              />
              <span class="field-hint">Behalte wöchentliche Snapshots für N Wochen</span>
            </div>

            <div class="field">
              <label for="keep-monthly">Monatliche Snapshots</label>
              <Input
                id="keep-monthly"
                type="number"
                bind:value={policy.keep_monthly}
                placeholder="z.B. 12"
                min="0"
                disabled={isApplying}
              />
              <span class="field-hint">Behalte monatliche Snapshots für N Monate</span>
            </div>

            <div class="field">
              <label for="keep-yearly">Jährliche Snapshots</label>
              <Input
                id="keep-yearly"
                type="number"
                bind:value={policy.keep_yearly}
                placeholder="z.B. 2"
                min="0"
                disabled={isApplying}
              />
              <span class="field-hint">Behalte jährliche Snapshots für N Jahre</span>
            </div>
          </div>

          {#if !isValidPolicy()}
            <div class="warning-message">
              ⚠️ Mindestens ein Wert muss gesetzt sein
            </div>
          {/if}
        </div>

        {#if showPreview && preview}
          <div class="preview-section">
            <h3>Preview</h3>
            <div class="preview-stats">
              <div class="stat keep">
                <span class="stat-label">Behalten:</span>
                <span class="stat-value">{preview.snapshots_to_keep.length}</span>
              </div>
              <div class="stat delete">
                <span class="stat-label">Löschen:</span>
                <span class="stat-value">{preview.snapshots_to_delete.length}</span>
              </div>
            </div>

            {#if preview.snapshots_to_delete.length > 0}
              <div class="snapshot-list">
                <h4>Snapshots die gelöscht werden:</h4>
                <div class="snapshot-ids">
                  {#each preview.snapshots_to_delete.slice(0, 10) as id}
                    <code class="snapshot-id">{id.substring(0, 12)}...</code>
                  {/each}
                  {#if preview.snapshots_to_delete.length > 10}
                    <span class="more-count">
                      ... und {preview.snapshots_to_delete.length - 10} weitere
                    </span>
                  {/if}
                </div>
              </div>
            {:else}
              <p class="no-delete-message">✅ Keine Snapshots zum Löschen gemäß dieser Policy</p>
            {/if}
          </div>
        {/if}
      </div>

      <footer class="modal-footer">
        <Button variant="secondary" onclick={handleClose} disabled={isApplying || isLoadingPreview}>
          Abbrechen
        </Button>
        <Button
          variant="primary"
          onclick={handlePreview}
          disabled={!isValidPolicy() || isApplying || isLoadingPreview}
          loading={isLoadingPreview}
        >
          Preview
        </Button>
        <Button
          variant="danger"
          onclick={handleApply}
          disabled={!showPreview || !preview || preview.snapshots_to_delete.length === 0 || isApplying}
          loading={isApplying}
        >
          Policy anwenden
        </Button>
      </footer>
    </div>
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
    backdrop-filter: blur(2px);
  }

  .modal-dialog {
    background: #1a1d2e;
    border-radius: 12px;
    border: 1px solid #2d3348;
    max-width: 700px;
    width: 90vw;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #2d3348;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    color: #e4e4e7;
  }

  .modal-close {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #71717a;
    padding: 0.25rem;
    line-height: 1;
  }

  .modal-close:hover {
    color: #e4e4e7;
  }

  .modal-content {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid #2d3348;
  }

  .description {
    color: #a1a1aa;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
  }

  .policy-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.25rem;
    margin-bottom: 1rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .field label {
    color: #e4e4e7;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .field-hint {
    color: #71717a;
    font-size: 0.8rem;
  }

  .warning-message {
    background: rgba(251, 191, 36, 0.1);
    border: 1px solid rgba(251, 191, 36, 0.3);
    border-radius: 8px;
    padding: 0.75rem;
    color: #fbbf24;
    font-size: 0.9rem;
    margin-top: 1rem;
  }

  .preview-section {
    margin-top: 2rem;
    padding-top: 1.5rem;
    border-top: 1px solid #2d3348;
  }

  .preview-section h3 {
    margin: 0 0 1rem 0;
    color: #e4e4e7;
    font-size: 1.1rem;
  }

  .preview-stats {
    display: flex;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .stat {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    flex: 1;
  }

  .stat.keep {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
  }

  .stat.delete {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .stat-label {
    color: #a1a1aa;
    font-size: 0.9rem;
  }

  .stat-value {
    color: #e4e4e7;
    font-size: 1.5rem;
    font-weight: 600;
  }

  .stat.keep .stat-value {
    color: #22c55e;
  }

  .stat.delete .stat-value {
    color: #ef4444;
  }

  .snapshot-list {
    margin-top: 1rem;
  }

  .snapshot-list h4 {
    margin: 0 0 0.75rem 0;
    color: #e4e4e7;
    font-size: 0.95rem;
  }

  .snapshot-ids {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .snapshot-id {
    background: rgba(239, 68, 68, 0.15);
    color: #fca5a5;
    padding: 0.375rem 0.75rem;
    border-radius: 6px;
    font-size: 0.85rem;
    font-family: 'Courier New', monospace;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  .more-count {
    color: #71717a;
    font-size: 0.85rem;
    align-self: center;
  }

  .no-delete-message {
    color: #22c55e;
    padding: 1rem;
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
    border-radius: 8px;
    text-align: center;
    margin: 0;
  }
</style>
