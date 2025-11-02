<!-- src/lib/components/pages/Snapshots.svelte -->
<!--
  @component
  Snapshots-Verwaltungs-Seite mit Filter, Vergleich, Restore.
  
  TODO.md: Phase 2 - Seiten mit Daten-Loading ✅ Implementiert
  Status: ✅ Seite erstellt, ⏳ Erweiterte Features fehlen noch
  Referenz: TODO.md Zeile 62, 118, Integration-Zusammenfassung Zeile 367
  
  Backend-Commands:
  - list_snapshots_command: src-tauri/src/lib.rs:96
  - get_snapshot_command: src-tauri/src/lib.rs:84
  - delete_snapshot_command: src-tauri/src/lib.rs:73
  - compare_snapshots: ⏳ noch nicht registriert (lib.rs:422 auskommentiert)
  
  Store: src/lib/stores/snapshots.ts (loadSnapshots implementiert)
  API: src/lib/api/snapshots.ts
  
  TODOs in dieser Datei (5 gesamt):
  - Zeile 87: Zeitraum/Größe Filter-Implementation
  - Zeile 237: Vergleichslogik für Snapshots implementieren
  - Zeile 245: Restore-Dialog öffnen (RestoreDialog.svelte)
  - Zeile 405: Restore-Dialog für einzelne Datei öffnen
  - Zeile 576: Restore-Dialog für Bulk-Restore öffnen
  
  Features implementiert:
  - ✅ Snapshot-Liste mit Daten-Loading (onMount)
  - ✅ Filter-Bar (Search, Hostname, Tags)
  - ✅ Pagination (25 pro Seite)
  - ✅ Kontextmenü (Rechtsklick auf Snapshot)
  - ✅ Bulk-Selection mit Checkboxes
  - ✅ Snapshot-Details-Modal
  
  Features noch zu implementieren:
  - ⏳ Snapshot-Vergleich (CompareSnapshotsDialog Integration)
  - ⏳ Restore-Funktionalität (RestoreDialog Integration)
  - ⏳ Erweiterte Filter (Zeitraum, Größe)
