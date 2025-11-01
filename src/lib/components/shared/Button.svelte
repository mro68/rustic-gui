<script lang="ts">
  /**
   * Universelle Button-Komponente für Rustic GUI.
   *
   * Bietet verschiedene Varianten, Größen und States (Loading, Disabled).
   * Unterstützt Icons und Tooltips für bessere UX.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Button variant="primary" onclick={() => save()}>Speichern</Button>
   * <Button variant="danger" loading={isSaving}>Löschen</Button>
   * <Button size="sm" icon="📁">Öffnen</Button>
   * ```
   */
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import type { Snippet } from 'svelte';

  interface ButtonProps {
    /** Button-Variante für visuelles Styling */
    variant?: 'primary' | 'secondary' | 'danger';
    /** Button-Größe */
    size?: 'sm' | 'md' | 'lg';
    /** Zeigt Loading-Spinner und deaktiviert Button */
    loading?: boolean;
    /** Deaktiviert den Button */
    disabled?: boolean;
    /** Icon (Emoji-String oder Svelte-Component) */
    icon?: any;
    /** HTML Button-Type */
    type?: 'button' | 'submit' | 'reset';
    /** Aria-Label für Accessibility */
    ariaLabel?: string;
    /** Tooltip-Text bei Hover */
    tooltip?: string;
    /** Click-Handler */
    onclick?: (() => void) | undefined;
    /** Children content */
    children?: Snippet;
  }

  // Props als exportierte Variablen (test-kompatibel)
  export let variant: ButtonProps['variant'] = 'primary';
  export let size: ButtonProps['size'] = 'md';
  export let loading: boolean = false;
  export let disabled: boolean = false;
  export let icon: any = null;
  export let type: ButtonProps['type'] = 'button';
  export let ariaLabel: string | undefined = undefined;
  export let tooltip: string | undefined = undefined;
  export let onclick: (() => void) | undefined = undefined;
  export let children: Snippet | undefined = undefined;

  // Kombinierte Klassen (reactive)
  $: btnClass = `btn btn-${variant} btn-${size} ${loading ? 'is-loading' : ''}`.trim();
</script>

<Tooltip text={tooltip}>
  <button
    class={btnClass}
    {type}
    disabled={disabled || loading}
    aria-busy={loading}
    aria-label={ariaLabel}
    {onclick}
  >
    {#if loading}
      <span class="spinner" aria-hidden="true"></span>
    {:else if icon}
      <span class="btn-icon" role="img" aria-hidden="true">
        {#if typeof icon === 'string'}
          {icon}
        {:else}
          <!-- Dynamic component rendering - icon should be a Snippet in Svelte 5 -->
          {icon}
        {/if}
      </span>
    {/if}
    {#if children}
      <span class="btn-content">{@render children()}</span>
    {/if}
  </button>
</Tooltip>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
    border-radius: 6px;
    border: none;
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      box-shadow 0.12s,
      transform 0.08s;
    outline: none;
  }
  .btn-primary {
    background: #3b82f6;
    color: #fff;
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
  .btn:active {
    transform: translateY(1px) scale(0.997);
  }
  .btn:focus-visible {
    box-shadow:
      0 6px 18px rgba(37, 99, 235, 0.12),
      0 0 0 4px rgba(59, 130, 246, 0.12);
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .btn-danger {
    background: #ef4444;
    color: #fff;
  }
  .btn-danger:hover {
    background: #dc2626;
  }
  .btn-sm {
    height: 32px;
    font-size: 13px;
    padding: 0 12px;
  }
  .btn-md {
    height: 40px;
    font-size: 15px;
    padding: 0 18px;
  }
  .btn-lg {
    height: 48px;
    font-size: 17px;
    padding: 0 24px;
  }
  .btn-icon {
    display: flex;
    align-items: center;
    font-size: 18px;
    margin-right: 4px;
  }
  .spinner {
    width: 18px;
    height: 18px;
    border: 2.5px solid #fff;
    border-top: 2.5px solid #2563eb;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    margin-right: 4px;
  }
  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
  @media (max-width: 768px) {
    .btn {
      min-height: 48px;
      padding: 12px 20px;
      font-size: 16px;
    }
  }
  .btn-content {
    display: flex;
    align-items: center;
    gap: 4px;
  }
</style>
