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
  import { createEventDispatcher, onMount, tick } from 'svelte';
  export let open: boolean = false;
  export let closeOnEsc: boolean = true;
  export let closeOnBackdrop: boolean = true;
  export let ariaLabel: string | undefined = undefined;

  const dispatch = createEventDispatcher();
  let modalRef: HTMLDivElement | null = null;
  let modalDialogRef: HTMLDivElement | null = null;
  let localOpen = open;
  let closing = false;
  // generate stable-ish id for aria
  const dialogId = `modal-${Math.random().toString(36).slice(2, 9)}`;

  function close() {
    // start close animation and dispatch after animation finished
    if (closing) return;
    closing = true;
    // wait for animation duration (match CSS below)
    setTimeout(() => {
      closing = false;
      dispatch('close');
    }, 180);
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
    localOpen = true;
    // focus next tick when opened
    tick().then(() => modalDialogRef && modalDialogRef.focus());
  } else {
    // if parent closed externally, animate close
    if (localOpen && !closing) {
      closing = true;
      setTimeout(() => {
        closing = false;
        localOpen = false;
        document.body.style.overflow = '';
      }, 180);
    } else {
      document.body.style.overflow = '';
    }
  }

  // focus dialog when opened (reactive handled above)
</script>

/* eslint-env browser */
{#if localOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="modal-backdrop {closing ? 'modal-closing' : ''}"
    bind:this={modalRef}
    role="presentation"
    aria-hidden="true"
    onclick={handleBackdropClick}
  >
    <div
      class="modal-dialog {closing ? 'modal-closing' : ''}"
      bind:this={modalDialogRef}
      role="dialog"
      aria-modal="true"
      aria-label={ariaLabel}
      aria-labelledby={ariaLabel ? undefined : dialogId + '-title'}
      tabindex="-1"
      id={dialogId}
    >
      <header class="modal-header">
        <slot name="header" />
        <button class="modal-close" aria-label="Schließen" onclick={close}>
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
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    z-index: 2000;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.18s;
  }
  .modal-dialog {
    background: #22273a;
    border-radius: 16px;
    border: 1px solid #2d3348;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    min-width: 340px;
    max-width: 96vw;
    min-height: 80px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    outline: none;
    animation: popIn 0.18s;
  }
  .modal-closing.modal-dialog,
  .modal-closing.modal-backdrop {
    animation: fadeOut 0.18s forwards;
  }
  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 24px;
    border-bottom: 1px solid #2d3348;
    font-size: 20px;
    font-weight: 600;
    color: #f4f4f5;
  }
  .modal-close {
    background: none;
    border: none;
    color: #71717a;
    font-size: 24px;
    cursor: pointer;
    padding: 0;
    width: 32px;
    height: 32px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }
  .modal-close:hover {
    background: #2d3348;
    color: #e4e4e7;
  }
  .modal-content {
    padding: 24px;
    color: #e4e4e7;
    overflow-y: auto;
    flex: 1;
  }
  .modal-footer {
    padding: 20px 24px;
    border-top: 1px solid #2d3348;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
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
  @keyframes fadeOut {
    from {
      opacity: 1;
      transform: none;
    }
    to {
      opacity: 0;
      transform: scale(0.995);
    }
  }
</style>
