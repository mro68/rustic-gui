<script lang="ts">
  /**
   * Haupt-Seite der Anwendung.
   *
   * Verwendet das MainLayout und zeigt verschiedene Seiten basierend auf der Router-Navigation.
   */

  import MainLayout from '$lib/components/layout/MainLayout.svelte';
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
      <div class="section-title">Dashboard</div>
      <div class="dashboard-grid">
        <div class="card">
          <div class="card-header">
            <div>
              <div class="card-title">Willkommen bei Rustic GUI</div>
              <div class="card-subtitle">Backup-Management-Anwendung</div>
            </div>
          </div>
          <p>Das Layout-System ist erfolgreich implementiert! 🎉</p>
          <p>Nächste Schritte:</p>
          <ul>
            <li>✅ CSS-Grundlage aus Mockup übernommen</li>
            <li>✅ MainLayout, Sidebar, Header erstellt</li>
            <li>✅ Router-Store implementiert</li>
            <li>🔄 Shared Components (Buttons, etc.)</li>
            <li>🔄 Stores & API-Wrapper</li>
            <li>🔄 Dashboard-Page mit echten Daten</li>
          </ul>
        </div>
      </div>
    {:else if page === 'repositories'}
      <div class="section-title">Repository-Management</div>
      <div class="card">
        <p>Hier werden die Repository-Verwaltungsfunktionen implementiert.</p>
        <p>Features: Repository hinzufügen, öffnen, prüfen, konfigurieren.</p>
      </div>
    {:else if page === 'snapshots'}
      <div class="section-title">Snapshots</div>
      <div class="card">
        <p>Hier werden alle Snapshots angezeigt und verwaltet.</p>
        <p>Features: Liste, Details, Vergleich, Löschen.</p>
      </div>
    {:else if page === 'backup-jobs'}
      <div class="section-title">Backup Jobs</div>
      <div class="card">
        <p>Hier werden Backup-Jobs erstellt und überwacht.</p>
        <p>Features: Job-Erstellung, Scheduling, Monitoring.</p>
      </div>
    {:else if page === 'settings'}
      <div class="section-title">Einstellungen</div>
      <div class="card">
        <p>Hier werden Anwendungseinstellungen konfiguriert.</p>
        <p>Features: Theme, Sprache, Sicherheit, About.</p>
      </div>
    {/if}
  {/snippet}
</MainLayout>