-->
<script lang="ts">
  import Button from '$lib/components/shared/Button.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { activeRepositoryId, repositories } from '$lib/stores/repositories';
  import { loadSnapshots, snapshots } from '$lib/stores/snapshots';
  import { toastStore } from '$lib/stores/toast';
  import { onMount } from 'svelte';

  import { deleteSnapshot, getSnapshotInfo } from '$lib/api/snapshots';
  import CompareSnapshotsDialog from '$lib/components/dialogs/CompareSnapshotsDialog.svelte';
  import RetentionPolicyDialog from '$lib/components/dialogs/RetentionPolicyDialog.svelte';
  import TagEditorDialog from '$lib/components/dialogs/TagEditorDialog.svelte';
  import ContextMenu from '$lib/components/shared/ContextMenu.svelte';
  import FilterBar from '$lib/components/shared/FilterBar.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import type { SnapshotDto } from '$lib/types/index';

  type ContextMenuAction =
    | { label: string; icon: string; danger?: boolean; action: () => void }
    | 'divider';

  // Filter State
  let filterSearch = $state('');
  let filterHostname = $state('');
  let filterDateRange = $state('');
  let filterSize = $state('');
  let filterTags: string[] = $state([]);
  let allTags: string[] = $state([]);

  $effect(() => {
    // Alle Tags aus allen Snapshots extrahieren (unique)
    const tags = new Set<string>();
    $snapshots.forEach((s) => s.tags?.forEach((t) => tags.add(t)));
    allTags = Array.from(tags).sort();
  });

  // Pagination State
  let page = $state(1);
  let pageSize = $state(25);

  let selectedSnapshots = $state(new Set<string>());
  let sortColumn = $state('date');
  let sortDirection = $state('desc');

  let showDetailsModal = $state(false);
  let selectedSnapshot = $state<SnapshotDto | null>(null);
  let snapshotDetails = $state<SnapshotDto | null>(null);

  let isLoading = $state(false);
  let isDeleting = $state(false);

  // Context Menu State
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuVisible = $state(false);
  let contextMenuSnapshot = $state<SnapshotDto | null>(null);

  // Compare Dialog State
  let compareDialogOpen = $state(false);
  let compareSnapshotA = $state<SnapshotDto | null>(null);
  let compareSnapshotB = $state<SnapshotDto | null>(null);
  let compareDiff = $state<any>(null);

  // Retention Policy Dialog State
  let retentionDialogOpen = $state(false);
  let compareStatsA = $state<any>(null);
  let compareStatsB = $state<any>(null);

  // Tag Editor State
  let tagEditorOpen = $state(false);
  let tagEditorSnapshotId = $state('');
  let tagEditorCurrentTags = $state<string[]>([]);

  // Filter, Sort und Paging
  let filteredSnapshots = $derived(() => {
    let filtered = [...$snapshots];

    // Filter anwenden
    if (filterSearch) {
      const query = filterSearch.toLowerCase();
      filtered = filtered.filter(
        (snapshot) =>
          snapshot.tags?.some((tag) => tag.toLowerCase().includes(query)) ||
          snapshot.hostname?.toLowerCase().includes(query) ||
          snapshot.id.toLowerCase().includes(query) ||
          new Date(snapshot.time).toLocaleDateString().toLowerCase().includes(query)
      );
    }
    if (filterHostname) {
      filtered = filtered.filter((s) => s.hostname === filterHostname);
    }
    if (filterTags.length > 0) {
      filtered = filtered.filter((s) => s.tags?.some((t) => filterTags.includes(t)));
    }
    // TODO: Zeitraum/Größe Filter (vereinfachte Platzhalter-Logik)
    // TODO.md: Integration-Zusammenfassung Zeile 367
    // Implementierung: Erweiterte Filter für Datum-Range und Snapshot-Größe
    // ...

    // Sortierung
    filtered.sort((a, b) => {
      let aVal, bVal;
      switch (sortColumn) {
        case 'date':
          aVal = new Date(a.time).getTime();
          bVal = new Date(b.time).getTime();
          break;
        case 'repository':
          aVal = a.repository_id || '';
          bVal = b.repository_id || '';
          break;
        case 'hostname':
          aVal = a.hostname || '';
          bVal = b.hostname || '';
          break;
        case 'size':
          aVal = a.total_size || 0;
          bVal = b.total_size || 0;
          break;
        default:
          return 0;
      }
      if (sortDirection === 'asc') {
        return aVal > bVal ? 1 : -1;
      } else {
        return aVal < bVal ? 1 : -1;
      }
    });
    return filtered;
  });

  // Paging
  let pagedSnapshots = $derived(() => {
    const start = (page - 1) * pageSize;
    return filteredSnapshots().slice(start, start + pageSize);
  });

  $effect(() => {
    // Wenn Filter/Snapshots sich ändern, Seite zurücksetzen
    page = 1;
  });

  import { formatBytes, formatDate } from '$lib/utils/format';

  async function refreshSnapshots() {
    try {
      isLoading = true;
      // Lade Snapshots für alle entsperrten Repositories
      for (const repo of $repositories) {
        if (repo.status !== 'Locked') {
          await loadSnapshots(repo.id);
        }
      }
      toastStore.success('Snapshots wurden erfolgreich aktualisiert');
    } catch (error) {
      console.error('Failed to refresh snapshots:', error);
      toastStore.error('Fehler beim Aktualisieren der Snapshots');
    } finally {
      isLoading = false;
    }
  }

  async function showSnapshotDetails(snapshotId: string) {
    try {
      const details = await getSnapshotInfo(snapshotId);
      selectedSnapshot = $snapshots.find((s) => s.id === snapshotId) || null;
      snapshotDetails = details;
      showDetailsModal = true;
    } catch (error) {
      console.error('Failed to load snapshot details:', error);
      toastStore.error('Fehler beim Laden der Snapshot-Details');
    }
  }

  /**
   * Task 4.1: Snapshot-Deletion
   * Löscht einen Snapshot mit Confirmation
   */
  async function handleDeleteSnapshot(snapshotId: string) {
    if (!confirm('Sind Sie sicher, dass Sie diesen Snapshot löschen möchten?')) {
      return;
    }

    isDeleting = true;
    try {
      await deleteSnapshot(snapshotId);
      toastStore.success('Snapshot wurde erfolgreich gelöscht');
      
      // Snapshot-Liste neu laden nach Deletion
      const repoId = $activeRepositoryId;
      if (repoId) {
        await loadSnapshots(repoId);
      }
    } catch (error) {
      console.error('Failed to delete snapshot:', error);
      toastStore.error('Fehler beim Löschen des Snapshots');
    } finally {
      isDeleting = false;
    }
  }

  function handleSort(column: string) {
    if (sortColumn === column) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortColumn = column;
      sortDirection = 'desc';
    }
  }

  function toggleSelection(snapshotId: string) {
    if (selectedSnapshots.has(snapshotId)) {
      selectedSnapshots.delete(snapshotId);
    } else {
      selectedSnapshots.add(snapshotId);
    }
  }

  function selectAll() {
    const currentPageSnapshots = pagedSnapshots();
    if (selectedSnapshots.size === currentPageSnapshots.length) {
      selectedSnapshots = new Set();
    } else {
      selectedSnapshots = new Set(currentPageSnapshots.map((s) => s.id));
    }
  }

  function openContextMenu(event: MouseEvent, snapshot: SnapshotDto) {
    event.preventDefault();
    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
    contextMenuSnapshot = snapshot;
    contextMenuVisible = true;
  }

  function closeContextMenu() {
    contextMenuVisible = false;
    contextMenuSnapshot = null;
  }

  function openTagEditor(snapshot: SnapshotDto) {
    tagEditorSnapshotId = snapshot.id;
    tagEditorCurrentTags = snapshot.tags || [];
    tagEditorOpen = true;
  }

  function handleTagsSaved() {
    // Refresh snapshots to get updated tags
    refreshSnapshots();
  }

  function openRetentionDialog() {
    retentionDialogOpen = true;
  }

  function handleRetentionPolicyApplied() {
    // Refresh snapshots nach Policy-Anwendung
    refreshSnapshots();
  }

  function openCompareDialog(snapshot: SnapshotDto) {
    if (!compareSnapshotA) {
      compareSnapshotA = snapshot;
      toastStore.info('Ersten Snapshot ausgewählt. Wähle einen zweiten zum Vergleich.');
    } else if (compareSnapshotA.id === snapshot.id) {
      toastStore.warning('Bitte einen anderen Snapshot zum Vergleich auswählen.');
    } else {
      compareSnapshotB = snapshot;
      compareDialogOpen = true;
    }
  }

  function contextMenuActions(): ContextMenuAction[] {
    if (!contextMenuSnapshot) return [];

    return [
      {
        label: 'Details anzeigen',
        icon: 'ℹ️',
        action: () => {
          showSnapshotDetails(contextMenuSnapshot!.id);
          closeContextMenu();
        },
      },
      {
        label: 'Tags bearbeiten',
        icon: '🏷️',
        action: () => {
          openTagEditor(contextMenuSnapshot!);
          closeContextMenu();
        },
      },
      {
        label: 'Vergleichen',
        icon: '⚖️',
        action: () => {
          openCompareDialog(contextMenuSnapshot!);
          closeContextMenu();
        },
      },
      {
        label: 'Wiederherstellen',
        icon: '📂',
        action: () => {
          // TODO: Restore-Dialog öffnen
          closeContextMenu();
        },
      },
      'divider' as const,
      {
        label: 'Löschen',
        icon: '🗑️',
        danger: true,
        action: () => {
          handleDeleteSnapshot(contextMenuSnapshot!.id);
          closeContextMenu();
        },
      },
    ];
  }

  function closeCompareDialog() {
    compareDialogOpen = false;
    compareSnapshotA = null;
    compareSnapshotB = null;
    compareDiff = null;
    compareStatsA = null;
    compareStatsB = null;
  }

  onMount(async () => {
    // Lade Snapshots für alle entsperrten Repositories
    for (const repo of $repositories) {
      if (repo.status !== 'Locked') {
        await loadSnapshots(repo.id);
      }
    }
  });
