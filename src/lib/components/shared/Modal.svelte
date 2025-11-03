<script lang="ts">
  import { onMount, tick } from 'svelte';
  import type { Snippet } from 'svelte';

  let {
    open = $bindable(false),
    closeOnEsc = true,
    closeOnBackdrop = true,
    size = 'medium',
    ariaLabel = undefined,
    onclose = undefined,
    header = undefined,
    children = undefined,
    footer = undefined,
  }: {
    open?: boolean;
    closeOnEsc?: boolean;
    closeOnBackdrop?: boolean;
    size?: 'small' | 'medium' | 'large';
    ariaLabel?: string;
    onclose?: () => void;
    header?: Snippet;
    children?: Snippet;
    footer?: Snippet;
  } = $props();

  let modalRef: HTMLDivElement | null = $state(null);
  let modalDialogRef: HTMLDivElement | null = $state(null);
  let closing: boolean = $state(false);
  const dialogId = `modal-${Math.random().toString(36).slice(2, 9)}`;

  function close() {
    if (closing) return;
    closing = true;
    setTimeout(() => {
      closing = false;
      open = false;
      onclose?.();
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
    window.addEventListener('keydown', handleKeydown);
    return () => {
      document.body.style.overflow = '';
      window.removeEventListener('keydown', handleKeydown);
    };
  });

  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      tick().then(() => modalDialogRef && modalDialogRef.focus());
    } else {
      document.body.style.overflow = '';
    }
  });
</script>

{#if open}
  <div
    class="modal-backdrop {closing ? 'modal-closing' : ''}"
    bind:this={modalRef}
    role="presentation"
    onclick={handleBackdropClick}
  >
    <div
      class="modal-dialog modal-{size} {closing ? 'modal-closing' : ''}"
      bind:this={modalDialogRef}
      role="dialog"
      aria-modal="true"
      aria-label={ariaLabel}
      aria-labelledby={ariaLabel ? undefined : dialogId + '-title'}
      tabindex="-1"
      id={dialogId}
    >
      {#if header}
        <header class="modal-header">
          {@render header()}
          <button class="modal-close" aria-label="Schließen" onclick={close}>
            <span aria-hidden="true">&times;</span>
          </button>
        </header>
      {/if}
      <div class="modal-content">
        {#if children}
          {@render children()}
        {/if}
      </div>
      {#if footer}
        <footer class="modal-footer">
          {@render footer()}
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
    max-width: 600px;
    min-height: 80px;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    outline: none;
    animation: popIn 0.18s;
  }
  .modal-small {
    max-width: 400px;
  }
  .modal-medium {
    max-width: 600px;
  }
  .modal-large {
    max-width: 900px;
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
