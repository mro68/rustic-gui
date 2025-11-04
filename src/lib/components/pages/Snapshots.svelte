<!-- src/lib/components/pages/Snapshots/Snapshots.svelte -->
<script lang="ts">
  import { deleteSnapshot, getSnapshotInfo } from '$lib/api/snapshots';
  import CompareSnapshotsDialog from '$lib/components/dialogs/CompareSnapshotsDialog.svelte';
  import RestoreDialog from '$lib/components/dialogs/RestoreDialog.svelte';
  import RetentionPolicyDialog from '$lib/components/dialogs/RetentionPolicyDialog.svelte';
  import TagEditorDialog from '$lib/components/dialogs/TagEditorDialog.svelte';
  import Button from '$lib/components/shared/Button.svelte';
  import FilterBar from '$lib/components/shared/FilterBar.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { activeRepositoryId, repositories } from '$lib/stores/repositories';
  import { loadSnapshots, snapshots } from '$lib/stores/snapshots';
  import { toastStore } from '$lib/stores/toast';
  import type { SnapshotDto } from '$lib/types';
  import { onMount } from 'svelte';
  import SnapshotContextMenu from './Snapshots/SnapshotContextMenu.svelte';
  import SnapshotDetailsModal from './Snapshots/SnapshotDetailsModal.svelte';
  import SnapshotTable from './Snapshots/SnapshotTable.svelte';

  // Filter State
  let filterSearch = $state('');
  let filterHostname = $state('');
  let filterDateRange = $state('');
  let filterSize = $state('');
  let filterTags: string[] = $state([]);

  // Unique Hostnames & Tags
  let allHostnames = $derived(() => {
    const hostnames = new Set<string>();
    $snapshots.forEach((s) => s.hostname && hostnames.add(s.hostname));
    return Array.from(hostnames).sort();
  });

  let allTags = $derived(() => {
    const tags = new Set<string>();
    $snapshots.forEach((s) => s.tags?.forEach((t) => tags.add(t)));
    return Array.from(tags).sort();
  });

  // Pagination State
  let page = $state(1);
  let pageSize = $state(25);

  // Selection State
  let selectedSnapshots = $state(new Set<string>());

  // Sort State
  let sortColumn = $state<'date' | 'repository' | 'hostname' | 'size'>('date');
  let sortDirection = $state<'asc' | 'desc'>('desc');

  // Modal States
  let showDetailsModal = $state(false);
  let selectedSnapshot = $state<SnapshotDto | null>(null);
  let snapshotDetails = $state<SnapshotDto | null>(null);

  // Context Menu State
  let contextMenuVisible = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);
  let contextMenuSnapshot = $state<SnapshotDto | null>(null);

  // Dialog States
  let compareDialogOpen = $state(false);
  let compareSnapshotA = $state<SnapshotDto | null>(null);
  let compareSnapshotB = $state<SnapshotDto | null>(null);

  let retentionDialogOpen = $state(false);

  let tagEditorOpen = $state(false);
  let tagEditorSnapshotId = $state('');
  let tagEditorCurrentTags = $state<string[]>([]);

  let restoreDialogOpen = $state(false);
  let restoreSnapshotId = $state('');
  let restorePassword = $state('');

  // Loading State
  let isLoading = $state(false);
  let isDeleting = $state(false);

  // Passwort laden für Restore
  $effect(() => {
    if (restoreDialogOpen && $activeRepositoryId) {
      import('$lib/api/keychain').then(async ({ getRepositoryPassword }) => {
        try {
          restorePassword = await getRepositoryPassword($activeRepositoryId);
        } catch (error) {
          toastStore.error('Repository-Passwort nicht verfügbar. Bitte Repository entsperren.');
          console.error('Keychain error:', error);
          restoreDialogOpen = false;
        }
      });
    }
  });

  // Filter, Sort & Paging
  const filteredSnapshots = $derived(() => {
    let filtered = [...$snapshots];

    if (filterSearch) {
      const query = filterSearch.toLowerCase();
      filtered = filtered.filter(
        (s) =>
          s.tags?.some((t) => t.toLowerCase().includes(query)) ||
          s.hostname?.toLowerCase().includes(query) ||
          s.id.toLowerCase().includes(query) ||
          new Date(s.time).toLocaleDateString().toLowerCase().includes(query)
      );
    }

    if (filterHostname) {
      filtered = filtered.filter((s) => s.hostname === filterHostname);
    }

    if (filterTags.length > 0) {
      filtered = filtered.filter((s) => s.tags?.some((t) => filterTags.includes(t)));
    }

    return filtered;
  });

  const sortedSnapshots = $derived(() => {
    const sorted = [...filteredSnapshots()];

    sorted.sort((a, b) => {
      let compareValue = 0;

      if (sortColumn === 'date') {
        compareValue = new Date(a.time).getTime() - new Date(b.time).getTime();
      } else if (sortColumn === 'hostname') {
        compareValue = (a.hostname || '').localeCompare(b.hostname || '');
      } else if (sortColumn === 'repository') {
        compareValue = a.repository_id.localeCompare(b.repository_id);
      } else if (sortColumn === 'size') {
        compareValue = (a.total_size || 0) - (b.total_size || 0);
      }

      return sortDirection === 'asc' ? compareValue : -compareValue;
    });

    return sorted;
  });

  const pagedSnapshots = $derived(() => {
    const start = (page - 1) * pageSize;
    return sortedSnapshots().slice(start, start + pageSize);
  });

  // Event Handlers
  function handleSort(column: string) {
    if (sortColumn === column) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortColumn = column as any;
      sortDirection = 'desc';
    }
  }

  function toggleSelection(id: string) {
    if (selectedSnapshots.has(id)) {
      selectedSnapshots.delete(id);
    } else {
      selectedSnapshots.add(id);
    }
    selectedSnapshots = selectedSnapshots;
  }

  function selectAll() {
    if (selectedSnapshots.size === pagedSnapshots().length) {
      selectedSnapshots = new Set();
    } else {
      selectedSnapshots = new Set(pagedSnapshots().map((s) => s.id));
    }
  }

  async function showSnapshotDetails(snapshotId: string) {
    try {
      const snapshot = $snapshots.find((s) => s.id === snapshotId);
      if (!snapshot) return;

      selectedSnapshot = snapshot;
      snapshotDetails = await getSnapshotInfo(snapshotId);
      showDetailsModal = true;
    } catch (error) {
      toastStore.error('Details konnten nicht geladen werden');
      console.error(error);
    }
  }

  async function handleDeleteSnapshot(snapshotId: string) {
    if (!confirm('Snapshot wirklich löschen?')) return;

    isDeleting = true;
    try {
      await deleteSnapshot(snapshotId);
      toastStore.success('Snapshot gelöscht');
      await refreshSnapshots();
    } catch (error) {
      toastStore.error('Löschen fehlgeschlagen');
      console.error(error);
    } finally {
      isDeleting = false;
    }
  }

  function openTagEditor(snapshot: SnapshotDto) {
    tagEditorSnapshotId = snapshot.id;
    tagEditorCurrentTags = snapshot.tags || [];
    tagEditorOpen = true;
  }

  function openCompareDialog(snapshot: SnapshotDto) {
    if (!compareSnapshotA) {
      compareSnapshotA = snapshot;
      toastStore.info('Ersten Snapshot ausgewählt. Wähle zweiten zum Vergleich.');
    } else if (compareSnapshotA.id === snapshot.id) {
      toastStore.warning('Bitte anderen Snapshot zum Vergleich auswählen.');
    } else {
      compareSnapshotB = snapshot;
      compareDialogOpen = true;
    }
  }

  function handleRestore(snapshot: SnapshotDto) {
    restoreSnapshotId = snapshot.id;
    restoreDialogOpen = true;
  }

  async function refreshSnapshots() {
    await loadSnapshots();
  }

  onMount(() => {
    refreshSnapshots();
  });
