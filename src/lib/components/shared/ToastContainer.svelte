/* eslint-env browser */
<script lang="ts">
  /**
   * ToastContainer-Komponente für globale Toast-Notifications
   *
   * Zeigt alle aktiven Toasts aus dem toastStore an.
   */

  import type { Toast as ToastType } from '$lib/stores/toast';
  import Toast from './Toast.svelte';
  import { toastStore } from '$lib/stores/toast';

  let toasts = $state<ToastType[]>([]);

  $effect(() => {
    const unsub = toastStore.subscribe((list) => {
      toasts = list;
    });
    return unsub;
  });
</script>

<div class="toast-container">
  {#each toasts as toast (toast.id)}
    <Toast
      type={toast.type}
      message={toast.message}
      duration={toast.duration}
      on:close={() => toastStore.remove(toast.id)}
    />
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 28px;
    right: 28px;
    z-index: 3000;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    pointer-events: none;
  }
  .toast-container :global(.toast) {
    pointer-events: auto;
  }
</style>
