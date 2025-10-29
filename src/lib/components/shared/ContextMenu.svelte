<!-- ContextMenu.svelte: Rechtsklick-Kontextmenü für Snapshots (siehe rustic_advanced_ui_mockup.html) -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  export let x = 0;
  export let y = 0;
  export let visible = false;
  type ContextMenuAction =
    | { label: string; icon: string; danger?: boolean; action: () => void }
    | 'divider';
  export type { ContextMenuAction };
  export let actions: ContextMenuAction[] = [];

  const dispatch = createEventDispatcher();

  function handleAction(action: () => void) {
    action();
    dispatch('close');
  }
</script>

{#if visible}
  <div class="context-menu active" style="left: {x}px; top: {y}px;">
    {#each actions as item}
      {#if item === 'divider'}
        <div class="context-menu-divider"></div>
      {:else}
        <div
          class="context-menu-item {item.danger ? 'danger' : ''}"
          role="menuitem"
          tabindex="0"
          on:click={() => handleAction(item.action)}
          on:keydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') handleAction(item.action);
          }}
        >
          <span class="context-menu-icon">{item.icon}</span>
          <span>{item.label}</span>
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style>
  /* CSS aus rustic_advanced_ui_mockup.html (Context Menu) */
  .context-menu {
    position: fixed;
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    padding: 4px;
    min-width: 200px;
    z-index: 9999;
    display: block;
  }
  .context-menu-item {
    padding: 10px 12px;
    cursor: pointer;
    border-radius: 4px;
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 10px;
    color: #e4e4e7;
    transition: background 0.2s;
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
    background: #3e4457;
    margin: 4px 0;
  }
  .context-menu-icon {
    width: 16px;
    text-align: center;
  }
</style>
