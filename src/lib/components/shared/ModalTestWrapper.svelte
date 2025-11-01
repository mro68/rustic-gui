<script lang="ts">
  import Modal from './Modal.svelte';

  export let open: boolean = false;
  export let onclose: (() => void) | undefined = undefined;
  export let showHeader: boolean = false;
</script>

<Modal {open} {onclose}>
  {#snippet header()}
    {#if showHeader}
      <h2>Test Header</h2>
    {/if}
  {/snippet}
  {#snippet children()}
    <p>Test Content</p>
  {/snippet}
</Modal>