</script>

<div class="snapshots-page">
  <div class="page-wrapper">
    <div class="snapshots-container">
      <div class="page-header">
        <h1>Snapshots</h1>
        <div class="header-actions">
          <Tooltip text="Snapshots neu laden">
            <Button variant="secondary" onclick={refreshSnapshots} disabled={isLoading}>
              {isLoading ? '⏳' : '🔄'} Aktualisieren
            </Button>
          </Tooltip>
          <Tooltip text="Retention Policy anwenden">
            <Button variant="primary" onclick={() => (retentionDialogOpen = true)}>
              🗑️ Retention Policy
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
        allTags={allTags()}
      />

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

      <SnapshotTable
        snapshots={pagedSnapshots()}
        selectedIds={selectedSnapshots}
        {sortColumn}
        {sortDirection}
        onSelectionToggle={toggleSelection}
        onSelectAll={selectAll}
        onSort={handleSort}
        onContextMenu={(e, s) => {
          contextMenuX = e.clientX;
          contextMenuY = e.clientY;
          contextMenuSnapshot = s;
          contextMenuVisible = true;
        }}
        onRestore={handleRestore}
        onShowDetails={(snapshotId) => showSnapshotDetails(snapshotId)}
      />

      {#if pagedSnapshots().length === 0}
        <div class="empty-state">
          {#if $snapshots.length === 0}
            <p>Keine Snapshots gefunden. Erstellen Sie zuerst ein Backup.</p>
          {:else}
            <p>Keine Snapshots entsprechen dem Filter.</p>
            <Button variant="secondary" onclick={() => (filterSearch = '')}
              >Filter zurücksetzen</Button
            >
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

    <!-- Context Menu -->
    <SnapshotContextMenu
      bind:visible={contextMenuVisible}
      x={contextMenuX}
      y={contextMenuY}
      snapshot={contextMenuSnapshot}
      onClose={() => (contextMenuVisible = false)}
      onShowDetails={(snapshotId) => showSnapshotDetails(snapshotId)}
      onCompare={openCompareDialog}
      onEditTags={openTagEditor}
      onRestore={handleRestore}
      onDelete={handleDeleteSnapshot}
    />

    <!-- Details Modal -->
    <SnapshotDetailsModal
      bind:open={showDetailsModal}
      snapshot={selectedSnapshot}
      details={snapshotDetails}
      onRestore={handleRestore}
    />

    <!-- Tag Editor -->
    <TagEditorDialog
      bind:open={tagEditorOpen}
      snapshotId={tagEditorSnapshotId}
      currentTags={tagEditorCurrentTags}
      on:saved={refreshSnapshots}
    />

    <!-- Compare Dialog -->
    <CompareSnapshotsDialog
      bind:open={compareDialogOpen}
      snapshotA={compareSnapshotA}
      snapshotB={compareSnapshotB}
      on:close={() => {
        compareDialogOpen = false;
        compareSnapshotA = null;
        compareSnapshotB = null;
      }}
    />

    <!-- Retention Policy Dialog -->
    <RetentionPolicyDialog bind:open={retentionDialogOpen} on:applied={refreshSnapshots} />

    <!-- Restore Dialog -->
    <RestoreDialog
      bind:isOpen={restoreDialogOpen}
      repositoryId={$activeRepositoryId ?? undefined}
      repositoryPath={$repositories.find((r) => r.id === $activeRepositoryId)?.path ?? ''}
      repositoryPassword={restorePassword}
      on:restored={(event) => {
        toastStore.success(
          `${event.detail.files.length} Dateien wiederhergestellt nach ${event.detail.targetPath}`
        );
        restoreDialogOpen = false;
      }}
      on:error={(event) => {
        toastStore.error(`Restore fehlgeschlagen: ${event.detail}`);
      }}
    />
  </div>
</div>

<style>
  .snapshots-page {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding: 24px 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .page-wrapper {
    width: 100%;
    min-width: 320px;
    max-width: 1600px;
    padding: 0 1rem;
  }

  .snapshots-container {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .page-header h1 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .bulk-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .selection-count {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .bulk-buttons {
    display: flex;
    gap: 0.5rem;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 3rem;
    text-align: center;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .page-header {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }

    .header-actions {
      justify-content: space-between;
    }

    .bulk-actions {
      flex-direction: column;
      gap: 0.75rem;
    }
  }
</style>