</script>

<div class="snapshots-page">
  <!-- Toolbar + FilterBar -->
  <div class="toolbar">
    <h1 class="page-title">Alle Snapshots</h1>
    <div class="toolbar-actions">
      <Tooltip text="Retention-Policy anwenden">
        <Button variant="secondary" size="sm" onclick={openRetentionDialog}>
          🗓️ Retention Policy
        </Button>
      </Tooltip>
      <Tooltip text="Snapshots aktualisieren">
        <Button variant="secondary" size="sm" disabled={isLoading} onclick={refreshSnapshots}>
          {isLoading ? '↻' : '↻'} Aktualisieren
        </Button>
      </Tooltip>
    </div>
  </div>
  <FilterBar
    bind:search={filterSearch}
    bind:hostname={filterHostname}
    bind:dateRange={filterDateRange}
    bind:size={filterSize}
    bind:tags={filterTags}
    {allTags}
    on:change={(e: CustomEvent) => {
      filterSearch = e.detail.search;
      filterHostname = e.detail.hostname;
      filterDateRange = e.detail.dateRange;
      filterSize = e.detail.size;
      filterTags = e.detail.tags;
    }}
  />

  <!-- Bulk Actions (when items selected) -->
  {#if selectedSnapshots.size > 0}
    <div class="bulk-actions">
      <span class="selection-count">{selectedSnapshots.size} Snapshots ausgewählt</span>
      <div class="bulk-buttons">
        <Tooltip text="Auswahl aufheben">
          <Button variant="secondary" size="sm" onclick={() => (selectedSnapshots = new Set())}>
            Auswahl aufheben
          </Button>
        </Tooltip>
        <Tooltip text="Snapshots löschen">
          <Button variant="danger" size="sm" disabled={isDeleting}>
            {isDeleting ? '...' : '🗑️'} Löschen
          </Button>
        </Tooltip>
      </div>
    </div>
  {/if}

  <!-- Snapshots Table + Pagination -->
  <div class="table-container">
    <table class="snapshots-table">
      <thead>
        <tr>
          <th class="checkbox-column">
            <input
              type="checkbox"
              checked={selectedSnapshots.size === pagedSnapshots().length &&
                pagedSnapshots().length > 0}
              indeterminate={selectedSnapshots.size > 0 &&
                selectedSnapshots.size < pagedSnapshots().length}
              onclick={selectAll}
            />
          </th>
          <th class="sortable" onclick={() => handleSort('date')}>
            Datum / Uhrzeit
            {sortColumn === 'date' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
          </th>
          <th class="sortable" onclick={() => handleSort('repository')}>
            Repository
            {sortColumn === 'repository' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
          </th>
          <th class="sortable" onclick={() => handleSort('hostname')}>
            Hostname
            {sortColumn === 'hostname' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
          </th>
          <th>Tags</th>
          <th class="sortable" onclick={() => handleSort('size')}>
            Größe
            {sortColumn === 'size' ? (sortDirection === 'asc' ? ' ↑' : ' ↓') : ''}
          </th>
          <th>Aktionen</th>
        </tr>
      </thead>
      <tbody>
        {#each pagedSnapshots() as snapshot (snapshot.id)}
          <tr
            class="snapshot-row"
            class:selected={selectedSnapshots.has(snapshot.id)}
            oncontextmenu={(e) => openContextMenu(e, snapshot)}
          >
            <td class="checkbox-column">
              <input
                type="checkbox"
                checked={selectedSnapshots.has(snapshot.id)}
                onclick={() => toggleSelection(snapshot.id)}
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
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => {
                    /* TODO: Open restore dialog */
                  }}
                >
                  📂
                </Button>
              </Tooltip>
              <Tooltip text="Details anzeigen">
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => showSnapshotDetails(snapshot.id)}
                >
                  ℹ️
                </Button>
              </Tooltip>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>

    <!-- Kontextmenü -->
    <ContextMenu
      x={contextMenuX}
      y={contextMenuY}
      visible={contextMenuVisible}
      actions={contextMenuActions()}
      on:close={closeContextMenu}
    />

    <!-- Tag-Editor-Dialog -->
    <TagEditorDialog
      bind:open={tagEditorOpen}
      snapshotId={tagEditorSnapshotId}
      currentTags={tagEditorCurrentTags}
      on:save={handleTagsSaved}
    />

    <!-- Vergleichs-Dialog -->
    <CompareSnapshotsDialog
      snapshots={$snapshots}
      open={compareDialogOpen}
      snapshotA={compareSnapshotA}
      snapshotB={compareSnapshotB}
      diff={compareDiff}
      statsA={compareStatsA}
      statsB={compareStatsB}
      on:close={closeCompareDialog}
    />

    {#if pagedSnapshots().length === 0}
      <div class="empty-state">
        {#if $snapshots.length === 0}
          <p>Keine Snapshots gefunden. Erstellen Sie zuerst ein Backup.</p>
        {:else if filterSearch}
          <p>Keine Snapshots entsprechen dem Filter.</p>
          <Tooltip text="Filter zurücksetzen">
            <Button variant="secondary" size="sm" onclick={() => (filterSearch = '')}>
              Filter zurücksetzen
            </Button>
          </Tooltip>
        {/if}
      </div>
    {/if}
    <Pagination
      {page}
      {pageSize}
      total={filteredSnapshots().length}
      on:page={(e) => (page = e.detail)}
      on:pageSize={(e) => {
        pageSize = e.detail;
        page = 1;
      }}
    />
  </div>
</div>

<!-- Snapshot Details Modal -->
<Modal bind:open={showDetailsModal}>
  {#snippet header()}
    <h2>Snapshot-Details</h2>
  {/snippet}
  {#if selectedSnapshot && snapshotDetails}
    <div class="snapshot-details">
      <div class="details-grid">
        <div class="detail-item">
          <div class="detail-label">ID:</div>
          <span class="mono">{selectedSnapshot?.id || '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Zeit:</div>
          <span>{selectedSnapshot ? formatDate(selectedSnapshot.time) : '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Repository:</div>
          <span>
            {#if selectedSnapshot}
              {$repositories.find((r) => r.id === selectedSnapshot?.repository_id)?.name ||
                selectedSnapshot?.repository_id}
            {:else}
              -
            {/if}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Hostname:</div>
          <span>{selectedSnapshot?.hostname || '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Tags:</div>
          <span>
            {#if selectedSnapshot?.tags && selectedSnapshot.tags.length > 0}
              {selectedSnapshot.tags.join(', ')}
            {:else}
              Keine Tags
            {/if}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Größe:</div>
          <span>
            {#if selectedSnapshot?.total_size !== undefined}
              {formatBytes(selectedSnapshot.total_size)}
            {:else}
              Unbekannt
            {/if}
          </span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Pfad:</div>
          <span class="mono">{selectedSnapshot?.paths?.join(', ') || '-'}</span>
        </div>
        <div class="detail-item">
          <div class="detail-label">Username:</div>
          <span>{selectedSnapshot?.username || '-'}</span>
        </div>
      </div>

      {#if snapshotDetails.summary}
        <div class="summary-section">
          <h3>Zusammenfassung</h3>
          <div class="summary-stats">
            <div class="stat">
              <span class="stat-label">Dateien:</span>
              <span class="stat-value"
                >{snapshotDetails.summary.files_count?.toLocaleString() || '-'}</span
              >
            </div>
            <div class="stat">
              <span class="stat-label">Verzeichnisse:</span>
              <span class="stat-value"
                >{snapshotDetails.summary.dirs_count?.toLocaleString() || '-'}</span
              >
            </div>
            <div class="stat">
              <span class="stat-label">Datengröße:</span>
              <span class="stat-value">
                {#if snapshotDetails.summary.data_size}
                  {formatBytes(snapshotDetails.summary.data_size)}
                {:else}
                  -
                {/if}
              </span>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#snippet footer()}
    <Tooltip text="Schließen">
      <Button variant="secondary" onclick={() => (showDetailsModal = false)}>Schließen</Button>
    </Tooltip>
    {#if selectedSnapshot}
      <Tooltip text="Wiederherstellen">
        <Button
          variant="primary"
          onclick={() => {
            /* TODO: Open restore dialog */
          }}
        >
          📂 Wiederherstellen
        </Button>
      </Tooltip>
    {/if}
  {/snippet}
</Modal>

<style>
  .snapshots-page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 0;
  }

  .page-title {
    font-size: 1.875rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .toolbar-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .bulk-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 0.75rem 1rem;
  }

  .selection-count {
    font-size: 0.875rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .bulk-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .table-container {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
  }

  .snapshots-table {
    width: 100%;
    border-collapse: collapse;
  }

  .snapshots-table thead {
    background: var(--bg-tertiary);
  }

  .snapshots-table th {
    padding: 0.875rem 1rem;
    text-align: left;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    cursor: pointer;
    user-select: none;
  }

  .snapshots-table th.sortable:hover {
    color: var(--text-primary);
  }

  .snapshots-table td {
    padding: 1rem;
    border-top: 1px solid var(--border-color);
    font-size: 0.875rem;
    color: var(--text-primary);
  }

  .snapshot-row:hover {
    background: var(--bg-tertiary);
  }

  .snapshot-row.selected {
    background: rgba(59, 130, 246, 0.1);
  }

  .checkbox-column {
    width: 3rem;
    text-align: center;
  }

  .date-column {
    min-width: 10rem;
  }

  .repository-column {
    min-width: 8rem;
  }

  .hostname-column {
    min-width: 6rem;
  }

  .tags-column {
    min-width: 8rem;
  }

  .size-column {
    min-width: 5rem;
    text-align: right;
  }

  .actions-column {
    width: 6rem;
  }

  .tag {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    border-radius: 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    background: rgba(59, 130, 246, 0.15);
    color: var(--color-primary);
    margin-right: 0.25rem;
    margin-bottom: 0.125rem;
  }

  .no-tags {
    color: var(--text-secondary);
    font-style: italic;
  }

  .unknown-size {
    color: var(--text-secondary);
    font-style: italic;
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .empty-state p {
    margin-bottom: 1rem;
    font-size: 1rem;
  }

  .snapshot-details {
    max-height: 70vh;
    overflow-y: auto;
  }

  .details-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .detail-item .detail-label {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .detail-item span {
    font-size: 0.875rem;
    color: var(--text-primary);
  }

  .mono {
    font-family: 'Courier New', monospace;
    font-size: 0.8125rem;
    background: var(--bg-tertiary);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    word-break: break-all;
  }

  .summary-section {
    border-top: 1px solid var(--border-color);
    padding-top: 1.5rem;
  }

  .summary-section h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .summary-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
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
    .toolbar {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }

    .toolbar-actions {
      justify-content: space-between;
    }

    .table-container {
      overflow-x: auto;
    }

    .snapshots-table {
      min-width: 600px;
    }

    .details-grid {
      grid-template-columns: 1fr;
    }

    .summary-stats {
      grid-template-columns: 1fr;
    }
  }
</style>

<!-- Retention Policy Dialog -->
<RetentionPolicyDialog
  bind:open={retentionDialogOpen}
  on:applied={handleRetentionPolicyApplied}
  on:cancel={() => {
    retentionDialogOpen = false;
  }}
/>
