<!-- Tooltip.svelte: Einfache Tooltip-Komponente -->
<script lang="ts">
  import type { Snippet } from 'svelte';

  /**
   * Tooltip-Komponente für Hover-Informationen.
   *
   * Zeigt Text bei Mouse-Hover über dem Slot-Content.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Tooltip text="Klicken Sie hier zum Speichern">
   *   <Button>Speichern</Button>
   * </Tooltip>
   * ```
   */
  let {
    text = '',
    position = 'top',
    class: className = '',
    children,
  }: {
    text?: string;
    position?: 'top' | 'bottom' | 'left' | 'right';
    class?: string;
    children?: Snippet;
  } = $props();
</script>

{#if text}
  <div class="tooltip {className}" role="tooltip">
    <div class="tooltip-trigger">
      {#if children}
        {@render children()}
      {/if}
    </div>
    <div class="tooltip-text tooltip-{position}" aria-hidden="true">
      {text}
    </div>
  </div>
{:else if children}
  {@render children()}
{/if}

<style>
  .tooltip {
    position: relative;
    display: inline-block;
  }

  .tooltip-trigger {
    display: inline-block;
  }

  .tooltip-text {
    visibility: hidden;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    text-align: center;
    padding: 6px 10px;
    border-radius: 6px;
    position: absolute;
    z-index: 1000;
    white-space: nowrap;
    font-size: 12px;
    opacity: 0;
    transition: opacity 0.3s;
    pointer-events: none;
  }

  .tooltip:hover .tooltip-text {
    visibility: visible;
    opacity: 1;
  }

  .tooltip-top {
    bottom: 125%;
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-bottom {
    top: 125%;
    left: 50%;
    transform: translateX(-50%);
  }

  .tooltip-left {
    top: 50%;
    right: 125%;
    transform: translateY(-50%);
  }

  .tooltip-right {
    top: 50%;
    left: 125%;
    transform: translateY(-50%);
  }
</style>
