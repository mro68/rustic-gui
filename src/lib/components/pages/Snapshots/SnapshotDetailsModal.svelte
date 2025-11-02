<!-- src/lib/components/pages/Snapshots/SnapshotDetailsModal.svelte -->
<script lang="ts">
  import Button from '$lib/components/shared/Button.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { repositories } from '$lib/stores/repositories';
  import type { SnapshotDto } from '$lib/types';

  interface Props {
    open?: boolean;
    snapshot: SnapshotDto | null;
    details: SnapshotDto | null;
    onClose?: () => void;
    onRestore?: (snapshot: SnapshotDto) => void;
  }

  let { open = $bindable(false), snapshot, details, onClose, onRestore }: Props = $props();

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }

  function handleClose() {
    open = false;
    onClose?.();
  }
</script>

<Modal bind:open>
  {#snippet header()}
    <h2>Snapshot-Details</h2>
  {/snippet}

  {#if snapshot && details}
    <div class="snapshot-details">
      <div class="details-grid">
        <div class="detail-item">
          <div class="detail-label">ID:</div>
          <span class="mono">{snapshot.id}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Zeit:</div>
          <span>{formatDate(snapshot.time)}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Repository:</div>
          <span>
            {$repositories.find((r) => r.id === snapshot.repository_id)?.name ||
              snapshot.repository_id}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Hostname:</div>
          <span>{snapshot.hostname || '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Tags:</div>
          <span>
            {#if snapshot.tags && snapshot.tags.length > 0}
              {snapshot.tags.join(', ')}
            {:else}
              Keine Tags
            {/if}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Größe:</div>
          <span>
            {#if snapshot.total_size !== undefined}
              {formatBytes(snapshot.total_size)}
            {:else}
              Unbekannt
            {/if}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Pfad:</div>
          <span class="mono">{snapshot.paths?.join(', ') || '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Username:</div>
          <span>{snapshot.username || '-'}</span>
        </div>
      </div>

      {#if details.summary}
        <div class="summary-section">
          <h3>Zusammenfassung</h3>
          <div class="summary-stats">
            <div class="stat">
              <span class="stat-label">Dateien:</span>
              <span class="stat-value">{details.summary.files_count?.toLocaleString() || '-'}</span>
            </div>
            <div class="stat">
              <span class="stat-label">Verzeichnisse:</span>
              <span class="stat-value">{details.summary.dirs_count?.toLocaleString() || '-'}</span>
            </div>
            <div class="stat">
              <span class="stat-label">Gesamtgröße:</span>
              <span class="stat-value">
                {details.summary.data_size !== undefined
                  ? formatBytes(details.summary.data_size)
                  : '-'}
              </span>
            </div>
            <div class="stat">
              <span class="stat-label">Datengröße:</span>
              <span class="stat-value">
                {details.summary.data_size !== undefined
                  ? formatBytes(details.summary.data_size)
                  : '-'}
              </span>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#snippet footer()}
    <Tooltip text="Modal schließen">
      <Button variant="secondary" onclick={handleClose}>Schließen</Button>
    </Tooltip>
    {#if snapshot}
      <Tooltip text="Wiederherstellen">
        <Button
          variant="primary"
          onclick={() => {
            onRestore?.(snapshot);
            handleClose();
          }}
        >
          📂 Wiederherstellen
        </Button>
      </Tooltip>
    {/if}
  {/snippet}
</Modal>

<style>
  .snapshot-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .detail-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .mono {
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    color: var(--text-primary);
    background: var(--bg-tertiary);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
  }

  .summary-section {
    border-top: 1px solid var(--border-color);
    padding-top: 1rem;
  }

  .summary-section h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .summary-stats {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }

  .stat {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
  }

  .stat-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .stat-value {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  @media (max-width: 768px) {
    .details-grid {
      grid-template-columns: 1fr;
    }

    .summary-stats {
      grid-template-columns: 1fr;
    }
  }
</style>
