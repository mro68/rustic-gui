<!--
  ActivityLog.svelte
  -----------------
  Activity Log für das Dashboard (gemäß Mockup)
-->

<script lang="ts">
  export let logEntries: { time: string; type: 'info' | 'warning' | 'error'; message: string }[] = [
    { time: '12:01', type: 'info', message: 'Backup erfolgreich abgeschlossen' },
    { time: '11:45', type: 'warning', message: 'Warnung: Verbindung langsam' },
    { time: '11:30', type: 'error', message: 'Fehler: Passwort falsch' },
  ];

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
  /* Styles gemäß Mockup werden in app.css und Komponenten-CSS gepflegt */
</style>
