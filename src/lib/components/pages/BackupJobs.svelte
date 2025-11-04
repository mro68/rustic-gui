<!-- BackupJobs.svelte: Backup-Job-Management Seite (gemäß Mockup) -->
<!--
  @component
  Backup-Job-Verwaltungs-Seite mit CRUD-Operationen.
  
  TODO.md: Phase 2 - Daten-Initialisierung (Stores & Pages) ✅ KOMPLETT
  Referenz: TODO.md Zeile 219, 342
  
  Status:
  - loadJobs in onMount implementiert ✅ (TODO.md Zeile 219)
  - Lädt Backup-Jobs via api.listBackupJobs ✅
  - Füllt $jobs Store ✅
  
  Backend-Commands:
  - list_backup_jobs: src-tauri/src/commands/backup.rs:289
  - create_backup_job: src-tauri/src/commands/backup.rs:21
  - update_backup_job: src-tauri/src/commands/backup.rs:122
  - delete_backup_job: src-tauri/src/commands/backup.rs:217
  - get_backup_job: src-tauri/src/commands/backup.rs:255
  
  API-Wrapper: src/lib/api/backup-jobs.ts
  Store: src/lib/stores/backup-jobs.ts
  
  Dialogs verwendet:
  - JobDialog.svelte (unified create/edit) ✅
  - DeleteJobDialog.svelte ✅ (TODO.md Zeile 240)
  
  TODO:
  - Zeile 101, 116, 121: Job-Status und Zeitrechnung (last_run, next_run)
-->
<script lang="ts">
  import DeleteJobDialog from '$lib/components/dialogs/DeleteJobDialog.svelte';
  import JobDialog from '$lib/components/dialogs/JobDialog.svelte';
  import Button from '$lib/components/shared/Button.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { jobs, loading } from '$lib/stores/backup-jobs';
  import { repositories } from '$lib/stores/repositories';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupJobDto } from '$lib/types';
  import { onMount } from 'svelte';

  let showJobDialog = $state(false);
  let jobDialogMode = $state<'create' | 'edit'>('create');
  let showDeleteDialog = $state(false);
  let selectedJob = $state<BackupJobDto | null>(null);
  let scheduledJobIds = $state<string[]>([]);

  async function loadJobs() {
    try {
      const { listBackupJobs, listScheduledBackups } = await import('$lib/api/backup-jobs');
      const jobList = await listBackupJobs();
      jobs.set(jobList);

      // Lade geplante Jobs
      try {
        scheduledJobIds = await listScheduledBackups();
        console.log('Scheduled jobs:', scheduledJobIds);
      } catch (err) {
        console.warn('Failed to load scheduled jobs:', err);
      }

      console.log('Backup jobs loaded:', jobList.length);
    } catch (error) {
      console.error('Failed to load jobs:', error);
      toastStore.error('Fehler beim Laden der Backup-Jobs');
    }
  }

  function isScheduled(jobId: string): boolean {
    return scheduledJobIds.includes(jobId);
  }

  async function toggleSchedule(job: BackupJobDto) {
    try {
      const { scheduleBackup, unscheduleBackup } = await import('$lib/api/backup-jobs');

      if (isScheduled(job.id)) {
        await unscheduleBackup(job.id);
        toastStore.success(`Job "${job.name}" entplant`);
      } else {
        if (!job.schedule) {
          toastStore.error('Job hat keine Cron-Expression');
          return;
        }
        await scheduleBackup(job.id, job.schedule);
        toastStore.success(`Job "${job.name}" geplant`);
      }

      // Reload scheduled jobs
      await loadJobs();
    } catch (error) {
      console.error('Failed to toggle schedule:', error);
      toastStore.error(`Fehler beim Planen/Entplanen: ${error}`);
    }
  }

  function handleJobCreated(event: any) {
    console.log('Job created:', event.detail);
    toastStore.success('Backup-Job wurde erfolgreich erstellt');
    loadJobs(); // Reload jobs
  }

  function handleJobSaved(event: any) {
    console.log('Job saved:', event.detail);
    loadJobs(); // Reload jobs
  }

  function handleJobDeleted(event: any) {
    console.log('Job deleted:', event.detail);
    toastStore.success('Backup-Job wurde gelöscht');
    loadJobs(); // Reload jobs
  }

  function handleEditJob(job: BackupJobDto) {
    selectedJob = job;
    jobDialogMode = 'edit';
    showJobDialog = true;
  }

  function handleDeleteJob(job: BackupJobDto) {
    selectedJob = job;
    showDeleteDialog = true;
  }

  let runningJobId = $state<string | null>(null);
  let backupProgress = $state<{
    filesProcessed: number;
    bytesUploaded: number;
    percent: number;
  } | null>(null);

  async function handleRunJob(job: BackupJobDto) {
    console.log('Running job:', job.id);

    // Repository-Passwort prüfen
    const repo = $repositories.find((r) => r.id === job.repository_id);
    if (!repo) {
      toastStore.error('Repository nicht gefunden');
      return;
    }

    // Passwort von Keychain holen (falls nicht im Job gespeichert)
    let password = job.password; // Falls im Job gespeichert

    if (!password) {
      try {
        const { getRepositoryPassword } = await import('$lib/api/keychain');
        password = await getRepositoryPassword(job.repository_id);
      } catch (error) {
        toastStore.error('Repository-Passwort nicht verfügbar. Bitte Repository entsperren.');
        console.error('Keychain error:', error);
        return;
      }
    }

    runningJobId = job.id;
    backupProgress = { filesProcessed: 0, bytesUploaded: 0, percent: 0 };
    toastStore.info(`Starte Backup-Job "${job.name}"...`);

    // Event-Listener-Variablen (für Cleanup)
    let unlistenProgress: (() => void) | null = null;
    let unlistenCompleted: (() => void) | null = null;
    let unlistenFailed: (() => void) | null = null;

    try {
      // Progress-Event-Listener
      const { listen } = await import('@tauri-apps/api/event');

      unlistenProgress = await listen('backup-progress', (event: any) => {
        const payload = event.payload;
        if (payload.jobId === job.id && payload.progress) {
          backupProgress = {
            filesProcessed: payload.progress.files_processed || 0,
            bytesUploaded: payload.progress.bytes_uploaded || 0,
            percent: payload.progress.percent || 0,
          };
        }
      });

      // Completion-Listener
      unlistenCompleted = await listen('backup-completed', (event: any) => {
        const payload = event.payload;
        if (payload.jobId === job.id) {
          toastStore.success(`Backup-Job "${job.name}" erfolgreich abgeschlossen`);
          runningJobId = null;
          backupProgress = null;
          loadJobs(); // Reload to update last_run
        }
      });

      // Error-Listener
      unlistenFailed = await listen('backup-failed', (event: any) => {
        const payload = event.payload;
        if (payload.jobId === job.id) {
          toastStore.error(`Backup-Job fehlgeschlagen: ${payload.message || 'Unbekannter Fehler'}`);
          runningJobId = null;
          backupProgress = null;
        }
      });

      // Backup starten
      const { runBackup } = await import('$lib/api/backup');
      await runBackup(job.id, password);
    } catch (error) {
      console.error('Failed to start backup:', error);
      toastStore.error(`Fehler beim Starten des Backups: ${error}`);
      runningJobId = null;
      backupProgress = null;
    } finally {
      // Cleanup: Event-Listener entfernen
      if (unlistenProgress) unlistenProgress();
      if (unlistenCompleted) unlistenCompleted();
      if (unlistenFailed) unlistenFailed();
    }
  }

  onMount(() => {
    loadJobs();
  });
