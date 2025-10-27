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
   *
   * Slots:
   * - default: Button-Inhalt
   */
  export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let loading: boolean = false;
  export let disabled: boolean = false;
  export let icon: any = null;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let ariaLabel: string | undefined = undefined;

  // Kombinierte Klassen
  $: btnClass = `btn btn-${variant} btn-${size} ${loading ? 'is-loading' : ''}`;
</script>

<button
  class={btnClass}
  type={type}
  disabled={disabled || loading}
  aria-busy={loading}
  aria-label={ariaLabel}
>
  {#if loading}
    <span class="spinner" aria-hidden="true"></span>
  {:else if icon}
    <span class="btn-icon" aria-hidden="true">{#if typeof icon === 'string'}{icon}{:else}<svelte:component this={icon} />{/if}</span>
  {/if}
  <span class="btn-content"><slot /></span>
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
    border-radius: var(--radius-md);
    border: none;
    cursor: pointer;
    transition: background 0.15s, color 0.15s, box-shadow 0.15s;
    outline: none;
    padding: 0 18px;
    height: 40px;
    font-size: 15px;
    background: var(--color-primary);
    color: #fff;
  }
  .btn:disabled,
  .btn[aria-busy='true'] {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
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
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  .btn-content {
    display: flex;
    align-items: center;
    gap: 4px;
  }
</style>
