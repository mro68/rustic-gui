<script lang="ts">
  /**
   * Sidebar-Komponente mit Logo und Navigation.
   *
   * Zeigt das Rustic Logo und die Haupt-Navigation-Items an.
   * Unterstützt Active-State für die aktuelle Seite.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Sidebar activePage="dashboard" />
   * ```
   */

  import { navigateTo } from '$lib/stores/router';

  // Props
  let {
    activePage = 'dashboard',
    mobileOpen = false,
    class: className = '',
  }: {
    activePage?: string;
    mobileOpen?: boolean;
    class?: string;
  } = $props();

  // Navigation-Items (entsprechend Mockup)
  const navItems = [
    { id: 'dashboard', label: 'Dashboard', icon: '📊' },
    { id: 'repositories', label: 'Repositories', icon: '📦' },
    { id: 'snapshots', label: 'Snapshots', icon: '⏱️' },
    { id: 'backup-jobs', label: 'Backup Jobs', icon: '🔄' },
    { id: 'settings', label: 'Settings', icon: '⚙️' },
  ];

  // Navigation Handler
  function handleNavigate(pageId: string) {
    navigateTo(pageId as any);
  }

  // Keyboard-Navigation für Accessibility
  function handleKeyDown(event: any, pageId: string) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      handleNavigate(pageId);
    }
  }
</script>

<div
  class="sidebar {mobileOpen ? 'mobile-open' : ''} {className}"
  role="navigation"
  aria-label="Main navigation"
>
  <!-- Logo Section -->
  <div class="logo">
    <img src="/icon.png" alt="Rustic Logo" class="logo-icon" />
    <div class="logo-text">
      <h1>Rustic</h1>
      <p>Backup Manager</p>
    </div>
  </div>

  <!-- Navigation Items -->
  <nav class="nav">
    {#each navItems as item (item.id)}
      <button
        class="nav-item"
        class:active={activePage === item.id}
        role="menuitem"
        tabindex="0"
        aria-label={item.label}
        aria-current={activePage === item.id ? 'page' : undefined}
        onclick={() => handleNavigate(item.id)}
        onkeydown={(e) => handleKeyDown(e, item.id)}
      >
        <span class="nav-icon" aria-hidden="true">{item.icon}</span>
        <span>{item.label}</span>
      </button>
    {/each}
  </nav>
</div>

<style>
  .sidebar {
    --sidebar-nav-padding-inline-start: 12px;
    --sidebar-nav-padding-inline-end: 12px;
    --sidebar-gradient-extension: calc(
      var(--layout-content-padding-x) - var(--sidebar-nav-padding-inline-end)
    );
    width: 280px;
    background: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    border-right: none;
    flex-shrink: 0;
    position: relative;
    overflow: visible;
    z-index: 2;
  }

  .sidebar::after {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    right: 0;
    width: 1px;
    background: var(--border-color);
    opacity: 1;
    pointer-events: none;
    z-index: 0;
  }

  .logo {
    padding: 24px 20px;
    display: flex;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .logo-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    object-fit: contain;
    background: var(--bg-tertiary);
    padding: 4px;
  }

  .logo-text h1 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .logo-text p {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }

  .nav {
    flex: 1;
    padding: 16px var(--sidebar-nav-padding-inline-end) 16px var(--sidebar-nav-padding-inline-start);
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 8px;
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 14px;
    margin-bottom: 4px;
    transition: all var(--transition-fast);
    border: none;
    background: none;
    width: 100%;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .nav-item.active {
    background-color: var(--color-primary);
    color: white;
    position: relative;
    z-index: 1;
    border-top-left-radius: var(--radius-md);
    border-bottom-left-radius: var(--radius-md);
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .nav-item.active::after {
    content: '';
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: calc(-1 * var(--sidebar-gradient-extension));
    background-image: linear-gradient(90deg, var(--color-primary) 0%, var(--bg-primary) 100%);
    border-top-left-radius: var(--radius-md);
    border-bottom-left-radius: var(--radius-md);
    border-top-right-radius: var(--sidebar-gradient-extension);
    border-bottom-right-radius: var(--sidebar-gradient-extension);
    pointer-events: none;
    z-index: -1;
  }

  .nav-item.active .nav-icon,
  .nav-item.active span {
    position: relative;
    z-index: 1;
  }

  .nav-item:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .nav-icon {
    font-size: 18px;
    width: 20px;
    flex-shrink: 0;
  }

  /* Mobile Styles */
  @media (max-width: 768px) {
    .sidebar {
      --sidebar-gradient-extension: 0px;
      position: fixed;
      left: -280px;
      transition: left 0.3s;
      z-index: 100;
      height: 100vh;
    }

    .sidebar.mobile-open {
      left: 0;
    }
  }
</style>
