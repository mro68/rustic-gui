<script lang="ts">
  /**
   * Status-Badge-Komponente.
   *
   * Zeigt farbige Status-Indikatoren mit Icon und Text.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Badge variant="healthy" text="OK" icon="✅" />
   * <Badge variant="error" text="Fehler" icon="❌" />
   * ```
   */
  import { createEventDispatcher } from 'svelte';

  interface BadgeProps {
    /** Variante für Farbe */
    variant?: 'healthy' | 'warning' | 'error' | 'info';
    /** Badge-Text */
    text?: string;
    /** Icon (Emoji oder Symbol) */
    icon?: string;
  }

  let { variant = 'healthy', text = '', icon = '●' }: BadgeProps = $props();

  const dispatch = createEventDispatcher();

  $: classes = `status-badge status-${variant}`;
</script>

<div class={classes} role="status" aria-label={text}>
  {#if icon}
    <span aria-hidden="true">{icon}</span>
  {/if}
  <span>{text}</span>
</div>

<style>
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

  .status-info {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
  }
</style>
