<!-- CreateJobDialog.svelte: 4-Tab Wizard für Backup-Job-Erstellung (gemäß rustic_backup_dialogs.html) -->
<!--
TODO.md: Phase 2 - Dialog-Workflow: Backup & Restore ✅ IMPLEMENTIERT
Referenz: TODO.md Zeile 237-244, 332-340

Status:
- createJob an api.createBackupJob angebunden ✅ (TODO.md Zeile 238)
- Cron-Schedule-Konvertierung implementiert ✅ (TODO.md Zeile 88, daily/weekly/monthly)
- Multi-Step-Wizard mit Tabs ✅

Backend-Command: create_backup_job
- src-tauri/src/commands/backup.rs:21

API-Wrapper: src/lib/api/backup-jobs.ts:createBackupJob

Verwendung:
- src/lib/components/pages/BackupJobs.svelte
- src/lib/components/pages/RepositoryCard.svelte
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Modal from '../shared/Modal.svelte';

  export let open = false;
  export let repositories: any[] = [];

  const dispatch = createEventDispatcher();

  // Form State
  let currentTab = 0;
  const tabs = ['General', 'Paths & Exclusions', 'Schedule', 'Retention'];

  let jobName = '';
  let selectedRepository = '';
  let jobTags = '';
  let sourcePaths: string[] = [''];
  let excludePatterns: string[] = [''];
  let oneFileSystem = true;
  let scheduleType = 'daily';
  let scheduleTime = '02:00';
  let scheduleTimezone = 'Local';
  let keepDaily = 7;
  let keepWeekly = 4;
  let keepMonthly = 6;
  let keepYearly = 2;
  let autoPrune = true;

  // Validation
  let errors: Record<string, string> = {};

  function validateCurrentTab(): boolean {
    errors = {};

    switch (currentTab) {
      case 0: // General
        if (!jobName.trim()) errors.jobName = 'Job-Name ist erforderlich';
        if (!selectedRepository) errors.repository = 'Repository ist erforderlich';
        break;
      case 1: // Paths
        if (sourcePaths.filter((p) => p.trim()).length === 0) {
          errors.sourcePaths = 'Mindestens ein Source-Pfad ist erforderlich';
        }
        break;
      case 2: // Schedule
        // Basic validation for time
        if (!scheduleTime) errors.scheduleTime = 'Zeit ist erforderlich';
        break;
      case 3: // Retention
        if (keepDaily < 0 || keepWeekly < 0 || keepMonthly < 0 || keepYearly < 0) {
          errors.retention = 'Retention-Werte müssen >= 0 sein';
        }
        break;
    }

    return Object.keys(errors).length === 0;
  }

  function nextTab() {
    if (validateCurrentTab()) {
      currentTab = Math.min(currentTab + 1, tabs.length - 1);
    }
  }

  function prevTab() {
    currentTab = Math.max(currentTab - 1, 0);
  }

  function addPath() {
    sourcePaths = [...sourcePaths, ''];
  }

  function removePath(index: number) {
    if (sourcePaths.length > 1) {
      sourcePaths = sourcePaths.filter((_, i) => i !== index);
    }
  }

  function addExcludePattern() {
    excludePatterns = [...excludePatterns, ''];
  }

  function removeExcludePattern(index: number) {
    if (excludePatterns.length > 1) {
      excludePatterns = excludePatterns.filter((_, i) => i !== index);
    }
  }

  function selectScheduleType(type: string) {
    scheduleType = type;
  }

  function resetForm() {
    jobName = '';
    selectedRepository = '';
    jobTags = '';
    sourcePaths = [''];
    excludePatterns = [''];
    oneFileSystem = true;
    scheduleType = 'daily';
    scheduleTime = '02:00';
    scheduleTimezone = 'Local';
    keepDaily = 7;
    keepWeekly = 4;
    keepMonthly = 6;
    keepYearly = 2;
    autoPrune = true;
    currentTab = 0;
    errors = {};
  }

  function close() {
    open = false;
    resetForm();
  }

  async function createJob() {
    if (!validateCurrentTab()) return;

    const jobData = {
      name: jobName,
      repositoryId: selectedRepository,
      tags: jobTags
        .split(',')
        .map((t) => t.trim())
        .filter((t) => t),
      sourcePaths: sourcePaths.filter((p) => p.trim()),
      excludePatterns: excludePatterns.filter((p) => p.trim()),
      oneFileSystem,
      schedule: {
        type: scheduleType,
        time: scheduleTime,
        timezone: scheduleTimezone,
      },
      retention: {
        keepDaily,
        keepWeekly,
        keepMonthly,
        keepYearly,
        autoPrune,
      },
    };

    try {
      const { createBackupJob } = await import('$lib/api/backup-jobs');

      // Konvertiere Schedule in Cron-Expression (vereinfacht)
      let cronSchedule: string | undefined;
      if (scheduleType !== 'manual') {
        const [hours, minutes] = scheduleTime.split(':');
        switch (scheduleType) {
          case 'daily':
            cronSchedule = `${minutes} ${hours} * * *`;
            break;
          case 'weekly':
            cronSchedule = `${minutes} ${hours} * * 0`;
            break;
          case 'monthly':
            cronSchedule = `${minutes} ${hours} 1 * *`;
            break;
        }
      }

      const jobId = await createBackupJob({
        name: jobName,
        repository_id: selectedRepository,
        source_paths: sourcePaths.filter((p) => p.trim()),
        exclude_patterns: excludePatterns.filter((p) => p.trim()),
        tags: jobData.tags,
        schedule: cronSchedule,
        retention: {
          keep_daily: keepDaily,
          keep_weekly: keepWeekly,
          keep_monthly: keepMonthly,
          keep_yearly: keepYearly,
        },
      });

      console.log('Job created with ID:', jobId);
      dispatch('created', jobData);
      close();
    } catch (error) {
      console.error('Failed to create job:', error);
      errors.general = `Fehler beim Erstellen des Jobs: ${error}`;
    }
  }

  $: schedulePreview = (() => {
    switch (scheduleType) {
      case 'daily':
        return `Täglich um ${scheduleTime}`;
      case 'weekly':
        return `Wöchentlich (Sonntag) um ${scheduleTime}`;
      case 'monthly':
        return `Monatlich (1.) um ${scheduleTime}`;
      case 'custom':
        return `Benutzerdefiniert`;
      case 'manual':
        return `Manuell`;
      default:
        return '';
    }
  })();

  $: retentionSummary = `Täglich: ${keepDaily}, Wöchentlich: ${keepWeekly}, Monatlich: ${keepMonthly}, Jährlich: ${keepYearly}`;
