<!-- RecentTab.svelte: Recent Locations List -->
<script lang="ts">
  interface RecentLocation {
    id: string;
    name: string;
    path: string;
    type: string;
    icon: string;
    lastUsed: string;
  }

  interface RecentTabProps {
    /** Liste der zuletzt verwendeten Locations */
    recentLocations: RecentLocation[];
    /** Callback bei Location-Auswahl */
    onSelect: (location: RecentLocation) => void;
  }

  let { recentLocations = [], onSelect }: RecentTabProps = $props();
</script>

<div class="tab-panel">
  <div class="info-box">🕐 Zuletzt verwendete Repository-Speicherorte</div>

  <div class="recent-list">
    {#each recentLocations as location}
      <button class="recent-item" onclick={() => onSelect(location)}>
        <span class="recent-icon">{location.icon}</span>
        <div class="recent-info">
          <div class="recent-path">{location.path}</div>
          <div class="recent-type">{location.type} • Zuletzt: {location.lastUsed}</div>
        </div>
      </button>
    {/each}
  </div>
</div>

<style>
  .tab-panel {
    animation: fadeIn 0.2s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    padding: 12px 16px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  .recent-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .recent-item {
    padding: 12px 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-secondary);
    transition: all 0.2s;
    text-align: left;
  }

  .recent-item:hover {
    background: var(--bg-tertiary);
    border-color: var(--color-primary);
  }

  .recent-icon {
    font-size: 24px;
  }

  .recent-info {
    flex: 1;
  }

  .recent-path {
    font-size: 14px;
    font-weight: 500;
    font-family: 'Courier New', monospace;
    margin-bottom: 4px;
  }

  .recent-type {
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>
