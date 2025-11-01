<script lang="ts">
  /**
   * Toast/Notification-Komponente für Rustic GUI.
   *
   * Zeigt zeitlich begrenzte Benachrichtigungen mit Auto-Dismiss.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Toast
   *   type="success"
   *   title="Erfolg"
   *   message="Backup erfolgreich erstellt"
   *   duration={3500}
   *   onClose={handleClose}
   * />
   * ```
   */
  import { onMount, onDestroy } from 'svelte';
  import type { Snippet } from 'svelte';

  // Props als normale Variablen (test-kompatibel)
  export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
  export let title: string | undefined = undefined;
  export let message: string = '';
  export let duration: number = 3500;
  export let onClose: (() => void) | undefined = undefined;
  export let children: Snippet | undefined = undefined;

  let visible: boolean = true;
  let timer: any;

  function startTimer() {
    if (duration > 0) {
      timer = setTimeout(() => {
        close();
      }, duration);
    }
  }

  function clearTimer() {
    if (timer) {
      clearTimeout(timer);
      timer = undefined;
    }
  }

  onMount(() => {
    startTimer();
    return () => clearTimer();
  });

  function close() {
    visible = false;
    // wait for animation (200ms) then invoke onClose
    setTimeout(() => {
      onClose?.();
    }, 200);
  }

  function handleMouseEnter() {
    clearTimer();
  }

  function handleMouseLeave() {
    startTimer();
  }

  // Derive icon from type
  $: icon = type === 'success' ? '✔️' : type === 'error' ? '❌' : type === 'warning' ? '⚠️' : 'ℹ️';
</script>

{#if visible}
  <div
    class="toast toast-{type}"
    role="status"
    aria-live="polite"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
  >
    <span class="toast-icon" aria-hidden="true">{icon}</span>
    <div class="toast-content">
      {#if title}
        <div class="toast-title">{title}</div>
      {/if}
      <div class="toast-message">
        {#if children}
          {@render children()}
        {:else}
          {message}
        {/if}
      </div>
    </div>
    <button class="toast-close" aria-label="Schließen" onclick={close}> &times; </button>
  </div>
{/if}

<style>
  .toast {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    min-width: 220px;
    max-width: 360px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    border-radius: var(--radius-md);
    box-shadow: 0 2px 16px rgba(0, 0, 0, 0.18);
    padding: 14px 18px;
    font-size: 15px;
    margin-bottom: 12px;
    border-left: 5px solid var(--color-primary);
    animation: toastIn 0.18s;
    position: relative;
    opacity: 1;
    transform: translateY(0);
    transition:
      opacity 0.2s,
      transform 0.2s;
  }
  .toast:not(.visible) {
    opacity: 0;
    transform: translateY(12px);
  }
  .toast-success {
    border-left-color: var(--color-success);
  }
  .toast-error {
    border-left-color: var(--color-error);
  }
  .toast-warning {
    border-left-color: var(--color-warning);
  }
  .toast-info {
    border-left-color: var(--color-primary);
  }
  .toast-icon {
    font-size: 20px;
    margin-right: 2px;
    flex-shrink: 0;
  }
  .toast-content {
    flex: 1;
  }
  .toast-title {
    font-weight: 600;
    margin-bottom: 4px;
  }
  .toast-message {
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
    from {
      opacity: 0;
      transform: translateY(16px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
