<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Modal from '../shared/Modal.svelte';

  const dispatch = createEventDispatcher();

  export let repositoryId: string = '';

  let isAnalyzing = false;
  let isPruning = false;
  let analysisComplete = false;
  let stats = {
    totalSize: 0,
    unusedSize: 0,
    unusedPacks: 0,
    snapshotsToDelete: 0,
  };

  let options = {
    keepWithin: '',
    keepHourly: '',
    keepDaily: '',
    keepWeekly: '',
    keepMonthly: '',
    keepYearly: '',
    maxUnused: '',
  };

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  async function analyzeRepository() {
    isAnalyzing = true;
    try {
      // Simulate analysis
      await new Promise((resolve) => setTimeout(resolve, 2000));

      // Mock stats - in real implementation, this would come from backend
      stats = {
        totalSize: 1024 * 1024 * 1024 * 5.2, // 5.2 GB
        unusedSize: 1024 * 1024 * 1024 * 0.8, // 800 MB
        unusedPacks: 12,
        snapshotsToDelete: 3,
      };

      analysisComplete = true;
    } finally {
      isAnalyzing = false;
    }
  }

  async function startPruning() {
    isPruning = true;
    try {
      dispatch('prune-repo', {
        repositoryId,
        options,
      });
    } finally {
      isPruning = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }

  function resetDialog() {
    isAnalyzing = false;
    isPruning = false;
    analysisComplete = false;
    stats = {
      totalSize: 0,
      unusedSize: 0,
      unusedPacks: 0,
      snapshotsToDelete: 0,
    };
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository bereinigen</div>
  <div class="prune-repo-dialog">
    {#if !analysisComplete}
      <!-- Analysis Section -->
      <div class="analysis-section">
        <h3>Repository-Analyse</h3>
        <p class="description">
          Bevor das Repository bereinigt werden kann, muss es analysiert werden, um zu bestimmen,
          welche Daten entfernt werden können.
        </p>

        <div class="analysis-info">
          <div class="info-icon">📊</div>
          <div class="info-text">
            <strong>Was wird analysiert?</strong><br />
            • Nicht mehr referenzierte Datenblöcke<br />
            • Alte Snapshots gemäß Aufbewahrungsrichtlinien<br />
            • Optimierungsmöglichkeiten für Speicherplatz
          </div>
        </div>

        <Button variant="primary" on:click={analyzeRepository} disabled={isAnalyzing}>
          {#if isAnalyzing}
            Analysiere Repository...
          {:else}
            Analyse starten
          {/if}
        </Button>
      </div>
    {:else}
      <!-- Results Section -->
      <div class="results-section">
        <h3>Analyse-Ergebnisse</h3>

        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-label">Gesamtgröße</div>
            <div class="stat-value">{formatBytes(stats.totalSize)}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Nicht verwendet</div>
            <div class="stat-value warning">{formatBytes(stats.unusedSize)}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Unbenutzte Packs</div>
            <div class="stat-value">{stats.unusedPacks}</div>
          </div>

          <div class="stat-card">
            <div class="stat-label">Zu löschende Snapshots</div>
            <div class="stat-value">{stats.snapshotsToDelete}</div>
          </div>
        </div>

        {#if stats.unusedSize > 0}
          <div class="savings-info">
            <div class="savings-icon">💾</div>
            <div class="savings-text">
              Durch die Bereinigung können <strong>{formatBytes(stats.unusedSize)}</strong>
              Speicherplatz freigegeben werden.
            </div>
          </div>
        {/if}

        <!-- Retention Options -->
        <div class="retention-section">
          <h4>Aufbewahrungsrichtlinien (optional)</h4>
          <p class="retention-description">
            Legen Sie fest, wie viele Snapshots verschiedener Zeiträume behalten werden sollen. Leer
            lassen für Standardwerte.
          </p>

          <div class="retention-grid">
            <div class="retention-field">
              <label for="keep-within">Innerhalb von (Tagen)</label>
              <input
                id="keep-within"
                type="number"
                placeholder="7"
                bind:value={options.keepWithin}
                min="1"
              />
            </div>

            <div class="retention-field">
              <label for="keep-hourly">Stündlich</label>
              <input
                id="keep-hourly"
                type="number"
                placeholder="24"
                bind:value={options.keepHourly}
                min="1"
              />
            </div>

            <div class="retention-field">
              <label for="keep-daily">Täglich</label>
              <input
                id="keep-daily"
                type="number"
                placeholder="7"
                bind:value={options.keepDaily}
                min="1"
              />
            </div>

            <div class="retention-field">
              <label for="keep-weekly">Wöchentlich</label>
              <input
                id="keep-weekly"
                type="number"
                placeholder="4"
                bind:value={options.keepWeekly}
                min="1"
              />
            </div>

            <div class="retention-field">
              <label for="keep-monthly">Monatlich</label>
              <input
                id="keep-monthly"
                type="number"
                placeholder="6"
                bind:value={options.keepMonthly}
                min="1"
              />
            </div>

            <div class="retention-field">
              <label for="keep-yearly">Jährlich</label>
              <input
                id="keep-yearly"
                type="number"
                placeholder="3"
                bind:value={options.keepYearly}
                min="1"
              />
            </div>
          </div>
        </div>

        <div class="warning-section">
          <div class="warning-icon">⚠️</div>
          <div class="warning-text">
            <strong>Wichtig:</strong> Die Bereinigung entfernt unwiderruflich Daten. Stellen Sie sicher,
            dass Sie alle wichtigen Snapshots behalten möchten.
          </div>
        </div>
      </div>
    {/if}
  </div>

  <div slot="footer">
    <Button variant="secondary" on:click={handleClose}>Abbrechen</Button>

    {#if analysisComplete}
      <Button variant="primary" on:click={startPruning} disabled={isPruning}>
        {#if isPruning}
          Bereinige Repository...
        {:else}
          Bereinigung starten
        {/if}
      </Button>
    {/if}
  </div>
</Modal>

<style>
  .prune-repo-dialog {
    max-width: 800px;
  }

  .analysis-section,
  .results-section {
    margin-bottom: 24px;
  }

  .analysis-section h3,
  .results-section h3 {
    font-size: 18px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 12px;
  }

  .description {
    color: #71717a;
    line-height: 1.5;
    margin-bottom: 20px;
  }

  .analysis-info,
  .savings-info,
  .warning-section {
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
    margin-bottom: 20px;
  }

  .info-icon,
  .savings-icon,
  .warning-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .info-text,
  .savings-text,
  .warning-text {
    font-size: 14px;
    color: #e4e4e7;
    line-height: 1.5;
  }

  .savings-text strong,
  .warning-text strong {
    color: #22c55e;
  }

  .warning-text strong {
    color: #fbbf24;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
  }

  .stat-card {
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    text-align: center;
  }

  .stat-label {
    font-size: 12px;
    font-weight: 500;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 8px;
  }

  .stat-value {
    font-size: 18px;
    font-weight: 600;
    color: #e4e4e7;
  }

  .stat-value.warning {
    color: #fbbf24;
  }

  .retention-section {
    margin-bottom: 24px;
  }

  .retention-section h4 {
    font-size: 16px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 8px;
  }

  .retention-description {
    color: #71717a;
    font-size: 14px;
    line-height: 1.5;
    margin-bottom: 16px;
  }

  .retention-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 12px;
  }

  .retention-field {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .retention-field label {
    font-size: 12px;
    font-weight: 500;
    color: #71717a;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .retention-field input {
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    padding: 8px 12px;
    color: #e4e4e7;
    font-size: 14px;
  }

  .retention-field input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .retention-field input::placeholder {
    color: #71717a;
  }

  @media (max-width: 768px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }

    .retention-grid {
      grid-template-columns: 1fr;
    }

    .analysis-info,
    .savings-info,
    .warning-section {
      flex-direction: column;
      text-align: center;
    }
  }
</style>
