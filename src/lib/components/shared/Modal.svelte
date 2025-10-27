<script lang="ts">
  /* eslint-env browser */
  /**
   * Universelle Modal-Komponente für Rustic GUI
   *
   * Props:
   * - open: boolean (default: false)
   * - closeOnEsc: boolean (default: true)
   * - closeOnBackdrop: boolean (default: true)
   * - ariaLabel: string (optional)
   *
   * Events:
   * - on:close
   *
   * Slots:
   * - default: Modal-Inhalt
   * - header: Optionaler Header
   * - footer: Optionaler Footer
   */
  import { createEventDispatcher, onMount } from 'svelte';
  export let open: boolean = false;
  export let closeOnEsc: boolean = true;
  export let closeOnBackdrop: boolean = true;
  export let ariaLabel: string | undefined = undefined;

  const dispatch = createEventDispatcher();
  let modalRef: HTMLDivElement | null = null;

  function close() {
    dispatch('close');
  }

  function handleBackdropClick(e: MouseEvent) {
    if (closeOnBackdrop && e.target === modalRef) {
      close();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (open && closeOnEsc && e.key === 'Escape') {
      close();
    }
  }

  onMount(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
    }
    window.addEventListener('keydown', handleKeydown);
    return () => {
      document.body.style.overflow = '';
      window.removeEventListener('keydown', handleKeydown);
    };
  });

  $: if (open) {
    document.body.style.overflow = 'hidden';
  } else {
    document.body.style.overflow = '';
  }
</script>

/* eslint-env browser */
{#if open}
  <div
    class="modal-backdrop"
    bind:this={modalRef}
    aria-modal="true"
    role="presentation"
    aria-label={ariaLabel}
    on:click={handleBackdropClick}
  >
    <div class="modal-dialog" tabindex="0">
      <header class="modal-header">
        <slot name="header" />
        <button class="modal-close" aria-label="Schließen" on:click={close}>
          <span aria-hidden="true">&times;</span>
        </button>
      </header>
      <div class="modal-content">
        <slot />
      </div>
      {#if $$slots.footer}
        <footer class="modal-footer">
          <slot name="footer" />
        </footer>
      {/if}
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(30, 32, 48, 0.75);
    z-index: 2000;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.2s;
  }
  .modal-dialog {
    background: var(--bg-primary);
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
    min-width: 340px;
    max-width: 96vw;
    min-height: 80px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    outline: none;
    animation: popIn 0.18s;
  }
  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 18px 24px 12px 24px;
    border-bottom: 1px solid var(--border-color);
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .modal-close {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 28px;
    cursor: pointer;
    padding: 0 4px;
    border-radius: 6px;
    transition: background 0.15s;
  }
  .modal-close:hover {
    background: var(--bg-tertiary);
    color: var(--color-error);
  }
  .modal-content {
    padding: 24px;
    color: var(--text-primary);
    overflow-y: auto;
  }
  .modal-footer {
    padding: 16px 24px 18px 24px;
    border-top: 1px solid var(--border-color);
    background: var(--bg-secondary);
    border-radius: 0 0 var(--radius-lg) var(--radius-lg);
  }
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
  @keyframes popIn {
    from {
      transform: scale(0.97);
      opacity: 0.7;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }
</style>
