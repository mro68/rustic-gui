<script lang="ts">
  /**
   * ProgressBar-Komponente für Fortschrittsanzeige.
   *
   * Unterstützt bestimmte (0-100) und unbestimmte (spinning) Progress.
   * Mit optionalem Label für Kontext.
   *
   * @component
   *
   * @example
   * ```svelte
   * <!-- Bestimmter Progress -->
   * <ProgressBar value={75} label="Backup läuft..." />
   *
   * <!-- Unbestimmter Progress -->
   * <ProgressBar label="Initialisiere..." />
   * ```
   */

  interface ProgressBarProps {
    /** Progress-Wert (0-100), undefined = unbestimmter Progress */
    value?: number;
    /** Optionales Label */
    label?: string;
  }

  let { value = undefined, label = '' }: ProgressBarProps = $props();
</script>

/* eslint-env browser */
<div
  class="progress-bar"
  role="progressbar"
  aria-valuenow={value ?? undefined}
  aria-valuemin={value !== undefined ? 0 : undefined}
  aria-valuemax={value !== undefined ? 100 : undefined}
>
  <div
    class="progress-fill {value === undefined ? 'indeterminate' : ''}"
    style={value !== undefined ? `width: ${value}%` : ''}
  ></div>
</div>
{#if label}
  <div class="progress-label">{label}</div>
{/if}

<style>
  .progress-bar {
    width: 100%;
    height: 8px;
    background: #2d3348;
    border-radius: 4px;
    overflow: hidden;
    margin-top: 12px;
    position: relative;
  }
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--color-primary) 0%, var(--color-primary-dark) 100%);
    border-radius: 4px;
    transition: width 0.3s;
  }
  .progress-fill.indeterminate {
    width: 40%;
    min-width: 80px;
    position: absolute;
    left: 0;
    animation: indeterminate 1.2s infinite linear;
  }
  @keyframes indeterminate {
    0% {
      left: -40%;
    }
    100% {
      left: 100%;
    }
  }
  .progress-label {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: #71717a;
    margin-top: 8px;
  }
</style>
