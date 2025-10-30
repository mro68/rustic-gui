<!-- BackupJobs.svelte: Backup-Job-Management Seite (gemäß Mockup) -->
<script lang="ts">
  import CreateJobDialog from '$lib/components/dialogs/CreateJobDialog.svelte';
  import DeleteJobDialog from '$lib/components/dialogs/DeleteJobDialog.svelte';
  import EditJobDialog from '$lib/components/dialogs/EditJobDialog.svelte';
  import Button from '$lib/components/shared/Button.svelte';
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { jobs, loading } from '$lib/stores/backup-jobs';
  import { repositories } from '$lib/stores/repositories';
  import { toastStore } from '$lib/stores/toast';
  import type { BackupJob } from '$lib/types/backup.types';
  import { onMount } from 'svelte';

  let showCreateDialog = false;
  let showEditDialog = false;
  let showDeleteDialog = false;
  let selectedJob: BackupJob | null = null;

  // TODO: API-Call zum Laden der Jobs
  async function loadJobs() {
    try {
      // const jobList = await getBackupJobs();
      // jobs.set(jobList);
      console.log('Loading backup jobs...');
    } catch (error) {
      console.error('Failed to load jobs:', error);
      toastStore.error('Fehler beim Laden der Backup-Jobs');
    }
  }

  function handleJobCreated(event: any) {
    console.log('Job created:', event.detail);
    toastStore.success('Backup-Job wurde erfolgreich erstellt');
    loadJobs(); // Reload jobs
  }

  function handleJobUpdated(event: any) {
    console.log('Job updated:', event.detail);
    toastStore.success('Backup-Job wurde erfolgreich aktualisiert');
    loadJobs(); // Reload jobs
  }

  function handleJobDeleted(event: any) {
    console.log('Job deleted:', event.detail);
    toastStore.success('Backup-Job wurde gelöscht');
    loadJobs(); // Reload jobs
  }

  function handleEditJob(job: BackupJob) {
    selectedJob = job;
    showEditDialog = true;
  }

  function handleDeleteJob(job: BackupJob) {
    selectedJob = job;
    showDeleteDialog = true;
  }

  function handleRunJob(job: BackupJob) {
    // TODO: Implement run job functionality
    console.log('Running job:', job.id);
    toastStore.info(`Starte Backup-Job "${job.name}"...`);
  }

  onMount(() => {
    loadJobs();
  });
</script>

<div class="backup-jobs-page">
  <!-- Toolbar -->
  <div class="toolbar">
    <h1 class="page-title">Backup-Jobs</h1>
    <div class="toolbar-actions">
      <Tooltip text="Neuen Backup-Job erstellen">
        <Button variant="primary" size="sm" onclick={() => (showCreateDialog = true)}>
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
        <Button variant="primary" onclick={() => (showCreateDialog = true)}>
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
                <!-- TODO: Status basierend auf Schedule und letzter Ausführung -->
                <span class="status-badge idle">Idle</span>
              </div>
            </div>

            <div class="job-details">
              <div class="detail-item">
                <span class="label">Repository:</span>
                <span class="value">
                  {$repositories.find((r) => r.id === job.repositoryId)?.name || 'Unbekannt'}
                </span>
              </div>
              <div class="detail-item">
                <span class="label">Nächste Ausführung:</span>
                <span class="value">Heute 02:00</span>
                <!-- TODO: Berechnen -->
              </div>
              <div class="detail-item">
                <span class="label">Letzte Ausführung:</span>
                <span class="value">Gestern 02:00</span>
                <!-- TODO: Aus Job-Daten -->
              </div>
            </div>

            <div class="job-actions">
              <Tooltip text="Backup jetzt ausführen">
                <Button variant="secondary" size="sm" onclick={() => handleRunJob(job)}>
                  Ausführen
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
</div>

<!-- Create Job Dialog -->
<CreateJobDialog
  bind:open={showCreateDialog}
  repositories={$repositories}
  on:created={handleJobCreated}
/>

<!-- Edit Job Dialog -->
<EditJobDialog
  bind:open={showEditDialog}
  job={selectedJob}
  repositories={$repositories}
  on:updated={handleJobUpdated}
/>

<!-- Delete Job Dialog -->
<DeleteJobDialog bind:open={showDeleteDialog} job={selectedJob} on:deleted={handleJobDeleted} />

<style>
  .backup-jobs-page {
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

  .status-badge.idle {
    background: rgba(156, 163, 175, 0.15);
    color: #9ca3af;
  }

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
