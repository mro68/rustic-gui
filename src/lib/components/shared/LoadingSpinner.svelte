<script lang="ts">
  /**
   * LoadingSpinner-Komponente für Ladezustände.
   *
   * Animierter Spinner mit konfigurierbarer Größe und Farbe.
   *
   * @component
   *
   * @example
   * ```svelte
   * <LoadingSpinner size={60} color="var(--color-primary)" />
   * ```
   */

  interface LoadingSpinnerProps {
    /** Spinner-Größe in Pixel */
    size?: number;
    /** Spinner-Farbe (CSS) */
    color?: string;
  }

  let { size = 40, color = 'var(--color-primary)' }: LoadingSpinnerProps = $props();
</script>

/* eslint-env browser */
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
    to {
      transform: rotate(360deg);
    }
  }
</style>
