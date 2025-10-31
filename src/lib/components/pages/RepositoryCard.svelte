<!--
  RepositoryCard.svelte
  --------------------
  Einzelne Repository-Karte für das Dashboard (gemäß Mockup)
-->

<script lang="ts">
  /**
   * Repository-Karte für Dashboard.
   *
   * Zeigt Repository-Status, Snapshot-Count, Größe.
   * Mit Quick-Actions (Backup starten, Details, etc.).
   *
   * @component
   *
   * @example
   * ```svelte
   * <RepositoryCard {repo} />
   * ```
   */
  import { onDestroy, onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { cancelBackup, runBackup } from '../../api/backup';
  import { jobs } from '../../stores/backup-jobs';
  import type { RepositoryDto } from '../../types';
  import RunBackupDialog from '../dialogs/RunBackupDialog.svelte';
  import Tooltip from '../shared/Tooltip.svelte';

  interface RepositoryCardProps {
    /** Repository-Daten */
    repo?: RepositoryDto;
  }

  let {
    repo = {
      id: '',
      name: 'Unknown Repository',
      path: '/',
      repository_type: 'Local',
      status: 'Healthy',
      snapshot_count: 0,
      total_size: 0,
      created_at: new Date().toISOString(),
    },
  }: RepositoryCardProps = $props();

  // Status-Badge dynamisch (Platzhalter-Logik)
  let status: 'healthy' | 'warning' | 'error' = 'healthy';
  let statusText = status === 'healthy' ? 'Healthy' : status === 'warning' ? 'Warning' : 'Error';

  // Backup-Dialog State
  let showBackupDialog = false;
  let currentJobId = '';
  let currentJobName = '';

  function handleBackup() {
    // Finde einen Backup-Job für dieses Repository
    const allJobs = get(jobs);
    const repoJob = allJobs.find((job) => job.repository_id === repo.id);

    if (!repoJob) {
      // TODO: Dialog öffnen um Job zu erstellen
      console.warn(`Kein Backup-Job für Repository ${repo.name} gefunden`);
      return;
    }

    // Job gefunden - Backup starten
    currentJobId = repoJob.id;
    currentJobName = repoJob.name;
    showBackupDialog = true;

    // Backup starten
    runBackup(repoJob.id).catch((error) => {
      console.error('Backup-Fehler:', error);
      showBackupDialog = false;
    });
  }

  function handleCancelBackup() {
    if (currentJobId) {
      cancelBackup(currentJobId).catch((error) => {
        console.error('Cancel-Backup-Fehler:', error);
      });
    }
  }

  function handleBrowse() {
    // TODO: Repository-Browser öffnen
    console.log(`Browse für ${repo.name} geöffnet (Platzhalter)`);
  }

  // Kontextmenü State
  let showContextMenu = $state(false);
  let contextMenuX = $state(0);
  let contextMenuY = $state(0);

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    event.stopPropagation();
    showContextMenu = true;
    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
  }

  function closeContextMenu() {
    showContextMenu = false;
  }

  function handleEdit() {
    // TODO: Edit-Repository-Dialog öffnen
    console.log(`Edit für ${repo.name} geöffnet (Platzhalter)`);
    closeContextMenu();
  }

  function handleDuplicate() {
    // TODO: Duplicate-Repository-Dialog öffnen
    console.log(`Duplicate für ${repo.name} geöffnet (Platzhalter)`);
    closeContextMenu();
  }

  function handleDelete() {
    // TODO: Delete-Repository-Dialog öffnen
    console.log(`Delete für ${repo.name} geöffnet (Platzhalter)`);
    closeContextMenu();
  }

  function handleContextMenuKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleContextMenu(event as any);
    }
  }

  // Schließe Menü bei Klick außerhalb
  function handleGlobalClick(event: MouseEvent) {
    if (showContextMenu && !(event.target as Element)?.closest('.context-menu')) {
      closeContextMenu();
    }
  }

  onMount(() => {
    document.addEventListener('click', handleGlobalClick);
  });

  onDestroy(() => {
    document.removeEventListener('click', handleGlobalClick);
  });
</script>

<div class="card">
  <div class="card-header">
    <div>
      <div class="card-title">{repo.name}</div>
      <div class="card-subtitle">{repo.path}</div>
    </div>
    <div style="display: flex; align-items: flex-start; gap: 8px;">
      <span class={`status-badge status-${status}`} aria-label={statusText} title={statusText}>
        <span class="status-dot" aria-hidden="true">●</span>
        <span>{statusText}</span>
      </span>
      <span
        class="menu-dots"
        onclick={handleContextMenu}
        onkeydown={handleContextMenuKeydown}
        role="button"
        tabindex="0"
        aria-label="Repository-Optionen"
        title="Optionen">&#8942;</span
      >
    </div>
  </div>
  <div class="repo-info">
    <div class="repo-stat">
      <div class="repo-stat-label">Last Accessed</div>
      <div class="repo-stat-value">
        {repo.last_accessed ? new Date(repo.last_accessed).toLocaleDateString() : '-'}
      </div>
    </div>
    <div class="repo-stat">
      <div class="repo-stat-label">Snapshots</div>
      <div class="repo-stat-value">{repo.snapshot_count}</div>
    </div>
    <div class="repo-stat">
      <div class="repo-stat-label">Size</div>
      <div class="repo-stat-value">{repo.total_size} bytes</div>
    </div>
    <div class="repo-stat">
      <div class="repo-stat-label">Status</div>
      <div class="repo-stat-value">{repo.status}</div>
    </div>
  </div>
  <div class="progress-bar" aria-label="Backup-Fortschritt">
    <div class="progress-fill" style="width: 0%"></div>
  </div>
  <div class="progress-label">
    <span></span>
  </div>
  <div class="card-actions">
    <Tooltip text="Backup jetzt starten">
      <button
        class="btn btn-primary"
        onclick={handleBackup}
        aria-label="Backup jetzt starten"
        title="Backup Now"
      >
        <span class="btn-icon" aria-hidden="true">🔄</span>
        <span class="btn-text">Backup Now</span>
      </button>
    </Tooltip>

    <Tooltip text="Repository durchsuchen">
      <button
        class="btn btn-secondary"
        onclick={handleBrowse}
        aria-label="Repository durchsuchen"
        title="Browse Repository"
      >
        <span class="btn-icon" aria-hidden="true">📂</span>
        <span class="btn-text">Browse</span>
      </button>
    </Tooltip>

    <Tooltip text="Repository konfigurieren">
      <button
        class="btn btn-secondary"
        aria-label="Repository konfigurieren"
        title="Configure Repository"
        disabled
      >
        <span class="btn-icon" aria-hidden="true">⚙️</span>
        <span class="btn-text">Configure</span>
      </button>
    </Tooltip>
  </div>
