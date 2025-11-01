<script lang="ts">
  import '../app.css';
  /**
   * Haupt-Seite der Anwendung.
   *
   * Verwendet das MainLayout und zeigt verschiedene Seiten basierend auf der Router-Navigation.
   */

  import MainLayout from '$lib/components/layout/MainLayout.svelte';
  import BackupJobs from '$lib/components/pages/BackupJobs.svelte';
  import DashboardPage from '$lib/components/pages/DashboardPage.svelte';
  import Repositories from '$lib/components/pages/Repositories.svelte';
  import Settings from '$lib/components/pages/Settings.svelte';
  import Snapshots from '$lib/components/pages/Snapshots.svelte';
  import { currentPage } from '$lib/stores/router';

  // Aktuelle Seite aus Store
  let page = $state('dashboard');

  // Subscribe to router changes
  $effect(() => {
    const unsubscribe = currentPage.subscribe((newPage) => {
      page = newPage;
    });
    return unsubscribe;
  });
</script>

<MainLayout>
  {#snippet children()}
    {#if page === 'dashboard'}
      <DashboardPage />
    {:else if page === 'repositories'}
      <Repositories />
    {:else if page === 'snapshots'}
      <Snapshots />
    {:else if page === 'backup-jobs'}
      <BackupJobs />
    {:else if page === 'settings'}
      <Settings />
    {/if}
  {/snippet}
</MainLayout>