</script>

<div class="backup-jobs-page">
  <div class="page-wrapper">
    <!-- Page Header -->
    <div class="page-header">
      <h1 class="page-title">Backup Jobs</h1>
      <div class="header-actions">
        <Tooltip text="Neuen Backup-Job erstellen">
          <Button
            variant="primary"
            size="sm"
            onclick={() => {
              jobDialogMode = 'create';
              selectedJob = null;
              showJobDialog = true;
            }}
          >
            <span>➕</span>
            <span>Create Job</span>
          </Button>
        </Tooltip>
      </div>
    </div>

    <!-- Toolbar -->
    <div class="toolbar">
      <h1 class="page-title">Backup-Jobs</h1>
      <div class="toolbar-actions">
        <Tooltip text="Neuen Backup-Job erstellen">
          <Button
            variant="primary"
            size="sm"
            onclick={() => {
              jobDialogMode = 'create';
              selectedJob = null;
              showJobDialog = true;
            }}
          >
            + Neuer Job
          </Button>
        </Tooltip>
      </div>
    </div>

    <!-- Jobs List -->
    <div class="jobs-container">
      {#if $loading}
        <div class="loading">Lade Backup-Jobs...</div>
      {:else if $jobs.length === 0}
        <div class="empty-state">
          <h3>Keine Backup-Jobs gefunden</h3>
          <p>Erstellen Sie Ihren ersten Backup-Job, um automatische Sicherungen zu planen.</p>
          <Button
            variant="primary"
            onclick={() => {
              jobDialogMode = 'create';
              selectedJob = null;
              showJobDialog = true;
            }}
          >
            Ersten Job erstellen
          </Button>
        </div>
      {:else}
        <div class="jobs-grid">
          {#each $jobs as job (job.id)}
            <div class="job-card">
              <div class="job-header">
                <h3 class="job-name">{job.name}</h3>
                <div class="job-status">
                  {#if isScheduled(job.id)}
                    <span class="status-badge scheduled">Geplant</span>
                  {:else if job.schedule}
                    <span class="status-badge paused">Pausiert</span>
                  {:else}
                    <span class="status-badge manual">Manuell</span>
                  {/if}
                </div>
              </div>

              <div class="job-details">
                <div class="detail-item">
                  <span class="label">Repository:</span>
                  <span class="value">
                    {$repositories.find((r) => r.id === job.repository_id)?.name || 'Unbekannt'}
                  </span>
                </div>
                {#if job.schedule}
                  <div class="detail-item">
                    <span class="label">Cron-Schedule:</span>
                    <span class="value">{job.schedule}</span>
                  </div>
                {/if}
                <div class="detail-item">
                  <span class="label">Quell-Pfade:</span>
                  <span class="value">{job.source_paths.length} Pfad(e)</span>
                </div>
              </div>

              <div class="job-actions">
                {#if runningJobId === job.id && backupProgress}
                  <div class="progress-section">
                    <div class="progress-bar-container">
                      <div class="progress-bar" style="width: {backupProgress.percent}%"></div>
                    </div>
                    <div class="progress-stats">
                      <span>{Math.round(backupProgress.percent)}%</span>
                      <span>{backupProgress.filesProcessed} Dateien</span>
                      <span>{(backupProgress.bytesUploaded / 1024 / 1024).toFixed(1)} MB</span>
                    </div>
                  </div>
                {/if}

                {#if job.schedule}
                  <Tooltip text={isScheduled(job.id) ? 'Job entplanen' : 'Job planen'}>
                    <Button
                      variant={isScheduled(job.id) ? 'secondary' : 'primary'}
                      size="sm"
                      onclick={() => toggleSchedule(job)}
                    >
                      {isScheduled(job.id) ? '⏸ Pausieren' : '▶ Aktivieren'}
                    </Button>
                  </Tooltip>
                {/if}
                <Tooltip text="Backup jetzt ausführen">
                  <Button
                    variant="secondary"
                    size="sm"
                    onclick={() => handleRunJob(job)}
                    disabled={runningJobId === job.id}
                  >
                    {runningJobId === job.id ? '⏳ Läuft...' : 'Ausführen'}
                  </Button>
                </Tooltip>
                <Tooltip text="Job bearbeiten">
                  <Button variant="secondary" size="sm" onclick={() => handleEditJob(job)}>
                    Bearbeiten
                  </Button>
                </Tooltip>
                <Tooltip text="Job löschen">
                  <Button variant="danger" size="sm" onclick={() => handleDeleteJob(job)}>
                    Löschen
                  </Button>
                </Tooltip>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Job Dialog (Create/Edit) -->
    <JobDialog
      bind:open={showJobDialog}
      mode={jobDialogMode}
      job={selectedJob}
      repositories={$repositories}
      on:created={handleJobCreated}
      on:saved={handleJobSaved}
    />

    <!-- Delete Job Dialog -->
    <DeleteJobDialog bind:open={showDeleteDialog} job={selectedJob} on:deleted={handleJobDeleted} />
  </div>
</div>

<style>
  .backup-jobs-page {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding: 24px 0;
  }

  .page-title {
    font-size: 28px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .page-wrapper {
    width: 100%;
    min-width: 320px;
    max-width: 1400px;
    padding: 0 1rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 0;
  }

  .page-title {
    font-size: 1.875rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .toolbar-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .jobs-container {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .loading {
    text-align: center;
    padding: 3rem;
    color: var(--text-secondary);
  }

  .empty-state {
    text-align: center;
    padding: 3rem 1rem;
    color: var(--text-secondary);
  }

  .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    margin-bottom: 1.5rem;
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }

  .jobs-grid {
    display: grid;
    gap: 1rem;
  }

  .job-card {
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .job-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .job-name {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .job-status {
    display: flex;
    align-items: center;
  }

  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 0.75rem;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
  }

  .status-badge.scheduled {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .status-badge.paused {
    background: rgba(251, 191, 36, 0.15);
    color: #fbbf24;
  }

  /* Removed unused .status-badge.idle - status 'idle' not currently used */

  .job-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .detail-item .label {
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  .detail-item .value {
    font-size: 0.875rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .job-actions {
    display: flex;
    gap: 0.5rem;
    justify-content: flex-end;
    flex-wrap: wrap;
  }

  .progress-section {
    width: 100%;
    padding: 0.75rem 0;
  }

  .progress-bar-container {
    background: var(--bg-tertiary);
    border-radius: 9999px;
    height: 8px;
    overflow: hidden;
    margin-bottom: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .progress-bar {
    background: linear-gradient(90deg, #3b82f6, #2563eb);
    height: 100%;
    transition: width 0.3s ease;
  }

  .progress-stats {
    display: flex;
    gap: 1rem;
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  @media (max-width: 768px) {
    .toolbar {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }

    .toolbar-actions {
      justify-content: space-between;
    }

    .job-card {
      padding: 1rem;
    }

    .job-header {
      flex-direction: column;
      align-items: stretch;
      gap: 0.5rem;
    }

    .job-actions {
      flex-wrap: wrap;
    }

    .detail-item {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>
