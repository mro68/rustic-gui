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

<div class="toolbar">
  <h2 class="section-title">Repositories</h2>
  <button class="btn btn-secondary" on:click={refreshRepos} disabled={loading}>
    {loading ? 'Lädt...' : 'Refresh'}
  </button>
</div>

{#if error}
  <div class="error-message">{error}</div>
{/if}

<div class="dashboard-grid">
  {#each repoList as repo}
    <RepositoryCard {repo} />
  {/each}
</div>

<ActivityLog {logEntries} />

<style>
  /* Styles gemäß Mockup werden in app.css und Komponenten-CSS gepflegt */
</style>
