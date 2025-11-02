<!-- src/lib/components/pages/Snapshots/SnapshotTable.svelte -->
<script lang="ts">
  import Button from '$lib/components/shared/Button.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { repositories } from '$lib/stores/repositories';
  import type { SnapshotDto } from '$lib/types';

  interface Props {
    snapshots?: SnapshotDto[];
    selectedIds?: Set<string>;
    sortColumn?: 'date' | 'repository' | 'hostname' | 'size';
    sortDirection?: 'asc' | 'desc';
    onSelectionToggle?: (id: string) => void;
    onSelectAll?: () => void;
    onSort?: (column: string) => void;
    onContextMenu?: (event: MouseEvent, snapshot: SnapshotDto) => void;
    onRestore?: (snapshot: SnapshotDto) => void;
    onShowDetails?: (snapshotId: string) => void;
  }

  let {
    snapshots = [],
    selectedIds = new Set(),
    sortColumn = 'date',
    sortDirection = 'desc',
    onSelectionToggle,
    onSelectAll,
    onSort,
    onContextMenu,
    onRestore,
    onShowDetails,
  }: Props = $props();

  const allSelected = $derived(selectedIds.size === snapshots.length && snapshots.length > 0);

  const someSelected = $derived(selectedIds.size > 0 && selectedIds.size < snapshots.length);

  function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
  }
</script>

<div class="table-container">
  <table class="snapshots-table">
    <thead>
      <tr>
        <th class="checkbox-column">
          <input
            type="checkbox"
            checked={allSelected}
            indeterminate={someSelected}
            onclick={() => onSelectAll?.()}
          />
        </th>
        <th class="sortable" onclick={() => onSort?.('date')}>
          Datum / Uhrzeit
          {sortColumn === 'date' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
        </th>
        <th class="sortable" onclick={() => onSort?.('repository')}>
          Repository
          {sortColumn === 'repository' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
        </th>
        <th class="sortable" onclick={() => onSort?.('hostname')}>
          Hostname
          {sortColumn === 'hostname' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
        </th>
        <th>Tags</th>
        <th class="sortable" onclick={() => onSort?.('size')}>
          Größe
          {sortColumn === 'size' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
        </th>
        <th>Aktionen</th>
      </tr>
    </thead>
    <tbody>
      {#each snapshots as snapshot (snapshot.id)}
        <tr
          class="snapshot-row"
          class:selected={selectedIds.has(snapshot.id)}
          oncontextmenu={(e) => onContextMenu?.(e, snapshot)}
        >
          <td class="checkbox-column">
            <input
              type="checkbox"
              checked={selectedIds.has(snapshot.id)}
              onclick={() => onSelectionToggle?.(snapshot.id)}
            />
          </td>
          <td class="date-column">{formatDate(snapshot.time)}</td>
          <td class="repository-column">
            {$repositories.find((r) => r.id === snapshot.repository_id)?.name ||
              snapshot.repository_id}
          </td>
          <td class="hostname-column">{snapshot.hostname || '-'}</td>
          <td class="tags-column">
            {#if snapshot.tags && snapshot.tags.length > 0}
              {#each snapshot.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            {:else}
              <span class="no-tags">-</span>
            {/if}
          </td>
          <td class="size-column">
            {#if snapshot.total_size !== undefined}
              {formatBytes(snapshot.total_size)}
            {:else}
              <span class="unknown-size">-</span>
            {/if}
          </td>
          <td class="actions-column">
            <Tooltip text="Wiederherstellen">
              <Button variant="secondary" size="sm" onclick={() => onRestore?.(snapshot)}>
                📂
              </Button>
            </Tooltip>
            <Tooltip text="Details anzeigen">
              <Button variant="secondary" size="sm" onclick={() => onShowDetails?.(snapshot.id)}>
                ℹ️
              </Button>
            </Tooltip>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
    overflow-x: auto;
    background: var(--bg-secondary);
    border-radius: 0.75rem;
    border: 1px solid var(--border-color);
  }

  .snapshots-table {
    width: 100%;
    border-collapse: collapse;
  }

  .snapshots-table thead {
    background: var(--bg-tertiary);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .snapshots-table th {
    padding: 0.75rem 1rem;
    text-align: left;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid var(--border-color);
  }

  .snapshots-table th.sortable {
    cursor: pointer;
    user-select: none;
    transition: color 0.2s;
  }

  .snapshots-table th.sortable:hover {
    color: var(--color-primary);
  }

  .snapshots-table td {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border-color);
    font-size: 0.875rem;
    color: var(--text-primary);
  }

  .snapshot-row {
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .snapshot-row:hover {
    background: rgba(59, 130, 246, 0.05);
  }

  .snapshot-row.selected {
    background: rgba(59, 130, 246, 0.1);
  }

  .checkbox-column {
    width: 40px;
    text-align: center;
  }

  .date-column {
    white-space: nowrap;
  }

  .tags-column {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .tag {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    background: var(--color-primary);
    color: white;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .no-tags,
  .unknown-size {
    color: var(--text-secondary);
    font-style: italic;
  }

  .actions-column {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
  }

  @media (max-width: 768px) {
    .table-container {
      overflow-x: auto;
    }

    .snapshots-table {
      min-width: 600px;
    }
  }
</style>
