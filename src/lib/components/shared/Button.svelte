<script lang="ts">
  /**
   * Universelle Button-Komponente für Rustic GUI
   *
   * Props:
   * - variant: 'primary' | 'secondary' | 'danger' (default: 'primary')
   * - size: 'sm' | 'md' | 'lg' (default: 'md')
   * - loading: boolean (default: false)
   * - disabled: boolean (default: false)
   * - icon: SvelteComponent | string (optional)
   * - tooltip: string (optional)
   *
   * Slots:
   * - default: Button-Inhalt
   */
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let loading: boolean = false;
  export let disabled: boolean = false;
  export let icon: any = null;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let ariaLabel: string | undefined = undefined;
  export let tooltip: string | undefined = undefined;
  export let onclick: (() => void) | undefined = undefined;

  // Kombinierte Klassen
  $: btnClass = `btn btn-${variant} btn-${size} ${loading ? 'is-loading' : ''}`;
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
      <span class="btn-icon" aria-hidden="true"
        >{#if typeof icon === 'string'}{icon}{:else}<svelte:component this={icon} />{/if}</span
      >
    {/if}
    <span class="btn-content"><slot /></span>
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
    padding: 10px 20px;
    height: 40px;
    font-size: 14px;
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
  .btn-danger {
    background: var(--color-error);
    color: #fff;
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
    border-top: 2.5px solid var(--color-primary-dark);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    margin-right: 4px;
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
