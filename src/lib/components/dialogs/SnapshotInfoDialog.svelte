<script lang="ts">
  /**
   * Snapshot-Info-Dialog mit Details.
   *
   * Zeigt vollständige Snapshot-Metadaten:
   * - Erstellungsdatum
   * - Hostname
   * - Tags
   * - Größe
   * - Anzahl Dateien
   *
   * @component
   *
   * @example
   * ```svelte
   * <SnapshotInfoDialog
   *   {snapshot}
   *   on:close={handleClose}
   * />
   * ```
   */
  import type { SnapshotDto } from '$lib/types';
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Modal from '../shared/Modal.svelte';

  const dispatch = createEventDispatcher();

  interface SnapshotInfoDialogProps {
    /** Anzuzeigender Snapshot */
    snapshot?: SnapshotDto | null;
  }

  let { snapshot = null }: SnapshotInfoDialogProps = $props();

  function formatDate(dateString: string): string {
    if (!dateString) return '-';
    try {
      return new Date(dateString).toLocaleString('de-DE', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
      });
    } catch {
      return dateString;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  function handleClose() {
    dispatch('close');
  }

  function handleAction(action: string) {
    dispatch(action, { snapshot });
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Snapshot-Details</div>
  {#if snapshot}
    <div class="snapshot-details">
      <!-- Header Info -->
      <div class="details-header">
        <div class="snapshot-id">
          <span class="detail-label">ID:</span>
          <span class="mono">{snapshot.id || '-'}</span>
        </div>
        <div class="snapshot-time">
          <span class="detail-label">Zeit:</span>
          <span>{formatDate(snapshot.time)}</span>
        </div>
      </div>

      <!-- Details Grid -->
      <div class="details-grid">
        <div class="detail-row">
          <span class="detail-label">Repository:</span>
          <span>{snapshot.repository_id || '-'}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">Hostname:</span>
          <span>{snapshot.hostname || '-'}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">Username:</span>
          <span>{snapshot.username || '-'}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">Pfad:</span>
          <span class="mono">{snapshot.paths?.join(', ') || '-'}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">Dateien:</span>
          <span>{snapshot.file_count?.toLocaleString('de-DE') || '-'}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">Größe:</span>
          <span>{snapshot.total_size ? formatBytes(snapshot.total_size) : '-'}</span>
        </div>
      </div>

      <!-- Tags -->
      {#if snapshot.tags && snapshot.tags.length > 0}
        <div class="detail-row">
          <span class="detail-label">Tags:</span>
          <div class="tags">
            {#each snapshot.tags as tag}
              <span class="tag">{tag}</span>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Actions -->
      <div class="actions-section">
        <h3>Verfügbare Aktionen</h3>
        <div class="action-buttons">
          <Button variant="primary" onclick={() => handleAction('restore')}>
            Dateien wiederherstellen
          </Button>
          <Button variant="secondary" onclick={() => handleAction('compare')}>
            Mit anderem Snapshot vergleichen
          </Button>
          <Button variant="secondary" onclick={() => handleAction('copy')}>
            Snapshot kopieren
          </Button>
          <Button variant="danger" onclick={() => handleAction('delete')}>Snapshot löschen</Button>
        </div>
      </div>
    </div>
  {:else}
    <div class="no-data">Kein Snapshot ausgewählt</div>
  {/if}

  <div slot="footer">
    <Button variant="secondary" onclick={handleClose}>Schließen</Button>
  </div>
</Modal>

<style>
  .snapshot-details {
    max-width: 800px;
  }

  .details-header {
    background: #22273a;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 24px;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .snapshot-id,
  .snapshot-time {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .snapshot-id .detail-label,
  .snapshot-time .detail-label {
    font-size: 12px;
    font-weight: 500;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .snapshot-id span,
  .snapshot-time span {
    font-size: 14px;
    color: #e4e4e7;
  }

  .mono {
    font-family: 'Courier New', monospace;
    font-size: 13px;
  }

  .details-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-bottom: 24px;
  }

  .detail-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .detail-row .detail-label {
    font-size: 12px;
    font-weight: 500;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .detail-row span {
    font-size: 14px;
    color: #e4e4e7;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tag {
    background: #3b82f6;
    color: white;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
  }

  .actions-section {
    border-top: 1px solid #2d3348;
    padding-top: 24px;
  }

  .actions-section h3 {
    font-size: 16px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 16px;
  }

  .action-buttons {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 12px;
  }

  .no-data {
    text-align: center;
    color: #71717a;
    padding: 40px;
    font-size: 16px;
  }

  @media (max-width: 768px) {
    .details-header {
      grid-template-columns: 1fr;
    }

    .details-grid {
      grid-template-columns: 1fr;
    }

    .action-buttons {
      grid-template-columns: 1fr;
    }
  }
</style>
