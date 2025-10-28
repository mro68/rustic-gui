<!--
  RepositoryCard.svelte
  --------------------
  Einzelne Repository-Karte für das Dashboard (gemäß Mockup)
-->

<script lang="ts">
  import { get } from 'svelte/store';
  import { cancelBackup, runBackup } from '../../api/backup';
  import { jobs } from '../../stores/backup-jobs';
  import type { RepositoryDto } from '../../types';
  import RunBackupDialog from '../dialogs/RunBackupDialog.svelte';

  export let repo: RepositoryDto;

  // Status-Badge dynamisch (Platzhalter-Logik)
  let status: 'healthy' | 'warning' | 'error' = 'healthy';
  let statusText = status === 'healthy' ? 'Healthy' : status === 'warning' ? 'Warning' : 'Error';

  // Backup-Dialog State
  let showBackupDialog = false;
  let currentJobId = '';
  let currentJobName = '';

  function handleBackup() {
    // Finde einen Backup-Job für dieses Repository
    const allJobs = get(jobs);
    const repoJob = allJobs.find((job) => job.repositoryId === repo.id);

    if (!repoJob) {
      // TODO: Dialog öffnen um Job zu erstellen
      console.warn(`Kein Backup-Job für Repository ${repo.name} gefunden`);
      return;
    }

    // Job gefunden - Backup starten
    currentJobId = repoJob.id;
    currentJobName = repoJob.name;
    showBackupDialog = true;

    // Backup starten
    runBackup(repoJob.id).catch((error) => {
      console.error('Backup-Fehler:', error);
      showBackupDialog = false;
    });
  }

  function handleCancelBackup() {
    if (currentJobId) {
      cancelBackup(currentJobId).catch((error) => {
        console.error('Cancel-Backup-Fehler:', error);
      });
    }
  }

  function handleBrowse() {
    // TODO: Repository-Browser öffnen
    console.log(`Browse für ${repo.name} geöffnet (Platzhalter)`);
  }
</script>

<div class="card">
  <div class="card-header">
    <div>
      <div class="card-title">{repo.name}</div>
      <div class="card-subtitle">{repo.path}</div>
    </div>
    <span class={`status-badge status-${status}`}>{statusText}</span>
  </div>
  <div class="repo-info">
    <div class="repo-stat">
      <div class="repo-stat-label">Snapshots</div>
      <div class="repo-stat-value">{repo.snapshot_count ?? '-'}</div>
    </div>
    <div class="repo-stat">
      <div class="repo-stat-label">Total Size</div>
      <div class="repo-stat-value">{repo.total_size ?? '-'}</div>
    </div>
  </div>
  <div class="card-actions">
    <button class="btn btn-primary" on:click={handleBackup}>Backup</button>
    <button class="btn btn-secondary" on:click={handleBrowse}>Browse</button>
    <span class="menu-dots">&#8942;</span>
  </div>
</div>

<!-- Backup-Dialog -->
<RunBackupDialog
  bind:open={showBackupDialog}
  jobName={currentJobName}
  jobId={currentJobId}
  onCancel={handleCancelBackup}
/>

<style>
  /* Styles gemäß Mockup werden in app.css und Komponenten-CSS gepflegt */
</style>
