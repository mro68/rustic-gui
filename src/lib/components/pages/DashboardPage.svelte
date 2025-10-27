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
  import type { Repository } from '../../types/repository.types';
  import ActivityLog from './ActivityLog.svelte';
  import RepositoryCard from './RepositoryCard.svelte';

  let repoList: Repository[] = [];
  let loading = false;
  let error: string | null = null;
  let logEntries: { time: string; type: 'info' | 'warning' | 'error'; message: string }[] = [
    { time: '12:01', type: 'info', message: 'Backup erfolgreich abgeschlossen' },
    { time: '11:45', type: 'warning', message: 'Warnung: Verbindung langsam' },
    { time: '11:30', type: 'error', message: 'Fehler: Passwort falsch' },
  ];

  async function refreshRepos() {
    loading = true;
    error = null;
    try {
      const repos = await listRepositories();
      setRepositories(repos);
      repoList = repos;
    } catch (e) {
      error = 'Fehler beim Laden der Repositories';
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    refreshRepos();
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
