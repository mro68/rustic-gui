<!--
  RepositoryCard.svelte
  --------------------
  Einzelne Repository-Karte für das Dashboard (gemäß Mockup)
-->

<script lang="ts">
  import type { Repository } from '../../types/repository.types';
  export let repo: Repository;

  // Status-Badge dynamisch (Platzhalter-Logik)
  let status: 'healthy' | 'warning' | 'error' = 'healthy';
  let statusText = status === 'healthy' ? 'Healthy' : status === 'warning' ? 'Warning' : 'Error';

  function handleBackup() {
    // TODO: Backup-Job starten
    console.log(`Backup für ${repo.name} gestartet (Platzhalter)`);
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
      <div class="repo-stat-value">{repo.snapshotCount ?? '-'}</div>
    </div>
    <div class="repo-stat">
      <div class="repo-stat-label">Total Size</div>
      <div class="repo-stat-value">{repo.size ?? '-'}</div>
    </div>
  </div>
  <div class="card-actions">
    <button class="btn btn-primary" on:click={handleBackup}>Backup</button>
    <button class="btn btn-secondary" on:click={handleBrowse}>Browse</button>
    <span class="menu-dots">&#8942;</span>
  </div>
</div>

<style>
  /* Styles gemäß Mockup werden in app.css und Komponenten-CSS gepflegt */
</style>
