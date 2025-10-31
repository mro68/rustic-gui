<!-- ContextMenu.svelte: Rechtsklick-Kontextmenü für Snapshots (siehe rustic_advanced_ui_mockup.html) -->
<script lang="ts">
  /**
   * Kontextmenü-Komponente für Rechtsklick-Aktionen.
   *
   * Zeigt Menü an Position (x, y) mit konfigurierbaren Actions.
   *
   * @component
   *
   * @example
   * ```svelte
   * <ContextMenu
   *   bind:visible={showMenu}
   *   x={menuX}
   *   y={menuY}
   *   {actions}
   *   on:close={handleClose}
   * />
   * ```
   */
  import { createEventDispatcher } from 'svelte';

  type ContextMenuAction =
    | { label: string; icon: string; danger?: boolean; action: () => void }
    | 'divider';
  export type { ContextMenuAction };

  interface ContextMenuProps {
    /** X-Position des Menüs */
    x?: number;
    /** Y-Position des Menüs */
    y?: number;
    /** Sichtbarkeit */
    visible?: boolean;
    /** Menü-Aktionen */
    actions?: ContextMenuAction[];
  }

  let {
    x = $bindable(0),
    y = $bindable(0),
    visible = $bindable(false),
    actions = [],
  }: ContextMenuProps = $props();

  const dispatch = createEventDispatcher();
  let containerRef: HTMLDivElement | null = $state(null);

  function handleAction(action: () => void) {
    action();
    dispatch('close');
  }

  // focus first item when menu opens for accessibility
  $effect(() => {
    if (visible) {
      // wait for DOM update
      setTimeout(() => {
        const first = containerRef?.querySelector('.context-menu-item') as HTMLElement | null;
        first?.focus();
      }, 0);
    }
  });
</script>

{#if visible}
  <div class="context-menu active" style="left: {x}px; top: {y}px;" bind:this={containerRef}>
    {#each actions as item}
      {#if item === 'divider'}
        <div class="context-menu-divider"></div>
      {:else}
        <div
          class="context-menu-item {item.danger ? 'danger' : ''}"
          role="menuitem"
          tabindex="0"
          onclick={() => handleAction(item.action)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') handleAction(item.action);
            else if (e.key === 'ArrowDown') {
              e.preventDefault();
              const next = (e.target as HTMLElement).nextElementSibling as HTMLElement | null;
              next?.focus();
            } else if (e.key === 'ArrowUp') {
              e.preventDefault();
              const prev = (e.target as HTMLElement).previousElementSibling as HTMLElement | null;
              prev?.focus();
            }
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
  .context-menu-item:focus {
    outline: none;
    background: #2d3348;
    box-shadow: 0 6px 18px rgba(59, 130, 246, 0.08);
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
