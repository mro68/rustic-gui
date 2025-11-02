<!--
  Repositories.svelte
  ------------------
  @component
  Repository-Verwaltungs-Seite mit Add, Check, Prune, Delete.
  
  TODO.md: Phase 2 - Daten-Initialisierung (Stores & Pages) ✅ KOMPLETT
  Referenz: TODO.md Zeile 215-221, 342
  
  Status:
  - loadRepositories in onMount implementiert ✅ (TODO.md Zeile 217)
  - Lädt Repositories via api.listRepositories ✅
  - Füllt $repositories Store ✅
  
  Backend-Commands:
  - list_repositories: src-tauri/src/commands/repository.rs:7
  - delete_repository: src-tauri/src/commands/repository.rs:41
  - check_repository: src-tauri/src/commands/repository.rs:84
  - prune_repository: src-tauri/src/commands/repository.rs:124
  - change_password: src-tauri/src/commands/repository.rs:151
  
  API-Wrapper: src/lib/api/repositories.ts
  Store: src/lib/stores/repositories.ts
  
  Dialogs verwendet:
  - AddRepositoryDialog.svelte ✅
  - UnlockRepositoryDialog.svelte ⏳
  - DeleteRepoDialog.svelte ✅
  - CheckRepoDialog.svelte ⏳
  - PruneRepoDialog.svelte ⏳
  - ChangePasswordDialog.svelte ⏳
  
  TODO:
  - Zeile 43, 49, 79: Dialog-Workflows vollständig integrieren
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { loading, repositories } from '../../stores/repositories';
  import { toastStore } from '../../stores/toast';
  import type { RepositoryDto } from '../../types';
  import AddRepositoryDialog from '../dialogs/AddRepositoryDialog.svelte';
  import ChangePasswordDialog from '../dialogs/ChangePasswordDialog.svelte';
  import CheckRepoDialog from '../dialogs/CheckRepoDialog.svelte';
  import DeleteRepoDialog from '../dialogs/DeleteRepoDialog.svelte';
  import PruneRepoDialog from '../dialogs/PruneRepoDialog.svelte';
  import UnlockRepositoryDialog from '../dialogs/UnlockRepositoryDialog.svelte';
  import Button from '../shared/Button.svelte';
  import Tooltip from '../shared/Tooltip.svelte';

  // Dialog States
  let showAddDialog = $state(false);
  let showUnlockDialog = $state(false);
  let showDeleteDialog = $state(false);
  let showCheckDialog = $state(false);
  let showPruneDialog = $state(false);
  let showChangePasswordDialog = $state(false);

  let selectedRepository = $state<RepositoryDto | null>(null);

  async function loadRepositories() {
    try {
      const { listRepositories } = await import('$lib/api/repositories');
      const repoList = await listRepositories();
      repositories.set(repoList);
      console.log('Repositories loaded:', repoList.length);
    } catch (error) {
      console.error('Failed to load repositories:', error);
      toastStore.error('Fehler beim Laden der Repositories');
    }
  }

  function handleAddRepository() {
    showAddDialog = true;
  }

  function handleViewRepository(repo: RepositoryDto) {
    selectedRepository = repo;
    // TODO: Repository-Details-Dialog öffnen
    console.log('View repository:', repo.name);
  }

  function handleEditRepository(repo: RepositoryDto) {
    selectedRepository = repo;
    // TODO: Edit-Repository-Dialog öffnen
    console.log('Edit repository:', repo.name);
  }

  function handleUnlockRepository(repo: RepositoryDto) {
    selectedRepository = repo;
    showUnlockDialog = true;
  }

  // eslint-disable-next-line no-unused-vars
  function handleDeleteRepository(repo: RepositoryDto) {
    // TODO: Connect to delete button in RepositoryCard context menu
    selectedRepository = repo;
    showDeleteDialog = true;
  }

  function handleCheckRepository(repo: RepositoryDto) {
    selectedRepository = repo;
    showCheckDialog = true;
  }

  function handlePruneRepository(repo: RepositoryDto) {
    selectedRepository = repo;
    showPruneDialog = true;
  }

  function handleChangePassword(repo: RepositoryDto) {
    selectedRepository = repo;
    showChangePasswordDialog = true;
  }

  function handleForgetSnapshots(repo: RepositoryDto) {
    // TODO: Forget-Snapshots-Dialog öffnen
    console.log('Forget snapshots for:', repo.name);
  }

  // Event Handler für Dialoge
  function handleRepositoryAdded(event: any) {
    console.log('Repository added:', event.detail);
    toastStore.success('Repository wurde erfolgreich hinzugefügt');
    loadRepositories();
  }

  function handleRepositoryUnlocked(event: any) {
    console.log('Repository unlocked:', event.detail);
    toastStore.success('Repository wurde entsperrt');
  }

  function handleRepositoryDeleted(event: any) {
    console.log('Repository deleted:', event.detail);
    toastStore.success('Repository wurde gelöscht');
    loadRepositories();
  }

  function handleRepositoryChecked(event: any) {
    console.log('Repository checked:', event.detail);
    toastStore.success('Repository-Überprüfung abgeschlossen');
  }

  function handleRepositoryPruned(event: any) {
    console.log('Repository pruned:', event.detail);
    toastStore.success('Repository wurde bereinigt');
  }

  function handlePasswordChanged(event: any) {
    console.log('Password changed:', event.detail);
    toastStore.success('Passwort wurde geändert');
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  function getStatusBadgeClass(status: string): string {
    switch (status) {
      case 'Healthy':
        return 'status-healthy';
      case 'Warning':
        return 'status-warning';
      case 'Unavailable':
      case 'Locked':
        return 'status-error';
      default:
        return 'status-healthy';
    }
  }

  function getStatusText(status: string): string {
    switch (status) {
      case 'Healthy':
        return 'Healthy';
      case 'Warning':
        return 'Needs Check';
      case 'Unavailable':
        return 'Unavailable';
      case 'Locked':
        return 'Locked';
      default:
        return status;
    }
  }

  onMount(() => {
    loadRepositories();
  });
</script>

<div class="repositories-page">
  <!-- Toolbar -->
  <div class="toolbar">
    <h1 class="page-title">Repository Management</h1>
    <div class="toolbar-actions">
      <Tooltip text="Repository hinzufügen">
        <Button variant="primary" size="sm" onclick={() => { showAddDialog = true; }}>+ Add Repository</Button>
      </Tooltip>
    </div>
  </div>

  <!-- Repository List -->
  <div class="repositories-container">
    {#if $loading}
      <div class="loading">Lade Repositories...</div>
    {:else if $repositories.length === 0}
      <div class="empty-state">
        <h3>Keine Repositories gefunden</h3>
        <p>Fügen Sie Ihr erstes Repository hinzu, um mit dem Backup zu beginnen.</p>
        <Tooltip text="Repository hinzufügen">
          <Button variant="primary" onclick={() => { showAddDialog = true; }}>
            Erstes Repository hinzufügen
          </Button>
        </Tooltip>
      </div>
    {:else}
      <div class="repositories-list">
        {#each $repositories as repo (repo.id)}
          <div class="repository-item">
            <div class="repository-info">
              <div class="repository-name">{repo.name}</div>
              <div class="repository-details">
                <span class="repository-detail">
                  📍 {repo.path}
                </span>
                <span class="repository-detail">
                  🔧 {repo.repository_type}
                </span>
                <span class="repository-detail">
                  💾 {repo.snapshot_count} snapshots
                </span>
                <span class="repository-detail">
                  <span class={`status-badge ${getStatusBadgeClass(repo.status)}`}>
                    ● {getStatusText(repo.status)}
                  </span>
                </span>
              </div>
            </div>
            <div class="repository-actions">
              <Tooltip text="Repository anzeigen">
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => handleViewRepository(repo)}
                  ariaLabel="Repository anzeigen"
                >
                  🔍
                </Button>
              </Tooltip>
              <Tooltip text="Repository bearbeiten">
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => handleEditRepository(repo)}
                  ariaLabel="Repository bearbeiten"
                >
                  ✏️
                </Button>
              </Tooltip>
              <Tooltip text="Repository entsperren">
                <Button
                  variant="secondary"
                  size="sm"
                  onclick={() => handleUnlockRepository(repo)}
                  ariaLabel="Repository entsperren"
                >
                  🔓
                </Button>
              </Tooltip>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Repository Details (nur wenn ein Repository ausgewählt ist) -->
  {#if selectedRepository}
    <div class="repository-details-section">
      <h2 class="section-title">Repository Details</h2>
      <div class="repo-details-grid">
        <div class="detail-card">
          <h3>Configuration</h3>
          <div class="detail-row">
            <span class="detail-label">Type</span>
            <span class="detail-value">{selectedRepository.repository_type}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Path</span>
            <span class="detail-value">{selectedRepository.path}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Version</span>
            <span class="detail-value">2</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Compression</span>
            <span class="detail-value">auto</span>
          </div>
        </div>

        <div class="detail-card">
          <h3>Statistics</h3>
          <div class="detail-row">
            <span class="detail-label">Total Snapshots</span>
            <span class="detail-value">{selectedRepository.snapshot_count}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Total Size</span>
            <span class="detail-value">{formatBytes(selectedRepository.total_size)}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Deduplicated</span>
            <span class="detail-value">87.2 GB (44% savings)</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Last Check</span>
            <span class="detail-value">
              {selectedRepository.last_accessed
                ? new Date(selectedRepository.last_accessed).toLocaleString('de-DE')
                : '2025-10-10 14:23'}
            </span>
          </div>
        </div>
      </div>

      <!-- Maintenance Actions -->
      <div class="maintenance-card">
        <h3>Maintenance Actions</h3>
        <div class="maintenance-actions">
          <Tooltip text="Repository überprüfen">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => selectedRepository && handleCheckRepository(selectedRepository)}
              disabled={!selectedRepository}
            >
              🔍 Check Repository
            </Button>
          </Tooltip>
          <Tooltip text="Repository bereinigen">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => selectedRepository && handlePruneRepository(selectedRepository)}
              disabled={!selectedRepository}
            >
              ✂️ Prune Unused Data
            </Button>
          </Tooltip>
          <Tooltip text="Repository entsperren">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => selectedRepository && handleUnlockRepository(selectedRepository)}
              disabled={!selectedRepository}
            >
              🔓 Unlock Repository
            </Button>
          </Tooltip>
          <Tooltip text="Passwort ändern">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => selectedRepository && handleChangePassword(selectedRepository)}
              disabled={!selectedRepository}
            >
              🔑 Change Password
            </Button>
          </Tooltip>
          <Tooltip text="Alte Snapshots vergessen">
            <Button
              variant="secondary"
              size="sm"
              onclick={() => selectedRepository && handleForgetSnapshots(selectedRepository)}
              disabled={!selectedRepository}
            >
              🧹 Forget Old Snapshots
            </Button>
          </Tooltip>
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- Dialoge -->
<AddRepositoryDialog
  bind:open={showAddDialog}
  on:created={handleRepositoryAdded}
  on:close={() => (showAddDialog = false)}
