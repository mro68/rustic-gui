<script lang="ts">
  /**
   * Haupt-Layout-Komponente für die gesamte Anwendung.
   *
   * Strukturiert die App in Sidebar (Navigation) und Haupt-Content-Bereich.
   * Beinhaltet responsive Design für mobile Geräte.
   *
   * @component
   *
   * @example
   * ```svelte
   * <MainLayout>
   *   <h1>Dashboard</h1>
   *   <p>Content goes here...</p>
   * </MainLayout>
   * ```
   */

  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import { currentPage } from '$lib/stores/router';

  // Props für responsive Verhalten (keine mehr benötigt)
  let { children }: { children: any } = $props();

  // State für mobile Navigation
  let isMobileMenuOpen = $state(false);

  // Funktion zum Umschalten des mobilen Menüs
  function toggleMobileMenu() {
    isMobileMenuOpen = !isMobileMenuOpen;
  }
</script>

<div class="app">
  <!-- Sidebar Navigation -->
  <Sidebar
    activePage={$currentPage}
    mobileOpen={isMobileMenuOpen}
  />

  <!-- Haupt-Content-Bereich -->
  <div class="main">
    <!-- Header mit Titel und Mobile-Menü-Button -->
    <Header
      onToggleMobileMenu={toggleMobileMenu}
    />

    <!-- Content-Bereich für Seiten-Inhalte -->
    <div class="content">
      {@render children()}
    </div>
  </div>
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    overflow: hidden;
  }
</style>