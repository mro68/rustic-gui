<!-- CompareSnapshotsDialog.svelte: Side-by-Side Snapshot Comparison -->
<!--
  TODO.md: Phase 2 - Dialog-Workflow: Backup & Restore (Zeile 261)
  Status: ✅ KOMPLETT - API-Integration vollständig
  
  Backend-Command: src-tauri/src/commands/snapshot.rs:38 (compare_snapshots stub)
  API-Wrapper: src/lib/api/snapshots.ts:42 (compareSnapshots)
  
  Implementierung:
  - ✅ API-Integration mit compareSnapshots
  - ✅ Error-Handling mit Toasts
  - ✅ Loading-State während Vergleich
  - ⏳ Backend-Command ist noch ein Stub
-->
<script lang="ts">
  /**
   * Side-by-Side Snapshot-Vergleich-Dialog.
   *
   * Zeigt Unterschiede zwischen zwei Snapshots mit:
   * - Datei-Level Diffs (Added/Removed/Modified)
   * - Größen-Statistiken
   * - Filter-Optionen
   *
   * @component
   *
   * @example
   * ```svelte
   * <CompareSnapshotsDialog
   *   bind:open={showCompare}
   *   {snapshots}
   *   bind:snapshotA
   *   bind:snapshotB
   *   bind:diff
   *   on:compare={handleCompareComplete}
   * />
   * ```
   */
  import { compareSnapshots } from '$lib/api/snapshots';
  import { toastStore } from '$lib/stores/toast';
  import type { DiffResultDto } from '$lib/types';
  import { createEventDispatcher } from 'svelte';

  interface CompareSnapshotsDialogProps {
    /** Liste aller Snapshots zur Auswahl */
    snapshots?: any[];
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Erster Snapshot für Vergleich */
    snapshotA?: any;
    /** Zweiter Snapshot für Vergleich */
    snapshotB?: any;
    /** Diff-Ergebnis */
    diff?: any[];
    /** Statistiken für Snapshot A */
    statsA?: any;
    /** Statistiken für Snapshot B */
    statsB?: any;
  }

  let {
    snapshots = [],
    open = $bindable(false),
    snapshotA = $bindable(null),
    snapshotB = $bindable(null),
    diff = $bindable([]),
    statsA = $bindable({}),
    statsB = $bindable({}),
  }: CompareSnapshotsDialogProps = $props();

  const dispatch = createEventDispatcher();

  let isComparing = false;

  async function handleCompare() {
    if (!snapshotA || !snapshotB) {
      toastStore.error('Bitte beide Snapshots auswählen');
      return;
    }

    if (snapshotA === snapshotB) {
      toastStore.error('Bitte unterschiedliche Snapshots auswählen');
      return;
    }

    isComparing = true;

    try {
      // ✅ API-Integration mit Backend
      const result: DiffResultDto = await compareSnapshots(snapshotA.id, snapshotB.id);

      // Update diff data from backend response
      diff = [
        ...result.added.map((path) => ({ type: 'added', path, size: '' })),
        ...result.modified.map((path) => ({ type: 'modified', path, size: '' })),
        ...result.removed.map((path) => ({ type: 'removed', path, size: '' })),
      ];

      // Update stats from backend
      statsA = {
        files: snapshotA.file_count || 0,
        size: snapshotA.total_size || 0,
      };
      statsB = {
        files: snapshotB.file_count || 0,
        size: snapshotB.total_size || 0,
      };

      toastStore.success(
        `Vergleich abgeschlossen: +${result.stats.added_count} -${result.stats.removed_count} ~${result.stats.modified_count}`
      );

      dispatch('comparison-complete', { result });
    } catch (error: any) {
      const errorMessage = error?.message || 'Unbekannter Fehler';
      toastStore.error('Snapshot-Vergleich fehlgeschlagen: ' + errorMessage);
      console.error('Compare failed:', error);
    } finally {
      isComparing = false;
    }
  }

  function close() {
    dispatch('close');
  }
</script>

