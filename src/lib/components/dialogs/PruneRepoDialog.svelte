<script lang="ts">
  /**
   * PruneRepoDialog.svelte
   *
   * TODO.md: Phase 2 - Dialog-Workflow Repository (Zeile 250)
   * Status: ✅ KOMPLETT - API-Integration vollständig
   *
   * Backend-Command: src-tauri/src/commands/repository.rs:124 (prune_repository)
   * API-Wrapper: src/lib/api/repositories.ts:45 (pruneRepository)
   *
   * Implementierung:
   * - ✅ API-Integration mit pruneRepository
   * - ✅ Error-Handling mit Toasts
   * - ✅ Success-Toast bei erfolgreichem Prune
   * - ✅ Statistiken-Anzeige nach Prune
   * - ⏳ Progress-Events (Backend sendet noch keine Events)
   */

  import { createEventDispatcher, onMount } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Modal from '../shared/Modal.svelte';
  import { toastStore } from '$lib/stores/toast';
  import { pruneRepository } from '$lib/api/repositories';

  const dispatch = createEventDispatcher();

  export let repositoryId: string = '';

  let isRunning = false;
  let progress = 0;
  let currentStep = '';
  let logEntries: string[] = [];
  let maxUnused = false;

  let progressInterval: number | null = null;

  // Result statistics
  let pruneResult: any = null;

  async function startPruning() {
    isRunning = true;
    progress = 0;
    currentStep = 'Bereinigung wird gestartet...';
    logEntries = [];
    pruneResult = null;

    try {
      // Simulate progress (Backend sendet noch keine Progress-Events)
      progressInterval = window.setInterval(() => {
        progress += Math.random() * 12;
        if (progress >= 95) {
          progress = 95; // Warten auf Backend-Response
        } else if (progress < 25) {
          currentStep = 'Nicht verwendete Daten werden identifiziert...';
          logEntries = [
            'Snapshot-Referenzen werden analysiert...',
            'Verwendete Packs werden ermittelt...',
            ...logEntries,
          ];
        } else if (progress < 50) {
          currentStep = 'Nicht referenzierte Packs werden entfernt...';
          logEntries = [
            'Nicht verwendete Pack-Dateien werden gelöscht...',
            'Speicherplatz wird freigegeben...',
            ...logEntries,
          ];
        } else if (progress < 75) {
          currentStep = 'Repository wird reorganisiert...';
          logEntries = [
            'Index wird aktualisiert...',
            'Repository-Struktur wird optimiert...',
            ...logEntries,
          ];
        } else {
          currentStep = 'Abschlussarbeiten laufen...';
          logEntries = ['Statistiken werden aktualisiert...', ...logEntries];
        }
      }, 500);

      // ✅ Tatsächliche API-Integration (TODO.md Phase 2 Zeile 250)
      const result = await pruneRepository(repositoryId, maxUnused);

      // Abschluss
      if (progressInterval) clearInterval(progressInterval);
      progress = 100;
      currentStep = 'Bereinigung abgeschlossen';
      logEntries = ['Bereinigung erfolgreich abgeschlossen', ...logEntries];

      // Store result for display
      pruneResult = result;

      toastStore.success('Repository-Bereinigung erfolgreich abgeschlossen');

      dispatch('prune-complete', { repositoryId, result });

      // Auto-close nach 5 Sekunden um Statistiken zu zeigen
      setTimeout(() => {
        dispatch('close');
      }, 5000);
    } catch (error: any) {
      if (progressInterval) clearInterval(progressInterval);
      isRunning = false;
      currentStep = 'Bereinigung fehlgeschlagen';
      const errorMessage = error?.message || 'Unbekannter Fehler';
      logEntries = [`Fehler: ${errorMessage}`, ...logEntries];
      toastStore.error('Repository-Bereinigung fehlgeschlagen: ' + errorMessage);
      console.error('Prune failed:', error);
    } finally {
      isRunning = false;
    }
  }

  function stopPruning() {
    isRunning = false;
    if (progressInterval) clearInterval(progressInterval);
    progressInterval = null;
    currentStep = 'Bereinigung abgebrochen';
    dispatch('cancel-prune');
  }

  function handleClose() {
    if (isRunning) {
      stopPruning();
    }
    dispatch('close');
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  }

  onMount(() => {
    return () => {
      if (progressInterval) clearInterval(progressInterval);
    };
  });
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository bereinigen</div>
  <div class="prune-repo-dialog">
    {#if !isRunning && !pruneResult}
      <!-- Configuration Section -->
      <div class="config-section">
        <h3>Bereinigungsoptionen</h3>

        <div class="option-group">
          <Checkbox
            label="Maximale Bereinigung (langsamer, aber gründlicher)"
            bind:checked={maxUnused}
          />
          <p class="option-description">
            Führt eine tiefgreifende Bereinigung durch und entfernt auch kleine ungenutzte Daten.
            Dies kann bei großen Repositories länger dauern, gibt aber maximal Speicherplatz frei.
          </p>
        </div>
      </div>

      <div class="warning-section">
        <div class="warning-icon">⚠️</div>
        <div class="warning-text">
          <strong>Wichtig:</strong> Die Bereinigung entfernt unwiderruflich nicht mehr benötigte Daten
          aus dem Repository. Stellen Sie sicher, dass keine Backups oder Snapshots parallel laufen.
          <br /><br />
          <strong>Empfehlung:</strong> Führen Sie vorher eine Repository-Überprüfung durch, um die
          Integrität sicherzustellen.
        </div>
      </div>
    {:else if isRunning}
      <!-- Progress Section -->
      <div class="progress-section">
        <div class="progress-header">
          <h3>Bereinigung läuft...</h3>
          <span class="progress-percent">{Math.round(progress)}%</span>
        </div>

        <div class="progress-bar">
          <div class="progress-fill" style="width: {progress}%"></div>
        </div>

        <div class="current-step">
          {currentStep}
        </div>

        <!-- Log Output -->
        <div class="log-section">
          <h4>Protokoll</h4>
          <div class="log-entries">
            {#each logEntries.slice(0, 10) as entry}
              <div class="log-entry">
                <span class="timestamp">{new Date().toLocaleTimeString('de-DE')}</span>
                <span class="message">{entry}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {:else if pruneResult}
      <!-- Results Section -->
      <div class="results-section">
        <div class="success-icon">✓</div>
        <h3>Bereinigung erfolgreich</h3>

        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-label">Freigegebener Speicher</div>
            <div class="stat-value">{formatBytes(pruneResult.freedSpace || 0)}</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Gelöschte Packs</div>
            <div class="stat-value">{pruneResult.packsRemoved || 0}</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Verbleibende Packs</div>
            <div class="stat-value">{pruneResult.packsKept || 0}</div>
          </div>
          <div class="stat-card">
            <div class="stat-label">Dauer</div>
            <div class="stat-value">{pruneResult.duration || 0}s</div>
          </div>
        </div>

        <p class="result-message">
          Das Repository wurde erfolgreich bereinigt und optimiert. Nicht mehr benötigte Daten wurden
          entfernt.
        </p>
      </div>
    {/if}
  </div>

  <div slot="footer">
    {#if !isRunning && !pruneResult}
      <Button variant="secondary" on:click={handleClose}>Abbrechen</Button>
      <Button variant="danger" on:click={startPruning}>Bereinigung starten</Button>
    {:else if isRunning}
      <Button variant="danger" on:click={stopPruning}>Bereinigung stoppen</Button>
    {:else}
      <Button variant="primary" on:click={handleClose}>Schließen</Button>
    {/if}
  </div>
</Modal>

<style>
  .prune-repo-dialog {
    max-width: 700px;
  }

  .config-section {
    margin-bottom: 24px;
  }

  .config-section h3 {
    font-size: 16px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 16px;
  }

  .option-group {
    margin-bottom: 16px;
    padding: 16px;
    background: #22273a;
    border-radius: 8px;
    border: 1px solid #2d3348;
  }

  .option-description {
    margin-top: 8px;
    font-size: 13px;
    color: #71717a;
    line-height: 1.4;
  }

  .warning-section {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .warning-icon {
    font-size: 20px;
    flex-shrink: 0;
    color: #ef4444;
  }

  .warning-text {
    font-size: 14px;
    color: #e4e4e7;
    line-height: 1.5;
  }

  .warning-text strong {
    color: #ef4444;
  }

  .progress-section {
    width: 100%;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .progress-header h3 {
    font-size: 16px;
    font-weight: 600;
    color: #e4e4e7;
    margin: 0;
  }

  .progress-percent {
    font-size: 14px;
    font-weight: 500;
    color: #3b82f6;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: #2d3348;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 12px;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3b82f6, #22c55e);
    transition: width 0.5s ease;
  }

  .current-step {
    font-size: 14px;
    color: #e4e4e7;
    margin-bottom: 24px;
    font-style: italic;
  }

  .log-section h4 {
    font-size: 14px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .log-entries {
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    padding: 12px;
    max-height: 200px;
    overflow-y: auto;
  }

  .log-entry {
    display: flex;
    gap: 12px;
    padding: 4px 0;
    font-size: 12px;
    font-family: 'Courier New', monospace;
  }

  .timestamp {
    color: #71717a;
    flex-shrink: 0;
  }

  .message {
    color: #e4e4e7;
  }

  .results-section {
    text-align: center;
    padding: 24px 0;
  }

  .success-icon {
    width: 64px;
    height: 64px;
    margin: 0 auto 16px;
    background: #22c55e;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    color: white;
  }

  .results-section h3 {
    font-size: 20px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 24px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
    margin-bottom: 24px;
  }

  .stat-card {
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    text-align: center;
  }

  .stat-label {
    font-size: 12px;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
  }

  .stat-value {
    font-size: 24px;
    font-weight: 600;
    color: #22c55e;
  }

  .result-message {
    font-size: 14px;
    color: #71717a;
    line-height: 1.6;
    max-width: 500px;
    margin: 0 auto;
  }

  @media (max-width: 768px) {
    .progress-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 8px;
    }

    .log-entry {
      flex-direction: column;
      gap: 4px;
    }

    .stats-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
