<!-- FilterBar.svelte: Advanced Filter UI for Snapshots Page (see rustic_advanced_ui_mockup.html) -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let search = '';
  export let hostname = '';
  export let dateRange = '';
  export let size = '';
  export let tags: string[] = [];
  export let allTags: string[] = [];

  const dispatch = createEventDispatcher();

  function update() {
    dispatch('change', { search, hostname, dateRange, size, tags });
  }

  function toggleTag(tag: string) {
    if (tags.includes(tag)) {
      tags = tags.filter((t) => t !== tag);
    } else {
      tags = [...tags, tag];
    }
    update();
  }

  function removeTag(tag: string) {
    tags = tags.filter((t) => t !== tag);
    update();
  }

  function reset() {
    search = '';
    hostname = '';
    dateRange = '';
    size = '';
    tags = [];
    update();
  }
</script>

<div class="filter-bar">
  <div class="filter-row">
    <div class="filter-group">
      <label class="filter-label" for="filter-search">Suchbegriff</label>
      <input
        id="filter-search"
        class="filter-input"
        type="text"
        bind:value={search}
        oninput={update}
        placeholder="Nach Snapshot suchen..."
      />
    </div>
    <div class="filter-group">
      <label class="filter-label" for="filter-hostname">Hostname</label>
      <select id="filter-hostname" class="filter-select" bind:value={hostname} onchange={update}>
        <option value="">Alle Hosts</option>
        <option value="workstation">workstation</option>
        <option value="server-01">server-01</option>
        <option value="laptop">laptop</option>
      </select>
    </div>
    <div class="filter-group">
      <label class="filter-label" for="filter-dateRange">Zeitraum</label>
      <select id="filter-dateRange" class="filter-select" bind:value={dateRange} onchange={update}>
        <option value="">Alle</option>
        <option value="today">Heute</option>
        <option value="week">Letzte 7 Tage</option>
        <option value="month">Letzter Monat</option>
        <option value="year">Letztes Jahr</option>
        <option value="custom">Benutzerdefiniert...</option>
      </select>
    </div>
    <div class="filter-group">
      <label class="filter-label" for="filter-size">Größe</label>
      <select id="filter-size" class="filter-select" bind:value={size} onchange={update}>
        <option value="">Alle</option>
        <option value="small">&lt; 100 MB</option>
        <option value="medium">100 MB - 1 GB</option>
        <option value="large">1 GB - 10 GB</option>
        <option value="xlarge">&gt; 10 GB</option>
      </select>
    </div>
  </div>
  <div class="filter-group">
    <fieldset style="border:0;padding:0;margin:0;">
      <legend class="filter-label">Tags</legend>
      <div class="tag-filter" aria-label="Tags">
        {#each allTags as tag}
          <div
            role="button"
            tabindex="0"
            class="tag-chip {tags.includes(tag) ? 'active' : ''}"
            onclick={() => toggleTag(tag)}
            onkeydown={(e) =>
              (e.key === 'Enter' || e.key === ' ') && (e.preventDefault(), toggleTag(tag))}
            aria-pressed={tags.includes(tag)}
          >
            <span>{tag}</span>
            <button
              type="button"
              class="tag-chip-remove"
              onclick={(e) => {
                e.stopPropagation();
                removeTag(tag);
              }}
              aria-label={`Remove ${tag}`}>×</button
            >
          </div>
        {/each}
        <button
          class="btn btn-secondary"
          type="button"
          style="padding: 4px 12px; font-size: 12px;"
          onclick={() => {
            /* TODO: add tag */
          }}>+ Tag</button
        >
      </div>
    </fieldset>
  </div>
  <div class="filter-actions">
    <button class="btn btn-secondary" onclick={reset}>Reset</button>
    <button class="btn btn-primary" onclick={update}>Filter anwenden</button>
  </div>
</div>

<style>
  /* CSS aus rustic_advanced_ui_mockup.html (FilterBar) übernehmen */
  .filter-bar {
    background: #2d3348;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
  }
  .filter-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 16px;
  }
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .filter-label {
    font-size: 13px;
    color: #a1a1aa;
    font-weight: 500;
  }
  .filter-input,
  .filter-select {
    background: #1a1d2e;
    border: 1px solid #3e4457;
    border-radius: 6px;
    padding: 8px 12px;
    color: #e4e4e7;
    font-size: 14px;
  }
  .filter-input:focus,
  .filter-select:focus {
    outline: none;
    border-color: #3b82f6;
  }
  .tag-filter {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .tag-chip {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .tag-chip:hover {
    background: rgba(59, 130, 246, 0.25);
  }
  .tag-chip.active {
    background: #3b82f6;
    color: white;
  }
  .tag-chip-remove {
    cursor: pointer;
    font-weight: bold;
  }
  .filter-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 16px;
  }
  .btn {
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
</style>
