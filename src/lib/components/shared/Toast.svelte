/* eslint-env browser */
<script lang="ts">
  /**
   * Toast/Notification-Komponente für Rustic GUI
   *
   * Props:
   * - type: 'success' | 'error' | 'warning' | 'info' (default: 'info')
   * - message: string
   * - duration: number (ms, default: 3500)
   * - onClose: () => void (optional)
   *
   * Slots:
   * - default: Optionaler eigener Inhalt
   */
  import { onMount } from 'svelte';
  export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
  export let message: string = '';
  export let duration: number = 3500;
  export let onClose: (() => void) | undefined = undefined;

  let timer: any;

  onMount(() => {
    if (duration > 0) {
      // eslint-disable-next-line no-undef
      timer = setTimeout(() => {
        close();
      }, duration);
    }
    // eslint-disable-next-line no-undef
    return () => clearTimeout(timer);
  });

  function close() {
    if (onClose) onClose();
  }

  $: icon =
    type === 'success' ? '✔️'
    : type === 'error' ? '❌'
    : type === 'warning' ? '⚠️'
    : 'ℹ️';
</script>

<div class="toast toast-{type}" role="status" aria-live="polite">
  <span class="toast-icon" aria-hidden="true">{icon}</span>
  <span class="toast-message">
    <slot>{message}</slot>
  </span>
  <button class="toast-close" aria-label="Schließen" on:click={close}>
    &times;
  </button>
</div>

<style>
  .toast {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 220px;
    max-width: 360px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border-radius: var(--radius-md);
    box-shadow: 0 2px 16px rgba(0,0,0,0.18);
    padding: 14px 18px;
    font-size: 15px;
    margin-bottom: 12px;
    border-left: 5px solid var(--color-primary);
    animation: toastIn 0.18s;
    position: relative;
  }
  .toast-success { border-left-color: var(--color-success); }
  .toast-error { border-left-color: var(--color-error); }
  .toast-warning { border-left-color: var(--color-warning); }
  .toast-info { border-left-color: var(--color-primary); }
  .toast-icon {
    font-size: 20px;
    margin-right: 2px;
    flex-shrink: 0;
  }
  .toast-message {
    flex: 1;
    word-break: break-word;
  }
  .toast-close {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 22px;
    cursor: pointer;
    margin-left: 8px;
    border-radius: 6px;
    padding: 0 4px;
    transition: background 0.15s;
  }
  .toast-close:hover {
    background: var(--bg-tertiary);
    color: var(--color-error);
  }
  @keyframes toastIn {
    from { opacity: 0; transform: translateY(16px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