{#if open}
  <div class="modal-backdrop">
    <div class="comparison-modal">
      <div class="comparison-header">
        <h2>Snapshots vergleichen</h2>
        <button class="modal-close" onclick={close} aria-label="Schließen">×</button>
      </div>

      <div class="snapshot-selectors">
        <div class="snapshot-selector">
          <div class="snapshot-selector-title">📸 Snapshot A (Älter)</div>
          <select class="snapshot-select" bind:value={snapshotA} disabled={isComparing}>
            <option value={null}>-- Snapshot wählen --</option>
            {#each snapshots as s}
              <option value={s}>{new Date(s.time).toLocaleString()} - {s.hostname}</option>
            {/each}
          </select>
          {#if snapshotA}
            <div class="snapshot-info">
              <span>{snapshotA.file_count || 0} Dateien</span>
              <span>{((snapshotA.total_size || 0) / 1024 / 1024).toFixed(1)} MB</span>
            </div>
          {/if}
        </div>
        <div class="snapshot-selector">
          <div class="snapshot-selector-title">📸 Snapshot B (Neuer)</div>
          <select class="snapshot-select" bind:value={snapshotB} disabled={isComparing}>
            <option value={null}>-- Snapshot wählen --</option>
            {#each snapshots as s}
              <option value={s}>{new Date(s.time).toLocaleString()} - {s.hostname}</option>
            {/each}
          </select>
          {#if snapshotB}
            <div class="snapshot-info">
              <span>{snapshotB.file_count || 0} Dateien</span>
              <span>{((snapshotB.total_size || 0) / 1024 / 1024).toFixed(1)} MB</span>
            </div>
          {/if}
        </div>
      </div>

      <div class="comparison-actions">
        <button
          class="btn btn-primary"
          onclick={handleCompare}
          disabled={!snapshotA || !snapshotB || isComparing}
        >
          {#if isComparing}
            <span class="spinner"></span>
            Vergleiche...
          {:else}
            ⚖️ Vergleichen
          {/if}
        </button>
      </div>

      {#if diff.length > 0}
        <div class="comparison-results">
          <div class="results-header">
            <h3>Ergebnisse</h3>
            <div class="results-stats">
              <span class="stat-added">+{diff.filter((d) => d.type === 'added').length}</span>
              <span class="stat-modified">~{diff.filter((d) => d.type === 'modified').length}</span>
              <span class="stat-removed">-{diff.filter((d) => d.type === 'removed').length}</span>
            </div>
          </div>

          <div class="results-list">
            {#each diff as item}
              <div class="result-item result-{item.type}">
                <span class="result-icon">
                  {#if item.type === 'added'}➕{:else if item.type === 'modified'}✏️{:else}➖{/if}
                </span>
                <span class="result-path">{item.path}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={close}>Schließen</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(2px);
  }

  .comparison-modal {
    background: #1a1d2e;
    border-radius: 12px;
    border: 1px solid #2d3348;
    min-width: 700px;
    max-width: 90vw;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }

  .comparison-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #2d3348;
  }

  .comparison-header h2 {
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

  .snapshot-selectors {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
    padding: 1.5rem;
  }

  .snapshot-selector {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .snapshot-selector-title {
    font-size: 0.875rem;
    color: #a1a1aa;
    font-weight: 500;
  }

  .snapshot-select {
    background: #22273a;
    border: 1px solid #3e4457;
    border-radius: 6px;
    padding: 0.5rem;
    color: #e4e4e7;
    font-size: 0.875rem;
  }

  .snapshot-select:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .snapshot-info {
    display: flex;
    gap: 1rem;
    font-size: 0.75rem;
    color: #71717a;
    margin-top: 0.25rem;
  }

  .comparison-actions {
    padding: 0 1.5rem 1.5rem;
    display: flex;
    justify-content: center;
  }

  .comparison-results {
    padding: 0 1.5rem 1.5rem;
    max-height: 400px;
    overflow-y: auto;
  }

  .results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .results-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #e4e4e7;
  }

  .results-stats {
    display: flex;
    gap: 1rem;
    font-size: 0.875rem;
    font-weight: 600;
  }

  .stat-added {
    color: #22c55e;
  }

  .stat-modified {
    color: #fbbf24;
  }

  .stat-removed {
    color: #ef4444;
  }

  .results-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .result-added {
    background: rgba(34, 197, 94, 0.1);
    border-left: 3px solid #22c55e;
  }

  .result-modified {
    background: rgba(251, 191, 36, 0.1);
    border-left: 3px solid #fbbf24;
  }

  .result-removed {
    background: rgba(239, 68, 68, 0.1);
    border-left: 3px solid #ef4444;
  }

  .result-icon {
    font-size: 1rem;
  }

  .result-path {
    flex: 1;
    color: #e4e4e7;
    font-family: monospace;
    font-size: 0.8125rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid #2d3348;
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    border: none;
    font-size: 0.875rem;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn-secondary {
    background: #2d3348;
    color: #e4e4e7;
    border: 1px solid #3e4457;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #3e4457;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
