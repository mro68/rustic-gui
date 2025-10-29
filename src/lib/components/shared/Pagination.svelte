<!-- Pagination.svelte: Pagination Controls für Snapshots (siehe rustic_advanced_ui_mockup.html) -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let page = 1;
  export let pageSize = 25;
  export let total = 0;
  export let pageSizes = [10, 25, 50, 100];

  const dispatch = createEventDispatcher();

  function setPage(p: number) {
    if (p < 1 || p > Math.ceil(total / pageSize)) return;
    dispatch('page', p);
  }
  function setPageSize(size: number) {
    dispatch('pageSize', size);
  }
</script>

<div class="pagination">
  <div class="pagination-info">
    Zeige {(page - 1) * pageSize + 1}-{Math.min(page * pageSize, total)} von {total} Snapshots
  </div>
  <div class="pagination-controls">
    <button class="pagination-btn" on:click={() => setPage(1)} disabled={page === 1}
      >‹‹ Erste</button
    >
    <button class="pagination-btn" on:click={() => setPage(page - 1)} disabled={page === 1}
      >‹ Zurück</button
    >
    {#each Array(Math.ceil(total / pageSize))
      .fill(0)
      .map((_, i) => i + 1) as p (p)}
      {#if Math.abs(p - page) <= 2 || p === 1 || p === Math.ceil(total / pageSize)}
        <button class="pagination-btn {p === page ? 'active' : ''}" on:click={() => setPage(p)}
          >{p}</button
        >
      {:else if p === page - 3 || p === page + 3}
        <span style="padding: 6px 12px; color: #a1a1aa;">...</span>
      {/if}
    {/each}
    <button
      class="pagination-btn"
      on:click={() => setPage(page + 1)}
      disabled={page === Math.ceil(total / pageSize)}>Weiter ›</button
    >
    <button
      class="pagination-btn"
      on:click={() => setPage(Math.ceil(total / pageSize))}
      disabled={page === Math.ceil(total / pageSize)}>Letzte ››</button
    >
  </div>
  <div class="page-size-select">
    <span style="color: #a1a1aa;">Zeilen pro Seite:</span>
    <select
      class="filter-select"
      bind:value={pageSize}
      on:change={(e) => {
        const target = e.target as HTMLSelectElement;
        setPageSize(+target.value);
      }}
    >
      {#each pageSizes as size}
        <option value={size}>{size}</option>
      {/each}
    </select>
  </div>
</div>

<style>
  /* CSS aus rustic_advanced_ui_mockup.html (Pagination) */
  .pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-top: 1px solid #3e4457;
  }
  .pagination-info {
    font-size: 13px;
    color: #a1a1aa;
  }
  .pagination-controls {
    display: flex;
    gap: 4px;
  }
  .pagination-btn {
    padding: 6px 12px;
    background: #2d3348;
    border: 1px solid #3e4457;
    border-radius: 6px;
    color: #e4e4e7;
    cursor: pointer;
    font-size: 13px;
    transition: all 0.2s;
  }
  .pagination-btn:hover:not(:disabled) {
    background: #3e4457;
  }
  .pagination-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .pagination-btn.active {
    background: #3b82f6;
    border-color: #3b82f6;
  }
  .page-size-select {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
  }
  .filter-select {
    background: #1a1d2e;
    border: 1px solid #3e4457;
    border-radius: 6px;
    padding: 4px 8px;
    color: #e4e4e7;
    font-size: 14px;
    cursor: pointer;
  }
</style>
