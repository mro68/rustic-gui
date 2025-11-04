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
  import ToastContainer from '$lib/components/shared/ToastContainer.svelte';
  import PortableNotice from '$lib/components/shared/PortableNotice.svelte';
  import { getPortableStoreStatus } from '$lib/api/system';
  import { onPortableStoreStatus } from '$lib/api/events';
  import { currentPage } from '$lib/stores/router';
  import { setPortableStatus } from '$lib/stores/system';
  import { toastStore } from '$lib/stores/toast';
  import type { PortableStoreStatus } from '$lib/types';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  // Props für responsive Verhalten (keine mehr benötigt)
  let { children }: { children: any } = $props();

  // State für mobile Navigation
  let isMobileMenuOpen = $state(false);
  let portableListener: UnlistenFn | null = null;
  let fallbackToastShown = $state(false);
  let noticeDismissed = $state(false);
  let showPortableNotice = $state(false);
  let activePortableStatus = $state<PortableStoreStatus | null>(null);

  // Funktion zum Umschalten des mobilen Menüs
  function toggleMobileMenu() {
    isMobileMenuOpen = !isMobileMenuOpen;
  }

  function handlePortableStatus(status: PortableStoreStatus) {
    setPortableStatus(status);
    activePortableStatus = status;

    const requiresNotice = status.fallback_used || status.read_only;

    if (requiresNotice) {
      showPortableNotice = !noticeDismissed;

      if (status.fallback_used && !fallbackToastShown) {
        fallbackToastShown = true;
        toastStore.warning(
          `Portabler Speicher schreibgeschützt. Fallback aktiv: ${status.effective_dir}`,
          9000
        );
      }
    } else {
      fallbackToastShown = false;
      showPortableNotice = false;
      noticeDismissed = false;
    }
  }

  function handlePortableNoticeDismiss() {
    noticeDismissed = true;
    showPortableNotice = false;
  }

  onMount(() => {
    async function initPortableStatus() {
      try {
        const status = await getPortableStoreStatus();
        handlePortableStatus(status);
      } catch (error) {
        console.error('Portable-Status konnte nicht geladen werden', error);
      }

      try {
        portableListener = await onPortableStoreStatus(handlePortableStatus);
      } catch (error) {
        console.error('Portable-Status-Listener konnte nicht registriert werden', error);
      }
    }

    initPortableStatus();

    return () => {
      portableListener?.();
      portableListener = null;
    };
  });
</script>

<div class="app">
  <ToastContainer />
  <!-- Sidebar Navigation -->
  <Sidebar activePage={$currentPage} mobileOpen={isMobileMenuOpen} />

  <!-- Haupt-Content-Bereich -->
  <div class="main">
    {#if showPortableNotice && activePortableStatus}
      <div class="main__banner">
        <PortableNotice status={activePortableStatus} on:dismiss={handlePortableNoticeDismiss} />
      </div>
    {/if}

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

  .main__banner {
    padding: 24px 32px 0;
  }

  .main__banner :global(.portable-notice) {
    max-width: 720px;
  }

  @media (max-width: 960px) {
    .main__banner {
      padding: 16px;
    }
  }
</style>