</div>

<!-- Backup-Dialog -->
<RunBackupDialog
  bind:open={showBackupDialog}
  jobName={currentJobName}
  jobId={currentJobId}
  onCancel={handleCancelBackup}
/>

<!-- Kontextmenü -->
{#if showContextMenu}
  <div
    class="context-menu"
    style="left: {contextMenuX}px; top: {contextMenuY}px;"
    role="menu"
    aria-label="Repository-Optionen"
  >
    <button class="context-menu-item" onclick={handleEdit} type="button">
      <span>✏️</span>
      <span>Edit</span>
    </button>
    <button class="context-menu-item" onclick={handleDuplicate} type="button">
      <span>📋</span>
      <span>Duplicate</span>
    </button>
    <div class="context-menu-divider"></div>
    <button class="context-menu-item danger" onclick={handleDelete} type="button">
      <span>🗑️</span>
      <span>Delete</span>
    </button>
  </div>
{/if}

<style>
  .card {
    background: #22273a;
    border-radius: 12px;
    padding: 24px;
    border: 1px solid #2d3348;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 20px;
  }
  .card-title {
    font-size: 18px;
    font-weight: 600;
    color: #f4f4f5;
  }
  .card-subtitle {
    font-size: 13px;
    color: #71717a;
    margin-top: 4px;
  }
  .status-badge {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 6px;
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
  .status-dot {
    font-size: 16px;
    margin-right: 2px;
  }
  .repo-info {
    display: flex;
    gap: 24px;
    margin-bottom: 16px;
    flex-wrap: wrap;
  }
  .repo-stat {
    flex: 1;
    min-width: 90px;
  }
  .repo-stat-label {
    font-size: 12px;
    color: #71717a;
    margin-bottom: 4px;
  }
  .repo-stat-value {
    font-size: 16px;
    font-weight: 600;
    color: #f4f4f5;
  }
  .progress-bar {
    width: 100%;
    height: 8px;
    background: #2d3348;
    border-radius: 4px;
    overflow: hidden;
    margin-top: 12px;
  }
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6 0%, #2563eb 100%);
    border-radius: 4px;
    transition: width 0.3s;
  }
  .progress-label {
    display: flex;
    justify-content: flex-end;
    font-size: 12px;
    color: #71717a;
    margin-top: 8px;
    min-height: 18px;
  }
  .card-actions {
    display: flex;
    gap: 8px;
    margin-top: 16px;
  }
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 6px;
    border: none;
    font-size: 13px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }
  .btn-primary {
    background: #3b82f6;
    color: white;
  }
  .btn-primary:hover {
    background: #2563eb;
  }
  .btn-secondary {
    background: #2d3348;
    color: #e4e4e7;
    border: 1px solid #3e4457;
  }
  .btn-secondary:hover {
    background: #3e4457;
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .btn-icon {
    font-size: 16px;
    display: flex;
    align-items: center;
  }
  .btn-text {
    display: inline;
  }
  .context-menu {
    position: fixed;
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    padding: 4px;
    min-width: 180px;
    z-index: 1500;
    display: block;
  }
  .context-menu-item {
    padding: 8px 12px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 10px;
    color: #e4e4e7;
    transition: background-color 0.2s;
  }
  .context-menu-item:hover {
    background: #2d3348;
  }
  .context-menu-item.danger {
    color: #f87171;
  }
  .context-menu-item.danger:hover {
    background: rgba(239, 68, 68, 0.1);
  }
  .context-menu-divider {
    height: 1px;
    background: #2d3348;
    margin: 4px 0;
  }
  .menu-dots {
    cursor: pointer;
    color: #71717a;
    font-size: 20px;
    padding: 4px;
    border-radius: 4px;
    transition: color 0.2s;
  }
  .menu-dots:hover {
    color: #e4e4e7;
    background: rgba(255, 255, 255, 0.1);
  }
  @media (max-width: 1024px) {
    .card {
      padding: 16px;
    }
    .repo-info {
      gap: 12px;
    }
  }
  @media (max-width: 768px) {
    .card {
      padding: 12px;
    }
    .card-header {
      flex-direction: column;
      gap: 8px;
    }
    .repo-info {
      flex-direction: column;
      gap: 8px;
    }
    .card-actions {
      flex-direction: column;
      gap: 8px;
      margin-top: 12px;
    }
  }
</style>
