<script lang="ts">
  /**
   * Header-Komponente mit Titel und Navigation-Controls.
   *
   * Zeigt den aktuellen Seiten-Titel und Mobile-Menü-Button.
   * Optional: User-Avatar für zukünftige Features.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Header onToggleMobileMenu={() => toggleMenu()} />
   * ```
   */

  import { currentPageTitle } from '$lib/stores/router';

  // Props
  let {
    showUserAvatar = false,
    onToggleMobileMenu,
  }: {
    showUserAvatar?: boolean;
    onToggleMobileMenu?: () => void;
  } = $props();

  // Mobile-Menü Toggle Handler
  function handleToggleMobileMenu() {
    onToggleMobileMenu?.();
  }
</script>

<header class="header">
  <div style="display: flex; align-items: center; gap: 16px;">
    <!-- Mobile Menu Toggle Button -->
    <button
      class="mobile-menu-toggle"
      onclick={handleToggleMobileMenu}
      aria-label="Toggle navigation menu"
      type="button"
    >
      ☰
    </button>

    <!-- Page Title -->
    <h1 id="page-title" class="page-title">{$currentPageTitle}</h1>
  </div>

  <!-- User Avatar (für zukünftige Features) -->
  {#if showUserAvatar}
    <div class="user-avatar" role="button" tabindex="0" aria-label="User profile"></div>
  {/if}
</header>

<style>
  .header {
    padding: 24px 32px;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-primary);
  }

  .page-title {
    font-size: 28px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .user-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: linear-gradient(135deg, #8b5cf6 0%, #6366f1 100%);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .user-avatar:hover {
    transform: scale(1.05);
  }

  .user-avatar:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .mobile-menu-toggle {
    display: none;
    background: none;
    border: none;
    color: var(--text-primary);
    font-size: 24px;
    cursor: pointer;
    padding: 8px;
    border-radius: 4px;
    transition: all var(--transition-fast);
  }

  .mobile-menu-toggle:hover {
    background: var(--bg-tertiary);
  }

  .mobile-menu-toggle:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  /* Mobile Styles */
  @media (max-width: 768px) {
    .header {
      padding: 16px;
    }

    .page-title {
      font-size: 24px;
    }

    .mobile-menu-toggle {
      display: block;
    }
  }
</style>
