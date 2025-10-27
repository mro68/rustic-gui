/* eslint-env browser */
<script lang="ts">
  /**
   * ProgressBar-Komponente für Fortschrittsanzeigen
   *
   * Props:
   * - value: number | undefined (0-100, determiniert; undefined = indeterminiert)
   * - label: string (optional)
   */
  export let value: number | undefined = undefined;
  export let label: string = '';
</script>

<div class="progress-bar" role="progressbar" aria-valuenow={value ?? undefined} aria-valuemin={value !== undefined ? 0 : undefined} aria-valuemax={value !== undefined ? 100 : undefined}>
  <div class="progress-fill {value === undefined ? 'indeterminate' : ''}" style={value !== undefined ? `width: ${value}%` : ''}></div>
</div>
{#if label}
  <div class="progress-label">{label}</div>
{/if}

<style>
  .progress-bar {
    width: 100%;
    height: 8px;
    background: var(--bg-tertiary);
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
    0% { left: -40%; }
    100% { left: 100%; }
  }
  .progress-label {
    margin-top: 4px;
    font-size: 13px;
    color: var(--text-secondary);
    text-align: right;
  }
</style>
