<!-- src/lib/components/dialogs/RestoreDialog.svelte -->
<script lang="ts">
  /**
   * Dialog zum Wiederherstellen von Dateien aus einem Snapshot.
   *
   * Mehrstufiger Prozess:
   * 1. Snapshot-Auswahl
   * 2. Datei-Browser mit Auswahl
   * 3. Restore-Optionen
   * 4. Wiederherstellung mit Progress
   */

  import { getFileTree, onRestoreProgress, restoreFiles } from '$lib/api/restore';
  import { listSnapshots } from '$lib/api/snapshots';
  import Button from '$lib/components/shared/Button.svelte';
  import Modal from '$lib/components/shared/Modal.svelte';
  import ProgressBar from '$lib/components/shared/ProgressBar.svelte';
  import type { FileTreeNode } from '$lib/types';
  import type { SnapshotDto } from '$lib/types/index';
  import { createEventDispatcher, onMount } from 'svelte';

  // Props
  interface Props {
    isOpen?: boolean;
    repositoryId?: string;
    repositoryPath?: string;
    repositoryPassword?: string;
  }

  let {
    isOpen = $bindable(false),
    repositoryId,
    repositoryPath,
    repositoryPassword,
  }: Props = $props();

  // State
  let snapshots = $state<SnapshotDto[]>([]);
  let selectedSnapshotId = $state<string>('');
  let fileTree = $state<FileTreeNode | null>(null);
  let selectedFiles = $state<Set<string>>(new Set());
  let currentPath = $state<string>('/');
  let breadcrumb = $state<string[]>(['home']);
  let isLoadingTree = $state(false);
  let isRestoring = $state(false);
  let restoreProgress = $state<number>(0);
  let restoreMessage = $state<string>('');

  // Restore Options
  let targetPath = $state('/tmp/restored');
  let overwritePolicy = $state<'skip' | 'overwrite' | 'rename'>('overwrite');
  let restorePermissions = $state(true);
  let restoreTimestamps = $state(true);

  // Events
  const dispatch = createEventDispatcher<{
    restored: { files: string[]; targetPath: string };
    error: string;
  }>();

  // Load snapshots on mount
  onMount(async () => {
    if (repositoryId) {
      try {
        snapshots = await listSnapshots();
        if (snapshots.length > 0) {
          selectedSnapshotId = snapshots[0].id;
          await loadFileTree();
        }
      } catch (error) {
        console.error('Failed to load snapshots:', error);
      }
    }
  });

  // Load file tree when snapshot changes
  $effect(() => {
    if (selectedSnapshotId && repositoryPath && repositoryPassword) {
      loadFileTree();
    }
  });

  async function loadFileTree() {
    if (!repositoryPath || !repositoryPassword || !selectedSnapshotId) return;

    isLoadingTree = true;
    try {
      fileTree = await getFileTree(
        repositoryPath,
        repositoryPassword,
        selectedSnapshotId,
        currentPath
      );
    } catch (error) {
      console.error('Failed to load file tree:', error);
      dispatch('error', 'Failed to load file tree');
    } finally {
      isLoadingTree = false;
    }
  }

  function navigateToPath(path: string) {
    currentPath = path;
    breadcrumb = path.split('/').filter((p) => p);
    if (breadcrumb.length === 0) breadcrumb = ['home'];
    loadFileTree();
  }

  function goUp() {
    const parts = currentPath.split('/').filter((p) => p);
    if (parts.length > 1) {
      parts.pop();
      navigateToPath('/' + parts.join('/'));
    } else {
      navigateToPath('/');
    }
  }

  function toggleFileSelection(filePath: string) {
    if (selectedFiles.has(filePath)) {
      selectedFiles.delete(filePath);
    } else {
      selectedFiles.add(filePath);
    }
    selectedFiles = new Set(selectedFiles); // Trigger reactivity
  }

  function getSelectedFileNames(): string {
    if (selectedFiles.size === 0) return 'No files selected';

    const names = Array.from(selectedFiles).map((path) => {
      const parts = path.split('/');
      return parts[parts.length - 1] || path;
    });

    if (names.length <= 3) {
      return names.join(', ');
    } else {
      return `${names.slice(0, 3).join(', ')} (+${names.length - 3} more)`;
    }
  }

  function getSelectedFileSize(): string {
    // This would need actual file size data from the tree
    // For now, return placeholder
    return `${selectedFiles.size} files`;
  }

  async function handleRestore() {
    if (!repositoryPath || !repositoryPassword || !selectedSnapshotId || selectedFiles.size === 0) {
      return;
    }

    isRestoring = true;
    restoreProgress = 0;
    restoreMessage = 'Starting restore...';

    try {
      // Listen for progress updates
      const unlisten = await onRestoreProgress(null, (progress) => {
        restoreProgress = progress.percentage;
        restoreMessage = progress.message || `Restoring ${progress.current_file || ''}`;
      });

      // Start restore
      await restoreFiles(
        repositoryPath,
        repositoryPassword,
        selectedSnapshotId,
        Array.from(selectedFiles),
        targetPath,
        {
          overwrite: overwritePolicy === 'overwrite',
          restore_permissions: restorePermissions,
          restore_timestamps: restoreTimestamps,
          dry_run: false,
        }
      );

      // Cleanup
      unlisten();

      // Success
      dispatch('restored', {
        files: Array.from(selectedFiles),
        targetPath,
      });

      // Close dialog
      handleClose();
    } catch (error) {
      console.error('Restore failed:', error);
      dispatch('error', error instanceof Error ? error.message : 'Restore failed');
    } finally {
      isRestoring = false;
    }
  }

  function handleClose() {
    isOpen = false;
    // Reset state
    selectedFiles = new Set();
    fileTree = null;
    currentPath = '/';
    breadcrumb = ['home'];
    isRestoring = false;
    restoreProgress = 0;
    restoreMessage = '';
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleString('de-DE', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  }
</script>

<Modal open={isOpen} on:close={handleClose}>
  <div slot="header">
    <h2>Restore Files</h2>
  </div>

  <div class="restore-dialog">
    {#if !isRestoring}
      <!-- Snapshot Selection -->
      <div class="form-group">
        <label for="snapshot-select" class="form-label">Select Snapshot</label>
        <select id="snapshot-select" class="form-input" bind:value={selectedSnapshotId}>
          {#each snapshots as snapshot}
            <option value={snapshot.id}>
              {formatDate(snapshot.time)} - {snapshot.id.slice(0, 8)}
            </option>
          {/each}
        </select>
      </div>

      <!-- File Browser -->
      {#if fileTree}
        <div class="file-browser">
          <div class="file-browser-header">
            <div class="breadcrumb">
              <button
                class="breadcrumb-item"
                onclick={() => navigateToPath('/')}
                disabled={currentPath === '/'}
              >
                home
              </button>
              {#each breadcrumb as crumb, index}
                <span>/</span>
                <button
                  class="breadcrumb-item"
                  onclick={() => {
                    const pathParts = breadcrumb.slice(0, index + 1);
                    navigateToPath('/' + pathParts.join('/'));
                  }}
                >
                  {crumb}
                </button>
              {/each}
            </div>
            <div class="browser-actions">
              <Button variant="secondary" size="sm" on:click={goUp} disabled={currentPath === '/'}>
                ↑ Up
              </Button>
              <Button
                variant="secondary"
                size="sm"
                on:click={loadFileTree}
                disabled={isLoadingTree}
              >
                ⟳ Refresh
              </Button>
            </div>
          </div>

          <div class="file-list">
            {#if isLoadingTree}
              <div class="loading">Loading files...</div>
            {:else if fileTree.children}
              {#each fileTree.children as node}
                <div
                  class="file-item"
                  class:selected={selectedFiles.has(node.path)}
                  role="button"
                  tabindex="0"
                  onclick={() => {
                    if (node.is_directory) {
                      navigateToPath(node.path);
                    } else {
                      toggleFileSelection(node.path);
                    }
                  }}
                  onkeydown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.preventDefault();
                      if (node.is_directory) {
                        navigateToPath(node.path);
                      } else {
                        toggleFileSelection(node.path);
                      }
                    }
                  }}
                >
                  <input
                    type="checkbox"
                    class="file-checkbox"
                    checked={selectedFiles.has(node.path)}
                    onchange={(e) => {
                      e.stopPropagation();
                      toggleFileSelection(node.path);
                    }}
                  />
                  <span class="file-icon">
                    {node.is_directory ? '📁' : '📄'}
                  </span>
                  <span class="file-name">{node.name}</span>
                  {#if !node.is_directory && node.size !== undefined}
                    <span class="file-size">{formatBytes(node.size)}</span>
                  {:else if node.is_directory}
                    <span class="file-size">{node.children?.length || 0} items</span>
                  {/if}
                </div>
              {/each}
            {/if}
          </div>
        </div>

        <!-- Selected Files Info -->
        {#if selectedFiles.size > 0}
          <div class="selected-files-info">
            <strong>{selectedFiles.size} files selected</strong> ({getSelectedFileSize()}) - {getSelectedFileNames()}
          </div>
        {/if}

        <!-- Restore Options -->
        <div class="restore-options">
          <div class="restore-options-title">Restore Options</div>

          <div class="form-group">
            <label for="target-path" class="form-label">Restore to</label>
            <div class="input-with-button">
              <input
                id="target-path"
                type="text"
                class="form-input monospace"
                bind:value={targetPath}
                placeholder="/path/to/restore"
              />
              <Button variant="secondary" size="sm">📁 Browse</Button>
            </div>
          </div>

          <div class="form-group">
            <label for="overwrite-policy" class="form-label">If file exists</label>
            <select id="overwrite-policy" class="form-input" bind:value={overwritePolicy}>
              <option value="skip">Skip existing files</option>
              <option value="overwrite">Overwrite existing files</option>
              <option value="rename">Rename restored files (add .restored suffix)</option>
            </select>
          </div>

          <div class="checkbox-group">
            <input id="restore-permissions" type="checkbox" bind:checked={restorePermissions} />
            <label for="restore-permissions">Restore original file permissions</label>
          </div>

          <div class="checkbox-group">
            <input id="restore-timestamps" type="checkbox" bind:checked={restoreTimestamps} />
            <label for="restore-timestamps">Restore original timestamps</label>
          </div>
        </div>
      {/if}
    {:else}
      <!-- Restore Progress -->
      <div class="restore-progress">
        <h3>Restoring Files...</h3>
        <ProgressBar value={restoreProgress} label="Restoring files..." />
        <p class="progress-message">{restoreMessage}</p>
      </div>
    {/if}
  </div>

  <div slot="footer">
    <Button variant="secondary" on:click={handleClose} disabled={isRestoring}>Cancel</Button>

    {#if !isRestoring}
      <Button
        variant="primary"
        on:click={handleRestore}
        disabled={selectedFiles.size === 0 || !fileTree}
      >
        Restore Selected Files
      </Button>
    {/if}
  </div>
</Modal>

<style>
  .restore-dialog {
    max-width: 900px;
    width: 100%;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: 8px;
  }

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 14px;
  }

  .form-input:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .monospace {
    font-family: 'Courier New', monospace;
  }

  .file-browser {
    border: 1px solid var(--border-color);
    border-radius: 8px;
    overflow: hidden;
    max-height: 400px;
    overflow-y: auto;
    margin-bottom: 16px;
  }

  .file-browser-header {
    background: var(--bg-tertiary);
    padding: 12px 16px;
    font-size: 13px;
    font-weight: 500;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .breadcrumb {
    display: flex;
    gap: 8px;
    align-items: center;
    color: var(--text-secondary);
    font-size: 13px;
  }

  .breadcrumb-item {
    cursor: pointer;
    color: var(--color-primary);
  }

  .breadcrumb-item:hover {
    text-decoration: underline;
  }

  .breadcrumb-item:disabled {
    cursor: not-allowed;
    color: var(--text-secondary);
  }

  .browser-actions {
    display: flex;
    gap: 8px;
  }

  .file-list {
    background: var(--bg-primary);
  }

  .file-item {
    padding: 12px 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--border-color);
    transition: background 0.2s;
  }

  .file-item:hover {
    background: var(--bg-hover);
  }

  .file-item.selected {
    background: rgba(59, 130, 246, 0.15);
  }

  .file-checkbox {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .file-icon {
    font-size: 18px;
    width: 20px;
  }

  .file-name {
    flex: 1;
    font-size: 14px;
  }

  .file-size {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .selected-files-info {
    background: var(--bg-tertiary);
    padding: 12px 16px;
    border-radius: 6px;
    font-size: 13px;
    margin-bottom: 16px;
  }

  .restore-options {
    background: var(--bg-tertiary);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
  }

  .restore-options-title {
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 12px;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
  }

  .input-with-button .form-input {
    flex: 1;
  }

  .checkbox-group {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
  }

  .checkbox-group input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .checkbox-group label {
    font-size: 14px;
    color: var(--text-primary);
    cursor: pointer;
  }

  .restore-progress {
    text-align: center;
    padding: 40px 20px;
  }

  .restore-progress h3 {
    margin-bottom: 20px;
    color: var(--text-primary);
  }

  .progress-message {
    margin-top: 10px;
    font-size: 14px;
    color: var(--text-secondary);
  }

  .loading {
    padding: 40px;
    text-align: center;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .restore-options {
      grid-template-columns: 1fr;
    }

    .file-browser {
      max-height: 300px;
    }

    .input-with-button {
      flex-direction: column;
    }

    .checkbox-group {
      flex-direction: column;
      align-items: flex-start;
      gap: 8px;
    }
  }
</style>