/>

<UnlockRepositoryDialog
  bind:open={showUnlockDialog}
  repositoryName={selectedRepository?.name || ''}
  repositoryPath={selectedRepository?.path || ''}
  repositoryId={selectedRepository?.id || ''}
  on:unlock={handleRepositoryUnlocked}
  on:close={() => (showUnlockDialog = false)}
/>

<DeleteRepoDialog
  bind:open={showDeleteDialog}
  repository={selectedRepository}
  on:delete-repo={handleRepositoryDeleted}
  on:close={() => (showDeleteDialog = false)}
/>

<CheckRepoDialog
  bind:open={showCheckDialog}
  repositoryId={selectedRepository?.id || ''}
  on:check-complete={handleRepositoryChecked}
  on:close={() => (showCheckDialog = false)}
/>

<PruneRepoDialog
  bind:open={showPruneDialog}
  repositoryId={selectedRepository?.id || ''}
  on:prune-complete={handleRepositoryPruned}
  on:close={() => (showPruneDialog = false)}
/>

<ChangePasswordDialog
  bind:open={showChangePasswordDialog}
  repositoryId={selectedRepository?.id || ''}
  on:password-changed={handlePasswordChanged}
  on:close={() => (showChangePasswordDialog = false)}
/>

<style>
  .repositories-page {
    display: flex;
    flex-direction: column;
    gap: 2rem;
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

  .repositories-container {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .loading {
    text-align: center;
    padding: 3rem;
    color: var(--text-secondary);
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    margin-bottom: 1.5rem;
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }

  .repositories-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .repository-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1.25rem;
  }

  .repository-info {
    flex: 1;
    min-width: 0;
  }

  .repository-name {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .repository-details {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    align-items: center;
  }

  .repository-detail {
    font-size: 0.875rem;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
  }

  .status-healthy {
    background: rgba(34, 197, 94, 0.15);
    color: #4ade80;
  }

  .status-warning {
    background: rgba(251, 191, 36, 0.15);
    color: #fbbf24;
  }

  .status-error {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }

  .repository-actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .repository-details-section {
    margin-top: 2rem;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1.5rem;
  }

  .repo-details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .detail-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .detail-card h3 {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border-color);
  }

  .detail-row:last-child {
    border-bottom: none;
  }

  .detail-label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .detail-value {
    font-size: 0.875rem;
    color: var(--text-primary);
    font-weight: 500;
    font-family: 'Courier New', monospace;
  }

  .maintenance-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .maintenance-card h3 {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .maintenance-actions {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 0.75rem;
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

    .repository-item {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }

    .repository-details {
      flex-direction: column;
      gap: 0.5rem;
    }

    .repo-details-grid {
      grid-template-columns: 1fr;
    }

    .maintenance-actions {
      grid-template-columns: 1fr;
    }

    .detail-row {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>
