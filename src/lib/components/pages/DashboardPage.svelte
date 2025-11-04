<!--
  DashboardPage.svelte
  -------------------
  TODO.md: Phase 2 - Daten-Initialisierung (Stores & Pages) ✅ KOMPLETT
  Referenz: TODO.md Zeile 215-221, 342
  
  Status:
  - refreshRepos in onMount implementiert ✅ (TODO.md Zeile 216)
  - Lädt Repositories via api.listRepositories ✅
  - Füllt $repositories Store ✅
  
  Backend-Commands:
  - list_repositories: src-tauri/src/commands/repository.rs:7
  
  API-Wrapper: src/lib/api/repositories.ts:listRepositories
  Store: src/lib/stores/repositories.ts
  
  Features:
  - Dashboard-Ansicht gemäß Mockup (docs/mockups/rustic_gui_mockup.html)
  - Grid mit Repository-Cards
  - Activity Log
  - Storage Usage Charts (optional)
  - Integration mit Stores
  - Pixel-perfect laut Mockup
  
  TODO:
  - Zeile 81: Dialog-Integration für Repository-Verwaltung
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import { listRepositories } from '../../api/repositories';
  import { setRepositories, repositories } from '../../stores/repositories';
  import type { RepositoryDto } from '../../types';
  import AddRepositoryDialog from '../dialogs/AddRepositoryDialog.svelte';
  import ActivityLog from './ActivityLog.svelte';
  import RepositoryCard from './RepositoryCard.svelte';
  import StorageChart from './StorageChart.svelte';
  // Test Tauri API (Tauri 2.x: ES-Module-Import)
  import Tooltip from '$lib/components/shared/Tooltip.svelte';
  import { getVersion } from '@tauri-apps/api/app';
  // State-Variablen
  let loading = $state(false);
  let error: string | null = $state(null);
  type LogEntry = { time: string; type: 'error' | 'warning' | 'info'; message: string };
  let logEntries: LogEntry[] = $state([]);
  let showAddRepoDialog = $state(false);

  // Reaktive Repository-Liste aus Store
  let repoList: RepositoryDto[] = $derived($repositories);

  // Lädt die Repository-Liste
  async function refreshRepos() {
    loading = true;
    error = null;
    try {
      const repos = await listRepositories();
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

<div class="dashboard-page">
  <div class="page-wrapper">
    <!-- Page Header -->
    <div class="page-header">
      <h1 class="page-title">Dashboard</h1>
      <div class="header-actions">
        <Tooltip text="Repositories neu laden">
          <button
            class="btn btn-secondary"
            aria-label="Repositories neu laden"
            onclick={refreshRepos}
            disabled={loading}
          >
            <span class="btn-icon">{loading ? '⏳' : '🔄'}</span>
            <span class="btn-text">{loading ? 'Lädt...' : 'Refresh'}</span>
          </button>
        </Tooltip>
      </div>
    </div>

    <div class="section-title">Repositories</div>

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
          <StorageChart
            totalSpace={100}
            usedSpace={30}
            label="Total Space"
            sublabel="30 of 100 GB"
          />
          <StorageChart
            totalSpace={60}
            usedSpace={30}
            label="Available Space"
            sublabel="30 of 60 GB"
          />
        </div>
      </div>
    </div>

    <div class="section-title">Recent Activity</div>
    <ActivityLog {logEntries} />

    <AddRepositoryDialog bind:open={showAddRepoDialog} mode="open" onCreated={refreshRepos} />
  </div>
</div>

<style>
  .dashboard-page {
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
    max-width: 1600px;
    padding: 0 1rem;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 1.5rem 0 1rem 0;
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