</script>

<Modal bind:open ariaLabel="Backup-Job erstellen">
  <div slot="header">
    <h2>Backup-Job erstellen</h2>
  </div>

  <!-- Tabs -->
  <div class="tabs">
    {#each tabs as tab, index}
      <button
        class="tab {currentTab === index ? 'active' : ''}"
        onclick={() => {
          if (validateCurrentTab() || index < currentTab) currentTab = index;
        }}
      >
        {tab}
      </button>
    {/each}
  </div>

  <div class="tab-content">
    <!-- Tab 0: General -->
    {#if currentTab === 0}
      <div class="form-group">
        <label class="form-label" for="job-name">Job-Name</label>
        <input
          id="job-name"
          class="form-input"
          type="text"
          bind:value={jobName}
          placeholder="z.B. Dokumente täglich sichern"
        />
        {#if errors.jobName}<div class="error">{errors.jobName}</div>{/if}
      </div>

      <div class="form-group">
        <label class="form-label" for="repository">Ziel-Repository</label>
        <select id="repository" class="form-select" bind:value={selectedRepository}>
          <option value="">Repository auswählen...</option>
          {#each repositories as repo}
            <option value={repo.id}>{repo.name} - {repo.path}</option>
          {/each}
        </select>
        {#if errors.repository}<div class="error">{errors.repository}</div>{/if}
      </div>

      <div class="form-group">
        <label class="form-label" for="tags">Tags</label>
        <input
          id="tags"
          class="form-input"
          type="text"
          bind:value={jobTags}
          placeholder="daily, documents, important"
        />
        <div class="form-help">Komma-getrennte Tags zur Organisation der Snapshots</div>
      </div>
    {/if}

    <!-- Tab 1: Paths & Exclusions -->
    {#if currentTab === 1}
      <div class="form-group">
        <div class="form-label">Quell-Pfade</div>
        {#each sourcePaths as path, index}
          <div class="input-with-button">
            <input
              class="form-input"
              type="text"
              bind:value={path}
              placeholder="/home/user/documents"
            />
            {#if sourcePaths.length > 1}
              <button class="btn-browse" onclick={() => removePath(index)}>✕</button>
            {/if}
          </div>
        {/each}
        <div style="margin-top: 8px;">
          <button class="btn-browse" onclick={addPath}>📁 Pfad hinzufügen</button>
        </div>
        <div class="form-help">Einen Pfad pro Zeile. Absolute Pfade verwenden.</div>
        {#if errors.sourcePaths}<div class="error">{errors.sourcePaths}</div>{/if}
      </div>

      <div class="form-group">
        <div class="form-label">Ausschluss-Muster</div>
        {#each excludePatterns as pattern, index}
          <div class="input-with-button">
            <input class="form-input" type="text" bind:value={pattern} placeholder="*.tmp" />
            {#if excludePatterns.length > 1}
              <button class="btn-browse" onclick={() => removeExcludePattern(index)}>✕</button>
            {/if}
          </div>
        {/each}
        <div style="margin-top: 8px;">
          <button class="btn-browse" onclick={addExcludePattern}>➕ Muster hinzufügen</button>
        </div>
        <div class="form-help">Glob-Muster zum Ausschließen von Dateien und Verzeichnissen</div>
      </div>

      <div class="checkbox-group">
        <input type="checkbox" id="one-file-system" bind:checked={oneFileSystem} />
        <label for="one-file-system"
          >Nur innerhalb desselben Dateisystems bleiben (Mount-Punkte nicht überschreiten)</label
        >
      </div>
    {/if}

    <!-- Tab 2: Schedule -->
    {#if currentTab === 2}
      <div class="form-group">
        <div class="form-label">Zeitplan-Typ</div>
        <div class="schedule-builder">
          <div class="schedule-quick">
            <div
              class="schedule-preset {scheduleType === 'daily' ? 'selected' : ''}"
              onclick={() => selectScheduleType('daily')}
              onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && selectScheduleType('daily')}
              role="button"
              tabindex="0"
            >
              <div style="margin-bottom: 4px;">📅</div>
              <div>Täglich</div>
            </div>
            <div
              class="schedule-preset {scheduleType === 'weekly' ? 'selected' : ''}"
              onclick={() => selectScheduleType('weekly')}
              onkeydown={(e) =>
                (e.key === 'Enter' || e.key === ' ') && selectScheduleType('weekly')}
              role="button"
              tabindex="0"
            >
              <div style="margin-bottom: 4px;">📆</div>
              <div>Wöchentlich</div>
            </div>
            <div
              class="schedule-preset {scheduleType === 'monthly' ? 'selected' : ''}"
              onclick={() => selectScheduleType('monthly')}
              onkeydown={(e) =>
                (e.key === 'Enter' || e.key === ' ') && selectScheduleType('monthly')}
              role="button"
              tabindex="0"
            >
              <div style="margin-bottom: 4px;">🗓️</div>
              <div>Monatlich</div>
            </div>
            <div
              class="schedule-preset {scheduleType === 'custom' ? 'selected' : ''}"
              onclick={() => selectScheduleType('custom')}
              onkeydown={(e) =>
                (e.key === 'Enter' || e.key === ' ') && selectScheduleType('custom')}
              role="button"
              tabindex="0"
            >
              <div style="margin-bottom: 4px;">⚙️</div>
              <div>Benutzerdefiniert</div>
            </div>
            <div
              class="schedule-preset {scheduleType === 'manual' ? 'selected' : ''}"
              onclick={() => selectScheduleType('manual')}
              onkeydown={(e) =>
                (e.key === 'Enter' || e.key === ' ') && selectScheduleType('manual')}
              role="button"
              tabindex="0"
            >
              <div style="margin-bottom: 4px;">🖐️</div>
              <div>Manuell</div>
            </div>
          </div>
        </div>
      </div>

      {#if scheduleType !== 'manual'}
        <div class="form-row">
          <div class="form-group">
            <label class="form-label" for="schedule-time">Zeit</label>
            <input id="schedule-time" class="form-input" type="time" bind:value={scheduleTime} />
            {#if errors.scheduleTime}<div class="error">{errors.scheduleTime}</div>{/if}
          </div>
          <div class="form-group">
            <label class="form-label" for="timezone">Zeitzone</label>
            <select id="timezone" class="form-select" bind:value={scheduleTimezone}>
              <option value="Local">Lokale Zeit</option>
              <option value="UTC">UTC</option>
            </select>
          </div>
        </div>
      {/if}

      <div class="info-box">
        ⏰ Dieser Job wird {schedulePreview} ausgeführt
      </div>
    {/if}

    <!-- Tab 3: Retention -->
    {#if currentTab === 3}
      <div class="form-group">
        <div class="form-label">Aufbewahrungs-Richtlinie</div>
        <select class="form-select">
          <option>Simple - Letzte N Snapshots behalten</option>
          <option selected>Erweitert - Tägliche/Wöchentliche/Monatliche behalten</option>
          <option>Benutzerdefinierte forget-Richtlinie</option>
        </select>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label class="form-label" for="keep-daily">Tägliche behalten</label>
          <input id="keep-daily" class="form-input" type="number" bind:value={keepDaily} min="0" />
        </div>
        <div class="form-group">
          <label class="form-label" for="keep-weekly">Wöchentliche behalten</label>
          <input
            id="keep-weekly"
            class="form-input"
            type="number"
            bind:value={keepWeekly}
            min="0"
          />
        </div>
      </div>

      <div class="form-row">
        <div class="form-group">
          <label class="form-label" for="keep-monthly">Monatliche behalten</label>
          <input
            id="keep-monthly"
            class="form-input"
            type="number"
            bind:value={keepMonthly}
            min="0"
          />
        </div>
        <div class="form-group">
          <label class="form-label" for="keep-yearly">Jährliche behalten</label>
          <input
            id="keep-yearly"
            class="form-input"
            type="number"
            bind:value={keepYearly}
            min="0"
          />
        </div>
      </div>

      <div class="info-box">
        📊 Richtlinie: {retentionSummary}
      </div>

      <div class="checkbox-group">
        <input type="checkbox" id="auto-prune" bind:checked={autoPrune} />
        <label for="auto-prune">Automatisch alte Snapshots nach Backup entfernen</label>
      </div>

      {#if errors.retention}<div class="error">{errors.retention}</div>{/if}
    {/if}
  </div>

  <div slot="footer">
    {#if currentTab > 0}
      <Button variant="secondary" on:click={prevTab}>Zurück</Button>
    {/if}
    {#if currentTab < tabs.length - 1}
      <Button variant="primary" on:click={nextTab}>Weiter</Button>
    {:else}
      <Button variant="primary" on:click={createJob}>Job erstellen</Button>
    {/if}
  </div>
</Modal>

<style>
  /* CSS aus rustic_backup_dialogs.html übernehmen */
  .tabs {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid #2d3348;
    margin-bottom: 24px;
  }

  .tab {
    padding: 10px 16px;
    cursor: pointer;
    color: #71717a;
    font-size: 14px;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
  }

  .tab:hover {
    color: #e4e4e7;
  }

  .tab.active {
    color: #e4e4e7;
    border-bottom-color: #3b82f6;
  }

  .tab-content {
    min-height: 300px;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: #e4e4e7;
    margin-bottom: 8px;
  }

  .form-input,
  .form-select {
    width: 100%;
    padding: 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    font-family: inherit;
  }

  .form-input:focus,
  .form-select:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .checkbox-group {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
  }

  .checkbox-group input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .checkbox-group label {
    font-size: 14px;
    color: #e4e4e7;
    cursor: pointer;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
  }

  .input-with-button .form-input {
    flex: 1;
  }

  .btn-browse {
    padding: 10px 16px;
    background: #2d3348;
    border: 1px solid #3e4457;
    color: #e4e4e7;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    white-space: nowrap;
  }

  .btn-browse:hover {
    background: #3e4457;
  }

  .form-help {
    font-size: 12px;
    color: #71717a;
    margin-top: 4px;
  }

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    padding: 12px 16px;
    font-size: 13px;
    color: #93c5fd;
    margin-bottom: 20px;
  }

  .schedule-builder {
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
  }

  .schedule-quick {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 8px;
    margin-bottom: 16px;
  }

  .schedule-preset {
    padding: 10px;
    background: #22273a;
    border: 2px solid #2d3348;
    border-radius: 6px;
    cursor: pointer;
    text-align: center;
    font-size: 13px;
    transition: all 0.2s;
  }

  .schedule-preset:hover {
    border-color: #3b82f6;
  }

  .schedule-preset.selected {
    background: rgba(59, 130, 246, 0.15);
    border-color: #3b82f6;
  }

  .error {
    color: #ef4444;
    font-size: 12px;
    margin-top: 4px;
  }

  @media (max-width: 768px) {
    .form-row {
      grid-template-columns: 1fr;
    }

    .schedule-quick {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
