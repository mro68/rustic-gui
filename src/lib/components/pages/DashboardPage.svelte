<!--
  DashboardPage.svelte
  -------------------
  Dashboard-Ansicht gemäß Mockup (docs/mockups/rustic_gui_mockup.html)
  - Grid mit Repository-Cards
  - Activity Log
  - Storage Usage Charts (optional)
  - Integration mit Stores
  - Pixel-perfect laut Mockup
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { listRepositories } from '../../api/repositories';
  import { setRepositories } from '../../stores/repositories';
  import type { RepositoryDto } from '../../types';
  import ActivityLog from './ActivityLog.svelte';
  import RepositoryCard from './RepositoryCard.svelte';
  import StorageChart from './StorageChart.svelte';
  // Test Tauri API (Tauri 2.x: ES-Module-Import)
  import { getVersion } from '@tauri-apps/api/app';
  // State-Variablen
  let loading = $state(false);
  let repoList: RepositoryDto[] = $state([]);
  let error: string | null = $state(null);
  type LogEntry = { time: string; type: 'error' | 'warning' | 'info'; message: string };
  let logEntries: LogEntry[] = $state([]);

  // Lädt die Repository-Liste
  async function refreshRepos() {
    loading = true;
    error = null;
    try {
      const repos = await listRepositories();
      repoList = repos;
      setRepositories(repos);
      logEntries = [
        {
          time: new Date().toLocaleTimeString(),
          type: 'info',
          message: `Repositories geladen: ${repos.length} gefunden`,
        },
        ...logEntries,
      ];
    } catch (e) {
      error = 'Fehler beim Laden der Repositories.';
      logEntries = [
        {
          time: new Date().toLocaleTimeString(),
          type: 'error',
          message: `Fehler beim Laden: ${(e as Error).message || e}`,
        },
        ...logEntries,
      ];
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await refreshRepos();
    try {
      const version = await getVersion();
      console.log('Tauri API funktioniert! App-Version:', version);
    } catch (e) {
      console.warn('Tauri API nicht verfügbar:', e);
    }
  });
</script>

<div class="toolbar dashboard-toolbar" role="region" aria-label="Repository Aktionen">
  <div class="section-title">Repositories</div>
  <div class="toolbar-actions">
    <button
      class="btn btn-primary"
      aria-label="Repository öffnen"
      title="Neues Repository öffnen"
      onclick={() => {
        /* TODO: Dialog öffnen */
      }}
    >
      <span class="btn-icon" aria-hidden="true">➕</span>
      <span class="btn-text">Repository öffnen</span>
    </button>
    <button
      class="btn btn-secondary"
      aria-label="Repositories neu laden"
      title="Repositories neu laden"
      onclick={refreshRepos}
      disabled={loading}
    >
      <span class="btn-icon" aria-hidden="true">{loading ? '⏳' : '🔄'}</span>
      <span class="btn-text">{loading ? 'Lädt...' : 'Refresh'}</span>
    </button>
  </div>
</div>

{#if error}
  <div class="error-message">{error}</div>
{/if}

<div class="dashboard-grid">
  {#each repoList as repo}
    <RepositoryCard {repo} />
  {/each}
</div>

<div class="section-title">Storage Usage</div>
<div class="dashboard-grid">
  <div class="card">
    <div class="chart-container">
      <StorageChart totalSpace={100} usedSpace={30} label="Total Space" sublabel="30 of 100 GB" />
      <StorageChart totalSpace={60} usedSpace={30} label="Available Space" sublabel="30 of 60 GB" />
    </div>
  </div>
</div>

<div class="section-title">Recent Activity</div>
<ActivityLog {logEntries} />

<style>
  .dashboard-toolbar {
    position: sticky;
    top: 0;
    z-index: 10;
    background: var(--bg-primary, #1a1d2e);
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding: 0 0 12px 0;
    border-bottom: 1px solid var(--border-color, #2d3348);
  }
  .toolbar-actions {
    display: flex;
    gap: 12px;
    align-items: center;
  }
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 6px;
    border: none;
    font-size: 13px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s;
  }
  .btn-primary {
    background: var(--color-primary, #3b82f6);
    color: white;
  }
  .btn-primary:hover {
    background: var(--color-primary-dark, #2563eb);
  }
  .btn-secondary {
    background: var(--bg-secondary, #2d3348);
    color: var(--text-primary, #e4e4e7);
    border: 1px solid #3e4457;
  }
  .btn-secondary:hover {
    background: #3e4457;
  }
  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  .btn-icon {
    font-size: 16px;
    display: flex;
    align-items: center;
  }
  .btn-text {
    display: inline;
  }
  @media (max-width: 768px) {
    .dashboard-toolbar {
      flex-direction: column;
      align-items: stretch;
      padding: 0 0 8px 0;
    }
    .toolbar-actions {
      margin-top: 8px;
      justify-content: flex-start;
    }
  }
  @media (max-width: 1024px) {
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }
  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 20px;
    margin-bottom: 32px;
  }
  .chart-container {
    display: flex;
    justify-content: space-around;
    align-items: center;
    margin-top: 20px;
  }
  .section-title {
    font-size: 20px;
    font-weight: 600;
    margin-bottom: 20px;
    color: #f4f4f5;
  }
</style>
