<!--
  ActivityLog.svelte
  -----------------
  Activity Log für das Dashboard (gemäß Mockup)
-->

<script lang="ts">
  /**
   * Activity-Log-Komponente für Dashboard.
   *
   * Zeigt chronologische Liste von Events (Info/Warning/Error).
   *
   * @component
   *
   * @example
   * ```svelte
   * <ActivityLog {logEntries} />
   * ```
   */

  interface ActivityLogProps {
    /** Log-Einträge mit Zeit, Typ, Nachricht */
    logEntries?: { time: string; type: 'info' | 'warning' | 'error'; message: string }[];
  }

  let {
    logEntries = [
      { time: '12:01', type: 'info', message: 'Backup erfolgreich abgeschlossen' },
      { time: '11:45', type: 'warning', message: 'Warnung: Verbindung langsam' },
      { time: '11:30', type: 'error', message: 'Fehler: Passwort falsch' },
    ],
  }: ActivityLogProps = $props();

  function getIcon(type: 'info' | 'warning' | 'error') {
    if (type === 'info') return '✔️';
    if (type === 'warning') return '⚠️';
    if (type === 'error') return '❌';
    return '';
  }
</script>

<div class="log-container">
  {#each logEntries as entry}
    <div class="log-line">
      <span class="log-time">{entry.time}</span>
      <span>{getIcon(entry.type)}</span>
      <span class={`log-${entry.type}`}>{entry.message}</span>
    </div>
  {/each}
</div>

<style>
  .log-container {
    background: #22273a;
    border-radius: 12px;
    border: 1px solid #2d3348;
    padding: 16px;
    margin-top: 24px;
    font-family: 'Courier New', monospace;
    font-size: 12px;
    max-height: 200px;
    overflow-y: auto;
  }

  .log-line {
    margin-bottom: 6px;
    display: flex;
    gap: 10px;
    line-height: 1.5;
  }

  .log-time {
    color: #71717a;
    flex-shrink: 0;
  }

  .log-info {
    color: #4ade80;
  }

  .log-warning {
    color: #fbbf24;
  }

  .log-error {
    color: #f87171;
  }
</style>
