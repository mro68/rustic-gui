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
      <div class="section-title">Repository-Management</div>
      <div class="card">
        <p>Hier werden die Repository-Verwaltungsfunktionen implementiert.</p>
        <p>Features: Repository hinzufügen, öffnen, prüfen, konfigurieren.</p>
      </div>
    {:else if page === 'snapshots'}
      <Snapshots />
    {:else if page === 'backup-jobs'}
      <BackupJobs />
    {:else if page === 'settings'}
      <div class="section-title">Einstellungen</div>
      <div class="card">
        <p>Hier werden Anwendungseinstellungen konfiguriert.</p>
        <p>Features: Theme, Sprache, Sicherheit, About.</p>
      </div>
    {/if}
  {/snippet}
</MainLayout>
