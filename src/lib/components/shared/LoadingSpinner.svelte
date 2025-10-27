/* eslint-env browser */
<script lang="ts">
  /**
   * LoadingSpinner-Komponente für Ladezustände
   *
   * Props:
   * - size: number (px, default: 40)
   * - color: string (CSS, default: var(--color-primary))
   */
  export let size: number = 40;
  export let color: string = 'var(--color-primary)';
</script>

<div class="spinner" style="width: {size}px; height: {size}px; border-top-color: {color};"></div>

<style>
  .spinner {
    border: 3px solid var(--bg-tertiary);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    display: inline-block;
  }
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
