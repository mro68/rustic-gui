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
  import { createEventDispatcher } from 'svelte';
  import { toastStore } from '$lib/stores/toast';
  import { compareSnapshots } from '$lib/api/snapshots';
  import type { DiffResultDto } from '$lib/types';
  
  export let snapshots: any[] = [];
  export let open = false;
  export let snapshotA: any = null;
  export let snapshotB: any = null;
  export let diff: any[] = [];
  export let statsA: any = {};
  export let statsB: any = {};
  
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
      // ✅ API-Integration (TODO.md Phase 2 Zeile 261)
      const result: DiffResultDto = await compareSnapshots(snapshotA, snapshotB);
      
      // Update diff data
      diff = result.changes || [];
      statsA = result.statsA || {};
      statsB = result.statsB || {};
      
      toastStore.success('Snapshot-Vergleich abgeschlossen');
      
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
        <div class="snapshot-selector">
          <div class="snapshot-selector-title">📸 Snapshot A (Älter)</div>
          <select class="snapshot-select" bind:value={snapshotA}>
            {#each snapshots as s}
              <option value={s.id}>{s.label}</option>
            {/each}
          </select>
        </div>
        <div class="snapshot-selector">
          <div class="snapshot-selector-title">📸 Snapshot B (Neuer)</div>
          <select class="snapshot-select" bind:value={snapshotB}>
            {#each snapshots as s}
              <option value={s.id}>{s.label}</option>
            {/each}
          </select>
        </div>
      </div>
      <div class="comparison-grid">
        <div class="comparison-panel">
          <div class="comparison-panel-header">📅 Snapshot A</div>
          <div class="comparison-stats">
            <div class="stat-item">
              <span class="stat-label">Dateien</span><span class="stat-value">{statsA.files}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Größe</span><span class="stat-value">{statsA.size}</span>
            </div>
          </div>
          <div class="comparison-list">
            {#each diff.filter((d) => d.type === 'added') as item}
              <div class="comparison-item added">
                <span class="diff-icon">➕</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
            {#each diff.filter((d) => d.type === 'modified') as item}
              <div class="comparison-item modified">
                <span class="diff-icon">✏️</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
            {#each diff.filter((d) => d.type === 'removed') as item}
              <div class="comparison-item removed">
                <span class="diff-icon">➖</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
          </div>
        </div>
        <div class="comparison-panel">
          <div class="comparison-panel-header">📅 Snapshot B</div>
          <div class="comparison-stats">
            <div class="stat-item">
              <span class="stat-label">Dateien</span><span class="stat-value">{statsB.files}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Größe</span><span class="stat-value">{statsB.size}</span>
            </div>
          </div>
          <div class="comparison-list">
            {#each diff.filter((d) => d.type === 'added') as item}
              <div class="comparison-item added">
                <span class="diff-icon">➕</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
            {#each diff.filter((d) => d.type === 'modified') as item}
              <div class="comparison-item modified">
                <span class="diff-icon">✏️</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
            {#each diff.filter((d) => d.type === 'removed') as item}
              <div class="comparison-item removed">
                <span class="diff-icon">➖</span><span class="file-path">{item.path}</span><span
                  class="file-size">{item.size}</span
                >
              </div>
            {/each}
          </div>
        </div>
      </div>
      <div class="comparison-summary">
        <div class="summary-item">
          <span class="summary-count added">+{diff.filter((d) => d.type === 'added').length}</span
          ><span style="color: #a1a1aa;">Hinzugefügt</span>
        </div>
        <div class="summary-item">
          <span class="summary-count removed"
            >-{diff.filter((d) => d.type === 'removed').length}</span
          ><span style="color: #a1a1aa;">Entfernt</span>
        </div>
        <div class="summary-item">
          <span class="summary-count modified"
            >{diff.filter((d) => d.type === 'modified').length}</span
          ><span style="color: #a1a1aa;">Geändert</span>
        </div>
      </div>
      <div style="display: flex; justify-content: flex-end; gap: 12px; margin-top: 24px;">
        <button class="btn btn-secondary" on:click={close}>Schließen</button>
        <button 
          class="btn btn-primary" 
          on:click={handleCompare}
          disabled={isComparing || !snapshotA || !snapshotB}
        >
          {isComparing ? 'Vergleiche...' : 'Vergleichen'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* CSS aus rustic_advanced_ui_mockup.html (Comparison, Modal) */
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(30, 32, 48, 0.7);
    z-index: 10000;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .comparison-modal {
    background: #22273a;
    border-radius: 16px;
    border: 1px solid #2d3348;
    padding: 32px;
    min-width: 900px;
    max-width: 98vw;
    max-height: 90vh;
    overflow-y: auto;
  }
  .comparison-header {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    margin-bottom: 20px;
  }
  .snapshot-selector {
    background: #2d3348;
    border-radius: 8px;
    padding: 16px;
  }
  .snapshot-selector-title {
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 12px;
    color: #a1a1aa;
  }
  .snapshot-select {
    width: 100%;
    background: #1a1d2e;
    border: 1px solid #3e4457;
    border-radius: 6px;
    padding: 10px 12px;
    color: #e4e4e7;
    font-size: 14px;
  }
  .comparison-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }
  .comparison-panel {
    background: #2d3348;
    border-radius: 12px;
    overflow: hidden;
  }
  .comparison-panel-header {
    background: #3e4457;
    padding: 12px 16px;
    font-size: 14px;
    font-weight: 500;
  }
  .comparison-stats {
    padding: 16px;
    border-bottom: 1px solid #3e4457;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
  }
  .stat-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .stat-label {
    font-size: 12px;
    color: #a1a1aa;
  }
  .stat-value {
    font-size: 16px;
    font-weight: 600;
  }
  .comparison-list {
    max-height: 400px;
    overflow-y: auto;
  }
  .comparison-item {
    padding: 12px 16px;
    border-bottom: 1px solid #3e4457;
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 13px;
  }
  .comparison-item.added {
    background: rgba(34, 197, 94, 0.1);
    border-left: 3px solid #22c55e;
  }
  .comparison-item.removed {
    background: rgba(239, 68, 68, 0.1);
    border-left: 3px solid #ef4444;
  }
  .comparison-item.modified {
    background: rgba(251, 191, 36, 0.1);
    border-left: 3px solid #fbbf24;
  }
  .diff-icon {
    width: 20px;
    font-size: 16px;
  }
  .file-path {
    flex: 1;
    font-family: 'Courier New', monospace;
  }
  .file-size {
    color: #a1a1aa;
    font-size: 12px;
  }
  .comparison-summary {
    padding: 16px;
    background: #3e4457;
    display: flex;
    justify-content: space-around;
    font-size: 13px;
  }
  .summary-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }
  .summary-count {
    font-size: 20px;
    font-weight: 600;
  }
  .summary-count.added {
    color: #4ade80;
  }
  .summary-count.removed {
    color: #f87171;
  }
  .summary-count.modified {
    color: #fbbf24;
  }
</style>
